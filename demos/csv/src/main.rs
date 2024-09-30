#![forbid(unsafe_code)]

mod model;

use {
    ::image::{ImageBuffer, RgbImage},
    flemish::{
        cascade, draw,
        enums::{Color, FrameType},
        frame::Frame,
        glib,
        group::Flex,
        image::SvgImage,
        menu::Choice,
        mpsc,
        prelude::*,
        surface::ImageSurface,
        Sandbox, Settings,
    },
    model::Model,
};

const NAME: &str = "FlCSV";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        ..Default::default()
    })
}

#[derive(Clone)]
pub enum Msg {
    Choice(usize),
}

impl Sandbox for Model {
    type Message = Msg;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        cascade!(
            Self::default();
            ..init();
        )
    }

    fn view(&mut self, sender: mpsc::Sender<Msg>) -> Flex {
        cascade!(
            Flex::default_fill();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..fixed(&cascade!(
                Flex::default_fill().column();
                ..set_pad(PAD);
                ..add(&Frame::default());
                ..fixed(&cascade!(
                    Choice::default();
                    ..set_tooltip("Browser");
                    ..add_choice(&self.list.join("|"));
                    ..set_value(self.curr as i32);
                    ..set_callback(glib::clone!(#[strong] sender, move |choice| {
                        sender.send(Msg::Choice(choice.value() as usize)).unwrap();
                    }));
                ), HEIGHT);
                ..end();
            ), WIDTH);
            ..add(&frame("Canvas", self.clone()));
            ..end();
        )
    }

    fn update(&mut self, message: Msg) -> bool {
        match message {
            Msg::Choice(value) => self.choice(value),
        }
    }
}

fn frame(tooltip: &str, value: Model) -> Frame {
    cascade!(
        Frame::default();
        ..set_frame(FrameType::DownBox);
        ..set_tooltip(tooltip);
        ..set_color(Color::Background2);
        ..set_callback(add_save);
        ..draw(move |frame| {
            if !value.list.is_empty() {
                if let Some(data) = value.cash.get(&value.list[value.curr]) {
                    let mut highest = data
                        .iter()
                        .map(|elem| elem.low)
                        .collect::<Vec<f64>>()
                        .iter()
                        .cloned()
                        .fold(f64::NAN, f64::max);
                    highest += (highest.to_string().len() * 10) as f64 / 3.;
                    let factor = frame.h() as f64 / highest;
                    if !data.is_empty() {
                        let step = frame.w() / data.len() as i32;
                        let mut idx = frame.x() + step;
                        for elem in data {
                            let open = frame.h() - (elem.open * factor) as i32;
                            let high = frame.h() - (elem.high * factor) as i32;
                            let low = frame.h() - (elem.low * factor) as i32;
                            let close = frame.h() - (elem.close * factor) as i32;
                            draw::draw_line(idx, high, idx, low);
                            draw::set_draw_color(match close > open {
                                true => Color::Red,
                                false => Color::Green,
                            });
                            draw::draw_rectf(idx - 2, open, 4, i32::abs(close - open));
                            draw::set_draw_color(Color::Blue);
                            idx += step;
                        }
                    };
                }
            }
        });
    )
}

fn add_save(frame: &mut Frame) {
    let sur = ImageSurface::new(frame.w(), frame.h(), false);
    ImageSurface::push_current(&sur);
    draw::set_draw_color(Color::White);
    draw::draw_rectf(0, 0, frame.w(), frame.h());
    sur.draw(frame, 0, 0);
    let img = sur.image().unwrap();
    ImageSurface::pop_current();
    let mut imgbuf: RgbImage = ImageBuffer::new(frame.w() as _, frame.h() as _); // this is from the image crate
    imgbuf.copy_from_slice(&img.to_rgb_data());
    imgbuf
        .save(frame.window().unwrap().label() + ".jpg")
        .unwrap();
}
