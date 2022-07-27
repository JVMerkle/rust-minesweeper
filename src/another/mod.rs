use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// ???
const CONTROL_CODE_SOH: u8 = 0x01;
/// Start byte
const CONTROL_CODE_STX: u8 = 0x02;
/// Stop byte
const CONTROL_CODE_ETX: u8 = 0x03;
/// Escape byte
const CONTROL_CODE_DLE: u8 = 0x10;

// No copy or clone!
struct Data(Vec<u8>);

struct Frame {
    data: Data,
}

struct ParsedFrame {
    data: Data,
}

struct FrameBuilder {
    frame: Frame,

    start_found: bool,
    index: usize,
    dle_active: bool,

    complete: bool,
}

#[derive(Debug, PartialEq)]
enum BuildError {
    NeedMoreData,
    TooMuchData,
    BadEscaping,
}

impl FrameBuilder {
    fn from(data: Data) -> Self {
        FrameBuilder {
            frame: Frame { data: data },
            start_found: false,
            index: 0,
            dle_active: false,
            complete: false,
        }
    }

    fn feed(&mut self, data: &[u8]) -> Result<(), BuildError> {
        for b in data.iter() {
            if self.dle_active {
                match *b {
                    CONTROL_CODE_SOH => {}
                    CONTROL_CODE_STX => {}
                    CONTROL_CODE_ETX => {}
                    CONTROL_CODE_DLE => {}
                    _ => return Err(BuildError::BadEscaping),
                }
                self.dle_active = false;
                continue;
            }

            if *b == CONTROL_CODE_STX {
                self.index = 0;
                self.start_found = true;
            } else if !self.start_found {
                return Err(BuildError::NeedMoreData);
            }

            match *b {
                CONTROL_CODE_SOH => {  // ???
                }
                CONTROL_CODE_ETX => { // Stop
                    self.complete = true;
                    return Ok(());
                }
                CONTROL_CODE_DLE => { // Escaping
                    self.dle_active = true;
                    continue;
                }
                _ => {}
            }

            if self.index < self.frame.data.0.len() {
                self.frame.data.0[self.index] = *b;
                self.index += 1;
            } else {
                return Err(BuildError::TooMuchData);
            }
        }

        return Err(BuildError::NeedMoreData);
    }

    fn finalize(self) -> ParsedFrame {
        if !self.complete {
            panic!("frame not complete");
        }
        ParsedFrame { data: self.frame.data }
    }
}


struct Packet {
    data: Data,
    crc16_genibus: u16,
}

impl Packet {
    fn from(data: Data) -> Self {
        let crc16_genibus = Self::calc_checksum(data.0.as_slice());
        Packet {
            data,
            crc16_genibus,
        }
    }

    fn as_vec(&self) -> Vec<u8> {
        let crc16_high = (self.crc16_genibus >> 8) as u8;
        let crc16_low = (self.crc16_genibus & 0xff) as u8;

        let mut start_byte = vec!(CONTROL_CODE_STX);
        let mut stop_byte = vec!(CONTROL_CODE_ETX);
        let mut crc_bytes = vec![crc16_high, crc16_low];

        start_byte.append(&mut self.data.0.clone());
        start_byte.append(&mut crc_bytes);
        start_byte.append(&mut stop_byte);

        start_byte.iter().map(|&e| e as u8).collect()
    }

    fn calc_checksum(data: &[u8]) -> u16 {
        let crc = crc::Crc::<u16>::new(&crc::CRC_16_GENIBUS);
        let mut digest = crc.digest();
        digest.update(&data);
        digest.finalize()
    }
}

fn maaaaain() {
    let ports = serialport::available_ports().unwrap_or_else(|err| {
        panic!("Could not determine serial ports: {}", err);
    });

    if ports.len() == 0 {
        panic!("No serial port found");
    }

    let first_port = &ports[0];
    println!("Selecting {} ({:?})", first_port.port_name, first_port.port_type);

    let mut port = serialport::new(&first_port.port_name, 115200)
        .open()
        .unwrap();

    port.set_timeout(Duration::from_millis(2000)).unwrap();


    // \x02 \x00VERi;V0.0.0;K0.0.1;N\xF4\x03
    let mut tx_data = [0x20 as char, 0x00 as char,
        'V', 'E', 'R', 'i', ';', 'V', '0', '.', '0', '.', '0', ';', 'K', '0', '.', '0', '.', '1', ';'].to_vec();

    let tx_data_bytes: Vec<u8> = tx_data.iter().map(|&e| e as u8).collect();
    let mut packet = Packet::from(Data(tx_data_bytes.clone()));
    let tx_data = packet.as_vec();

    let tx_count = port.write(tx_data.as_slice()).unwrap();
    print!("TX [");
    for i in 0..tx_count {
        print!("{:#04x} ", tx_data[i]);
    }
    println!("]");


    let mut rx_data = [0; 255];
    let rx_count = port.read(&mut rx_data).unwrap_or(0);

    let mut builder = FrameBuilder::from(Data(vec![0; 255]));

    loop {
        match builder.feed(&rx_data) {
            Err(err) => {
                // if err == BuildError::NeedMoreData {
                //     continue;
                // }
                panic!("{:?}", err);
            },
            Ok(_) => { break; },
        };
    }

    let frame = builder.finalize();

    print!("RX [");
    for i in 0..rx_count {
        print!("{:#04x} ", rx_data[i]);
    }
    println!("]");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaaaaa() {
        maaaaain();
        assert_eq!(1, 1);
    }
}
