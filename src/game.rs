use image::{DynamicImage, GenericImageView, SubImage};
use rand::{thread_rng, Rng};

use crate::capture::Capture;
use crate::images::{compare_color, compare_image, find_sub, open};
use crate::mouse::{left_click, move_to, rigth_click};

const CELL_SIZE: i32 = 16;

pub struct Game {
    capture: Capture,
    img: DynamicImage,
    reset: (f64, f64),
    top_left: (i32, i32),
    cell_count: (i32, i32),
    field: Vec<Vec<FieldState>>,
}

impl Game {
    pub fn new() -> Game {
        let capture = Capture::new();

        let img = capture.get_img();

        let reset = open("./resources/alive.png");

        let reset_location = find_sub(&img, &reset).unwrap();
        let reset_center_location = (
            (reset_location.0 as f64 + (reset.width() / 2) as f64),
            (reset_location.1 as f64 + (reset.height() / 2) as f64),
        );

        let screen_y = Game::find_screen_y(reset_location, &img);
        let screen_x = Game::find_screen_x(reset_location, screen_y, &img);

        let cell_count = (
            (screen_x.1 - screen_x.0) / CELL_SIZE,
            (screen_y.1 - screen_y.0) / CELL_SIZE,
        );

        Game {
            capture,
            img,
            reset: reset_center_location,
            top_left: (screen_x.0 + 1, screen_y.0 + 1),
            cell_count,
            field: vec![],
        }
    }

    fn find_screen_y(reset_location: (u32, u32), img: &DynamicImage) -> (i32, i32) {
        let search_x = (reset_location.0 - 3) as i32;
        let mut search_y = reset_location.1 as i32;

        let mut min_y = -1;
        let max_y;

        let mut count = 0;

        loop {
            let pixel = img.get_pixel(search_x as u32, search_y as u32);

            if compare_color(pixel.0, [128, 128, 128, 255]) < 3 {
                count += 1;

                if count >= 3 {
                    if min_y == -1 {
                        min_y = search_y;
                        count = 0;
                    } else {
                        max_y = search_y;
                        break;
                    }
                }
            } else {
                count = 0;
            }

            search_y += 1;
        }

        (min_y, max_y - 2 - 9)
    }

    fn find_screen_x(
        reset_location: (u32, u32),
        screen_y: (i32, i32),
        img: &DynamicImage,
    ) -> (i32, i32) {
        let mut search_x = reset_location.0 as i32;
        let search_y = screen_y.0 + 5;

        let min_x;
        let max_x;

        let mut count = 0;

        loop {
            let pixel = img.get_pixel(search_x as u32, search_y as u32);

            if compare_color(pixel.0, [128, 128, 128, 255]) < 3 {
                count += 1;

                if count >= 3 {
                    min_x = search_x;
                    break;
                }
            } else {
                count = 0;
            }

            search_x -= 1;
        }

        count = 0;
        search_x = reset_location.0 as i32;

        loop {
            let pixel = img.get_pixel(search_x as u32, search_y as u32);

            if compare_color(pixel.0, [128, 128, 128, 255]) < 3 {
                count += 1;

                if count >= 3 {
                    max_x = search_x;
                    break;
                }
            } else {
                count = 0;
            }

            search_x += 1;
        }

        (min_x + 2, max_x - 2 - 9)
    }

    pub fn update_img(&mut self) {
        self.img = self.capture.get_img();
    }

    pub fn update_field(&mut self) {
        self.field.clear();

        for x in 0..self.cell_count.0 {
            let mut line = vec![];
            for y in 0..self.cell_count.1 {
                let x_pos = self.top_left.0 + x * CELL_SIZE;
                let y_pos = self.top_left.1 + y * CELL_SIZE;

                line.push(Game::identify_field_state(self.img.view(
                    x_pos as u32,
                    y_pos as u32,
                    CELL_SIZE as u32,
                    CELL_SIZE as u32,
                )));
            }

            self.field.push(line);
        }
    }

