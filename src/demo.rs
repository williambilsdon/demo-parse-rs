use std::fs;
use bitreader_rs::Bitreader;

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

// TODO: All unwraps need proper handling.
impl Demo {
    pub fn parse(file_name: &str) -> Demo {
        let file = fs::read(file_name).unwrap();
        let mut bitreader = Bitreader::new(file.as_slice());

        let header = bitreader.read_string(8).unwrap();
        let demo_protocol = bitreader.read_i32().unwrap();
        let network_protocol = bitreader.read_i32().unwrap();
        let server_name = bitreader.read_string(260).unwrap();
        let client_name = bitreader.read_string(260).unwrap();
        let map_name = bitreader.read_string(260).unwrap();
        let game_directory = bitreader.read_string(260).unwrap();
        let playback_time = bitreader.read_f32().unwrap();
        let ticks = bitreader.read_i32().unwrap();let frames = bitreader.read_i32().unwrap();
        let sign_on_length = bitreader.read_i32().unwrap();  

        Demo { 
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let expected = Demo { 
            header: String::from("HL2DEMO"), 
            demo_protocol: 67108864, 
            network_protocol: 557187072, 
            server_name: String::from("localhost:27015"), 
            client_name: String::from("will"), 
            map_name: String::from("de_dust2"), 
            game_directory: String::from("csgo"), 
            playback_time: 0.000000000000000000000000000000000000019110306,
            ticks: 116260863, 
            frames: -535232512, 
            sign_on_length: -1685975552
        };
        let result = Demo::parse("test_demo.dem");
        assert_eq!(result, expected)
    }
}