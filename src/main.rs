use crossbeam::channel;
use invaders::{
    frame::{self, new_frame, Drawable},
    player::Player,
    render,
};
use std::{
    error::Error,
    io,
    sync::mpsc,
    thread::{self, sleep, spawn},
    time::Duration,
};
mod sound_player;
use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use sound_player::sounds::Sound;

fn main() -> Result<(), Box<dyn Error>> {
    let mut sound_player = sound_player::SoundPlayer::new();
    sound_player.setup();

    // Terminal setup
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Run rendering in a seperate thread.
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        while let Ok(new_frame) = render_rx.recv() {
            render::render(&mut stdout, &last_frame, &new_frame, false);
            last_frame = new_frame;
        }
    });

    // Setup player
    let mut player = Player::new();

    // Setup field

    // GameLoop
    'gameloop: loop {
        // wait for input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        sound_player.play_sound(Sound::Lose);
                        break 'gameloop;
                    }
                    KeyCode::Char('A') | KeyCode::Char('a') | KeyCode::Left => {
                        player.move_left();
                    }
                    KeyCode::Char('D') | KeyCode::Char('d') | KeyCode::Right => {
                        player.move_right();
                    }
                    _ => {}
                }
            }
        }

        // Frame creation
        // initialize frame
        let mut new_frame = new_frame();

        // Draw player 
        player.draw(&mut new_frame);

        // Send to the render thread. This might fail if that thread is not ready yet-thats fine.
        let _ = render_tx.send(new_frame);
        thread::sleep(Duration::from_millis(50));
    }

    sound_player.play_sound(Sound::Startup);

    // Rendering cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    // Terminal cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
