use jack::{
    AsyncClient, AudioIn, AudioOut, Client, ClientStatus, ClosureProcessHandler, Control, Port,
    ProcessScope,
};

use crate::controller::{Controller, Event};
use crate::jacker::{Jack, Status};
use crate::waveform::Waveform;

pub struct Osc {
    waveform: Waveform,

    name: String,
    status: Status,

    sample_rate: usize,

    client: Option<Client>,
    client_status: Option<ClientStatus>,
    port_in: Option<Port<AudioIn>>,
    port_out: Option<Port<AudioOut>>,
}

impl Osc {
    pub fn new(name: &str) -> Self {
        Self {
            waveform: Waveform::Sine,

            name: name.to_string(),
            status: Status::Init,

            sample_rate: 0,
            client: None,
            client_status: None,
            port_in: None,
            port_out: None,
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn start(
        mut self,
        controller: &Controller,
    ) -> AsyncClient<(), ClosureProcessHandler<impl FnMut(&Client, &ProcessScope) -> Control>> {
        let client = self.client.unwrap();
        let mut port_out = self.port_out.unwrap();
        let port_in = self.port_in.unwrap();

        let rx = controller.rx.clone();

        let mut waveform = self.waveform;

        let cback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let nframes = ps.n_frames();
            let in_buffer = port_in.as_slice(ps);
            let out_buffer = port_out.as_mut_slice(ps);

            println!("{}", controller.freq)

            while let Ok(new_wf) = rx.try_recv() {
                match new_wf {
                    Event::Wave(new_waveform) => waveform = dbg!(new_waveform),
                  _ => {}
                }
            }

            waveform.process(nframes as usize, in_buffer, out_buffer);

            Control::Continue
        };

        let a_client = client
            .activate_async((), ClosureProcessHandler::new(cback))
            .unwrap();
        self.status = Status::Runnning;

        a_client
    }
}

impl Jack for Osc {
    fn register(&mut self) -> Result<Status, &str> {
        if self.status == Status::Runnning {
            return Err("Already running");
        }

        let (client, status) = self.spawn_client(self.name.as_str());

        self.sample_rate = client.sample_rate();
        self.client_status = Some(status);

        self.port_in = Some(client.register_port("phs", AudioIn::default()).unwrap());
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
