use jack::{Client, ClientOptions, ClientStatus};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Stopped = -1,
    Init = 0,
    Ready = 1,
    Runnning = 2,
}

pub trait Jack {
    // fn get_client(&self) -> &Client;
    // fn get_out(&self) -> Port<AudioOut>;

    fn spawn_client(&self, name: &str) -> (Client, ClientStatus) {
        let options = ClientOptions::NO_START_SERVER;

        Client::new(name, options).unwrap()
    }

    fn register(&mut self) -> Result<Status, &str>;
    fn stop(&mut self) -> Result<Status, &str>;

    // fn register_port(&self) {
    //     let port_out = client.register_port("out", AudioOut::default()).unwrap();
    // }
}
