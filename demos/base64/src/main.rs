mod model;

use {
    flemish::{
        cascade,
        color_themes,
        enums::{CallbackTrigger, Event, Font},
        frame::Frame,
        group::{Flex, FlexType},
        image::SvgImage,
        prelude::*,
        text::{TextBuffer, TextEditor, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

const PAD: i32 = 10;
const NAME: &str = "FlBase64";

#[derive(Clone)]
pub enum Msg {
    Enc(String),
    Dec(String),
}

fn main() {
    Model::new().run(Settings {
        resizable: true,
        size: (360, 640),
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        cascade!(
            Flex::default_fill();
            ..set_pad(0);
            ..set_margin(PAD);
            ..add(&cascade!(
                build_editor("Normal text", self.decode());
                ..clone().on_event(move |text| Msg::Dec(text.buffer().unwrap().text()));
            ));
            ..add(&Frame::default());
            ..add(&cascade!(
                build_editor("Base64 text", self.encode());
                ..clone().on_event(move |text| Msg::Enc(text.buffer().unwrap().text()));
            ));
            ..handle(crate::resize);
            ..handle_event(Event::Resize);
            ..end();
        )
        .child(self.focus())
        .unwrap()
        .take_focus()
        .unwrap();
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Dec(value) => self.set_decode(value),
            Msg::Enc(value) => self.set_encode(value),
        }
    }
}

fn build_editor(tooltip: &str, value: &str) -> TextEditor {
    cascade!(
        TextEditor::default();
        ..set_tooltip(tooltip);
        ..set_linenumber_width(0);
        ..set_buffer(TextBuffer::default());
        ..buffer().unwrap().set_text(value);
        ..wrap_mode(WrapMode::AtBounds, 0);
        ..set_text_size(16);
        ..kf_end();
        ..set_text_font(Font::CourierBold);
        ..set_trigger(CallbackTrigger::Changed);
    )
}

fn resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        if let Some(window) = flex.window() {
            flex.set_type(match window.w() < window.h() {
                true => FlexType::Column,
                false => FlexType::Row,
            });
            flex.fixed(&flex.child(1).unwrap(), PAD);
        }
        return true;
    }
    false
}
