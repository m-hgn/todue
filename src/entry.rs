#[derive(Default, Debug, Clone)]
pub struct Entry<'a> {
    pub label: &'a str,
    pub due: Option<Date>,
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

impl<'a> Entry<'a> {
    pub fn from_label(label: &'a str) -> Entry<'a> {
        Entry {
            label: label,
            due: None,
            done: false,
        }
    }

    pub fn from_label_and_date(label: &'a str, due: Date) -> Entry<'a> {
        Entry {
            label: label,
            due: Some(due),
            done: false,
        }
    }
}

impl Date {
    pub fn from(y: i32, m: i32, d: i32, h: i32, min: i32) -> Date {
        Date {
            year: y,
            month: m,
            day: d,
            hour: h,
            minute: min,
        }
    }
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
