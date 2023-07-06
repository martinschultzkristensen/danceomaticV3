extern crate vlc;

use std::process;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use vlc::prelude::*;

fn main() {
    // Initialize libVLC
    let instance = Instance::new().unwrap();

    // Create a media player
    let media_player = instance.media_player().unwrap();

    // Load the video file
    let media = instance.media_path("path/to/video.mp4").unwrap();
    media_player.set_media(&media);

    // Start playing the video in a loop
    media_player.play().unwrap();

    // Create a channel to receive keyboard input
    let (tx, rx) = mpsc::channel();

    // Spawn a separate thread to listen for keyboard input
    thread::spawn(move || {
        loop {
            // Read a single character from the user
            let input = getch();
            if let Ok(ch) = input {
                // Send the character through the channel
                tx.send(ch).unwrap();
            }
        }
    });

    // Main event loop
    loop {
        // Check for keyboard input
        if let Ok(ch) = rx.try_recv() {
            match ch {
                b'q' | b'Q' => {
                    // Restart the video
                    media_player.stop().unwrap();
                    media_player.play().unwrap();
                }
                27 => {
                    // Quit the program on Esc key
                    media_player.stop().unwrap();
                    process::exit(0);
                }
                _ => {}
            }
        }

        // Wait for a short duration to avoid high CPU usage
        thread::sleep(Duration::from_millis(100));
    }
}

fn getch() -> std::io::Result<u8> {
    use std::io::Read;

    let stdin = std::io::stdin();
    let mut termios = termios::Termios::from_fd(stdin.lock().as_raw_fd())?;

    // Set terminal in non-canonical mode
    termios.c_lflag &= !(termios::ICANON | termios::ECHO);
    termios::tcsetattr(stdin.lock().as_raw_fd(), termios::TCSANOW, &termios)?;

    let mut buffer: [u8; 1] = [0];
    let _ = std::io::Read::read_exact(&mut std::io::stdin(), &mut buffer)?;

    // Restore terminal settings
    termios.c_lflag |= termios::ICANON | termios::ECHO;
    termios::tcsetattr(stdin.lock().as_raw_fd(), termios::TCSANOW, &termios)?;

    Ok(buffer[0])
}

