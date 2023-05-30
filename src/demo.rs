use bitreader_rs::{errors::BitreadError, bitreader::Bitreader};

pub struct Demo<'a> {
    bitreader: Bitreader<'a>,
    pub header: Header,
    pub frames: Vec<Frame>
}

impl<'a> Demo<'a> {
    pub fn new(bitreader: Bitreader<'a>) -> Demo<'a> {
        Demo { 
            bitreader, 
            header: Header::default(), 
            frames: vec![]
        }
    }

    pub fn parse_header(&mut self) -> Result<(), BitreadError> {
        self.header = Header {
            header: self.read_string(8)?,
            demo_protocol: self.bitreader.read_i32()?,
            network_protocol: self.bitreader.read_i32()?,
            server_name: self.read_string(260)?,
            client_name: self.read_string(260)?,
            map_name: self.read_string(260)?,
            game_directory: self.read_string(260)?,
            playback_time: self.bitreader.read_f32()?,
            ticks: self.bitreader.read_i32()?,
            frames: self.bitreader.read_i32()?,
            sign_on_length: self.bitreader.read_i32()?,
        };

        Ok(())
    }

    fn read_string(&mut self, len: u64) -> Result<String, BitreadError> {
        let string = self.bitreader.read_string(len)?;
        Ok(String::from(string.trim_matches('\0')))
    }
}

#[derive(Default, Debug)]
pub struct Header{
    header: String,
    demo_protocol: i32,
    network_protocol: i32,
    server_name: String,
    client_name: String,
    map_name: String,
    game_directory: String,
    playback_time: f32,
    ticks: i32,
    frames: i32,
    sign_on_length: i32
}

#[derive(Debug)]
pub struct Frame {
    server_frame: i32,
    client_frame: i32,
    sub_packet_size: i32,
    buffer: Vec<char>,
    packet: Packet
}

#[derive(Debug)]
struct Packet {
    cmd_type: char,
    unknown: i32,
    tick_count: i32,
    size_of_packet: i32,
    buffer: Vec<char>
}
