extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate urlshortener;
extern crate libnotify;
extern crate validator;
extern crate gdk_pixbuf;
extern crate glib;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::GridExt;
use gtk::{WidgetExt, StyleContextExt};
use self::urlshortener::{Provider, UrlShortener};
mod buildui;

fn main () {
    let app = gtk::Application::new ("com.github.arshubham.srtnr", gio::ApplicationFlags::empty())
        .expect ("Failed to create Application");

    app.connect_startup (|app| {
        buildui::ui (&app);
    });
    app.connect_activate (|_| {});

    app.run (&std::env::args ().collect::<Vec<_>> ());
}
