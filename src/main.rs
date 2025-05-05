use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Detect session type
    let wayland = std::env::var("WAYLAND_DISPLAY").is_ok();
    let x11 = std::env::var("DISPLAY").is_ok();

    if wayland {
        // Try wl-copy
        let mut child = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start wl-copy. Make sure it's installed.");
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        child.wait().unwrap();
    } else if x11 {
        // Try xclip
        let mut child = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Failed to start xclip. Make sure it's installed.");
        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(input.as_bytes())
            .unwrap();
        child.wait().unwrap();
    } else {
        eprintln!("No known clipboard system detected.");
    }
}
