use std::fs::File;
use std::io::BufReader;
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use log::info;

pub fn alarm_sound(is_playing: Arc<AtomicBool>) {
    thread::spawn(move || {
        // 1. Silences low-level ALSA/JACK C-library errors from spamming the screen
        if let Ok(null_file) = File::create("/dev/null") {
            unsafe {
                libc::dup2(null_file.as_raw_fd(), 2);
            }
        }

        // 2. Try to open the default sink after silencing stderr
        let Ok(mut stream_handle) = rodio::DeviceSinkBuilder::open_default_sink() else {
            info!("Failed to open default audio sink backend");
            return;
        };
        stream_handle.log_on_drop(false);
        let mixer = stream_handle.mixer();

        while is_playing.load(Ordering::Relaxed) {
            // 3. Clean check for your asset file, logging to your debug file using info!
            let file_path = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/timer_finished.mp3");
            
            let file = match File::open(file_path) {
                Ok(f) => f,
                Err(e) => {
                    info!("Alarm sound file not found at {}: {}", file_path, e);
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
