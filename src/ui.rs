use crate::action::Action;
use crate::action::Action::*;
use crate::entry::*;
use ncurses::*;
use std::collections::HashMap;

pub const COL_NORMAL: i16 = 0;
pub const COL_HIGHLIGHT: i16 = 1;

pub struct Ui<'a> {
    pub active: bool,
    pub window: Option<WINDOW>,

    pub width: i32,
    pub height: i32,

    pub entries: Vec<Entry<'a>>,
    pub current_position: usize,

    pub key_to_action: HashMap<i32, Action>,
}

impl<'a> Ui<'a> {
    pub fn from_entries(entries: &[Entry<'a>]) -> Ui<'a> {
        Ui {
            active: true,
            window: None,
            width: 0,
            height: 0,
            entries: entries.to_vec(),
            current_position: 0,
            key_to_action: hash_map![
                b'k' as i32 => Up,
                b'j' as i32 => Down,

                b'K' as i32 => ShiftUp,
                b'J' as i32 => ShiftDown,

                b'i' as i32 => Edit,
                b'I' as i32 => Edit,
                b'a' as i32 => Edit,
                b'A' as i32 => Edit,

                b'o' as i32 => NewAfter,
                b'O' as i32 => NewBefore,

                b'v' as i32 => Selection,
                b'V' as i32 => BlockSelection,

                b' ' as i32 => MarkDone,

                b'q' as i32 => Quit,
                b'Q' as i32 => Quit,
                27 => Quit
            ],
        }
    }

    pub fn setup(&mut self) {
        self.window = Some(initscr());
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        start_color();
        init_pair(COL_NORMAL, COLOR_WHITE, COLOR_BLACK);
        init_pair(COL_HIGHLIGHT, COLOR_BLACK, COLOR_WHITE);
        getmaxyx(self.window.unwrap(), &mut self.height, &mut self.width);
    }

    pub fn parse_input(&mut self) {
        let key = getch();

        if let Some(action) = self.key_to_action.get(&key) {
            match action {
                Down => self.down(),
                Up => self.up(),
                ShiftDown => todo!(),
                ShiftUp => todo!(),
                Edit => todo!(),
                NewBefore => todo!(),
                NewAfter => todo!(),
                Selection => todo!(),
                BlockSelection => todo!(),
                MarkDone => self.toggle_done(),
                Quit => self.quit(),
            }
        }
    }

    pub fn render_entries(&self) {
        for (i, entry) in self.entries.iter().enumerate() {
            let style = if i == self.current_position {
                COLOR_PAIR(COL_HIGHLIGHT)
            } else {
                COLOR_PAIR(COL_NORMAL)
            };
            attron(style);
            mv(i as i32 + 1, 0);

            let text = entry.render(self.width);

            addstr(&text);
            attroff(style);
        }
        refresh();
    }

    fn toggle_done(&mut self) {
        self.entries[self.current_position].done = !self.entries[self.current_position].done;
    }

    fn down(&mut self) {
        self.current_position = (self.current_position + 1) % self.entries.len()
    }

    fn up(&mut self) {
        self.current_position = match self.current_position {
            0 => self.entries.len() - 1,
            p => p - 1,
        }
    }

    fn quit(&mut self) {
        endwin();
        self.active = false;
    }

    fn save() {
        todo!()
    }
}
