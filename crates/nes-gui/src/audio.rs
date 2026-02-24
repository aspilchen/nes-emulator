use sdl2::audio::{AudioCallback, AudioSpecDesired};

struct ApuCallback {
    buffer: Arc<Mutex<VecDeque<f32>>>,
}

impl AudioCallback for ApuCallback {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        let mut buf = self.buffer.lock().unwrap();
        for sample in out.iter_mut() {
            *sample = buf.pop_front().unwrap_or(0.0);
        }
    }
}
