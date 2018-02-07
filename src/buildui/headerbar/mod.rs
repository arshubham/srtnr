extern crate gtk;

use gtk::{HeaderBar, IconSize, Button};
use gtk::{WidgetExt, StyleContextExt, HeaderBarExt};

pub struct HeaderUi {
    pub headerbar: HeaderBar,
    pub preferences_button: Button,
}

impl HeaderUi {
    pub fn new () -> HeaderUi {
        let headerbar = HeaderBar::new ();
        HeaderBarExt::set_title (&headerbar, "Srtnr");
        HeaderBarExt::set_show_close_button (&headerbar, true);

        let preferences_button = Button::new_from_icon_name ("preferences-other", IconSize::LargeToolbar.into ());

        HeaderBarExt::pack_end (&headerbar, &preferences_button);

        WidgetExt::get_style_context(&headerbar).map(|c| c.add_class("flat"));

        HeaderUi {
            headerbar,
            preferences_button,
        }
    }
}