use anyhow::{Result, anyhow};

use std::io::Write;
use std::process::{Command, Stdio};

use crate::powerprofile::PowerProfile;

pub struct Menu {
	pub name: String,
	pub args: String,
	pub use_index: bool,
}

impl Menu {
	pub fn get_profile(&self, additional_args: Option<&str>) -> Result<PowerProfile> {
		let profiles = PowerProfile::all();
		let mut proc = Command::new(&self.name);
		if let Some(shlexed_args) = shlex::split(&self.args) {
			for arg in shlexed_args {
				proc.arg(&arg);
			}
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
}
