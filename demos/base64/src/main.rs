mod model;

use {
    flemish::{
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
pub enum Message {
    Encode(String),
    Decode(String),
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
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        let mut page = Flex::default_fill();
        {
            crate::build_editor("Normal text", &self.decode())
                .on_event(move |text| Message::Decode(text.buffer().unwrap().text()));
            Frame::default();
            crate::build_editor("Base64 text", &self.encode())
                .on_event(move |text| Message::Encode(text.buffer().unwrap().text()));
        }
        page.end();
        page.set_pad(0);
        page.set_margin(PAD);
        page.handle(crate::resize);
        page.child(self.focus()).unwrap().take_focus().unwrap();
        crate::resize(&mut page, Event::Resize);
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Decode(value) => self.set_decode(value),
            Message::Encode(value) => self.set_encode(value),
        }
    }
}

fn build_editor(tooltip: &str, value: &str) -> TextEditor {
    let mut element = TextEditor::default();
    element.set_tooltip(tooltip);
    element.set_linenumber_width(0);
    element.set_buffer(TextBuffer::default());
    element.buffer().unwrap().set_text(value);
    element.wrap_mode(WrapMode::AtBounds, 0);
    element.set_text_size(16);
    element.kf_end();
    element.set_text_font(Font::CourierBold);
    element.set_trigger(CallbackTrigger::Changed);
    element
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
