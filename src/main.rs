use rppal::gpio::{Gpio, InputPin, Level, Trigger};
use std::convert::TryFrom;
use std::process::exit;
use std::str::FromStr;
use std::time::{Duration, Instant};
use std::{env, fmt};
use std::{fs, thread};
use tokio::signal::unix::{signal, SignalKind};

#[tokio::main]
async fn main() {
    // Create signal handlers
    let mut sighup = signal(SignalKind::hangup()).unwrap();
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    // Parse pin numbers from command line
    let mut args: Vec<String> = env::args().collect();

    let pins = args
        .iter()
        .skip(1)// first element contains binary name
        .map(|pin_string| {
            println!("parsing {}", pin_string);
            u8::from_str(pin_string).unwrap_or_else(|e| {
                println!("Unable to parse pin number from string: [{}]", e);
                exit(-1);
            })
        })
        .collect::<Vec<_>>();

    /* let pin_number = match args.get(1) {
        Some(pin_string) => u8::from_str(pin_string).unwrap_or_else(|e| {
            println!("Unable to parse pin number from string: [{}]", e);
            exit(-1);
        }),
        None => {
            println!("No pin specified!\nUsage: button <pin>");
            exit(-1);
        }
    };*/

    // Initialize GPIO communications
    let gpio = Gpio::new().unwrap_or_else(|e| {
        println!("Error initializing gpio communication: [{}]", e);
        exit(-1)
    });

    let mut outputs = Vec::new();

    for pin_number in pins {
        // Create an input pin to use with our button
        let mut pin = gpio
            .get(pin_number)
            .unwrap_or_else(|e| {
                println!("Error initializing gpio pin: [{}]", e);
                exit(-1);
            })
            .into_output();

        outputs.push(pin);

    }

    loop {
        println!("Toggling..");
        for mut pin in &mut outputs {
            pin.toggle();
        }
        thread::sleep(Duration::from_millis(2000));
    }

    // Do nothing, just need to keep the program alive to wait for the callback or until
    // we are interrupted
    // We handle all signals the same and shut down orderly by cleaning up our interrupt
    // on the pin
    /* tokio::select! {
    _ = sigint.recv() => cleanup(pin, "SIGINT", pin_number),
    _ = sighup.recv() => cleanup(pin, "SIGHUP", pin_number),
    _ = sigterm.recv() => cleanup(pin, "SIGTERM", pin_number),
    }*/
}
