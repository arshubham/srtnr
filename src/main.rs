extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate glib;

use gio::prelude::*;
use gtk::Application;
use gio::{ApplicationExt, ApplicationFlags};    

mod buildui;

fn main () {
    let app = Application::new ("com.github.arshubham.srtnr", ApplicationFlags::empty ())
        .expect ("Failed to create Application");
    
    ApplicationExt::connect_startup (&app, |app| {
        buildui::ui (&app);
    });
    ApplicationExt::connect_activate (&app, |_| { });

    // Error
    app.run (&std::env::args ().collect::<Vec<_>> ());
}
