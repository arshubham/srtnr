extern crate gtk;
use gtk::IsA;

use gtk::{Dialog, Grid, Label};
use gtk::{DialogExt, GridExt, ContainerExt, WidgetExt, GtkWindowExt};

pub struct PrefDialogUi {
    pub dialog: Dialog,
}

impl PrefDialogUi {
    pub fn new<P>(parent: &P) -> PrefDialogUi 
    where P: IsA<gtk::Window>, {

    let dialog = Dialog::new ();
    let grid = Grid::new ();
    GridExt::set_row_spacing (&grid, 12);
    GridExt::set_column_spacing (&grid, 6);
    let label = Label::new_with_mnemonic (Some ("Some Text"));
    GridExt::attach (&grid, &label, 0, 0, 1, 1);
    DialogExt::get_content_area (&dialog).add (&grid);
    dialog.set_transient_for (parent);

    PrefDialogUi {
             dialog,
        }
    }

    pub fn run(&self) { 

        WidgetExt::show_now (&self.dialog);
    }

}