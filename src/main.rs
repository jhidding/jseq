extern crate pretty_env_logger;
extern crate alsa;

use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use log::{info};

use alsa::{Direction};
use alsa::seq::{Seq,EventType,EvNote,EvCtrl};

use std::ffi::{CString};

#[derive(Debug)]
enum Error {
    Alsa(alsa::Error),
    FFI(std::ffi::NulError),
    Other(String)
}

enum SeqMsg {
    Debug(&'static str),
    Note(EvNote),
    NoteOn(EvNote),
    NoteOff(EvNote),
    Ctrl(EvCtrl),
    Pitch(EvCtrl),
    Other(EventType)
}

fn sequencer(source: mpsc::Receiver<SeqMsg>) {
    let seq_time = std::time::SystemTime::now();
    for msg in source {
        let t = seq_time.elapsed().unwrap().as_secs_f64();
        match msg {
            SeqMsg::Debug(x) => { info!("[{:12.6}] - {}", t, x); }
            SeqMsg::Note(x)  => { info!("[{:12.6}] - {:?}", t, x); }
            SeqMsg::NoteOn(x)  => { info!("[{:12.6}] - NoteOn  - {:?}", t, x); }
            SeqMsg::NoteOff(x)  => { info!("[{:12.6}] - NoteOff - {:?}", t, x); }
            SeqMsg::Ctrl(x)     => { info!("[{:12.6}] - Ctrl    - {:?}", t, x); }
            SeqMsg::Pitch(x)     => { info!("[{:12.6}] - Pitch   - {:?}", t, x); }
            SeqMsg::Other(p)    => { info!("[{:12.6}] - {:?}", t, p); }
            _                   => { info!("???"); }
        }
    }
}

fn seq_input(sink: mpsc::Sender<SeqMsg>) -> Result<(), Error> {
    let interface_name = CString::new("default").map_err(Error::FFI)?;
    let seq = Seq::open(
        Some(&interface_name), None, false).map_err(Error::Alsa)?;
    let client_name = CString::new("JSeq").map_err(Error::FFI)?;
    seq.set_client_name(&client_name).map_err(Error::Alsa)?;

    let port_name = CString::new("Input").map_err(Error::FFI)?;
    let port = seq.create_simple_port(
        &port_name, alsa::seq::PortCap::WRITE | alsa::seq::PortCap::SUBS_WRITE,
        alsa::seq::PortType::APPLICATION).map_err(Error::Alsa);

    let mut input = seq.input();
    loop {
        let event = input.event_input().map_err(Error::Alsa)?;
        match event.get_type() {
            EventType::Note       => { sink.send(SeqMsg::Note(event.get_data().unwrap())).unwrap(); }
            EventType::Noteon     => { sink.send(SeqMsg::NoteOn(event.get_data().unwrap())).unwrap(); }
            EventType::Noteoff    => { sink.send(SeqMsg::NoteOff(event.get_data().unwrap())).unwrap(); }
            EventType::Controller => { sink.send(SeqMsg::Ctrl(event.get_data().unwrap())).unwrap(); }
            EventType::Pitchbend  => { sink.send(SeqMsg::Pitch(event.get_data().unwrap())).unwrap(); }
            _                     => { sink.send(SeqMsg::Other(event.get_type())).unwrap(); }
        }
    }
}

fn main() {
    pretty_env_logger::init();
    let (sink, source) = mpsc::channel();

    let sink1 = sink.clone();
    thread::spawn(move || {
        sink1.send(SeqMsg::Debug("Hello, world!")).unwrap();
    });

    let sink2 = sink.clone();
    thread::spawn(move || {
        seq_input(sink2).unwrap();
    });

    sequencer(source);
}
