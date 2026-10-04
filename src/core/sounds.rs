use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use log::info;

pub fn alarm_sound(is_playing: Arc<AtomicBool>) {
    thread::spawn(move || {
        #[cfg(unix)]
        {
            use std::os::unix::io::AsRawFd;
            if let Ok(null_file) = File::create("/dev/null") {
                unsafe {
                    libc::dup2(null_file.as_raw_fd(), 2);
                }
            }
        }

        let Ok(mut stream_handle) = rodio::DeviceSinkBuilder::open_default_sink() else {
            info!("Failed to open default audio sink backend");
            return;
        };
        stream_handle.log_on_drop(false);
        let mixer = stream_handle.mixer();

        while is_playing.load(Ordering::Relaxed) {
            let file_path = std::env::current_dir()
                .map(|mut p| {
                    p.push("assets");
                    p.push("timer_finished.mp3");
                    p
                })
                .unwrap_or_else(|_| std::path::PathBuf::from("assets/timer_finished.mp3"));

            let file = match File::open(&file_path) {
                Ok(f) => f,
                Err(e) => {
                    info!("Alarm sound file not found at {:?}: {}", file_path, e);
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
