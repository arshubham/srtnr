extern crate gtk;
extern crate gio;
extern crate urlshortener;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::WidgetExt;
use gtk::GridExt;

use urlshortener::{Provider, UrlShortener};

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
    shorten_url_button.set_margin_top (30);

    let goo_gl_radiobutton = gtk::RadioButton::new_with_label("goo.gl");
    let bit_ly_radiobutton = gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "bit.ly");
    let is_gd_radiobutton = gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "is.gd");
    let v_gd_radiobutton = gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "v.gd");
    let tny_im_radiobutton = gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "tny.im");
    let hmm_rs_radiobutton = gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "hmm.rs");

    goo_gl_radiobutton.set_margin_top (30);
    bit_ly_radiobutton.set_margin_top (30);
    short_url_label.set_margin_top (20);

    let goo_gl_rb_clone = goo_gl_radiobutton.clone ();
    let bit_ly_rb_clone = bit_ly_radiobutton.clone ();
    let is_gd_rb_clone = is_gd_radiobutton.clone ();
    let v_gd_rb_clone = v_gd_radiobutton.clone ();
    let tny_im_rb_clone = tny_im_radiobutton.clone ();
    let hmm_rs_rb_clone = hmm_rs_radiobutton.clone ();
    let label_clone = short_url_label.clone ();
    let entry_clone = full_url_entry.clone ();

	shorten_url_button.connect_clicked( move |_| {

	    let url = gtk::EntryExt::get_text (&entry_clone).unwrap ();
	    let us = UrlShortener::new().unwrap();

	    let short_url = us.generate(url, &Provider::IsGd);

	    let short_url = match short_url {
        Ok(short_url) => label_clone.set_label(&short_url),
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };

    });


    GridExt::attach (&main_grid, &url_label, 0, 0, 1, 1);
	GridExt::attach (&main_grid, &full_url_entry , 1 , 0 ,3 ,1);

    GridExt::attach (&main_grid, &goo_gl_rb_clone, 1 ,1 ,1,1);
    GridExt::attach (&main_grid, &bit_ly_rb_clone, 2 ,1 ,1,1);
	GridExt::attach (&main_grid, &is_gd_rb_clone, 0 ,2 ,1,1);
	GridExt::attach (&main_grid, &v_gd_rb_clone, 1, 2, 1, 1);
	GridExt::attach (&main_grid, &tny_im_rb_clone, 2 ,2 ,1,1);
	GridExt::attach (&main_grid, &hmm_rs_rb_clone, 3 ,2 ,1,1);

    GridExt::attach (&main_grid, &shorten_url_button , 1 , 3 ,2 ,1);
	GridExt::attach (&main_grid, &short_url_label, 0, 4, 3, 1);

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

    app.connect_activate(|_| {});
	//run app
    app.run(&std::env::args().collect::<Vec<_>>());
}