use std::{process::Command, ffi::OsString, fs, io::ErrorKind};

use serde_json;
use steamlocate::SteamDir;
use crate::{common::log::{LogResult, LogType}, log_info};

fn run_with_arg(arg: &str) -> LogResult<String> {
    let steam_dir = SteamDir::locate();
    if let Some(mut steam_dir) = steam_dir {
        let app_dir = steam_dir.app(&427520);
        if let Some(app_dir) = app_dir {
            let mut path = app_dir.path.clone();
            #[cfg(target_os = "windows")]
            path.push("bin/x86/factorio.exe");
            #[cfg(target_os = "linux")]
            path.push("bin/x86/factorio");
            let result = Command::new(path).arg(arg).output();
            if let Ok(result) = result {
                let output = String::from_utf8(result.stdout);
                if let Ok(output) = output {
                    return Ok(output);
                }
            }
        }
    }
    Err(LogType::Error("run_with_arg failure".to_string()))
}

//TODO: Get the actual directory in case a different location is listed in config.ini
fn get_user_data_dir() -> OsString {
    #[cfg(target_os = "windows")]
    let path = OsString::from("%appdata%/Factorio/");
    #[cfg(target_os = "linux")]
    let path = OsString::from("~/.factorio/");
    #[cfg(target_os = "macos")]
    let path = OsString::from("~/Library/Application Support/factorio/");
    path
}

pub fn check_for_updates() {
    todo!();
}

pub fn update() {
    let mut script_output = get_user_data_dir();
    let mut script_output_backup = script_output.clone();
    script_output.push("script-output");
    script_output_backup.push("script-output-backup");

    log_info!("Temporarily renaming script-output to script-output-backup");
    let result = fs::rename(&script_output, &script_output_backup);
    let existed;
    match result {
        Ok(_) => existed = true,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => { log_info!("Unable to find script-output directory, skipping renaming."); existed = false; }
            ErrorKind::PermissionDenied => todo!("Encourage user to run with elevated permissions."),
            _ => todo!("Return error type"),
        }
    }
    log_info!("Creating temporary script-output directory");

    let result = fs::create_dir(&script_output);
    if let Err(e) = result {
        todo!("Check for invalid permissions and either warn user or return error type");
    }

    //TODO: Display a warning to the user about Steam's security popup for external programs trying to run with command line args.
    //TODO: Explain why.
    log_info!("Attempting to run factorio with --dump-data");
    let result = run_with_arg("--dump-data");
    //TODO: handle result. Check if successful.
    log_info!("Attempting to run factorio with --dump-icon-sprites");
    let result = run_with_arg("--dump-icon-sprites");
    //TODO: handle result. Check if successful.
    log_info!("Attempting to run factorio with --dump-prototype-locale");
    let result = run_with_arg("--dump-prototype-locale");
    //TODO: handle result. Check if successful.

    //TODO: Do stuff with the information.

    log_info!("Deleting temporary script-output directory.");
    let result = fs::remove_dir(&script_output);
    //TODO: handle result.
    if existed {
        log_info!("Restoring script-output-backup to original name.");
        let result = fs::rename(&script_output_backup, &script_output);
        //TODO: handle result.
    }
}