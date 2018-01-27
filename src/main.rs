extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::WidgetExt;
use gtk::GridExt;

struct HeaderUi {
    headerbar: gtk::HeaderBar,
}

impl HeaderUi {
    fn new () -> HeaderUi {
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
    
	let main_grid = gtk::Grid::new ();
	GridExt::set_row_spacing (&main_grid, 12);
	GridExt::set_column_spacing (&main_grid, 6);
	main_grid.set_margin_top (20);
	main_grid.set_halign(gtk::Align::Center);
	main_grid.set_column_homogeneous (true);

	let url_label = gtk::Label::new_with_mnemonic(Some("Enter Url:"));
	url_label.set_halign (gtk::Align::Start);

    let full_url_entry = gtk::Entry::new ();

    let shorten_url_button = gtk::Button::new_with_label("Shorten URL!");

    let short_url_label = gtk::Label::new ("");

    let label_clone = short_url_label.clone();
	shorten_url_button.connect_clicked( move |_| {
        label_clone.set_label("I've been clicked!");
    });


    GridExt::attach (&main_grid, &url_label, 0, 0, 1, 1);
	GridExt::attach (&main_grid, &full_url_entry , 1 , 0 ,4 ,1);
	GridExt::attach (&main_grid, &shorten_url_button , 1 , 1 ,1 ,1);
	GridExt::attach (&main_grid, &short_url_label, 1, 2, 4, 1);

    window.add(&main_grid);

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