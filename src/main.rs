use dialoguer::{Confirm, MultiSelect};
use owo_colors::OwoColorize;
use serde_json::Value;
use std::env;
use std::path::Path;
use std::process::Command;

struct SimCleanerCLI {
    devices: Vec<Device>,
    victims: Vec<usize>,
}

#[derive(Debug)]
struct Device {
    name: String,
    state: String,
    udid: String,
    device_type: String,
    runtime: String,
}

impl SimCleanerCLI {
    fn new() -> Self {
        let mut cli = SimCleanerCLI {
            devices: Vec::new(),
            victims: Vec::new(),
        };
        cli.navigate_to_device_directory();
        cli.get_device_data();
        cli.get_target_devices();
        println!("\nDeleting device data:");
        cli.delete_device_data();
        if cli.recreate_devices_prompt() {
            println!("Recreating following devices:");
            cli.recreate_devices();
        }
        println!("Device data cleared. \nNew devices state:\n");
        cli.get_device_data();
        cli.list_devices();
        cli
    }

    fn navigate_to_device_directory(&self) {
        env::set_current_dir(
            dirs::home_dir()
                .unwrap()
                .join("Library/Developer/CoreSimulator/Devices/"),
        )
        .unwrap();
    }

    fn get_device_data(&mut self) {
        let output = Command::new("xcrun")
            .arg("simctl")
            .arg("list")
            .arg("devices")
            .arg("--json")
            .output()
            .expect("Failed to execute command");
        let devices_json: Value = serde_json::from_slice(&output.stdout).unwrap();
        self.devices = devices_json["devices"]
            .as_object()
            .unwrap()
            .iter()
            .flat_map(|(runtime, device_list)| {
                device_list
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(move |device| Device {
                        name: device["name"].as_str().unwrap().to_string(),
                        state: device["state"].as_str().unwrap().to_string(),
                        udid: device["udid"].as_str().unwrap().to_string(),
                        device_type: device["deviceTypeIdentifier"].as_str().unwrap().to_string(),
                        runtime: runtime.clone(),
                    })
            })
            .collect();
    }

    fn parse_runtime(&self, device_index: usize) -> String {
        self.devices[device_index]
            .runtime
            .split('.')
            .last()
            .unwrap()
            .to_string()
    }

    fn get_target_devices(&mut self) {
        let choices: Vec<(String, usize)> = self
            .devices
            .iter()
            .enumerate()
            .map(|(index, device)| {
                let size = self.get_device_image_size(&device.udid);
                let runtime = self.parse_runtime(index);
                (
                    format!(
                        "{:<30} {:<15} {:<10} {}",
                        device.name, runtime, device.state, size
                    ),
                    index,
                )
            })
            .collect();

        let selections = MultiSelect::new()
            .with_prompt(
                "[↑↓] navigate, [space] un/select, [enter] submit, [q] quit"
                    .blue()
                    .to_string(),
            )
            .items(
                &choices
                    .iter()
                    .map(|(name, _)| name.as_str())
                    .collect::<Vec<_>>(),
            )
            .interact_opt()
            .unwrap();

        match selections {
            Some(sel) => {
                self.victims = sel.iter().map(|&i| choices[i].1).collect();
                if self.victims.is_empty() {
                    std::process::exit(0);
                }
            }
            None => {
                // Pressed `q`/`esc` - built in to interact_opt()
                std::process::exit(0);
            }
        }
    }

    fn recreate_devices_prompt(&self) -> bool {
        Confirm::new()
            .with_prompt("Would you like to recreate these devices?\nThis will restore them to their default state.")
            .interact()
            .unwrap()
    }

    fn recreate_devices(&self) {
        self.navigate_to_device_directory();
        for &index in &self.victims {
            println!(
                "  {} [{}]...",
                self.devices[index].name,
                self.parse_runtime(index)
            );
            Command::new("xcrun")
                .arg("simctl")
                .arg("create")
                .arg(&self.devices[index].name)
                .arg(&self.devices[index].device_type)
                .arg(&self.devices[index].runtime)
                .output()
                .expect("Failed to execute command");
        }
    }

    fn delete_device_data(&self) {
        for &index in &self.victims {
            println!(
                "  {} [{}]...",
                self.devices[index].name,
                self.parse_runtime(index)
            );
            Command::new("xcrun")
                .arg("simctl")
                .arg("delete")
                .arg(&self.devices[index].udid)
                .output()
                .expect("Failed to execute command");
        }
    }

    fn get_device_image_size(&self, udid: &str) -> String {
        if Path::new(udid).exists() {
            let output = Command::new("du")
                .arg("-sh")
                .arg(udid)
                .arg("|")
                .arg("cut")
                .arg("-f1")
                .output()
                .expect("Failed to execute command");
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            "Directory not found".to_string()
        }
    }

    fn list_devices(&self) {
        for (index, device) in self.devices.iter().enumerate() {
            let size = self.get_device_image_size(&device.udid);
            let runtime = self.parse_runtime(index);
            println!("{:<30} {:<10} {}", device.name, runtime, size);
        }
    }
}

fn main() {
    SimCleanerCLI::new();
}
