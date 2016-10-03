use glib;
use gtk;
use gtk::prelude::*;


/// Setup a gtk::ComboBox model to have id in column 1 and label in column 2
/// All text.
pub fn setup_text_combo(combo: &gtk::ComboBox, model: &gtk::ListStore) {
    combo.set_model(Some(model));
    combo.set_id_column(0);
    combo.set_entry_text_column(1);

    let cell = gtk::CellRendererText::new();
    combo.pack_start(&cell, true);
    combo.add_attribute(&cell, "text", 1);
}

/// Add a row with two text column into the list store.
pub fn add_text_row(store: &gtk::ListStore,
                    col1: &str, col2: &str) -> gtk::TreeIter {
    store.insert_with_values(None, &[0, 1],
                             &[&String::from(col1), &String::from(col2)])
}

/// Block a signal and run the function f.
pub fn block_signal<T, F>(obj: &mut T, signal: u64, f: F)
    where T: IsA<glib::Object>, F: Fn(&mut T) {

    glib::signal_handler_block(obj, signal);
    f(obj);
    glib::signal_handler_unblock(obj, signal);
}
