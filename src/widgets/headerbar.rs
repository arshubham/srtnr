struct HeaderUi {
    headerbar: gtk::HeaderBar,
    switch: gtk::Switch,
    label: gtk::Label,
}

impl HeaderUi {
   pub fn new() -> HeaderUi {
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