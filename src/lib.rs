#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::prelude::*;
pub use fltk::*;
pub use fltk_theme::*;

pub trait OnEvent<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn on_event(self, msg: T) -> Self
    where
        Self: Sized;
    fn on_event_deferred<F: 'static + FnMut(&mut Self) -> T>(self, cb: F) -> Self
    where
        Self: Sized;
}

pub trait OnMenuEvent<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn on_item_event(
        self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        msg: T,
    ) -> Self
    where
        Self: Sized;
    fn on_item_event_deferred<F: 'static + FnMut(&mut Self) -> T>(
        self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        cb: F,
    ) -> Self
    where
        Self: Sized;
}

impl<W, T> OnEvent<T> for W
where
    W: WidgetExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_event(mut self, msg: T) -> Self {
        let (s, _) = app::channel::<T>();
        self.emit(s, msg);
        self
    }

    fn on_event_deferred<F: 'static + FnMut(&mut Self) -> T>(mut self, mut cb: F) -> Self {
        let (s, _) = app::channel::<T>();
        self.set_callback(move |w| {
            s.send(cb(w));
        });
        self
    }
}

impl<M, T> OnMenuEvent<T> for M
where
    M: MenuExt,
    T: Send + Sync + Clone + 'static,
{
    fn on_item_event(
        mut self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        msg: T,
    ) -> Self {
        let (s, _) = app::channel::<T>();
        self.add_emit(name, shortcut, flag, s, msg);
        self
    }

    fn on_item_event_deferred<F: 'static + FnMut(&mut Self) -> T>(
        mut self,
        name: &str,
        shortcut: enums::Shortcut,
        flag: menu::MenuFlag,
        mut cb: F,
    ) -> Self {
        let (s, _) = app::channel::<T>();
        self.add(name, shortcut, flag, move |m| {
            s.send(cb(m));
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
    pub font_size: u8,
    pub scheme: Option<app::Scheme>,
    pub color_map: Option<&'static [fltk_theme::ColorMap]>,
    pub theme: Option<fltk_theme::ThemeType>,
    pub ignore_esc_close: bool,
}

pub trait Sandbox {
    type Message: Clone + Send + Sync + 'static;
    fn new() -> Self;
    fn title(&self) -> String;
    fn view(&mut self);
    fn update(&mut self, message: Self::Message);
    fn run(&mut self, settings: Settings) {
        let a = app::App::default();
        let color_theme = if let Some(color_map) = settings.color_map {
            fltk_theme::ColorTheme::from_colormap(color_map)
        } else {
            fltk_theme::ColorTheme::from_colormap(fltk_theme::color_themes::BLACK_THEME)
        };
        color_theme.apply();
        if let Some(theme) = settings.theme {
            let widget_theme = fltk_theme::WidgetTheme::new(theme);
            widget_theme.apply();
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
        if settings.font_size != 0 {
            app::set_font_size(settings.font_size);
        }
        if let Some(scheme) = settings.scheme {
            app::set_scheme(scheme);
        } else {
            app::set_scheme(app::Scheme::Gtk);
        }
        if let Some(font) = settings.font {
            app::set_font(font);
        }
        let (w, h) = settings.size;
        let w = if w == 0 { 400 } else { w };
        let h = if h == 0 { 300 } else { h };
        let (x, y) = settings.pos;
        let mut win = window::Window::default()
            .with_size(w, h)
            .with_label(&self.title());
        if (x, y) != (0, 0) {
            win.set_pos(x, y);
        }
        if settings.ignore_esc_close {
            win.set_callback(move |_| {
                if app::event() == enums::Event::Close {
                    app::quit();    
                }
            });
        }
        self.view();
        win.end();
        win.make_resizable(settings.resizable);
        win.show();
        let (_, r) = app::channel::<Self::Message>();
        while a.wait() {
            if let Some(msg) = r.recv() {
                self.update(msg);
                win.clear();
                win.begin();
                self.view();
                win.end();
                app::redraw();
            }
        }
    }
}
