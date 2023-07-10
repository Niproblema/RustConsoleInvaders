use strum_macros::{Display, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Display, IntoStaticStr)]  // strum macros.
pub enum Sound {
    //#[strum(serialize = "explode")]
    Explode,
    //#[strum(serialize = "lose")]
    Lose,
    //#[strum(serialize = "move")]
    Move,
    //#[strum(serialize = "pew")]
    Pew,
    //#[strum(serialize = "startup")]
    Startup,
    //#[strum(serialize = "win")]
    Win,
    //#[strum(serialize = "none")]
    None,
}

pub const SOUND_PATHS: [(Sound, &str); 6] = [
    (Sound::Explode, "resources/explode.wav"),
    (Sound::Lose, "resources/lose.wav"),
    (Sound::Move, "resources/move.wav"),
    (Sound::Pew, "resources/pew.wav"),
    (Sound::Startup, "resources/startup.wav"),
    (Sound::Win, "resources/win.wav"),
];
