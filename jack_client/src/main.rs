mod jacker;
mod osc;
mod phasor;
mod waveform;
mod controller;

use std::io;
use jacker::Jack;
use osc::Osc;
use phasor::Phasor;
use controller::{Controller, Event};


fn main() {
    let mut osc = Osc::new("sine");
    let mut phasor = Phasor::new("phasor");

    let controller = Controller::new();

    match osc.register() {
        Ok(status) => println!("Osc status: {:?}", status),
        Err(e) => eprintln!("Failed to register an osc: {}", e),
    }
    match phasor.register() {
        Ok(status) => println!("Phasor status: {:?}", status),
        Err(msg) => eprintln!("Failed to register a phasor: {}", msg),
    }


    let osc_client = osc.start(&controller);
    let phasor_client = phasor.start();

    println!("Provide instructions:");
    while let Some(f) = read_wave() {
        controller.tx.send(Event::Wave(f)).unwrap();
    }

    println!("Shutting down");
    osc_client.deactivate().unwrap();
    phasor_client.deactivate().unwrap();
}

//Attempt to read a frequency from standard in. Will block until there is
/// user input. `None` is returned if there was an error reading from standard
/// in, or the retrieved string wasn't a compatible u16 integer.
fn read_wave() -> Option<waveform::Waveform> {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => match user_input.trim() {
            "1" => Some(waveform::Waveform::Sine),
            "2" => Some(waveform::Waveform::Square),
            "3" => Some(waveform::Waveform::Sawtooth),
            _ => None,
        },
        Err(_) => None,
    }
}
