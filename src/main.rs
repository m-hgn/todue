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
            Date::from(2022, 12, 20, 13, 25),
        ),
        Entry::from_label("Do the dishes"),
        Entry::from_label("Do the dishes more"),
        Entry::from_label("Do the dishes even more"),
        Entry::from_label("Do the dishes the most"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Give"),
        Entry::from_label("You"),
        Entry::from_label("Up"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Let"),
        Entry::from_label("You"),
        Entry::from_label("Down"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Run"),
        Entry::from_label("Around"),
        Entry::from_label("And"),
        Entry::from_label("Desert"),
        Entry::from_label("You"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Make"),
        Entry::from_label("You"),
        Entry::from_label("Cry"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Say"),
        Entry::from_label("Goodbye"),
        Entry::from_label("Never"),
        Entry::from_label("Gonna"),
        Entry::from_label("Tell"),
        Entry::from_label("A"),
        Entry::from_label("Lie"),
        Entry::from_label("And"),
        Entry::from_label("Hurt"),
        Entry::from_label("You"),
        Entry::from_label("Implement end of todo list"),
    ]);

    ui.setup();

    while ui.active {
        ui.render_entries();
        ui.parse_input();
    }
}
