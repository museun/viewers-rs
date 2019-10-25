use crate::{twitch, util};
use std::time::Duration;
use {gio::prelude::*, gtk::prelude::*};

pub struct App {
    app: gtk::Application,
}

impl App {
    // TODO refactor this into multiple steps
    pub fn new(api_key: impl ToString, channel: impl ToString, timeout: Duration) -> Self {
        let channel = channel.to_string();
        let api_key = api_key.to_string();

        let res = util::Resources::load();

        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let _ = std::thread::spawn(move || loop {
            let viewers = twitch::get_viewers(&api_key, &channel).unwrap_or_default();
            if tx.send(viewers).is_err() {
                return;
            }
            std::thread::sleep(timeout)
        });

        let app = gtk::Application::new(Some("com.github.museun.viewers"), Default::default())
            .expect("intialize application");

        let label = gtk::Label::new(None);
        {
            let label = label.clone();
            rx.attach(None, move |viewers| {
                label.set_text(&viewers.to_string());
                gtk::Continue(true)
            });
        }

        // TODO pack this
        let image = gtk::Image::new_from_file(&res.icon);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hbox.pack_start(&image, true, true, 5);
        hbox.pack_end(&label, true, true, 5);

        // TODO pack this
        let provider = gtk::CssProvider::new();
        provider
            .load_from_path(res.css.to_str().unwrap())
            .expect("load CSS");

        app.connect_activate(move |app| {
            gtk::StyleContext::add_provider_for_screen(
                &gdk::Screen::get_default().expect("initialize gtk css provider"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            let window = gtk::ApplicationWindow::new(app);
            window.set_title("viewer count");
            window.set_keep_above(true);
            window.set_decorated(false);
            window.set_default_size(20, 42);
            window.set_size_request(20, 42);
            window.set_border_width(0);

            window.add_events(gdk::EventMask::SCROLL_MASK | gdk::EventMask::BUTTON_PRESS_MASK);
            window.connect_key_press_event(move |win, key| {
                if gdk::keyval_name(key.get_keyval())
                    .filter(|s| s.as_str() == "q")
                    .is_some()
                {
                    if let Some(app) = win.get_application() {
                        log::info!("quitting!");
                        app.quit()
                    }
                }
                gtk::Inhibit(false)
            });

            window.connect_button_press_event(move |win, button| {
                if let (gdk::EventType::ButtonPress, gdk::ModifierType::CONTROL_MASK) = (
                    button.get_event_type(),
                    button.get_state() ^ gdk::ModifierType::MOD2_MASK,
                ) {
                    let root = button.get_root();
                    win.begin_move_drag(
                        button.get_button() as _,
                        root.0 as _,
                        root.1 as _,
                        button.get_time(),
                    );
                }
                gtk::Inhibit(false)
            });

            window.connect_scroll_event(move |win, ev| {
                let current = win.get_opacity();
                let opacity = match ev.get_direction() {
                    gdk::ScrollDirection::Up => (current + 0.05).min(1.0),
                    gdk::ScrollDirection::Down => (current - 0.05).max(0.25),
                    _ => current,
                };
                win.set_opacity(opacity);
                gtk::Inhibit(false)
            });

            window.add(&hbox);
            window.show_all();
        });

        Self { app }
    }

    pub fn run(self) {
        let _ = self.app.run(&[]);
    }
}
