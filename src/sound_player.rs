use rusty_audio::Audio;

pub mod sounds;

pub struct SoundPlayer {
    lib_audio: Audio,
}

impl SoundPlayer {
    pub fn new() -> Self {
        Self {
            lib_audio: Audio::new(),
        }
    }

    pub fn setup(&mut self) {
        for tup in sounds::SOUND_PATHS {
            self.lib_audio.add::<&str, &str>(tup.0.into(), tup.1);
        }
    }

    pub fn play_sound(&mut self, sound: sounds::Sound){
        self.lib_audio.play::<&str>(sound.into());
        self.lib_audio.wait();
    }
}
