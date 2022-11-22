use ncurses::*;

mod entry;
use entry::*;

#[allow(dead_code)]
mod ui;
use ui::*;

fn main() {
    let mut ui = Ui {
        // example entries
        entries: vec![
            entry!["Make cup of tea", date![2022, 12, 20, 13, 25]],
            entry!["Take out the trash", date![2022, 12, 20, 16, 25]],
            entry!["Do more things", date![2022, 12, 20, 16, 25]],
        ],
        current_position: 0,
    };

    ui::Ui::setup();

    loop {
        ui.render_entries();

        let c = getch();
        if ui::QUIT_KEYS.contains(&c) {
            ui.quit();
            break;
        }
        if ui::UP_KEYS.contains(&c) {
            ui.up();
        }
        if ui::DOWN_KEYS.contains(&c) {
            ui.down();
        }
        if ui::DONE_KEYS.contains(&c) {
            ui.toggle_done();
        }
    }
}
