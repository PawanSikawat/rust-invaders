use std::error::Error;
use invaders::add_audios;
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    add_audios(&mut audio);

    // Starting up
    audio.play("startup");

    // Cleanup post game
    audio.wait();
    Ok(())
}