    fn identify_field_state(img: SubImage<&DynamicImage>) -> FieldState {
        if compare_image(&open("./resources/unknown.png"), &img) {
            return FieldState::Unknown;
        }

        if compare_image(&open("./resources/0.png"), &img) {
            return FieldState::Value(0);
        }
        if compare_image(&open("./resources/1.png"), &img) {
            return FieldState::Value(1);
        }
        if compare_image(&open("./resources/2.png"), &img) {
            return FieldState::Value(2);
        }
        if compare_image(&open("./resources/3.png"), &img) {
            return FieldState::Value(3);
        }
        if compare_image(&open("./resources/4.png"), &img) {
            return FieldState::Value(4);
        }
        if compare_image(&open("./resources/5.png"), &img) {
            return FieldState::Value(5);
        }
        if compare_image(&open("./resources/6.png"), &img) {
            return FieldState::Value(6);
        }
        if compare_image(&open("./resources/7.png"), &img) {
            return FieldState::Value(7);
        }
        if compare_image(&open("./resources/8.png"), &img) {
            return FieldState::Value(8);
        }

        if compare_image(&open("./resources/flag.png"), &img) {
            return FieldState::Flag;
        }

        panic!("unknown Field")
    }

    pub fn has_lost(&self) -> bool {
        let dead = open("./resources/dead.png");

        let view = self.img.view(
            (self.reset.0 - dead.width() as f64 / 2.) as u32,
            (self.reset.1 - dead.height() as f64 / 2.) as u32,
            dead.width(),
            dead.height(),
        );

        compare_image(&dead, &view)
    }

    pub fn reset(&self) {
        move_to(self.reset);
        left_click();
    }

    pub fn choose_random(&self) {
        let mut rng = thread_rng();

        let mut fields = vec![];

        for (x, line) in self.field.iter().enumerate() {
            for (y, state) in line.iter().enumerate() {
                match state {
                    FieldState::Unknown => fields.push((x, y)),
                    _ => {}
                }
            }
        }

        if fields.len() == 0 {
            panic!("Finished");
        }

        let location = fields.get(rng.gen_range(0..fields.len())).unwrap();
        self.move_to_cell(location.0 as i32, location.1 as i32);

        left_click();
    }

    fn move_to_cell(&self, x: i32, y: i32) {
        move_to((
            (self.top_left.0 + x * CELL_SIZE + CELL_SIZE / 2) as f64,
            (self.top_left.1 + y * CELL_SIZE + CELL_SIZE / 2) as f64,
        ));
    }

    pub fn choose_field(&self) {
        for (x, line) in self.field.iter().enumerate() {
            let x = x as i32;
            for (y, state) in line.iter().enumerate() {
                let y = y as i32;

                match state {
                    FieldState::Value(value) => {
                        let mut flags = vec![];
                        let mut unknowns = vec![];

                        for x_off in -1..=1 {
                            for y_off in -1..=1 {
                                if x_off == 0 && y_off == 0 {
                                    continue;
                                }

                                if (x_off + x) < 0 || (x_off + x) >= self.field.len() as i32 {
                                    continue;
                                }

                                if (y_off + y) < 0 || (y_off + y) >= line.len() as i32 {
                                    continue;
                                }

                                let x = (x + x_off) as usize;
                                let y = (y + y_off) as usize;

                                let state = self.field.get(x).unwrap().get(y).unwrap();

                                match state {
                                    FieldState::Value(_) => {}
                                    FieldState::Unknown => unknowns.push((x, y)),
                                    FieldState::Flag => flags.push((x, y)),
                                }
                            }
                        }

                        if (unknowns.len() + flags.len()) == *value as usize && unknowns.len() > 0 {
                            for pos in unknowns {
                                self.move_to_cell(pos.0 as i32, pos.1 as i32);
                                rigth_click();
                            }
                            return;
                        } else if flags.len() == *value as usize && unknowns.len() > 0 {
                            for pos in unknowns {
                                self.move_to_cell(pos.0 as i32, pos.1 as i32);
                                left_click();
                            }
                            return;
                        }
                    }
                    _ => {}
                }
            }
        }

        self.choose_random();
    }
}

pub enum FieldState {
    Value(i32),
    Unknown,
    Flag,
}
