mod config;
use config::Config;

use evdev::Device;
use rodio::Decoder;
use shellexpand;
use std::borrow::Cow;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Read};

const KEYBROAD_DEFAULT_PATH: &str = "/dev/input/event3";
const KEYSOUND_BIN: &[u8] = include_bytes!("./assert/typing.mp3");

fn main() -> Result<(), Box<dyn Error>> {
    let config: Option<Config> = {
        let path = shellexpand::tilde("~/.config/keyvibe/config.toml");
        match fs::read_to_string(path.as_ref()) {
            Ok(context) => Some(toml::from_str(&context)?),
            _ => None,
        }
    };

    // println!("{config:?}");
    let mut device = {
        let path = config
            .as_ref()
            .and_then(|config| config.keyboard.as_ref())
            .and_then(|dev| dev.path.as_ref())
            .cloned()
            .unwrap_or_else(|| KEYBROAD_DEFAULT_PATH.to_string());

        Device::open(shellexpand::tilde(path.as_str()).as_ref())?
    };

    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;

    let sound_stream = {
        let buffer: Cow<'static, [u8]> = config
            .and_then(|config| config.keysound) // Changed from .keyboard to .keysound for sound path
            .as_ref()
            .and_then(|sound| sound.path.as_ref())
            .and_then(|path| {
                File::open(path).ok().and_then(|mut file| {
                    let mut buf = Vec::new();
                    file.read_to_end(&mut buf).ok()?; // Convert Result<(), Error> to Option<()>
                    Some(Cow::Owned(buf)) // If successful, wrap in Some(Cow::Owned)
                })
            })
            .unwrap_or(Cow::Borrowed(KEYSOUND_BIN));

        Cursor::new(buffer)
    };

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
