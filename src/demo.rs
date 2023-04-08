use std::fs;
use std::slice::Iter;
use std::vec::IntoIter;

struct Demo {
    header: String,
    demo_protocol: u32,
    network_protocol: u32,
    server_name: String,
    client_name: String,
    map_name: String,
    game_directory: String,
    playback_time: f32,
    ticks: u32,
    frames: u32,
    sign_on_length: u32
}

// TODO: All unwraps need proper handling.
impl Demo {
    pub fn parse(file_name: &str) {
        let mut demo_bytes = fs::read(file_name).unwrap().into_iter();
        let header = Demo::parse_string(&mut demo_bytes, 8);
        println!("{}", header);
        
        let demo_protocol = Demo::parse_u32(&mut demo_bytes);
        println!("{}", demo_protocol);
    }

    fn parse_string(bytes: &mut IntoIter<u8>, string_length: u8) -> String {
        let mut header_bytes = vec![];
    
        for _ in 0..string_length {
            header_bytes.push(bytes.next().unwrap());
        };

        String::from_utf8(header_bytes).unwrap()
    }

    fn parse_u32(bytes: &mut IntoIter<u8>) -> u32 {
        let mut int32_bytes = vec![];

        for _ in 0..4 {
            int32_bytes.push(bytes.next().unwrap());
        }
        // FIXME:  There's some bug here causing a crash, I believe it's to do with the ints being stores as Little Endian.
        str::parse::<u32>(&String::from_utf8(int32_bytes).unwrap()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: These UTs should test each individual parse method

    #[test]
    fn test_parse() {
        let result = Demo::parse("test_demo.dem");

        assert_eq!(result, ())
    }
}