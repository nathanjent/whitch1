use agb::rng;
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};

const BAT_FLAP: &[u8] = agb::include_wav!("sfx/BatFlap.wav");
const JUMP1: &[u8] = agb::include_wav!("sfx/Jump1.wav");
const JUMP2: &[u8] = agb::include_wav!("sfx/Jump2.wav");
const JUMP3: &[u8] = agb::include_wav!("sfx/Jump3.wav");

pub struct Sfx<'a> {
    bgm: Option<ChannelId>,
    mixer: &'a mut Mixer<'a>,
}

impl<'a> Sfx<'a> {
    pub fn new(mixer: &'a mut Mixer<'a>) -> Self {
        Self { mixer, bgm: None }
    }

    pub fn frame(&mut self) {
        self.mixer.frame();
    }

    pub fn stop_music(&mut self) {
        if let Some(bgm) = &self.bgm {
            let channel = self.mixer.channel(bgm).unwrap();
            channel.stop();
        }
        self.bgm = None;
    }

    pub fn bat_flap(&mut self) {
        self.mixer.play_sound(SoundChannel::new(BAT_FLAP));
    }

    pub fn jump(&mut self) {
        let r = rng::gen() % 3;

        let channel = match r {
            0 => SoundChannel::new(JUMP1),
            1 => SoundChannel::new(JUMP2),
            _ => SoundChannel::new(JUMP3),
        };

        self.mixer.play_sound(channel);
    }
}
