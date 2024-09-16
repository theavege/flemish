#![forbid(unsafe_code)]

mod model;

use {
    chrono::prelude::*,
    flemish::{
        button::Button, cascade, color_themes, dialog::alert_default, enums::FrameType,
        frame::Frame, group::Flex, menu::Choice, output::Output, prelude::*, Calendar, OnEvent,
        Sandbox, Settings,
    },
    model::Model,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

const NAME: &str = "7GUI: Flightbooker";
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = PAD * 12;

#[derive(Clone)]
pub enum Msg {
    Direct(i32),
    Start(String),
    Back(String),
    Book,
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
            Flex::default().with_size(PAD * 26, PAD * 17).center_of_parent();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..set_frame(FrameType::UpBox);
            ..fixed(&Frame::default(), WIDTH);
            ..add(&cascade!(
                Flex::default().column();
                ..fixed(&cascade!(
                    Choice::default().with_label("Direct");
                    ..add_choice("one-way flight|return flight");
                    ..set_value(self.direct);
                    ..clone().on_event(move |choice| Msg::Direct(choice.value()));
                ), HEIGHT);
                ..fixed(&cascade!(
                    crate::input(&self.start, self.start_active).with_label("Start");
                    ..clone().on_event(move |_| {
                        if let Some(date) = Calendar::default().get_date() {
                            Msg::Start(format!("{}-{}-{}", date.year(), date.month(), date.day()));
                        }
                    });
                ), HEIGHT);
                ..fixed(&cascade!(
                    crate::input(&self.back, self.back_active).with_label("Back");
                    ..clone().on_event(move |_| {
                        if let Some(date) = Calendar::default().get_date() {
                            Msg::Back(format!("{}-{}-{}", date.year(), date.month(), date.day()));
                        }
                    });
                ), HEIGHT);
                ..fixed(&cascade!(
                    crate::button(self.book_active).with_label("Book");
                        ..clone().on_event(move |_| Msg::Book);
                ), HEIGHT);
                ..end();
                ..set_pad(PAD);
            ));
        )
        .end();
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Direct(value) => self.direct(value),
            Msg::Start(value) => self.start(value),
            Msg::Back(value) => self.back(value),
            Msg::Book => {
                alert_default(&if self.direct == 0 {
                    format!("You have booked a one-way flight for {}.", self.start)
                } else {
                    format!(
                        "You have booked a return flight from {} to {}",
                        self.start, self.back
                    )
                });
            }
        }
    }
}

fn input(value: &str, active: bool) -> Output {
    let mut element = Output::default();
    element.set_value(value);
    match active {
        true => element.activate(),
        false => element.deactivate(),
    };
    element
}

fn button(active: bool) -> Button {
    let mut element = Button::default();
    match active {
        true => element.activate(),
        false => element.deactivate(),
    };
    element
}
