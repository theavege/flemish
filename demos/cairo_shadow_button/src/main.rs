#![forbid(unsafe_code)]

mod model;

use {
    cairo::{Context, Format, ImageSurface},
    flemish::{
        app,
        button::Button,
        cascade, draw,
        enums::{Align, Color, ColorDepth, Event, Font, Shortcut},
        frame::Frame,
        group::{Flex, FlexType},
        image::{RgbImage, SvgImage},
        menu::{MenuButton, MenuButtonType, MenuFlag},
        prelude::*,
        OnEvent, OnMenuEvent, Sandbox, Settings,
    },
    model::Model,
};

#[derive(Clone, Copy)]
pub enum Message {
    Inc,
    Dec,
    Quit,
}

const PAD: i32 = 100;
const NAME: &str = "FlCairoButton";

fn main() {
    Model::new().run(Settings {
        size: (360, 640),
        resizable: true,
        xclass: Some(String::from(NAME)),
        icon: Some(SvgImage::from_data(include_str!("../../assets/logo.svg")).unwrap()),
        background: Some(Color::from_u32(0xfdf6e3)),
        on_close_fn: Some(Box::new(move |_| {
            if app::event() == Event::Close {
                let (s, _) = app::channel::<Message>();
                s.send(Message::Quit);
            }
        })),
        ..Default::default()
    })
}

impl Sandbox for Model {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        format!("{} - {NAME}", self.value())
    }

    fn view(&mut self) {
        cascade!(
            Flex::default_fill();
            ..set_pad(0);
            ..set_margin(PAD);
            ..add(&cascade!(
                Flex::default(); //HERO
                ..add(&cascade!(
                    cairobutton().with_label("@#<");
                    ..clone().on_event(move |_| Message::Dec);
                ));
                ..add(&cascade!(
                    Frame::default();
                    ..set_label_size(60);
                    ..set_label(&self.value());
                    ..handle(crate::popup);
                ));
                ..add(&cascade!(
                    cairobutton().with_label("@#>");
                    ..clone().on_event(move |_| Message::Inc);
                ));
                ..end();
            ..handle(add_resize);
            ..handle_event(Event::Resize);
            ));
        )
        .end();
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Inc => self.inc(),
            Message::Dec => self.dec(),
            Message::Quit => {
                self.save();
                app::quit();
            }
        }
    }
}

fn add_resize(flex: &mut Flex, event: Event) -> bool {
    if event == Event::Resize {
        if let Some(window) = flex.window() {
            flex.set_type(match window.w() < window.h() {
                true => FlexType::Column,
                false => FlexType::Row,
            });
            flex.fixed(&flex.child(1).unwrap(), 0);
        }
        return true;
    }
    false
}

fn menu() -> MenuButton {
    MenuButton::default()
        .with_type(MenuButtonType::Popup3)
        .on_item_event(
            "@#+  &Increment",
            Shortcut::Ctrl | 'i',
            MenuFlag::Normal,
            move |_| Message::Inc,
        )
        .on_item_event(
            "@#-  &Decrement",
            Shortcut::Ctrl | 'd',
            MenuFlag::Normal,
            move |_| Message::Dec,
        )
        .on_item_event(
            "@#1+  Quit",
            Shortcut::Ctrl | 'q',
            MenuFlag::Normal,
            move |_| Message::Quit,
        )
}

fn popup(_: &mut Frame, event: Event) -> bool {
    match event {
        Event::Push => match app::event_mouse_button() {
            app::MouseButton::Right => {
                crate::menu().popup();
                true
            }
            _ => false,
        },
        _ => false,
    }
}

fn cairobutton() -> Button {
    cascade!(
        Button::default();
        ..super_draw(false);
        ..draw(|button| {
            draw::draw_rect_fill(
                button.x(),
                button.y(),
                button.w(),
                button.h(),
                Color::from_u32(0xfdf6e3),
            );
            let mut surface = ImageSurface::create(Format::ARgb32, button.w(), button.h())
                .expect("Couldn’t create surface");
            crate::draw_surface(&mut surface, button.w(), button.h());
            if !button.value() {
                cairo_blur::blur_image_surface(&mut surface, 20);
            }
            surface
                .with_data(|surface| {
                    RgbImage::new(surface, button.w(), button.h(), ColorDepth::Rgba8)
                        .unwrap()
                        .draw(button.x(), button.y(), button.w(), button.h());
                })
                .unwrap();
            draw::set_draw_color(Color::Black);
            draw::set_font(Font::Helvetica, app::font_size());
            if !button.value() {
                draw::draw_rbox(
                    button.x() + 1,
                    button.y() + 1,
                    button.w() - 6,
                    button.h() - 6,
                    15,
                    true,
                    Color::White,
                );
                draw::draw_text2(
                    &button.label(),
                    button.x() + 1,
                    button.y() + 1,
                    button.w() - 6,
                    button.h() - 6,
                    Align::Center,
                );
            } else {
                draw::draw_rbox(
                    button.x() + 1,
                    button.y() + 1,
                    button.w() - 4,
                    button.h() - 4,
                    15,
                    true,
                    Color::White,
                );
                draw::draw_text2(
                    &button.label(),
                    button.x() + 1,
                    button.y() + 1,
                    button.w() - 4,
                    button.h() - 4,
                    Align::Center,
                );
            }
        });
    )
}

fn draw_surface(surface: &mut ImageSurface, w: i32, h: i32) {
    let corner_radius = h as f64 / 10.0;
    let radius = corner_radius / 1.0;
    let degrees = std::f64::consts::PI / 180.0;
    let ctx = Context::new(surface).unwrap();
    ctx.save().unwrap();
    ctx.new_sub_path();
    ctx.arc(w as f64 - radius, radius, radius, -90. * degrees, 0.0);
    ctx.arc(
        w as f64 - radius,
        h as f64 - radius,
        radius,
        0.0,
        90. * degrees,
    );
    ctx.arc(
        radius,
        h as f64 - radius,
        radius,
        90. * degrees,
        180. * degrees,
    );
    ctx.arc(radius, radius, radius, 180. * degrees, 270. * degrees);
    ctx.close_path();
    ctx.set_source_rgba(150.0 / 255.0, 150.0 / 255.0, 150.0 / 255.0, 40.0 / 255.0);
    ctx.set_line_width(4.);
    ctx.fill().unwrap();
    ctx.restore().unwrap();
}
