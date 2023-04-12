use std::fs;
use std::str;

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
        let file = fs::read(file_name).unwrap();
        
        // let demo_protocol = Demo::parse_i32(&mut demo_bytes_iter);
        // println!("{}", demo_protocol);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let result = Demo::parse("test_demo.dem");
        assert_eq!(result, ())
    }
}