use anyhow::Result;
use argh::FromArgs;
use notify_rust::Notification;
use zbus::blocking::Connection;

mod menu;
use menu::Menu;

mod powerprofile;
use powerprofile::PowerProfile;

#[derive(FromArgs)]
/// Power profile selector for power-profiles-daemon (or tlp-pd) using a dmenu-compatible launcher
struct PPMenuArgs {
    /// launcher to use (supported launchers are fuzzel, rofi, walker, dmenu, bemenu, wmenu, wofi, tofi)
    #[argh(option, short = 'l')]
    launcher: String,

    /// custom arguments to pass to the launcher
    #[argh(option, short = 'a')]
    launcher_args: Option<String>,
}


fn main() -> Result<()> {
    let connection = Connection::system()?;
    let current_profile = PowerProfile::get_active(&connection)?;
    let placeholder = format!("Current profile: {}", current_profile.name());
    
    let menus: Vec<Menu> = vec![
        Menu::new(&format!("fuzzel --dmenu --index --placeholder \"{}\"", placeholder), true),
        Menu::new(&format!("rofi -dmenu -i -p \"{}\"", placeholder), true),
        Menu::new(&format!("walker -d -i -p \"{}\"", placeholder), true),
        Menu::new(&format!("dmenu -p \"{}\"", placeholder), false),
        Menu::new(&format!("bemenu -p \"{}\"", placeholder), false),
        Menu::new(&format!("wmenu -p \"{}\"", placeholder), false),
        Menu::new(&format!("wofi --show=dmenu -p \"{}\"", placeholder), false),
        Menu::new(&format!("tofi -p \"{}\"", placeholder), false),
    ];

    let args: PPMenuArgs = argh::from_env();

    let all_menu_names: Vec<&str> = menus.iter().map(|m| m.name()).collect();
    if !all_menu_names.contains(&args.launcher.as_str()) {
        anyhow::bail!(
            "Invalid launcher '{}'. Must be one of: {}",
            args.launcher,
            all_menu_names.join(", ")
        )
    }

    let menu = menus.iter().find(|m| m.name() == args.launcher).expect("Could not find menu");
    if !menu.is_installed() {
        anyhow::bail!("Menu '{}' not installed!", args.launcher)
    }
    
    let new_profile = menu.get_profile(args.launcher_args.as_deref())?;

    if new_profile == current_profile {
        Notification::new()
            .summary("Power Profile Menu")
            .body(&format!("Power profile is already set to {}", current_profile.name()))
            .show()?;
        return Ok(());
    }

    match new_profile.apply(&connection) {
        Ok(()) => {
            Notification::new()
                .summary("Power Profile Menu")
                .body(&format!("Power profile set to {}", new_profile.name()))
                .show()?;
        },
        Err(e) => {
            Notification::new()
                .summary("Power Profile Menu")
                .body(&format!("Unable to set power profile: {:?}", e))
                .show()?;
        }
    }
     
    Ok(())
}
