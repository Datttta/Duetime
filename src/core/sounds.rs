use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use log::info;

pub fn alarm_sound(is_playing: Arc<AtomicBool>) {
    thread::spawn(move || {
        // 1. Silences low-level ALSA/JACK C-library errors (Linux/Unix only)
        #[cfg(unix)]
        {
            use std::fs::File;
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

        // Embed the MP3 file directly into the binary at compile time
        const ALARM_BYTES: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/timer_finished.mp3"));

        while is_playing.load(Ordering::Relaxed) {
            let cursor = Cursor::new(ALARM_BYTES);

            // Pass the cursor directly; rodio::play handles decoding internally
            let Ok(player) = rodio::play(mixer, cursor) else { break; };

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
