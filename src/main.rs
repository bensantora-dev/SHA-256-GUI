slint::include_modules!();
use sha2::{Digest, Sha256};
use std::fs;
use std::io;

fn main() {
    // Suppress DBus theme detection errors on Linux
    #[cfg(target_os = "linux")]
    std::env::set_var("SLINT_NO_THEME_DETECTION", "1");

    let ui = MainWindow::new().unwrap();

    ui.on_compute_hash({
        let ui = ui.as_weak();
        move || {
            let ui = ui.unwrap();
            let path = ui.get_file_path().to_string();

            // Open file (streaming - no full load into memory)
            let mut file = match fs::File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    ui.set_result(format!("Error: {}", e).into());
                    return;
                }
            };

            // Compute SHA-256 by streaming file into hasher
            let mut hasher = Sha256::new();
            if let Err(e) = io::copy(&mut file, &mut hasher) {
                ui.set_result(format!("Error reading file: {}", e).into());
                return;
            }

            let digest = hasher.finalize();
            ui.set_result(format!("{:x}", digest).into());
        }
    });

    ui.run().unwrap();
}
