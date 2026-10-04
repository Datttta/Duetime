use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::path::PathBuf;
use log::info;

pub fn alarm_sound(is_playing: Arc<AtomicBool>) {
    thread::spawn(move || {
        // 1. Silences low-level ALSA/JACK C-library errors (Linux/Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            if let Ok(null_file) = File::create("/dev/null") {
                unsafe {
                    libc::dup2(null_file.as_raw_fd(), 2);
                }
            }
        }

        // 2. Try to open the default sink
        let Ok(mut stream_handle) = rodio::DeviceSinkBuilder::open_default_sink() else {
            info!("Failed to open default audio sink backend");
            return;
        };
        stream_handle.log_on_drop(false);
        let mixer = stream_handle.mixer();

        while is_playing.load(Ordering::Relaxed) {
            // 3. Resolve path dynamically for both dev and production
            let mut file_path = std::env::current_exe()
                .ok()
                .and_then(|mut path| {
                    path.pop(); // Remove binary name, get parent dir
                    path.push("assets/timer_finished.mp3");
                    Some(path)
                })
                .filter(|p| p.exists());

            // Fallback to CARGO_MANIFEST_DIR if running locally via `cargo run`
            if file_path.is_none() {
                let dev_path = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/timer_finished.mp3"));
                if dev_path.exists() {
                    file_path = Some(dev_path);
                }
            }

            let resolved_path = match file_path {
                Some(p) => p,
                None => {
                    info!("Alarm sound file not found in executable directory or manifest dir.");
                    return;
                }
            };

            let file = match File::open(&resolved_path) {
                Ok(f) => f,
                Err(e) => {
                    info!("Failed to open alarm sound at {:?}: {}", resolved_path, e);
                    return;
                }
            };

            let source = BufReader::new(file);
            let Ok(player) = rodio::play(mixer, source) else { break; };

            while !player.empty() && is_playing.load(Ordering::Relaxed) {
                thread::sleep(Duration::from_millis(100));
            }

            if !is_playing.load(Ordering::Relaxed) {
                player.stop();
                break;
            }
        }
    });
}
