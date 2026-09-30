use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

#[cfg(target_os = "linux")]
pub fn copy_password(password: &str) -> Result<(), std::io::Error> {
    match env::var("WAYLAND_DISPLAY").ok() {
        Some(_) => {
            let mut child = Command::new("wl-copy").stdin(Stdio::piped()).spawn()?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(password.as_bytes())?;
            }

            Ok(())
        }
        None => {
            let mut child = Command::new("xclip")
                .args(["-selection", "clipboard"])
                .stdin(Stdio::piped())
                .spawn()?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(password.as_bytes())?;
            }

            child.wait()?;

            Ok(())
        }
    }
}

#[cfg(target_os = "macos")]
pub fn copy_password(password: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new("pbcopy").stdin(Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(password.as_bytes())?;
    }

    child.wait()?;

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn copy_password(password: &str) -> Result<(), std::io::Error> {
    let mut child = Command::new("powershell")
        .args(["-NoProfile", "-Command", "$input | Set-Clipboard"])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(password.as_bytes())?;
    }

    child.wait()?;

    Ok(())
}
