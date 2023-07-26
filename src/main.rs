use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use vlc::MediaPlayer;

fn main() {
    // Sti til din MP4-video
    let video_path = "/Users/martinsk/Movies/Intro-Movie_2015Horsens.mp4";

    // Opret en ny instans af VLC MediaPlayer
    let mut player = MediaPlayer::new().expect("Failed to initialize VLC");

    // Afspil videoen
    player.play().expect("Failed to play the video");

    // Lad programmet køre, mens videoen afspilles
    loop {
        // Tjek om videoen stadig afspilles
        if !player.is_playing() {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    // Videoen er færdig med at afspille, så afslut programmet
    player.stop().expect("Failed to stop the video");
}

