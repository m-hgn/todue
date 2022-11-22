use crate::entry::*;
use ncurses::*;

pub const DOWN_KEYS: [i32; 2] = [b'j' as i32, ncurses::KEY_UP];
pub const UP_KEYS: [i32; 2] = [b'k' as i32, ncurses::KEY_UP];

pub const SHIFT_DOWN_KEYS: [i32; 1] = [b'J' as i32];
pub const SHIFT_UP_KEYS: [i32; 1] = [b'K' as i32];

pub const VISUAL_MODE_KEYS: [i32; 1] = [b'v' as i32];
pub const BLOCK_MODE_KEYS: [i32; 1] = [b'b' as i32];

pub const QUIT_KEYS: [i32; 2] = [b'q' as i32, b'Q' as i32];
pub const DONE_KEYS: [i32; 2] = [b' ' as i32, ncurses::KEY_ENTER];

pub const NORMAL: i16 = 0;
pub const HIGHLIGHT: i16 = 1;

pub struct Ui<'a> {
    pub entries: Vec<Entry<'a>>,
    pub current_position: usize,
}

impl<'a> Ui<'a> {
    pub fn setup() {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        start_color();
        init_pair(NORMAL, COLOR_WHITE, COLOR_BLACK);
        init_pair(HIGHLIGHT, COLOR_BLACK, COLOR_WHITE);
    }

    pub fn render_entries(&self) {

        for (i, entry) in self.entries.iter().enumerate() {
            let style = if i == self.current_position {
                COLOR_PAIR(HIGHLIGHT)
            } else {
                COLOR_PAIR(NORMAL)
            };
            attron(style);
            mv(i as i32 + 1, 0);
            let mut text = if entry.done { "- [x] " } else { "- [ ] " }.to_string();
            text.push_str(entry.label);
            addstr(&text);
            attroff(style);
        }
        refresh();
    }

    pub fn toggle_done(&mut self) {
        self.entries[self.current_position].done = !self.entries[self.current_position].done;
    }

    pub fn down(&mut self) {
        self.current_position = (self.current_position + 1) % self.entries.len()
    }

    pub fn up(&mut self) {
        self.current_position = match self.current_position {
            0 => self.entries.len() - 1,
            p => p - 1,
        }
    }

    pub fn quit(&self) {
        endwin();
    }

    pub fn save() {
        todo!()
    }
}
