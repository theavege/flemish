#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

pub use {
    cascade::cascade, fltk::prelude::*, fltk::*, fltk_calendar::calendar::*, fltk_theme::*, glib,
    std::sync::mpsc,
};

#[derive(Default)]
pub struct Settings {
    pub size: (i32, i32),
    pub font_size: u8,
    pub background: Option<enums::Color>,
    pub foreground: Option<enums::Color>,
    pub background2: Option<enums::Color>,
    pub inactive: Option<enums::Color>,
    pub selection: Option<enums::Color>,
    pub font: Option<enums::Font>,
    pub xclass: Option<String>,
    pub icon: Option<image::SvgImage>,
    pub scheme: Option<app::Scheme>,
    pub color_map: Option<&'static [fltk_theme::ColorMap]>,
    pub theme: Option<fltk_theme::ThemeType>,
}

pub trait Sandbox {
    type Message: Clone + Send + Sync + 'static;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self, sender: mpsc::Sender<Self::Message>) -> group::Flex;
    fn update(&mut self, message: Self::Message) -> bool;
    fn run(&mut self, settings: Settings) {
        let a = app::App::default();
        let (r, g, b) = match settings.background {
            Some(color) => color,
            None => enums::Color::from_hex(0xeee8d5),
        }
        .to_rgb();
        app::set_background_color(r, g, b);
        let (r, g, b) = match settings.background2 {
            Some(color) => color,
            None => enums::Color::from_hex(0xfdf6e3),
        }
        .to_rgb();
        app::set_background2_color(r, g, b);
        let (r, g, b) = match settings.foreground {
            Some(color) => color,
            None => enums::Color::from_hex(0x586e75),
        }
        .to_rgb();
        app::set_foreground_color(r, g, b);
        let (r, g, b) = match settings.selection {
            Some(color) => color,
            None => enums::Color::from_hex(0xcb4b16),
        }
        .to_rgb();
        app::set_selection_color(r, g, b);
        let (r, g, b) = match settings.inactive {
            Some(color) => color,
            None => enums::Color::from_hex(0xb58900),
        }
        .to_rgb();
        app::set_inactive_color(r, g, b);
        for (color, hex) in [
            (enums::Color::Yellow, 0xb58900),
            (enums::Color::Red, 0xdc322f),
            (enums::Color::Magenta, 0xd33682),
            (enums::Color::Blue, 0x268bd2),
            (enums::Color::Cyan, 0x2aa198),
            (enums::Color::Green, 0x859900),
        ] {
            let (r, g, b) = enums::Color::from_hex(hex).to_rgb();
            app::set_color(color, r, g, b);
        }
        app::set_visible_focus(false);
        app::set_font(match settings.font {
            Some(font) => font,
            None => enums::Font::CourierBold,
        });
        if let Some(value) = settings.scheme {
            app::set_scheme(value);
        };
        if let Some(value) = settings.color_map {
            fltk_theme::ColorTheme::from_colormap(value).apply();
        };
        if let Some(theme) = settings.theme {
            fltk_theme::WidgetTheme::new(theme).apply();
        }
        if settings.font_size != 0 {
            app::set_font_size(settings.font_size);
        }
        let (w, h) = settings.size;
        let w = if w == 0 { 640 } else { w };
        let h = if h == 0 { 360 } else { h };
        let (s, r) = mpsc::channel::<Self::Message>();
        let mut win = cascade!(
            window::Window::default().with_size(w, h).center_screen();
            ..set_label(&self.title());
            ..set_xclass(&match settings.xclass {
                Some(value) => value,
                None => self.title(),
            });
            ..size_range(w, h, 0, 0);
            ..make_resizable(true);
            ..set_icon(settings.icon);
            ..set_callback(move |window| {
                if app::event() == enums::Event::Close {
                    window.child(0).unwrap().do_callback();
                    app::quit();
                }
            });
            ..add(&self.view(s.clone()));
            ..end();
            ..show();
        );
        while a.wait() {
            if let Ok(msg) = r.try_recv() {
                if self.update(msg) {
                    win.clear();
                    win.begin();
                    win.add(&self.view(s.clone()));
                    win.end();
                    win.set_label(&self.title());
                    win.redraw();
                }
            }
        }
    }
}
