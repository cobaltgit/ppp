use anyhow::{Result, anyhow};

use std::{env, fs};
use std::io::Write;
use std::process::{Command, Stdio};

use crate::powerprofile::PowerProfile;

pub struct Menu {
	name: String,
	args: Vec<String>,
	use_index: bool,
}

impl Menu {
	pub fn new(cli_args: &str, use_index: bool) -> Self {
		let shlexed_args = shlex::split(cli_args).unwrap_or_default();
		Self {
			name: shlexed_args.get(0).cloned().unwrap_or_default(),
			args: shlexed_args.get(1..).unwrap_or(&[]).to_vec(),
			use_index: use_index
		}
	}

	pub fn name(&self) -> &str {
		&self.name
	}

	pub fn get_profile(&self, additional_args: Option<&str>) -> Result<PowerProfile> {
		let profiles = PowerProfile::all();
		let mut proc = Command::new(&self.name);
		for arg in &self.args {
			proc.arg(&arg);
		}

		if let Some(more_args) = additional_args {
			if let Some(shlexed_more_args) = shlex::split(&more_args) {
				for arg in shlexed_more_args {
					proc.arg(&arg);
				}
			}
		}

		let mut child = proc.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn()?;

		let stdin = child.stdin.as_mut().unwrap();
		let input = profiles.iter()
			.map(|p| p.entry())
			.fold(String::new(), |a, b| a + b + "\n");
		let _ = stdin.write_all(input.as_bytes());

		let output = child.wait_with_output()?;

		let profile = if self.use_index {
			let index = String::from_utf8(output.stdout)?
				.trim()
				.parse::<usize>()?;
			profiles[index]
		} else {
			let selected_entry = String::from_utf8(output.stdout)?
				.trim()
				.to_string();

			*profiles.iter()
				.find(|p| p.entry() == selected_entry)
				.ok_or_else(|| anyhow!("Selected entry not found"))?
		};

		Ok(profile)
	}

	pub fn is_installed(&self) -> bool {
		if let Ok(path) = env::var("PATH") {
			for p in env::split_paths(&path) {
				let fullpath = p.join(self.name.clone());
				if fs::metadata(&fullpath).is_ok() {
					return true
				}
			}
		}
		false
	}
}
