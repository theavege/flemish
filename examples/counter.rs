#![forbid(unsafe_code)]

use flemish::{
    button::Button, cascade, enums::FrameType, frame::Frame, glib, group::Flex, mpsc, prelude::*,
    Sandbox, Settings,
};

pub fn main() {
    Model::new().run(Settings {
        size: (640, 360),
        ..Default::default()
    })
}

#[derive(Debug, Clone, Copy)]
enum Msg {
    Inc,
    Dec,
}

#[derive(Default)]
struct Model {
    value: u8,
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("{} - 7GUI: Counter", self.value)
    }

    fn view(&mut self, sender: mpsc::Sender<Msg>) -> Flex {
        cascade!(
            Flex::default().with_size(300, 100).center_of_parent();
            ..set_margin(10);
            ..set_pad(0);
            ..add(&cascade!(
                Button::default().with_label("@#<");
                ..set_label_size(30);
                ..set_callback(glib::clone!(#[strong] sender, move |_| {
                    sender.send(Msg::Dec).unwrap();
                }));
            ));
            ..add(&cascade!(
                Frame::default().with_label(&self.value.to_string());
                ..set_frame(FrameType::UpBox);
                ..set_label_size(30);
            ));
            ..add(&cascade!(
                Button::default().with_label("@#>");
                ..set_label_size(30);
                ..set_callback(glib::clone!(#[strong] sender, move |_| {
                    sender.send(Msg::Inc).unwrap();
                }));
            ));
            ..end();
        )
    }

    fn update(&mut self, message: Msg) -> bool {
        match message {
            Msg::Dec => {
                self.value = self.value.saturating_sub(1);
                true
            }
            Msg::Inc => {
                self.value = self.value.saturating_add(1);
                true
            }
        }
    }
}
