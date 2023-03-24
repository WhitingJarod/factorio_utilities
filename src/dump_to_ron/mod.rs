use serde_json::Value;
use std::{ffi::OsString, fs, io::ErrorKind, process::Command};
use steamlocate::SteamDir;

mod belt;
mod crafter;
mod inserter;
mod item;
mod parsing;
mod recipe;

use crate::common::factorio::belt::Belt;
use crate::common::factorio::crafter::Crafter;
use crate::common::factorio::inserter::Inserter;
use crate::common::factorio::item::Item;
use crate::common::factorio::recipe::Recipe;

fn run_with_arg(arg: &str) -> Result<String, String> {
    let steam_dir = SteamDir::locate();
    if let Some(mut steam_dir) = steam_dir {
        let app_dir = steam_dir.app(&427520);
        if let Some(app_dir) = app_dir {
            let mut path = app_dir.path.clone();
            #[cfg(target_os = "windows")]
            path.push("bin/x64/factorio.exe");
            #[cfg(target_os = "linux")]
            path.push("bin/x64/factorio");
            let result = Command::new(path).arg(arg).output();
            match result {
                Ok(result) => {
                    let output = String::from_utf8(result.stdout);
                    if let Ok(output) = output {
                        return Ok(output);
                    } else {
                        return Err("unable to parse Factorio program output".to_string());
                    }
                }
                Err(message) => {
                    return Err(message.to_string());
                }
            }
        } else {
            return Err("unable to locate Factorio directory".to_string());
        }
    } else {
        return Err("unable to locate Steam install directory".to_string());
    }
}

//TODO: Get the actual directory in case a different location is listed in config.ini
fn get_user_data_dir() -> Result<OsString, String> {
    #[cfg(target_os = "windows")]
    {
        let result = std::env::var("APPDATA");
        if let Ok(path) = result {
            let mut path = OsString::from(path);
            path.push("/Factorio/");
            return Ok(path);
        } else {
            return Err(Log::Error("unable to get APPDATA as a system variable"));
            return OsString::from("");
        }
    }
    #[cfg(target_os = "linux")]
    return Ok(OsString::from("~/.factorio/"));
    #[cfg(target_os = "macos")]
    return Ok(OsString::from("~/Library/Application Support/factorio/"));
}

fn read_json(dir: &OsString) -> Result<Value, String> {
    let result = fs::read_to_string(dir);
    if let Ok(file) = result {
        let json = serde_json::from_str(&file);
        if let Ok(json) = json {
            return Ok(json);
        } else {
            return Err("error not implemented yet".to_string());
        }
    } else {
        return Err("error not implemented yet 2".to_string());
    }
}

pub fn check_for_updates() {
    todo!();
}

pub fn update() -> Result<(), String> {
    let script_output = get_user_data_dir();
    if let Err(message) = script_output {
        return Err(format!("get_user_data_dir returned with {}", message));
    }

    let mut script_output = script_output.unwrap();

    let mut script_output_backup = script_output.clone();
    script_output.push("script-output/");
    script_output_backup.push("script-output-backup/");

    let result = fs::rename(&script_output, &script_output_backup);
    let existed;
    match result {
        Ok(_) => existed = true,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                existed = false;
            }
            ErrorKind::PermissionDenied => {
                todo!("encourage user to run with elevated permissions.")
            }
            _ => todo!("return error type"),
        },
    }

    let result = fs::create_dir(&script_output);
    if let Err(e) = result {
        panic!("{}", e);
    }

    //TODO: Display a Error to the user about Steam's security popup for external programs trying to run with command line args.
    //TODO: Explain why.
    let result = run_with_arg("--dump-data");
    if let Err(error) = result {
        //error.write_to_log();
    }
    //TODO: handle result. Check if successful.
    let result = run_with_arg("--dump-icon-sprites");
    //TODO: handle result. Check if successful.
    let result = run_with_arg("--dump-prototype-locale");
    //TODO: handle result. Check if successful.

    let mut data = script_output.clone();
    data.push("data-raw-dump.json");
    let result = read_json(&data);
    //TODO: Do stuff with the information.

    let result = fs::remove_dir(&script_output);
    //TODO: handle result.
    if existed {
        let result = fs::rename(&script_output_backup, &script_output);
        //TODO: handle result.
    }

    unimplemented!()
}
