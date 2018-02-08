extern crate gtk;
use gtk::IsA;

use gtk::{Dialog, Label, Grid, Button};
use gtk::{DialogExt, WidgetExt, GridExt, GtkWindowExt, ContainerExt, ButtonExt};

pub struct PrefDialogUi {
    pub pref_dialog: Dialog,
}

impl PrefDialogUi {
    pub fn new<P>(parent: &P) -> PrefDialogUi 
    where P: IsA<gtk::Window>, {
    let pref_dialog = Dialog::new ();
    let content_grid = Grid::new ();
    let label = Label::new_with_mnemonic (Some ("Some Text"));
    GridExt::attach (&content_grid, &label, 0, 0, 1, 1);
    DialogExt::get_content_area (&pref_dialog).add (&content_grid);
    GtkWindowExt::set_transient_for (&pref_dialog, parent);
    
    let close_button = Button::new_with_label("Close");
    DialogExt::add_action_widget (&pref_dialog, &close_button, 0);
    DialogExt::get_content_area (&pref_dialog).show_all ();

    let pref_dialog_clone = pref_dialog.clone ();
    close_button.connect_clicked (move |_| {
        pref_dialog_clone.emit_close ();
    });

    PrefDialogUi {
             pref_dialog,
        }
    }

    pub fn run(&self) { 
        WidgetExt::show_now (&self.pref_dialog);
    }

}