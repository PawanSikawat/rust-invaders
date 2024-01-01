use std::{error::Error, io, time::Duration};
use crossterm::{terminal::{self, LeaveAlternateScreen, EnterAlternateScreen}, ExecutableCommand, cursor::{Hide, Show}, event::{self, Event, KeyCode}};
use invaders::add_audios;
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    add_audios(&mut audio);

    // Starting up
    audio.play("startup");

    // Initialize Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game Loop
    'gameloop: loop {
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }


    // Cleanup post game
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
