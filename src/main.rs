extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::WidgetExt;

struct HeaderUi {
	headerbar: gtk::HeaderBar,
}

impl HeaderUi {
	fn new() -> HeaderUi {
		let headerbar = gtk::HeaderBar::new();
		headerbar.set_title("Srtnr");
		headerbar.set_show_close_button(true);

		HeaderUi { headerbar }
	}
}

fn ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let header = HeaderUi::new();

    // Window HeaderBar
	window.set_titlebar(&header.headerbar);

	// (width, height);
    window.set_default_size(600, 300);

    let container = gtk::Box::new(
		gtk::Orientation::Vertical,
		0
	);
    

    let full_url = gtk::Entry::new ();

    let shorten_url_button = gtk::Button::new_with_label("Shorten URL!");


    container.pack_start(
		&entry,
		false,
		false,
		0
    );
    container.pack_start(
		&entry2,
		false,
		false,
		0
    );
    container.pack_start (
        &shorten_url_button,
        false,
        false,
        0
    );

    window.add(&container);

    window.show_all();
}

fn main () {

    let app = gtk::Application::new(
		"com.github.arshubham.srtnr",
		gio::ApplicationFlags::empty()
	).expect("Failed..");

	app.connect_startup(|app| {
		ui(&app);
	});

	//run app
    app.run(&std::env::args().collect::<Vec<_>>());
}