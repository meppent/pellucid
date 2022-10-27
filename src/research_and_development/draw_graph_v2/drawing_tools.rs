use font_kit::{
    family_name::FamilyName,
    font::Font,
    properties::{Properties, Weight},
    source::SystemSource,
};
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, Path, PathBuilder, Point, SolidSource, Source,
    StrokeStyle,
};
use std::ops::Add;

use crate::tools::utils::random_u8;

use super::specs::{ColorGeneration, BACKGROUND_COLOR, COLOR_GEN_MODE, CYCLIC_COLORS};

/*
(0,0) ――> x
  |
  |
  V
  y
*/
#[derive(Clone, Copy, Debug)]
pub struct Coord {
    pub x: f32,
    pub y: f32,
}
#[derive(Clone, Copy, Debug)]
pub struct Dims {
    pub width: f32,
    pub height: f32,
}

impl Add<Dims> for Coord {
    type Output = Coord;

    fn add(self, dims: Dims) -> Coord {
        return Coord {
            x: self.x + dims.width,
            y: self.y + dims.height,
        };
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

static mut RANDOM_COLOR_INDEX: usize = 0;
impl Color {
    pub fn new_random(alpha: u8) -> Self {
        match COLOR_GEN_MODE {
            ColorGeneration::_RANDOM => {
                return Color {
                    alpha,
                    red: random_u8(50, 200),
                    green: random_u8(50, 200),
                    blue: random_u8(50, 200),
                };
            }
            ColorGeneration::CYCLIC => {
                let index: usize;
                unsafe {
                    index = RANDOM_COLOR_INDEX;
                    RANDOM_COLOR_INDEX = (RANDOM_COLOR_INDEX + 1) % CYCLIC_COLORS.len();
                }
                return CYCLIC_COLORS[index];
            }
        };
    }
}

impl Color {
    pub fn dim(&self) -> Color {
        return Color {
            alpha: self.alpha / 3,
            red: self.red,
            green: self.green,
            blue: self.blue,
        };
    }
}
pub struct Drawing {
    dt: DrawTarget,
    font: Font,
    line_width: f32,
    box_padding: f32,
}

impl Drawing {
    pub fn new(image_width: i32, image_height: i32) -> Self {
        let mut dt: DrawTarget = DrawTarget::new(image_width, image_height);
        dt.clear(Self::_to_solid_source(BACKGROUND_COLOR));

        let font: Font = SystemSource::new()
            .select_best_match(
                &[FamilyName::Serif],
                &Properties::new().weight(Weight::THIN),
            )
            .unwrap()
            .load()
            .unwrap();

        let line_width: f32 = 2.;
        let box_padding: f32 = 10.;

        return Drawing {
            dt,
            font,
            line_width,
            box_padding,
        };
    }

    pub fn get_text_height(&self, text: &str, text_size: f32) -> f32 {
        let height: f32 = text_size * text.split("\n").count() as f32;
        return height;
    }
    pub fn draw_text(
        &mut self,
        text: &str,
        start: Coord,
        max_chars_per_line: usize,
        text_color: Color,
        text_size: f32,
    ) {
        for (line_index, line_str) in text.split("\n").enumerate() {
            let line_str = if line_str.len() <= max_chars_per_line {
                line_str.to_owned()
            } else {
                format!("{}...", line_str[0..(max_chars_per_line - 3)].to_owned())
            };
            self.dt.draw_text(
                &self.font,
                text_size,
                &line_str,
                Point::new(
                    start.x as f32,
                    start.y + (line_index + 1) as f32 * text_size,
                ),
                &Source::Solid(Self::_to_solid_source(text_color)),
                &DrawOptions::new(),
            );
        }
    }

    pub fn draw_boxed_text(
        &mut self,
        text: &str,
        start: Coord,
        inner_width: f32,
        color: Color,
        text_size: f32,
    ) -> Dims {
        let inner_height: f32 = self.get_text_height(text, text_size);
        let (inner_start, out_dims): (Coord, Dims) = self.draw_box(
            start,
            Dims {
                width: inner_width,
                height: inner_height,
            },
            color,
        );
        let max_chars_per_line: usize = (2. * inner_width / text_size) as usize;
        self.draw_text(text, inner_start, max_chars_per_line, color, text_size);
        return out_dims;
    }

    pub fn draw_box(&mut self, out_start: Coord, inner_dims: Dims, color: Color) -> (Coord, Dims) {
        let width: f32 = inner_dims.width + 2. * (self.line_width + self.box_padding);
        let height: f32 = inner_dims.height + 2. * (self.line_width + self.box_padding);
        let up_right: Coord = out_start + Dims { width, height: 0. };
        let down_left: Coord = out_start + Dims { width: 0., height };
        let down_right: Coord = out_start + Dims { width, height };
        self.draw_line(out_start, up_right, color);
        self.draw_line(up_right, down_right, color);
        self.draw_line(down_right, down_left, color);
        self.draw_line(down_left, out_start, color);

        let inner_start: Coord = Coord {
            x: out_start.x + self.line_width + self.box_padding,
            y: out_start.y + self.line_width + self.box_padding,
        };
        let out_dims: Dims = Dims { width, height };
        return (inner_start, out_dims);
    }

    pub fn draw_line(&mut self, from: Coord, to: Coord, color: Color) {
        let mut pb: PathBuilder = PathBuilder::new();
        pb.move_to(from.x, from.y);
        pb.line_to(to.x, to.y);
        let path: Path = pb.finish();
        self.dt.stroke(
            &path,
            &Source::Solid(Self::_to_solid_source(color)),
            &StrokeStyle {
                width: self.line_width,
                cap: LineCap::Butt,
                join: LineJoin::Miter,
                miter_limit: 10.,
                dash_array: Vec::new(),
                dash_offset: 0.,
            },
            &DrawOptions::new(),
        );
    }

    pub fn save_image(&self, path: &str) {
        log::info!("building png...");
        self.dt.write_png(path).unwrap();
    }

    fn _to_solid_source(color: Color) -> SolidSource {
        return SolidSource::from_unpremultiplied_argb(
            color.alpha,
            color.red,
            color.green,
            color.blue,
        );
    }
}
