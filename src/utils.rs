use gtk;
use gtk::traits::*;


/// Setup a gtk::ComboBox model to have id in column 1 and label in column 2
/// All text.
pub fn setup_text_combo(combo: &gtk::ComboBox, model: gtk::TreeModel) {
    combo.set_model(model);
    combo.set_id_column(0);
    combo.set_entry_text_column(1);

    let cell = gtk::CellRendererText::new().unwrap();
    combo.pack_start(&cell, true);
    combo.add_attribute(&cell, "text", 1);
}

/// Add a row with two text column into the list store.
pub fn add_text_row(store: &gtk::ListStore, col1: &str, col2: &str) {
    let iter = store.append();
    store.set_string(&iter, 0, col1);
    store.set_string(&iter, 1, col2);
}
