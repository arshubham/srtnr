extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate urlshortener;
extern crate libnotify;
extern crate validator;
extern crate gdk_pixbuf;
extern crate glib;

mod headerbar;

use std;
use gtk::prelude::*;
use gtk::GridExt;
use gtk::{WidgetExt, StyleContextExt};
use self::urlshortener::{Provider, UrlShortener};

use self::headerbar::HeaderUi;

 pub fn ui (app: &gtk::Application) {

    let google_key = "AIzaSyBCT1JDPSXKOhbSbDAqnaSOzAVMFQ46EZ4";
    let bitly_token = "659fe6b3a2686e9f04c6f73ad50f3d601bb2e0fa";

    let window = gtk::ApplicationWindow::new (app);

    //headerbar
    let headerbar = HeaderUi::new ();
    let _headerbar_preferences_button = headerbar.preferences_button;
    window.set_titlebar (&headerbar.headerbar);


    //window size control
    window.set_default_size (700, 500);
    window.set_size_request (700, 500);
    gtk::GtkWindowExt::set_resizable (&window, false);

    //Main Grid
    let main_grid = gtk::Grid::new ();
    GridExt::set_row_spacing (&main_grid, 12);
    GridExt::set_column_spacing (&main_grid, 6);
    main_grid.set_margin_top (20);
    main_grid.set_margin_start (10);
    main_grid.set_margin_end (30);
    main_grid.set_column_homogeneous (true);

    //Input Grid
    let input_group_grid = gtk::Grid::new ();
    input_group_grid.set_column_spacing (0);
    input_group_grid.set_row_spacing (30);
    input_group_grid.set_margin_start (0);
    input_group_grid.set_margin_end (0);
    input_group_grid.set_column_homogeneous (true);
    input_group_grid.set_halign (gtk::Align::Fill);
    
    let url_label = gtk::Label::new_with_mnemonic (Some ("Full Url:"));
    let full_url_entry = gtk::Entry::new ();
    let entry_clone = full_url_entry.clone ();
    GridExt::attach (&input_group_grid, &url_label, 0, 0, 1, 1);
    GridExt::attach_next_to (
        &input_group_grid,
        &entry_clone,
        &url_label,
        gtk::PositionType::Right,
        5,
        1,
    );

    gtk::EntryExt::set_activates_default (&full_url_entry, true);
    
    let provider_label = gtk::Label::new_with_mnemonic (Some ("Provider:"));
    let combobox = gtk::ComboBoxText::new ();
    gtk::ComboBoxTextExt::append_text (&combobox, "goo.gl");
    gtk::ComboBoxTextExt::append_text (&combobox, "bit.ly");
    gtk::ComboBoxTextExt::append_text (&combobox, "is.gd");
    gtk::ComboBoxTextExt::append_text (&combobox, "bam.bz");
    gtk::ComboBoxTextExt::append_text (&combobox, "tny.im");
    gtk::ComboBoxTextExt::append_text (&combobox, "hmm.rs");
    gtk::ComboBoxExt::set_active (&combobox, 0);
    GridExt::attach (&input_group_grid, &provider_label, 0, 1, 1, 1);
    GridExt::attach_next_to (
        &input_group_grid,
        &combobox,
        &provider_label,
        gtk::PositionType::Right,
        2,
        1,
    );

    let combobox_clone = combobox.clone ();
    //shorten url button
    let shorten_url_button = gtk::Button::new_with_label ("Shorten URL!");
    shorten_url_button.set_margin_top (30);
    shorten_url_button.get_style_context().map(|c| c.add_class("suggested-action"));

    //short url label
    let short_url_label = gtk::Label::new ("");
    short_url_label.set_margin_top (20);
    short_url_label.set_selectable (true);
    short_url_label.set_halign (gtk::Align::Center);

    let label_clone = short_url_label.clone();
    let entry_clone2 = entry_clone.clone();
    let input_group_grid_clone = input_group_grid.clone();

    let display = window.get_display ().unwrap ();
    let gclipboard = gtk::Clipboard::get_default (&display).unwrap ();
    let url = gtk::ClipboardExt::wait_for_text (&gclipboard).unwrap ();
    let is_valid_url = validate_url (&url);
    
    if url.len() > 0 && is_valid_url {
        gtk::EntryExt::set_text (&full_url_entry, &url);
    }

    let window_clone = window.clone ();

    shorten_url_button.connect_clicked (move |_| {
        let protocol_str = "http://".to_string ();
        let url_entry_text = gtk::EntryExt::get_text (&entry_clone2).unwrap ();
        let is_valid_url = validate_url (&url_entry_text);
        let mut full_url  =  protocol_str + &url_entry_text;
        if is_valid_url {         
            full_url = url_entry_text;
        }
        
        libnotify::init("Srtnr").unwrap();
        let n = libnotify::Notification::new("Short Url Copied into clipboard.",
                                         None,
                                         None);
        
        libnotify::Notification::set_app_name (&n, None);

        let display = window_clone.get_display ().unwrap ();
        let gclipboard = gtk::Clipboard::get_default (&display).unwrap ();
        

        let us = UrlShortener::new ().unwrap ();
        let selected_index = gtk::ComboBoxExt::get_active (&combobox_clone);

        match selected_index {
        0 => {
            let short_url = us.generate (
                full_url,
                &urlshortener::Provider::GooGl {
                    api_key: google_key.to_owned (),
                },
            );
            
            let _short_url = match short_url {
                Ok (short_url) => {
                    label_clone.set_label (&short_url);
                    gclipboard.set_text (&short_url);
                    n.show ().unwrap ();         
                },
                Err (error) => {
                    label_clone.set_label (&std::string::ToString::to_string (&error));
                }
            };
        }, 
        1 => {
            let short_url = us.generate(
                full_url,
                &urlshortener::Provider::BitLy {
                    token: bitly_token.to_owned(),
                },
            );
            let _short_url = match short_url {
                Ok (short_url) => {
                    label_clone.set_label (&short_url);
                    gclipboard.set_text (&short_url);
                    n.show().unwrap();
                },
                Err (error) => {
                    label_clone.set_label (&std::string::ToString::to_string (&error));
                }
            };
        
        } ,
        2 =>  {
            let short_url = us.generate(full_url, &Provider::BamBz);
            let _short_url = match short_url {
                Ok (short_url) => {
                    label_clone.set_label(&short_url);
                    gclipboard.set_text (&short_url);
                    n.show().unwrap();
                },
                Err (error) => { 
                    label_clone.set_label(&std::string::ToString::to_string(&error));
                }
            };
        },
        
        3 => {
            let short_url = us.generate(full_url, &Provider::IsGd);
            let _short_url = match short_url {
                Ok(short_url) => {
                    label_clone.set_label(&short_url);
                    gclipboard.set_text (&short_url);
                    n.show().unwrap();
                },
                Err(error) => {
                    label_clone.set_label(&std::string::ToString::to_string(&error));
                }
            };
        } ,
        
        4 => {
            let short_url = us.generate(full_url, &Provider::TnyIm);
            let _short_url = match short_url {
                 Ok(short_url) => {
                    label_clone.set_label(&short_url);
                    gclipboard.set_text (&short_url);
                    n.show().unwrap();
                },
                Err(error) => {
                    label_clone.set_label(&std::string::ToString::to_string(&error));
                }
            };
        } 
        
        5 => {
            let short_url = us.generate(full_url, &Provider::HmmRs);
            let _short_url = match short_url {
                Ok(short_url) => {
                    label_clone.set_label(&short_url);
                    gclipboard.set_text (&short_url);
                    n.show().unwrap();
                },
                Err(error) => {
                    label_clone.set_label(&std::string::ToString::to_string(&error));
                }
            };
        },

        _ =>  label_clone.set_label("Please choose a provider"),
        }
            libnotify::uninit();
    });

    GridExt::attach (&main_grid, &input_group_grid_clone, 0, 0, 7, 2);

    GridExt::attach (&main_grid, &shorten_url_button, 2, 4, 3, 1);
    GridExt::attach (&main_grid, &short_url_label, 0, 5, 7, 1);

    window.add (&main_grid);
    window.show_all ();

 
}

fn validate_url (url: &String) -> bool {
    validator::validate_url (&url)
}