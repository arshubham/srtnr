extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate urlshortener;
extern crate libnotify;
extern crate validator;
extern crate gdk_pixbuf;
extern crate glib;

mod headerbar;
mod prefdialog;

use std;
use gtk::prelude::*;
use gtk::{Grid, Application, ApplicationWindow, Align, Label, Entry, ComboBoxText, Button, Clipboard };
use gtk::{GridExt, GtkWindowExt, WidgetExt, StyleContextExt, EntryExt, ComboBoxTextExt, ClipboardExt, LabelExt};
use gio::Settings;
use gio::SettingsExt;
use self::urlshortener::{Provider, UrlShortener};

use self::headerbar::HeaderUi;
use self::prefdialog::PrefDialogUi;

 pub fn ui (app: &Application) {

    let google_key = "AIzaSyBCT1JDPSXKOhbSbDAqnaSOzAVMFQ46EZ4";
    let bitly_token = "659fe6b3a2686e9f04c6f73ad50f3d601bb2e0fa";

    let window = ApplicationWindow::new (app);

    //headerbar
    let headerbar = HeaderUi::new ();
    let headerbar_preferences_button = headerbar.preferences_button;
    GtkWindowExt::set_titlebar (&window, &headerbar.headerbar);

    let win_clone = window.clone ();
    
    headerbar_preferences_button.connect_clicked (move |_| {
        let pref = PrefDialogUi::new (&win_clone);
        PrefDialogUi::run (&pref);
    });

    //window size control
    GtkWindowExt::set_default_size (&window, 650, 450);
    //Error: the trait `gtk::GtkWindowExt` cannot be made into an object
    window.set_size_request (650, 450);
    GtkWindowExt::set_resizable (&window, false);

    

    //Main Grid
    let main_grid = Grid::new ();
    GridExt::set_row_spacing (&main_grid, 12);
    GridExt::set_column_spacing (&main_grid, 6);
    GridExt::set_column_homogeneous (&main_grid, true);
    WidgetExt::set_margin_top (&main_grid, 20);
    WidgetExt::set_margin_start (&main_grid, 10);
    WidgetExt::set_margin_end (&main_grid, 30);

    //Input Grid
    let input_group_grid = Grid::new ();
    GridExt::set_column_spacing (&input_group_grid, 0);
    GridExt::set_row_spacing (&input_group_grid, 30);
    GridExt::set_column_homogeneous (&input_group_grid,true);
    WidgetExt::set_margin_start (&input_group_grid, 0);
    WidgetExt::set_margin_end (&input_group_grid, 0);
    WidgetExt::set_halign (&input_group_grid, Align::Fill);
    
    let url_label = Label::new_with_mnemonic (Some ("Full Url:"));
    let full_url_entry = Entry::new ();
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

    EntryExt::set_activates_default (&full_url_entry, true);
    
    let settings = Settings::new ("com.github.arshubham.srtnr");

    let dark_settings = gtk::Settings::get_default ().unwrap ();
    gtk::SettingsExt::set_property_gtk_application_prefer_dark_theme (&dark_settings, SettingsExt::get_boolean (&settings, "use-dark-theme"));
    
    let default_provider = SettingsExt::get_int (&settings, "default-provider");

    let provider_label = Label::new_with_mnemonic (Some ("Provider:"));
    let combobox = ComboBoxText::new ();
    ComboBoxTextExt::append_text (&combobox, "goo.gl");
    ComboBoxTextExt::append_text (&combobox, "bit.ly");
    ComboBoxTextExt::append_text (&combobox, "is.gd");
    ComboBoxTextExt::append_text (&combobox, "bam.bz");
    ComboBoxTextExt::append_text (&combobox, "tny.im");
    ComboBoxTextExt::append_text (&combobox, "hmm.rs");
    ComboBoxExt::set_active (&combobox, default_provider);
    GridExt::attach (&input_group_grid, &provider_label, 0, 1, 1, 1);
    GridExt::attach_next_to (
        &input_group_grid,
        &combobox,
        &provider_label,
        gtk::PositionType::Right,
        2,
        1,
    );

    //shorten url button
    let shorten_url_button = Button::new_with_label ("Shorten URL!");
    WidgetExt::set_margin_top (&shorten_url_button, 30);
    WidgetExt::get_style_context (&shorten_url_button).map (|c| c.add_class("suggested-action"));
    WidgetExt::set_can_default (&shorten_url_button, true);

    //short url label
    let short_url_label = gtk::Label::new ("");
    WidgetExt::set_margin_top (&short_url_label, 20);
    WidgetExt::set_halign (&short_url_label, Align::Center);
    LabelExt::set_selectable (&short_url_label, true);
    GtkWindowExt::set_default (&window, &shorten_url_button);

    //Clones
    let combobox_clone = combobox.clone ();
    let short_label_clone = short_url_label.clone ();
    let entry_clone2 = entry_clone.clone ();
    let input_group_grid_clone = input_group_grid.clone ();

    let display = window.get_display ().unwrap ();
    let gclipboard = Clipboard::get_default (&display).unwrap ();
    let url = ClipboardExt::wait_for_text (&gclipboard).unwrap ();
    let is_valid_url = validate_url (&url);
    
    if url.len() > 0 && is_valid_url {
        EntryExt::set_text (&full_url_entry, &url);
    }

    let window_clone = window.clone ();

    shorten_url_button.connect_clicked (move |_| {
        let protocol_str = "http://".to_string ();
        let url_entry_text = EntryExt::get_text (&entry_clone2).unwrap ();
        let is_valid_url = validate_url (&url_entry_text);
        let mut full_url  =  protocol_str + &url_entry_text;
        if is_valid_url {         
            full_url = url_entry_text;
        }
        let mut successful = false;
        
        

        let display = window_clone.get_display ().unwrap ();
        let gclipboard = Clipboard::get_default (&display).unwrap ();
        

        let us = UrlShortener::new ().unwrap ();
        let selected_index = ComboBoxExt::get_active (&combobox_clone);

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
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;         
                },
                Err (error) => {
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        }, 
        1 => {
            let short_url = us.generate (
                full_url,
                &urlshortener::Provider::BitLy {
                    token: bitly_token.to_owned(),
                },
            );
            let _short_url = match short_url {
                Ok (short_url) => {
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;
                },
                Err (error) => {
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        
        } ,
        2 =>  {
            let short_url = us.generate (full_url, &Provider::IsGd);
            let _short_url = match short_url {
                Ok (short_url) => {
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;
                },
                Err (error) => { 
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        },
        
        3 => {
            let short_url = us.generate (full_url, &Provider::BamBz);
            let _short_url = match short_url {
                Ok (short_url) => {
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;
                },
                Err (error) => {
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        } ,
        
        4 => {
            let short_url = us.generate (full_url, &Provider::TnyIm);
            let _short_url = match short_url {
                 Ok (short_url) => {
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;
                },
                Err (error) => {
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        } 
        
        5 => {
            let short_url = us.generate (full_url, &Provider::HmmRs);
            let _short_url = match short_url {
                Ok (short_url) => {
                    LabelExt::set_label (&short_label_clone, &short_url);
                    gclipboard.set_text (&short_url);
                    notification (short_url);
                    successful = true;
                },
                Err (error) => {
                    LabelExt::set_label (&short_label_clone, &std::string::ToString::to_string (&error));
                }
            };
        },

        _ =>  short_label_clone.set_label ("Please choose a provider"),
        }
            
        

        if successful {
            SettingsExt::set_int (&settings, "default-provider", selected_index);
        }
        
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

fn notification (short_url: String) {
    libnotify::init ("Srtnr").unwrap ();
    let url = String::from(short_url);

    // https://stackoverflow.com/questions/31233938/converting-from-optionstring-to-optionstr#31234028
    let opt: Option<String> = Some(url.to_owned());
    let value = opt.as_ref().map(|x| &**x).unwrap_or(" ");

    let n = libnotify::Notification::new ("Short Url Copied into clipboard.",
                                        Some (value),
                                        None);
        
        libnotify::Notification::set_app_name (&n, None);
        n.show ().unwrap ();
        libnotify::uninit ();
}