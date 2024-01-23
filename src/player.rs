use std::{path, io::BufReader, time::Duration};

use rodio::Source;

#[derive(Debug)]
pub struct Player<'a> {
    file_path: &'a path::Path
}

impl<'a> Player<'a> {

    pub fn new(file_path: &'a String) -> Self{
        Player {
            file_path: path::Path::new(file_path)
        }
    }

    pub fn play(&self) {

        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = std::fs::File::open(self.file_path.to_str().unwrap()).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);

        sink.sleep_until_end();

    }

    pub fn play_until(&self, duration: Duration) {


        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        let file = std::fs::File::open(self.file_path.to_str().unwrap()).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap()
            .stoppable()
            .take_duration(duration);
        sink.append(source);

        sink.sleep_until_end();

    }

}

mod test{
    
    #[test]
    fn play_test() {

        // let path = String::from("assets/hotel-bell-ding-1-174457.mp3");
        let path = String::from("assets/old-style-door-bell-101191.mp3");
        let player = Player::new(&path);
        player.play();

    }

    #[test]
    fn play_until_test() {

        let path = String::from("assets/old-style-door-bell-101191.mp3");
        let player = Player::new(&path);
        player.play_until(Duration::from_secs(2));
        
    }

}
