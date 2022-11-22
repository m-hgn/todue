#[derive(Default, Debug, Clone)]
pub struct Entry<'a> {
    pub label: &'a str,
    pub due: Date,
    pub done: bool,
}

#[derive(Default, Debug, Clone)]
pub struct Date {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}-{:02}-{:02}, {:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[macro_export]
macro_rules! entry {
    ( $t:expr, $d:expr ) => {{
        Entry {
            label: $t,
            due: $d,
            done: false,
        }
    }};
}

#[macro_export]
macro_rules! date {
    ( $y:expr, $m:expr, $d:expr, $h:expr, $min:expr ) => {{
        Date {
            year: $y,
            month: $m,
            day: $d,
            hour: $h,
            minute: $min,
        }
    }};
}
