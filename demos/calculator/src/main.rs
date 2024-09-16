#![forbid(unsafe_code)]
mod model;

use {
    flemish::{
        app,
        button::Button,
        cascade,
        color_themes,
        enums::{Align, Color, Event, Font, FrameType, Key, Shortcut},
        frame::Frame,
        group::Flex,
        image::SvgImage,
        menu::MenuButton,
        prelude::*,
        text::{TextBuffer, TextDisplay, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const EQUAL: &str = "=";
const COLORS: [[Color; 6]; 2] = [
    [
        Color::from_hex(0xfdf6e3),
        Color::from_hex(0x586e75),
        Color::from_hex(0xb58900),
        Color::from_hex(0xeee8d5),
        Color::from_hex(0xcb4b16),
        Color::from_hex(0xdc322f),
    ],
    [
        Color::from_hex(0x002b36),
        Color::from_hex(0x93a1a1),
        Color::from_hex(0x268bd2),
        Color::from_hex(0x073642),
        Color::from_hex(0x6c71c4),
        Color::from_hex(0xd33682),
    ],
];
const NAME: &str = "FlCalculator";

fn main() {
    Model::new().run(Settings {
        size: (360, 640),
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        color_map: Some(color_themes::TAN_THEME),
        on_close_fn: Some(Box::new(move |_| {
            if app::event() == Event::Close {
                let (s, _) = app::channel::<Message>();
                s.send(Message::Quit);
            }
        })),
        ..Default::default()
    })
}

#[derive(PartialEq, Clone)]
pub enum Message {
    Click(String),
    Theme,
    Quit,
}

impl Sandbox for Model {
    type Message = Message;

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn new() -> Self {
        Model::default()
    }

    fn view(&mut self) {
        cascade!(
            Flex::default_fill().column();
            ..set_margin(PAD);
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..set_frame(FrameType::FlatBox);
            ..set_color(COLORS[self.theme as usize][0]);
            ..add(&crate::display("Output", &self.output, self.theme as usize));
            ..fixed(&cascade!(
                Flex::default();
                ..set_pad(0);
                ..set_margin(0);
                ..fixed(&menu(self.theme as usize),30);
                ..fixed(
                    &crate::output("Operation", self.theme as usize).with_label(&self.operation),
                    30
                );
                ..add(&cascade!(
                    Flex::default().column();
                    ..set_pad(0);
                    ..add(&output("Previous", self.theme as usize).with_label(&self.prev.to_string()));
                    ..add(&output("Current", self.theme as usize).with_label(&self.current));
                    ..end();
                ));
                ..end();
            ), 60);
            ..fixed(&{
                let mut buttons = Flex::default_fill().column();
                for line in [
                    ["CE", "C", "%", "/"],
                    ["7", "8", "9", "x"],
                    ["4", "5", "6", "-"],
                    ["1", "2", "3", "+"],
                    ["0", ".", "@<-", crate::EQUAL],
                ] {
                    let mut row = Flex::default();
                    for label in line {
                        crate::button(label, self.theme as usize)
                            .on_event(move |_| Message::Click(label.to_string()));
                    }
                    row.end();
                    row.set_pad(PAD);
                    row.set_margin(0);
                }
                buttons.end();
                buttons.set_pad(PAD);
                buttons.set_margin(0);
                buttons
            }, 425);
        ).end();
        app::set_font(Font::Courier);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Click(value) => self.click(&value),
            Message::Theme => self.theme = !self.theme,
            Message::Quit => {
                self.save();
                app::quit();
            }
        };
    }
}

fn display(tooltip: &str, value: &str, theme: usize) -> TextDisplay {
    cascade!(
        TextDisplay::default();
        ..set_tooltip(tooltip);
        ..set_buffer(TextBuffer::default());
        ..buffer().unwrap().set_text(value);
        ..set_text_size(HEIGHT - 5);
        ..set_scrollbar_size(3);
        ..set_frame(FrameType::FlatBox);
        ..wrap_mode(WrapMode::AtBounds, 0);
        ..set_color(COLORS[theme][0]);
        ..set_text_color(COLORS[theme][1]);
        ..set_text_font(Font::CourierBold);
        ..scroll(String::from(value).split_whitespace().count() as i32,0);
    )
}

fn output(tooltip: &str, theme: usize) -> Frame {
    cascade!(
        Frame::default();
        ..set_align(Align::Right | Align::Inside);
        ..set_tooltip(tooltip);
        ..set_label_size(HEIGHT);
        ..set_frame(FrameType::FlatBox);
        ..set_label_font(Font::CourierBold);
        ..set_color(COLORS[theme][0]);
        ..set_label_color(COLORS[theme][1]);
    )
}

fn button(label: &'static str, theme: usize) -> Button {
    cascade!(
        Button::default().with_label(label);
        ..set_label_size(HEIGHT);
        ..set_label_font(Font::CourierBold);
        ..set_frame(FrameType::OFlatFrame);
        ..set_shortcut(match label {
            "@<-" => Shortcut::None | Key::BackSpace,
            "CE" => Shortcut::None | Key::Delete,
            crate::EQUAL => Shortcut::None | Key::Enter,
            "x" => Shortcut::None | '*',
            _ => Shortcut::None | label.chars().next().unwrap(),
        });
        ..set_color(match label {
            "CE" | "C" | "x" | "/" | "+" | "-" | "%" => COLORS[theme][2],
            crate::EQUAL => COLORS[theme][5],
            _ => COLORS[theme][4],
        });
        ..set_label_color(COLORS[theme][0]);
        ..set_selection_color(COLORS[theme][0]);
    )
}

pub fn menu(theme: usize) -> MenuButton {
    cascade!(
        MenuButton::default();
        ..set_tooltip("Theme");
        ..add_choice("Switch");
        ..set_frame(FrameType::FlatBox);
        ..set_color(COLORS[theme][0]);
        ..set_text_color(COLORS[theme][1]);
        ..clone().on_event(move |_| Message::Theme);
    )
}
