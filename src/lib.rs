#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::prelude::*;
pub use fltk::*;
pub use fltk_theme::*;

pub trait OnEvent<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn on_event<F: 'static + Fn(&Self) -> T>(self, cb: F) -> Self;
    fn set_activate(self, flag: bool) -> Self;
    fn set_visible(self, flag: bool) -> Self;
}

pub trait OnMenuEvent<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn on_item_event<F: 'static + Fn(&Self) -> T>(
        self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        cb: F,
    ) -> Self;
}

impl<W, T> OnEvent<T> for W
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_event<F: 'static + Fn(&Self) -> T>(mut self, cb: F) -> Self {
        let (s, _) = app::channel::<T>();
        self.set_callback(move |w| {
            s.send(cb(w));
        });
        self
    }
    fn set_activate(mut self, flag: bool) -> Self {
        match flag {
            true => self.activate(),
            false => self.deactivate(),
        }
        self
    }
    fn set_visible(mut self, flag: bool) -> Self {
        match flag {
            true => self.show(),
            false => self.hide(),
        }
        self
    }
}

impl<M, T> OnMenuEvent<T> for M
where
    M: MenuExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_item_event<F: 'static + Fn(&Self) -> T>(
        mut self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        cb: F,
    ) -> Self {
        let (s, _) = app::channel::<T>();
        self.add(name, shortcut, flag, move |w| {
            s.send(cb(w));
        });
        self
    }
}

#[derive(Default)]
pub struct Settings {
    pub pos: (i32, i32),
    pub size: (i32, i32),
    pub resizable: bool,
    pub background: Option<enums::Color>,
    pub foreground: Option<enums::Color>,
    pub background2: Option<enums::Color>,
    pub inactive: Option<enums::Color>,
    pub selection: Option<enums::Color>,
    pub font: Option<enums::Font>,
    pub xclass: Option<String>,
    pub icon: Option<image::SvgImage>,
    pub font_size: u8,
    pub scheme: Option<app::Scheme>,
    pub color_map: Option<&'static [fltk_theme::ColorMap]>,
    pub theme: Option<fltk_theme::ThemeType>,
    pub size_range: Option<(i32, i32, i32, i32)>,
    pub on_close_fn: Option<Box<dyn FnMut(&mut window::Window)>>,
}

pub trait Sandbox {
    type Message: Clone + Send + Sync + 'static;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self);
    fn update(&mut self, message: Self::Message);
    fn run(&mut self, settings: Settings) {
        let a = app::App::default();
        if let Some(scheme) = settings.scheme {
            app::set_scheme(scheme);
        } else {
            app::set_scheme(app::Scheme::Base);
        }
        if let Some(color_map) = settings.color_map {
            fltk_theme::ColorTheme::from_colormap(color_map)
        } else {
            fltk_theme::ColorTheme::from_colormap(fltk_theme::color_themes::GRAY_THEME)
        }.apply();
        if let Some(theme) = settings.theme {
            fltk_theme::WidgetTheme::new(theme).apply();
        }
        if let Some(color) = settings.background {
            let c = color.to_rgb();
            app::background(c.0, c.1, c.2);
        }
        if let Some(color) = settings.background2 {
            let c = color.to_rgb();
            app::background2(c.0, c.1, c.2);
        }
        if let Some(color) = settings.foreground {
            let c = color.to_rgb();
            app::foreground(c.0, c.1, c.2);
        }
        if let Some(color) = settings.inactive {
            let c = color.to_rgb();
            app::set_inactive_color(c.0, c.1, c.2);
        }
        if let Some(color) = settings.selection {
            let c = color.to_rgb();
            app::set_inactive_color(c.0, c.1, c.2);
        }
        if let Some(font) = settings.font {
            app::set_font(font);
        }
        if settings.font_size != 0 {
            app::set_font_size(settings.font_size);
        }
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let (mut x, mut y) = settings.pos;
        if (x, y) == (0, 0) {
            x = ((app::screen_size().0 - w as f64) / 2.0) as i32;
            y = ((app::screen_size().1 - h as f64) / 2.0) as i32;
        }
        let mut win = window::Window::default()
            .with_size(w, h)
            .with_pos(x, y)
            .with_label(&self.title());
        if let Some(value) = settings.xclass {
            win.set_xclass(&value);
        }
        if let Some(value) = settings.icon {
            win.set_icon(Some(value));
        }
        if let Some(close_fn) = settings.on_close_fn {
            win.set_callback(close_fn);
        } else {
            win.set_callback(move |_| {
                if app::event() == enums::Event::Close {
                    app::quit();
                }
            });
        }
        self.view();
        win.end();
        if settings.resizable {
            win.make_resizable(settings.resizable);
            if let Some((min_w, min_h, max_w, max_h)) = settings.size_range {
                win.size_range(min_w, min_h, max_w, max_h);
            } else {
                win.size_range(w, h, 0, 0);
            }
        };
        win.show();
        let (_, r) = app::channel::<Self::Message>();
        while a.wait() {
            if let Some(msg) = r.recv() {
                self.update(msg);
                win.clear();
                win.begin();
                win.set_label(&self.title());
                self.view();
                win.end();
                app::redraw();
            }
        }
    }
}
