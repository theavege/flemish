#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        cascade,
        browser::{Browser, BrowserType},
        button::Button,
        color_themes, draw,
        enums::{Color, FrameType},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        prelude::*,
        OnEvent, Sandbox, Settings,
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
        resizable: true,
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

#[derive(Clone)]
pub enum Msg {
    Choice(usize),
    Save,
}

impl Sandbox for Model {
    type Message = Msg;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        Self::default()
    }

    fn view(&mut self) {
        cascade!(
            Flex::default_fill();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..fixed(&cascade!(
                Flex::default_fill()
                .column();
                ..set_pad(PAD);
                ..add(&cascade!(
                    crate::browser("Browser", self.clone());
                    ..clone().on_event(move |browser| {
                        Msg::Choice((browser.value() as usize).saturating_sub(1))
                    });
                ));
                ..fixed(&cascade!(
                    Button::default();
                    ..set_label("@#refresh");
                    ..set_tooltip("Load image");
                    ..clone().on_event(move |_| Msg::Save);
                ), HEIGHT);
                ..end();
            ), WIDTH);
            ..add(&frame("Canvas", self.clone()));
        )
        .end();
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Save => self.init(),
            Msg::Choice(value) => self.choice(value),
        }
    }
}

fn browser(tooltip: &str, value: Model) -> Browser {
    let mut element = Browser::default().with_type(BrowserType::Hold);
    element.set_tooltip(tooltip);
    if !value.temp.is_empty() {
        for item in value.temp {
            element.add(&item);
        }
        element.select(value.curr as i32 + 1);
    }
    element
}

fn frame(tooltip: &str, value: Model) -> Frame {
    cascade!(
        Frame::default();
        ..set_frame(FrameType::DownBox);
        ..set_tooltip(tooltip);
        ..set_color(Color::Black);
        ..draw(move |frame| {
            if !value.temp.is_empty() {
                if let Some(data) = value.cash.get(&value.temp[value.curr]) {
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
                            let col = if close > open {
                                Color::Red
                            } else {
                                Color::Green
                            };
                            draw::set_draw_color(col);
                            draw::draw_rectf(idx - 2, open, 4, i32::abs(close - open));
                            draw::set_draw_color(Color::White);
                            idx += step;
                        }
                    };
                }
            }
        });
    )
}
