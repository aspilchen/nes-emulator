use ringbuf::{
    traits::{Consumer, Observer, Split},
    HeapCons, HeapProd, HeapRb,
};
use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};

pub struct SdlAudioDevice {
    pub device: AudioDevice<ApuCallback>,
}

struct ApuCallback {
    audio_consumer: HeapCons<f32>,
}

impl SdlAudioDevice {
    pub fn new(sdl: &sdl2::Sdl) -> (Self, HeapProd<f32>) {
        let rb = HeapRb::<f32>::new(4096);
        let (producer, consumer) = rb.split();
        let audio = sdl.audio().unwrap();
        let desired = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: Some(512),
        };
        let device = audio
            .open_playback(None, &desired, |_spec| ApuCallback::new(consumer))
            .unwrap();
        (Self { device }, producer)
    }

    pub fn start(&mut self) {
        self.device.resume();
    }
}

impl AudioCallback for ApuCallback {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        let n = self.audio_consumer.pop_slice(out);
        if n < out.len() {
            out[n..].fill(0.0);
        }
    }
}

impl ApuCallback {
    pub fn new(audio_consumer: HeapCons<f32>) -> Self {
        Self { audio_consumer }
    }
}
