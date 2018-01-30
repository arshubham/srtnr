extern crate gio;
extern crate gtk;
extern crate urlshortener;
extern crate gdk;

use gtk::prelude::*;
use gio::prelude::*;
use gtk::WidgetExt;
use gtk::GridExt;
use urlshortener::{Provider, UrlShortener};

struct HeaderUi {
    headerbar: gtk::HeaderBar,
    switch: gtk::Switch,
    label: gtk::Label,
}

impl HeaderUi {
    fn new() -> HeaderUi {
        let headerbar = gtk::HeaderBar::new();
        headerbar.set_title("Srtnr");
        headerbar.set_show_close_button(true);

        let switch = gtk::Switch::new();
        let label = gtk::Label::new_with_mnemonic(Some("Using http"));

        
        headerbar.pack_end(&switch);
        headerbar.pack_end(&label);
        HeaderUi {
            headerbar,
            switch,
            label,
        }
    }
}

fn ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    let header = HeaderUi::new();
    let switch = header.switch;
    let label = header.label;
    // Window HeaderBar
    window.set_titlebar(&header.headerbar);
    // (width, height);
    window.set_default_size(700, 550);
    window.set_size_request (700,550);
        gtk::GtkWindowExt::set_resizable (&window, false);


    let main_grid = gtk::Grid::new();
    GridExt::set_row_spacing(&main_grid, 12);
    GridExt::set_column_spacing(&main_grid, 6);
    main_grid.set_margin_top(20);
    main_grid.set_margin_start(10);
    main_grid.set_margin_end (30);
    main_grid.set_column_homogeneous(true);

    let input_group_grid = gtk::Grid::new();
    input_group_grid.set_column_spacing (0);
    let protocol_label = gtk::Label::new_with_mnemonic(Some("http://"));
    let full_url_entry = gtk::Entry::new();
    let entry_clone = full_url_entry.clone();
    GridExt::attach(&input_group_grid, &protocol_label, 0, 0, 1, 1);
    GridExt::attach_next_to (&input_group_grid, &entry_clone, &protocol_label, gtk::PositionType::Right, 5 , 1);
    input_group_grid.set_column_homogeneous(true);
    input_group_grid.set_margin_start(0);
    input_group_grid.set_margin_end(0);
    input_group_grid.set_halign(gtk::Align::Fill);
    input_group_grid.set_column_spacing(0);

    let shorten_url_button = gtk::Button::new_with_label("Shorten URL!");

    let short_url_label = gtk::Label::new("");
    shorten_url_button.set_margin_top(30);

    let goo_gl_radiobutton = gtk::RadioButton::new_with_label("goo.gl");
    let bit_ly_radiobutton =
        gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "bit.ly");
    let is_gd_radiobutton =
        gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "is.gd");
    let bam_bz_radiobutton =
        gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "bam.bz");
    let tny_im_radiobutton =
        gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "tny.im");
    let hmm_rs_radiobutton =
        gtk::RadioButton::new_with_label_from_widget(&goo_gl_radiobutton, "hmm.rs");

    goo_gl_radiobutton.set_margin_top(30);
    bit_ly_radiobutton.set_margin_top(30);
    short_url_label.set_margin_top(20);
    short_url_label.set_selectable (true);
    short_url_label.set_halign (gtk::Align::Center);

    let copy_button = gtk::Button::new_from_icon_name ("edit-copy", gtk::IconSize::LargeToolbar.into ());

    let goo_gl_rb_clone = goo_gl_radiobutton.clone();
    let bit_ly_rb_clone = bit_ly_radiobutton.clone();
    let is_gd_rb_clone = is_gd_radiobutton.clone();
    let bam_bz_rb_clone = bam_bz_radiobutton.clone();
    let tny_im_rb_clone = tny_im_radiobutton.clone();
    let hmm_rs_rb_clone = hmm_rs_radiobutton.clone();
    let label_clone = short_url_label.clone();
    let entry_clone2 = entry_clone.clone();
    let input_group_grid_clone = input_group_grid.clone();
    let protocol_label_clone = protocol_label.clone();
    let protocol_label_clone2 = protocol_label.clone();
    let copy_button_clone = copy_button.clone ();
    let header_label_clone = label.clone();


    switch.connect_property_active_notify(move |switch| {
        if switch.get_active() {
            protocol_label_clone.set_text("https://");
            header_label_clone.set_text("Using https");
        } else {
            protocol_label_clone.set_text("http://");
            header_label_clone.set_text("Using http");
        }
    });
         let display = window.get_display().unwrap();
    let short_url_label_clone = short_url_label.clone ();
    copy_button.connect_clicked(move |_| {
         let str_to_cpy = gtk::LabelExt::get_text (&short_url_label_clone).unwrap ();
         let gclipboard = gtk::Clipboard::get_default(&display).unwrap();
         gclipboard.set_text (&str_to_cpy);
    });
    shorten_url_button.connect_clicked(move |_| {
        
        let protocol_str = gtk::LabelExt::get_text(&protocol_label_clone2).unwrap();
        let url_entry_text = gtk::EntryExt::get_text(&entry_clone2).unwrap();

        let url = protocol_str + &url_entry_text;
        if gtk::ToggleButtonExt::get_active(&goo_gl_radiobutton) {
            let us0 = UrlShortener::new().unwrap();
            let key = "AIzaSyBCT1JDPSXKOhbSbDAqnaSOzAVMFQ46EZ4";
            let googl_short_url = us0.generate(
                url,
                &urlshortener::Provider::GooGl {
                    api_key: key.to_owned(),
                },
            );
            let _googl_short_url = match googl_short_url {
                Ok(googl_short_url) => label_clone.set_label(&googl_short_url),
                Err(error) => {label_clone.set_label(&std::string::ToString::to_string(&error));
                }
            };
        } else if gtk::ToggleButtonExt::get_active(&bit_ly_radiobutton) {
            let us04 = UrlShortener::new().unwrap();
            let token = "659fe6b3a2686e9f04c6f73ad50f3d601bb2e0fa";
            let bitly_short_url = us04.generate(
                url,
                &urlshortener::Provider::BitLy {
                    token: token.to_owned(),
                },
            );
            let _bitly_short_url = match bitly_short_url {
                Ok(bitly_short_url) => label_clone.set_label(&bitly_short_url),
                Err(error) => label_clone.set_label(&std::string::ToString::to_string(&error)),
            };
        } else if gtk::ToggleButtonExt::get_active(&bam_bz_radiobutton) {
            let us1 = UrlShortener::new().unwrap();
            let bambz_short_url = us1.generate(url, &Provider::BamBz);
            let _bambz_short_url = match bambz_short_url {
                Ok(bambz_short_url) => label_clone.set_label(&bambz_short_url),
                Err(error) => label_clone.set_label(&std::string::ToString::to_string(&error)),
            };
        } else if gtk::ToggleButtonExt::get_active(&is_gd_radiobutton) {
            let us2 = UrlShortener::new().unwrap();
            let isgd_short_url = us2.generate(url, &Provider::IsGd);
            let _isgd_short_url = match isgd_short_url {
                Ok(isgd_short_url) => label_clone.set_label(&isgd_short_url),
                Err(error) => label_clone.set_label(&std::string::ToString::to_string(&error)),
            };
        } else if gtk::ToggleButtonExt::get_active(&hmm_rs_radiobutton) {
            let us3 = UrlShortener::new().unwrap();
            let hmmrs_short_url = us3.generate(url, &Provider::HmmRs);
            let _hmmrs_short_url = match hmmrs_short_url {
                Ok(hmmrs_short_url) => label_clone.set_label(&hmmrs_short_url),
                Err(error) => label_clone.set_label(&std::string::ToString::to_string(&error)),
            };
        } else if gtk::ToggleButtonExt::get_active(&tny_im_radiobutton) {
            let us4 = UrlShortener::new().unwrap();
            let tnyim_short_url = us4.generate(url, &Provider::TnyIm);
            let _tnyim_short_url = match tnyim_short_url {
                Ok(tnyim_short_url) => label_clone.set_label(&tnyim_short_url),
                Err(error) => label_clone.set_label(&std::string::ToString::to_string(&error)),
            };
        } else {
            label_clone.set_label("Choose");
        }
    });

    let rb_grid =  gtk::Grid::new();
    rb_grid.set_column_homogeneous (true);
    rb_grid.set_column_spacing (20);
    rb_grid.set_row_spacing (30);
    rb_grid.set_halign(gtk::Align::Center);
    rb_grid.set_margin_start (0);
    rb_grid.set_margin_end (0);
    GridExt::attach(&rb_grid, &goo_gl_rb_clone, 1, 1, 1, 1);
    GridExt::attach(&rb_grid, &bit_ly_rb_clone, 2, 1, 1, 1);
    GridExt::attach(&rb_grid, &is_gd_rb_clone, 0, 2, 1, 1);
    GridExt::attach(&rb_grid, &bam_bz_rb_clone, 1, 2, 1, 1);
    GridExt::attach(&rb_grid, &tny_im_rb_clone, 2, 2, 1, 1);
    GridExt::attach(&rb_grid, &hmm_rs_rb_clone, 3, 2, 1, 1);

    GridExt::attach(&main_grid, &input_group_grid_clone, 0, 0, 7, 1);

    GridExt::attach(&main_grid, &rb_grid, 0, 2, 7, 2);

    GridExt::attach(&main_grid, &shorten_url_button, 2, 4, 3, 1);
    GridExt::attach(&main_grid, &short_url_label, 0, 5, 7, 1);
    GridExt::attach(&main_grid , &copy_button_clone, 3 , 6 , 1 , 1);

    window.add(&main_grid);

    window.show_all();
}

fn main() {
    let app = gtk::Application::new("com.github.arshubham.srtnr", gio::ApplicationFlags::empty())
        .expect("Failed..");

    app.connect_startup(|app| {
        ui(&app);
    });

    app.connect_activate(|_| {});
    //run app
    app.run(&std::env::args().collect::<Vec<_>>());
}
