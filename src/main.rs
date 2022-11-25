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
        Entry::from_label_and_date(
            "Discard of the body before it is too late and they \
            find out about your misdeeds in '96 before your conscience \
            got the better of you",
            Date::from(2022, 12, 20, 13, 25)),
        Entry::from_label("Do the dishes"),
    ]);

    ui.setup();

    while ui.active {
        ui.render_entries();
        ui.parse_input();
    }
}
