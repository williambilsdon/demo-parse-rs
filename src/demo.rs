use std::fs;
use std::vec::IntoIter;

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
    pub fn parse(file_name: &str) {
        let example = fs::read(file_name).unwrap();

        let mut demo_bytes = fs::read(file_name).unwrap().into_iter();

        let header = Demo::parse_string(&mut demo_bytes, 8);
        println!("{}", header);
        
        let demo_protocol = Demo::parse_i32(&mut demo_bytes);
        println!("{}", demo_protocol);
    }

    fn parse_string(bytes: &mut IntoIter<u8>, string_length: u8) -> String {
        let mut header_bytes = vec![];
    
        for _ in 0..string_length {
            header_bytes.push(bytes.next().unwrap());
        };

        String::from_utf8(header_bytes).unwrap()
    }

    fn parse_i32(bytes: &mut IntoIter<u8>) -> i32 {
        let mut int32_bytes = vec![];

        for _ in 0..4 {
            int32_bytes.push(bytes.next().unwrap());
        }

        let as_utf8 = String::from_utf8(int32_bytes).unwrap();

        println!("converted int as string: {}", as_utf8);

        // FIXME:  There needs to be a betterway to handle integers in decimal byte format.
        // This approach of expecting 4 bytes can quickly fail.
       as_utf8.trim_end().parse().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_string() {
        let mut input: IntoIter<u8> = vec![72, 101, 108, 108, 111, 44, 32, 87, 111, 114, 108, 100, 33].into_iter();
        let result = Demo::parse_string(&mut input, 13);

        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn parse_i32() {
        let mut input: IntoIter<u8>= vec![49, 48, 48].into_iter();
        let result = Demo::parse_i32(&mut input);

        assert_eq!(result, 100)
    }
}