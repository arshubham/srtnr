extern crate gtk;
use gtk::IsA;

use gtk::{Dialog, Label, Button, Grid, Switch, Box, Orientation};
use gtk::{DialogExt, GridExt, WidgetExt, BoxExt, GtkWindowExt, ContainerExt, ButtonExt, SwitchExt};

use gio::Settings;
use gio::SettingsExt;
use gio;


pub struct PrefDialogUi {
    pub pref_dialog: Dialog,
}

impl PrefDialogUi {
    pub fn new<P>(parent: &P) -> PrefDialogUi 
    where P: IsA<gtk::Window>, {
    let pref_dialog = Dialog::new ();
    let content_grid = Grid::new ();
    GridExt::set_row_spacing (&content_grid, 12);
    GridExt::set_column_spacing (&content_grid, 6);
    GridExt::set_column_homogeneous (&content_grid, true);
    WidgetExt::set_margin_top (&content_grid, 20);
    WidgetExt::set_margin_start (&content_grid, 10);
    WidgetExt::set_margin_end (&content_grid, 30);
    GtkWindowExt::set_modal (&pref_dialog, true);

    let dark_setting_box = Box::new (Orientation::Horizontal, 5);
    let dark_setting_label = Label::new_with_mnemonic (Some ("Use Dark Theme"));
    WidgetExt::set_halign (&dark_setting_label, gtk::Align::Start); 
    let dark_setting_switch = Switch::new ();


    let settings = Settings::new ("com.github.arshubham.srtnr");
    SwitchExt::set_active (&dark_setting_switch, SettingsExt::get_boolean (&settings, "use-dark-theme"));
    let dark_settings = gtk::Settings::get_default ().unwrap ();
    dark_setting_switch.connect_state_set(move |_, _| {
        if SettingsExt::get_boolean (&settings, "use-dark-theme") {
            SettingsExt::set_boolean (&settings, "use-dark-theme", false);
        } else {
            SettingsExt::set_boolean (&settings, "use-dark-theme", true);
        }
         gtk::SettingsExt::set_property_gtk_application_prefer_dark_theme (&dark_settings, SettingsExt::get_boolean (&settings, "use-dark-theme"));
        gio::signal::Inhibit(false)
    });

    GtkWindowExt::set_default_size (&pref_dialog, 500, 300);
    pref_dialog.set_size_request (500, 300);
    GtkWindowExt::set_resizable (&pref_dialog, false);
    
    BoxExt::pack_start (&dark_setting_box, &dark_setting_label, false, false, 0);
    BoxExt::pack_end (&dark_setting_box, &dark_setting_switch, false, false, 0);

    GridExt::attach (&content_grid, &dark_setting_box, 0, 0, 1, 1);
    DialogExt::get_content_area (&pref_dialog).add (&content_grid);
    GtkWindowExt::set_transient_for (&pref_dialog, parent);
    let close_button = Button::new_with_label("Close");
    DialogExt::add_action_widget (&pref_dialog, &close_button, 0);
    DialogExt::get_content_area (&pref_dialog).show_all ();

    
    let pref_dialog_clone = pref_dialog.clone ();
    close_button.connect_clicked (move |_| {
        DialogExt::emit_close (&pref_dialog_clone);
    });

    PrefDialogUi {
             pref_dialog,
        }
    }

    pub fn run(&self) { 
        WidgetExt::show_now (&self.pref_dialog);
    }

}