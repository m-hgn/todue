#[derive(Debug)]
pub enum Action {
    Down,
    Up,
    ShiftDown,
    ShiftUp,
    Edit,
    NewBefore,
    NewAfter,
    Selection,
    BlockSelection,
    MarkDone,
    Quit,
}

impl Action {}
