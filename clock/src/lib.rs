#[derive(Debug, PartialEq)]
pub struct Clock{
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut fixed_minutes: i32 = minutes % 60;
        let mut hours_in_minutes: i32 = minutes / 60;

        if fixed_minutes < 0{
            fixed_minutes = 60 + fixed_minutes;
            hours_in_minutes -= 1;
        }

        let mut fixed_hours: i32 = (hours + hours_in_minutes) % 24;

        if fixed_hours < 0{
            fixed_hours += 24
        }

        Self{
            hours: fixed_hours,
            minutes:  fixed_minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}
