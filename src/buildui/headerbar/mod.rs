extern crate gtk;

use gtk::{WidgetExt, StyleContextExt};

pub struct HeaderUi {
    pub headerbar: gtk::HeaderBar,
    pub preferences_button: gtk::Button,
}

impl HeaderUi {
    pub fn new () -> HeaderUi {
        let headerbar = gtk::HeaderBar::new ();
        gtk::HeaderBarExt::set_title (&headerbar, "Srtnr");
        gtk::HeaderBarExt::set_show_close_button (&headerbar, true);

        let preferences_button = gtk::Button::new_from_icon_name ("preferences-other", gtk::IconSize::LargeToolbar.into ());

        gtk::HeaderBarExt::pack_end (&headerbar, &preferences_button);

        headerbar.get_style_context().map(|c| c.add_class("flat"));
        HeaderUi {
            headerbar,
            preferences_button,
        }
    }
}