use jack::{AudioIn, ClosureProcessHandler};

use std::sync::mpsc::sync_channel;
use {
    crate::jacker::{Jack, Status},
    jack::{AsyncClient, AudioOut, Client, ClientStatus, Control, MidiIn, Port, ProcessScope},
};

const MAX_MIDI: usize = 3;

const INITIAL_FREQ: f32 = 220.0;

//a fixed size container to copy data out of real-time thread
#[derive(Copy, Clone, Debug)]
struct MidiCopy {
    len: usize,
    data: [u8; MAX_MIDI],
    time: jack::Frames,
}

impl From<jack::RawMidi<'_>> for MidiCopy {
    fn from(midi: jack::RawMidi<'_>) -> Self {
        let len = std::cmp::min(MAX_MIDI, midi.bytes.len());
        let mut data = [0; MAX_MIDI];
        data[..len].copy_from_slice(&midi.bytes[..len]);
        MidiCopy {
            len,
            data,
            time: midi.time,
        }
    }
}

pub struct Phasor {
    name: String,
    status: Status,

    sample_rate: usize,

    client: Option<Client>,
    client_status: Option<ClientStatus>,
    port_in: Option<Port<AudioIn>>,
    port_midi_in: Option<Port<MidiIn>>,
    port_out: Option<Port<AudioOut>>,
}

impl Phasor {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: Status::Init,

            sample_rate: 0,
            client: None,
            client_status: None,
            port_in: None,
            port_out: None,
            port_midi_in: None,
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn start(
        mut self,
    ) -> AsyncClient<(), ClosureProcessHandler<impl FnMut(&Client, &ProcessScope) -> Control>> {
        let client = self.client.unwrap();
        let mut port_out = self.port_out.unwrap();
        let midi = self.port_midi_in.unwrap();

        let mut phs = 0.0;
        let sr = self.sample_rate as f32;

        let mut freq = INITIAL_FREQ;

        let (sender, receiver) = sync_channel(64);

        let process_handler = ClosureProcessHandler::new(move |_: &Client, ps: &ProcessScope| {
            let nframes = ps.n_frames();
            let out_buffer = port_out.as_mut_slice(ps);

            let mut fr = freq;

            let show_p = midi.iter(ps);
            for e in show_p {
                let c: MidiCopy = e.into();
                let _ = sender.try_send(c);

                let control = c.data[0];
                let chan = c.data[1];
                let data = c.data[2];
                if control == 176 {
                    if chan == 0 {
                        fr = (data as f32 / 127.0) * 400.0;
                        freq = fr;
                    } else if chan == 3 {
                        //
                    }
                }
            }

            for i in 0..nframes {
                out_buffer[i as usize] = phs;
                phs += fr / sr;

                // [0; 1)
                while phs >= 1.0 {
                    phs -= 1.0;
                }
                while phs < 0.0 {
                    phs += 1.0;
                }
            }

            Control::Continue
        });

        //spawn a non-real-time thread that prints out the midi messages we get
        std::thread::spawn(move || {
            while let Ok(m) = receiver.recv() {
                let control = m.data[0];
                let channel = m.data[1];
                let data = m.data[2];

                if control == 176 {
                    freq = (data as f32 / 127.0) * 10_000.0;
                }

                println!(
                    "Control: {} | Channel: {} | Data: {}",
                    control, channel, data
                );
            }
        });

        let a_client = client.activate_async((), process_handler).unwrap();
        self.status = Status::Runnning;

        a_client
    }
}

impl Jack for Phasor {
    fn register(&mut self) -> Result<Status, &str> {
        if self.status == Status::Runnning {
            return Err("Already running");
        }

        let (client, status) = self.spawn_client(self.name.as_str());

        self.sample_rate = client.sample_rate();
        self.client_status = Some(status);

        self.port_midi_in = Some(client.register_port("midi", MidiIn::default()).unwrap());
        self.port_out = Some(client.register_port("out", AudioOut::default()).unwrap());

        self.client = Some(client);

        self.status = Status::Ready;
        Ok(Status::Ready)
    }

    fn stop(&mut self) -> Result<Status, &str> {
        if self.status == Status::Stopped {
            return Err("Already stopped");
        }

        self.status = Status::Stopped;
        Ok(Status::Stopped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phasor() {
        let mut phasor = Phasor::new("phasor");
        phasor.register().unwrap();
        let phasor_client = phasor.start();

        phasor_client.deactivate().unwrap();
    }
}
