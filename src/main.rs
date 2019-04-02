extern crate byteorder;
extern crate mio;
extern crate parking_lot;

use parking_lot::Mutex;
use std::collections::VecDeque;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

use mio::net::UdpSocket;

mod audio;
mod net;

type BoxedErr = Box<std::error::Error>;

fn main() -> Result<(), BoxedErr> {
    let bind_addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let connect_addr = env::args().nth(2).unwrap_or(bind_addr.clone());
    let bind_addr: SocketAddr = bind_addr.parse()?;
    let connect_addr: SocketAddr = connect_addr.parse()?;

    let socket = UdpSocket::bind(&bind_addr)?;
    println!("Listening on: {}", socket.local_addr()?);

    // TODO: use `socket.peer_addr()` when it lands to stable.
    // https://github.com/rust-lang/rust/issues/59127
    socket.connect(connect_addr.clone())?;
    println!("Connected to: {}", &connect_addr);

    let capture_buf = Arc::new(Mutex::new(VecDeque::with_capacity(30_000_000)));
    let playback_buf = Arc::new(Mutex::new(VecDeque::with_capacity(30_000_000)));

    let audio_backend_builder = audio::BackendBuilder {
        capture_buf: capture_buf.clone(),
        playback_buf: playback_buf.clone(),
    };
    run_audio_backend(audio_backend_builder)?;

    let net_service = net::NetService {
        capture_buf: capture_buf.clone(),
        playback_buf: playback_buf.clone(),
    };
    net_service.r#loop(socket)?;

    Ok(())
}

#[derive(Debug)]
enum AudioBackendToUse {
    Cpal,
    PulseSimple,
}

impl AudioBackendToUse {
    fn from_env() -> Result<Self, std::env::VarError> {
        Ok(match std::env::var("AUDIO_BACKEND") {
            Ok(ref val) if val == "pulse_simple" => AudioBackendToUse::PulseSimple,
            Ok(ref val) if val == "cpal" => AudioBackendToUse::Cpal,
            // Defaults.
            Ok(_) => AudioBackendToUse::Cpal,
            Err(std::env::VarError::NotPresent) => AudioBackendToUse::Cpal,
            // Invalid value.
            Err(e) => return Err(e),
        })
    }
}

fn run_audio_backend(builder: audio::BackendBuilder) -> Result<(), BoxedErr> {
    use audio::Backend;
    use audio::BackendBuilderFor;

    use audio::pulse_simple_backend::Backend as PulseSimpleBackend;
    use audio::Cpal as CpalBackend;

    let backend_to_use = AudioBackendToUse::from_env()?;
    println!("Using audio backend: {:?}", backend_to_use);

    match backend_to_use {
        AudioBackendToUse::Cpal => {
            let audio_backend: CpalBackend = builder.build()?;
            std::thread::spawn(move || audio_backend.run());
            return Ok(());
        }
        AudioBackendToUse::PulseSimple => {
            let audio_backend: PulseSimpleBackend = builder.build()?;
            std::thread::spawn(move || audio_backend.run());
            return Ok(());
        }
    }
}
