use agb::rng;
use agb::sound::mixer::{ChannelId, Mixer, SoundChannel};
use agb_tracker::include_xm;
use agb_tracker::Track;
use agb_tracker::Tracker;
use alloc::vec::Vec;

static BAT_FLAP: &[u8] = agb::include_wav!("sfx/BatFlap.wav");
static JUMP1: &[u8] = agb::include_wav!("sfx/Jump1.wav");
static JUMP2: &[u8] = agb::include_wav!("sfx/Jump2.wav");
static JUMP3: &[u8] = agb::include_wav!("sfx/Jump3.wav");

static CRAWL_XM: Track = include_xm!("music/crawl.xm");

pub struct Sfx<'a> {
    bgm: Option<ChannelId>,
    trackers: Vec<Tracker>,
    mixer: &'a mut Mixer<'a>,
}

impl<'a> Sfx<'a> {
    pub fn new(mixer: &'a mut Mixer<'a>) -> Self {
        Self {
            mixer,
            trackers: Vec::new(),
            bgm: None,
        }
    }

    pub fn frame(&mut self) {
        for tracker in self.trackers.iter_mut() {
            tracker.step(self.mixer);
        }
        self.mixer.frame();
    }

    pub fn tink(&mut self) {
        // We can play a sample
        if let Some(sample) = CRAWL_XM.samples.first() {
            let sample_channel = SoundChannel::new(&sample.data);
            self.mixer.play_sound(sample_channel);
        }
    }

    pub fn stop_music(&mut self) {
        if let Some(bgm) = &self.bgm {
            let channel = self.mixer.channel(bgm).unwrap();
            channel.stop();
        }
        self.trackers.clear();
        self.bgm = None;
    }

    pub fn crawl(&mut self) {
        self.trackers.push(Tracker::new(&CRAWL_XM));
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
