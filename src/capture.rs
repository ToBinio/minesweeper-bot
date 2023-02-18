use std::io::Cursor;
use std::thread;
use std::time::Duration;

use image::io::Reader;
use image::DynamicImage;
use screenshots::Screen;

pub struct Capture {
    screen: Screen,
}

impl Capture {
    pub fn new() -> Capture {
        Capture {
            screen: Screen::all().unwrap().get(0).unwrap().to_owned(),
        }
    }

    pub fn get_img(&self) -> DynamicImage {
        thread::sleep(Duration::from_millis(20));

        let img = self.screen.capture().unwrap();

        let img = Reader::new(Cursor::new(img.buffer()))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();

        let img = DynamicImage::from(img);

        img.save("./resources/img.png").unwrap();

        img
    }
}
