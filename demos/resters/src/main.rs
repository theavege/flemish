#![forbid(unsafe_code)]

mod model;

use {
    flemish::{
        app,
        button::Button,
        cascade, color_themes,
        enums::{CallbackTrigger, Color, Event, Font, FrameType},
        frame::Frame,
        group::{Flex, FlexType},
        image::SvgImage,
        menu::Choice,
        misc::{InputChoice, Progress},
        prelude::*,
        text::{StyleTableEntry, TextBuffer, TextDisplay, WrapMode},
        OnEvent, Sandbox, Settings,
    },
    json_tools::{Buffer, BufferType, Lexer, Span, TokenType},
    model::Model,
};

const NAME: &str = "FlResters";

fn main() {
    Model::new().run(Settings {
        resizable: true,
        size: (640, 360),
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        color_map: Some(color_themes::DARK_THEME),
        ..Default::default()
    })
}

#[derive(Clone)]
pub enum Msg {
    Method(u8),
    Url(String),
    Thread,
}

impl Sandbox for Model {
    type Message = Msg;

    fn new() -> Self {
        Model::default()
    }

    fn title(&self) -> String {
        String::from(NAME)
    }

    fn view(&mut self) {
        cascade!(
            Flex::default_fill();
            ..set_pad(PAD);
            ..set_margin(PAD);
            ..set_frame(FrameType::FlatBox);
            ..set_type(FlexType::Column);
            ..fixed(&cascade!(
                Flex::default(); //HEADER
                ..set_pad(0);
                ..fixed(&Frame::default(), WIDTH);
                ..fixed(&cascade!(
                    Choice::default();
                    ..add_choice("GET|POST");
                    ..set_label("Method: ");
                    ..set_value(self.method as i32);
                    ..clone().on_event(move |choice| Msg::Method(choice.value() as u8));
                ), WIDTH);
                ..fixed(&Frame::default(), WIDTH);
                ..add(&cascade!(
                    input(&self.url);
                    ..clone().on_event(move |input| Msg::Url(input.value().unwrap()));
                ));
                ..fixed(&cascade!(
                    Button::default();
                    ..set_label("@#search");
                    ..set_label_size(18);
                    ..clone().on_event(move |_| Msg::Thread);
                ), HEIGHT);
                ..end();
            ), HEIGHT);
            ..add(&build_display("Responce", &self.responce));
            ..fixed(&cascade!(
                Flex::default();
                ..fixed(&Frame::default().with_label("Status: "), WIDTH);
                ..add(&Frame::default());
                ..fixed(&crate::progress().with_label(&self.status), WIDTH);
                ..end();
            ), HEIGHT);
        )
        .end();
    }

    fn update(&mut self, message: Msg) {
        match message {
            Msg::Method(value) => self.method = value,
            Msg::Url(value) => self.url = value,
            Msg::Thread => {
                let clone = self.clone();
                let handler = std::thread::spawn(move || -> (bool, String) { clone.click() });
                while !handler.is_finished() {
                    app::wait();
                    app::handle_main(SPINNER).unwrap();
                    app::sleep(0.02);
                }
                if let Ok((status, check)) = handler.join() {
                    self.status = match status {
                        true => "OK",
                        false => "Fail",
                    }
                    .to_string();
                    self.responce = check;
                }
            }
        }
    }
}

fn progress() -> Progress {
    const MAX: u8 = 120;
    cascade!(
        Progress::default();
        ..set_maximum((MAX / 4 * 3) as f64);
        ..set_value(MAX as f64);
        ..handle(move |progress, event| {
            if event == crate::SPINNER {
                progress.set_value(if progress.value() == (MAX - 1) as f64 {
                    progress.minimum()
                } else {
                    progress.value() + 1f64
                });
                true
            } else {
                false
            }
        });
    )
}

fn input(value: &str) -> InputChoice {
    let mut element = InputChoice::default().with_label("URL: ");
    for item in ["users", "posts", "albums", "todos", "comments", "posts"] {
        element.add(&(format!(r#"https:\/\/jsonplaceholder.typicode.com\/{item}"#)));
    }
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/languages"#);
    element.add(r#"https:\/\/lingva.thedaviddelta.com\/api\/v1\/en\/de\/mother"#);
    element.add(r#"https:\/\/ipinfo.io\/json"#);
    element.set_value(value);
    element
}

fn build_display(tooltip: &str, value: &str) -> TextDisplay {
    cascade!(
        TextDisplay::default();
        ..set_tooltip(tooltip);
        ..set_linenumber_width(0);
        ..set_buffer(TextBuffer::default());
        ..buffer().unwrap().set_text(value);
        ..wrap_mode(WrapMode::AtBounds, 0);
        ..set_trigger(CallbackTrigger::Changed);
        ..set_callback(add_highlight);
        ..do_callback();
    )
}

fn add_highlight(display: &mut TextDisplay) {
    let text = display.buffer().unwrap().text();
    let styles: Vec<StyleTableEntry> = [0xdc322f, 0x268bd2, 0x859900]
        .into_iter()
        .map(|color| StyleTableEntry {
            color: Color::from_hex(color),
            font: Font::CourierBold,
            size: 16,
        })
        .collect();
    let mut buffer = vec![b'A'; text.len()];
    for token in Lexer::new(text.bytes(), BufferType::Span) {
        use TokenType::*;
        let c = match token.kind {
            CurlyOpen | CurlyClose | BracketOpen | BracketClose | Colon | Comma | Invalid => 'A',
            String => 'B',
            BooleanTrue | BooleanFalse | Null => 'C',
            Number => 'D',
        };
        if let Buffer::Span(Span { first, end }) = token.buf {
            let start = first as _;
            let last = end as _;
            buffer[start..last].copy_from_slice(c.to_string().repeat(last - start).as_bytes());
        }
    }
    let mut buf = TextBuffer::default();
    buf.set_text(&String::from_utf8_lossy(&buffer));
    display.scroll(text.split_whitespace().count() as i32, 0);
    display.set_highlight_data(buf, styles);
}

const SPINNER: Event = Event::from_i32(405);
const PAD: i32 = 10;
const HEIGHT: i32 = PAD * 3;
const WIDTH: i32 = HEIGHT * 3;
