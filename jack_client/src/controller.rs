use crate::waveform::Waveform;
use crossbeam_channel::{bounded, Receiver, Sender};

pub enum Event {
    Frequency(f32),
    Wave(Waveform),
}

pub struct Controller {
    pub tx: Sender<Event>,
    pub rx: Receiver<Event>,

  pub freq: f32,
}

impl Controller {
    pub fn new() -> Self {
        const CHANNEL_CAPACITY: usize = 100;

        let (tx, rx) = bounded(CHANNEL_CAPACITY);

        Self { tx, rx, freq: 1.0 }
    }
}
