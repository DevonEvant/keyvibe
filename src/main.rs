use evdev::{Device, InputEvent};
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::{Cursor, Read};

fn main() -> Result<(), Box<dyn Error>> {
    // 尋找鍵盤設備
    let path = "/dev/input/event3"; // ❗換成正確的鍵盤裝置！
    let mut device = Device::open(path)?;
    // device.set_nonblocking(true)?;

    // 準備音效播放
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    // let sink = rodio::Sink::connect_new(stream_handle.mixer());
    // stream_handle.mixer().add(fd_src);

    let sound_stream = {
        let mut file = File::open("./src/assert/typing.mp3").unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Cursor::new(buffer)
    };

    loop {
        for ev in device.fetch_events()? {
            if ev.value() == 1 {
                println!("{ev:?}");

                let sink = rodio::Sink::connect_new(stream_handle.mixer());
                sink.append(Decoder::try_from(sound_stream.clone())?);
                sink.detach();
            }
        }
    }
}

