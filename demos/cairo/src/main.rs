mod model;

use {
    cairo::Context,
    flemish::{
        cascade,
        enums::{Color, Event},
        frame::Frame,
        prelude::*,
        OnEvent, Sandbox, Settings,
    },
    model::Model,
};

#[derive(Clone, Copy)]
pub enum Message {
    Change(usize),
}

fn main() {
    Model::new().run(Settings {
        size: (260, 260),
        background: Some(Color::White),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("{} - FlCairo", self.state[0])
    }

    fn view(&mut self) {
        fltk::app::cairo::set_autolink_context(true);
        cascade!(
            cairowidget(5, 5, 100, 100, "Box0");
            ..set_color(match self.state[0] {
                true => Color::Red,
                false => Color::DarkRed,
            });
        )
        .on_event(move |_| Message::Change(0));
        cascade!(
            cairowidget(80, 80, 100, 100, "Box1");
            ..set_color(match self.state[0] {
                true => Color::Yellow,
                false => Color::DarkYellow,
            });
        )
        .on_event(move |_| Message::Change(1));
        cascade!(
            cairowidget(155, 155, 100, 100, "Box2");
            ..set_color(match self.state[0] {
                true => Color::Green,
                false => Color::DarkGreen,
            });
        )
        .on_event(move |_| Message::Change(2));
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Change(idx) => self.change(idx),
        }
    }
}

pub fn cairowidget(x: i32, y: i32, w: i32, h: i32, label: &str) -> Frame {
    cascade!(
        Frame::new(x, y, w, h, None);
        ..set_label(label);
        ..super_draw_first(false); // required for windows
        ..handle(move |frame, event| {
            if event == Event::Push {
                frame.do_callback();
                return true;
            };
            false
        });
        ..draw(move |frame| {
            let (r, g, b) = frame.color().to_rgb();
            let ctx = unsafe { Context::from_raw_none(fltk::app::cairo::cc() as _) };
            ctx.save().unwrap();
            ctx.move_to(frame.x() as f64, frame.y() as f64);
            ctx.line_to((frame.x() + frame.w()) as f64, frame.y() as f64);
            ctx.line_to((frame.x() + frame.w()) as f64, (frame.y() + frame.h()) as f64);
            ctx.line_to(frame.x() as f64, (frame.y() + frame.h()) as f64);
            ctx.close_path();
            ctx.set_source_rgba(
                r as f64 / 255.0,
                g as f64 / 255.0,
                b as f64 / 255.0,
                100.0 / 255.0,
            );
            ctx.fill().unwrap();
            ctx.restore().unwrap();
        });
    )
}
