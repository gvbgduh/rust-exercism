use std::fmt;

const DAY: i32 = 24 * 60;

#[derive(Debug, PartialEq)]
pub struct Clock(i32);


fn wrapper(num: i32, wrap: i32) -> i32 {
    (num + ((-num / wrap) * wrap) + wrap) % wrap
}

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        Self(wrapper(wrapper(h, 24) * 60 + m, DAY))
    }

    pub fn add_minutes(&mut self, minutes: i32) -> Self {
        Self(wrapper(self.0 + minutes, DAY))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0 / 60, self.0 % 60)
    }
}


#[derive(Debug)]
pub struct OldClock {
    hours: i32,
    minutes: i32,
}

impl OldClock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours + minutes / 60) % 24;
        let mut min = minutes % 60;

        if h < 0 {h += 24};
        if min < 0 {
            min += 60;
            h -= 1;
        };

        Self {
            hours: h,
            minutes: min
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        if minutes < 0 {
            Self::new(self.hours - 1, 60 + self.minutes + minutes)
        } else {
            Self::new(self.hours, self.minutes + minutes)
        }
        
    }
}

impl fmt::Display for OldClock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl PartialEq for OldClock {
    fn eq(&self, other: &OldClock) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
