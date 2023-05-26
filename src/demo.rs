use bitreader_rs::{errors::BitreadError, bitreader::Bitreader};

#[derive(PartialEq, Debug)]
pub struct Demo {
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

impl Demo {
    pub fn parse(bitreader: &mut Bitreader) -> Result<Demo, BitreadError> {
        let header = Self::read_string(bitreader, 8)?;
        let demo_protocol = bitreader.read_i32()?;
        let network_protocol = bitreader.read_i32()?;
        let server_name = Self::read_string(bitreader, 260)?;
        let client_name = Self::read_string(bitreader, 260)?;
        let map_name = Self::read_string(bitreader, 260)?;
        let game_directory = Self::read_string(bitreader, 260)?;
        let playback_time = bitreader.read_f32()?;
        let ticks = bitreader.read_i32()?;
        let frames = bitreader.read_i32()?;
        let sign_on_length = bitreader.read_i32()?;  

        Ok(Demo {
            header, 
            demo_protocol, 
            network_protocol, 
            server_name, 
            client_name, 
            map_name, 
            game_directory, 
            playback_time,
            ticks, 
            frames, 
            sign_on_length
        })
    }

    fn read_string(bitreader: &mut Bitreader, len: u64) -> Result<String, BitreadError> {
        let string = bitreader.read_string(len)?;
        Ok(String::from(string.trim_matches('\0')))
    }
}