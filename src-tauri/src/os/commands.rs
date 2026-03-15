use anyhow::Context;
use std::path::Path;

#[cfg(target_os = "windows")]
use std::process::Command as ProcessCommand;

#[cfg(target_os = "macos")]
use std::process::Command as ProcessCommand;

#[cfg(target_os = "linux")]
use std::process::Command as ProcessCommand;

#[tauri::command]
#[specta::specta]
pub async fn reveal_in_explorer(path: String) -> Result<(), String> {
    let path = Path::new(&path);
    let parent = path.parent().unwrap_or(path);

    #[cfg(target_os = "windows")]
    {
        ProcessCommand::new("explorer")
            .args(["/select,", &path.display().to_string()])
            .spawn()
            .context("failed to open explorer")
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "macos")]
    {
        ProcessCommand::new("open")
            .args(["-R", &path.display().to_string()])
            .spawn()
            .context("failed to open finder")
            .map_err(|e| e.to_string())?;
    }

    #[cfg(target_os = "linux")]
    {
        ProcessCommand::new("xdg-open")
            .arg(parent)
            .spawn()
            .context("failed to open file manager")
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}
