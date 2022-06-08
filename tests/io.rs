use std::{
    ffi::CString,
    io::{BufRead, BufReader, Write},
};

use anyhow::Result;
use visa_rs::{
    event::{self, Event},
    flags::AccessMode,
    DefaultRM, Instrument, TIMEOUT_IMMEDIATE,
};

#[test]
fn list_instr() -> Result<()> {
    let rm = DefaultRM::new()?;
    let mut list = rm.find_res(&CString::new("?*INSTR").unwrap().into())?;
    while let Some(n) = list.find_next()? {
        eprintln!("{}", n);
    }
    Ok(())
}

#[test]
fn send_idn() -> Result<()> {
    let rm = DefaultRM::new()?;
    let mut list = rm.find_res(&CString::new("?*KEYSIGH?*INSTR").unwrap().into())?;
    if let Some(n) = list.find_next()? {
        let mut instr = rm.open(&n, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)?;
        instr.write_all(b"*IDN?\n").unwrap();
        let mut buf_reader = BufReader::new(instr);
        let mut buf = String::new();
        buf_reader.read_line(&mut buf).unwrap();
        eprintln!("{}", buf);
    }
    Ok(())
}

#[test]
fn handler() -> Result<()> {
    // tried EventKind::Trig, but not supported by my keysight osc :(
    let rm = DefaultRM::new()?;
    let mut list = rm.find_res(&CString::new("?*KEYSIGH?*INSTR").unwrap().into())?;
    if let Some(n) = list.find_next()? {
        let instr = rm.open(&n, AccessMode::NO_LOCK, TIMEOUT_IMMEDIATE)?;
        let call_back1 = |ins: &Instrument, t: &Event| -> () {
            println!("call1: {:?} {:?}", ins, t);
        };
        let call_back2 = |ins: &Instrument, t: &Event| -> () {
            println!("call2: {:?} {:?}", ins, t);
        };
        let event = event::EventKind::IoCompletion;
        let h1 = instr.install_handler(event, call_back1)?;
        let h2 = instr.install_handler(event, call_back2)?;
        instr.enable_event(event, event::Mechanism::Handler, event::EventFilter::Null)?;
        (&instr).write_async(b"*IDN?\n")?;
        h1.receiver().recv()?;
        h2.receiver().recv()?;
        h1.uninstall();
        let mut v = vec![0u8; 256];
        (&instr).read_async(&mut v)?;
        eprintln!("{}", String::from_utf8_lossy(v.as_ref()));
        h2.receiver().recv()?;
        eprintln!("{}", String::from_utf8_lossy(v.as_ref()))
    }
    Ok(())
}
