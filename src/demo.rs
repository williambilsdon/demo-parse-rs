use std::fs;
use bitreader_rs::{errors::BitreadError, bitreader::Bitreader};

#[derive(PartialEq, Debug)]
struct Demo {
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
    pub fn parse(file_name: &str) -> Result<Demo, BitreadError> {
        // TODO: move file read out of this impl
        let file = fs::read(file_name).unwrap();
        let mut bitreader = Bitreader::new(file.as_slice());

        let header = bitreader.read_string(8)?;
        let demo_protocol = bitreader.read_i32()?;
        let network_protocol = bitreader.read_i32()?;
        let server_name = bitreader.read_string(260)?;
        let client_name = bitreader.read_string(260)?;
        let map_name = bitreader.read_string(260)?;
        let game_directory = bitreader.read_string(260)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let result = Demo::parse("test_demo.dem");
        assert!(result.is_ok())
    }
}