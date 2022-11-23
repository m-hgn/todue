#[macro_use]
mod macros;

mod entry;
use entry::*;

mod action;

#[allow(dead_code)]
mod ui;
use ui::*;

fn main() {
    let mut ui = Ui::from_entries(&[
        Entry::from_label("Take out the trash"),
        Entry::from_label_and_date("Discard of the body", Date::from(2022, 12, 20, 13, 25)),
        Entry::from_label("Do the dishes"),
    ]);

    ui::Ui::setup();

    while ui.active {
        ui.render_entries();
        ui.parse_input();
        ui.do_input_action();
    }
}
