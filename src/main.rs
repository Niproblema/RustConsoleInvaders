//use rusty_audio::Audio;
use std::error::Error;
mod sound_player;
use sound_player::sounds::Sound;

fn main() -> Result<(), Box<dyn Error>> {
    let mut player = sound_player::SoundPlayer::new();
    player.setup();
    player.play_sound(Sound::Explode);

    Ok(())
}
