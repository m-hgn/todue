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
            label,
            due: None,
            done: false,
        }
    }

    pub fn from_label_and_date(label: &'a str, due: Date) -> Entry<'a> {
        Entry {
            label,
            due: Some(due),
            done: false,
        }
    }

    pub fn render(&self, width: i32) -> String {
        let mut available_space = width;
        let mut label: String;

        available_space -= " - [ ] ".len() as i32;
        if self.due.is_some() {
            available_space -= " [yyyy-mm-dd, HH:MM] ".len() as i32;
        } else {
            available_space -= " ".len() as i32;
        }

        if available_space < self.label.len() as i32 {
            available_space -= "...".len() as i32;
            label = self.label.chars().take(available_space as usize).collect();
            label.push_str("...");
        } else {
            label = self.label.to_string();
            let padding = available_space as usize - label.len();
            let padding = " ".repeat(padding);
            label.push_str(&padding);
        }

        let mut text: String = if self.done { " - [x] " } else { " - [ ] " }.to_string();
        text.push_str(&label);

        if self.due.is_some() {
            text.push_str(" [");
            text.push_str(&format!("{}", self.due.as_ref().unwrap()));
            text.push_str("] ");
        }

        text
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
