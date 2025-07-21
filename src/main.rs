mod config;
use config::Config;

use evdev::Device;
use rodio::Decoder;
use shellexpand;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = {
        let path = shellexpand::tilde("~/.config/keyvibe/config.toml");
        let context = fs::read_to_string(path.as_ref())?;
        toml::from_str(&context)?
    };

    // println!("{config:?}");
    let mut device = {
        let path = shellexpand::tilde(config.keyboard.path.as_str());
        Device::open(path.as_ref())?
    };

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    // let sink = rodio::Sink::connect_new(stream_handle.mixer());
    // stream_handle.mixer().add(fd_src);

    let sound_stream = {
        let path = shellexpand::tilde(config.keysound.path.as_str());
        let mut file = File::open(path.as_ref())?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Cursor::new(buffer)
    };

    drop(config);
    loop {
        for ev in device.fetch_events()? {
            if ev.value() == 1 {
                // println!("{ev:?}");

                let sink = rodio::Sink::connect_new(stream_handle.mixer());
                sink.append(Decoder::try_from(sound_stream.clone())?);
                sink.detach();
            }
        }
    }
}
