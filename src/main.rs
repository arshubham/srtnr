extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate glib;

use gio::prelude::*;    
mod buildui;

fn main () {
    let app = gtk::Application::new ("com.github.arshubham.srtnr", gio::ApplicationFlags::empty ())
        .expect ("Failed to create Application");
    
    app.connect_startup (|app| {
        buildui::ui (&app);
    });
    app.connect_activate (|_| {});

    app.run (&std::env::args ().collect::<Vec<_>> ());
}
