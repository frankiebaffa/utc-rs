//! Computes UTC datetimes after 1970-01-01T00:00:00 using only the
//! [time](`std::time`) module.

#[cfg(test)]
mod test;

const EPOCH_YEAR: usize = 1970;
const EPOCH_MONTH: usize = 1;
const EPOCH_DAY: usize = 1;

const DAYS: [&'static str; 7] = [
    "Thursday", // unix epoch started on a thursday
    "Friday",
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday"
];

const MONTHS: [&'static str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const MONTH_MAX_DAY: [[usize; 2]; 12] = [
    [ 31, 31, ],
    [ 28, 29, ],
    [ 31, 31, ],
    [ 30, 30, ],
    [ 31, 31, ],
    [ 30, 30, ],
    [ 31, 31, ],
    [ 31, 31, ],
    [ 30, 30, ],
    [ 31, 31, ],
    [ 30, 30, ],
    [ 31, 31, ],
];

/// A date/time represented in UTC.
#[derive(Clone, Copy, Debug)]
pub struct Utc {
    year: usize,
    month: usize,
    month_of_year: &'static str,
    day: usize,
    day_of_week: &'static str,
    hour: usize,
    minute:usize,
    second: usize,
    nano: f64,
}

impl PartialEq<Utc> for Utc {
    fn eq(&self, other: &Utc) -> bool {
        self.year.eq(&other.year) &&
            self.month.eq(&other.month) &&
            self.month_of_year.eq(other.month_of_year) &&
            self.day.eq(&other.day) &&
            self.day_of_week.eq(other.day_of_week) &&
            self.hour.eq(&other.hour) &&
            self.minute.eq(&other.minute) &&
            self.nano.eq(&other.nano)
    }
}

impl Eq for Utc {}

impl Utc {
    pub fn is_leap_year(year: usize) -> bool {
        year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
    }

    pub fn leap_year(&self) -> bool {
        Self::is_leap_year(self.year)
    }

    pub fn year(&self) -> usize {
        self.year
    }

    pub fn month(&self) -> usize {
        self.month
    }

    pub fn month_of_year(&self) -> &'static str {
        self.month_of_year
    }

    pub fn day(&self)-> usize {
        self.day
    }

    pub fn day_of_week(&self) -> &'static str {
        self.day_of_week
    }

    pub fn hour(&self) -> usize {
        self.hour
    }

    pub fn minute(&self) -> usize {
        self.minute
    }

    pub fn second(&self) -> usize {
        self.second
    }

    pub fn nano(&self) -> f64 {
        self.nano
    }

    pub fn second_with_nano(&self) -> f64 {
        self.second as f64 + self.nano
    }

    /// Gets the date/time in yyyy-MM-ddTHH:mm:ss format.
    pub fn as_rfc3339(&self) -> String {
        format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// Gets the date/time in yyyy-MM-ddTHH:mm:ss.f format with the
    /// specified precision.
    pub fn as_rfc3339_with_nano(&self, precision: usize) -> String {
        let nano = &format!("{:.precision$}", self.nano)[2..];
        format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}.{nano}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// Gets the date/time in Day, dd Mon yyyy HH:mm:ss Z format.
    pub fn as_rfc7231(&self) -> String {
        format!(
            "{}, {:0>2} {} {:0>4} {:0>2}:{:0>2}:{:0>2} GMT",
            &self.day_of_week[0..3], self.day, &self.month_of_year[0..3],
            self.year, self.hour, self.minute, self.second
        )
    }

    // TODO: Handle negative seconds since epoch

    fn from_seconds_since_epoch(seconds_with_nanos: f64) -> Self {
        let total_seconds_since_epoch = seconds_with_nanos as usize;
        let nano = seconds_with_nanos - total_seconds_since_epoch as f64;
        let mut minute = total_seconds_since_epoch / 60_usize;
        let second = total_seconds_since_epoch - (minute * 60_usize);
        let mut hour = minute / 60_usize;
        minute -= hour * 60_usize;
        let days = hour / 24_usize;
        hour -= days * 24_usize;

        let day_of_week = DAYS[days % DAYS.len()];

        let mut day = EPOCH_DAY;
        let mut month = EPOCH_MONTH;
        let mut year = EPOCH_YEAR;
        for _ in 0..days {
            let month_days = Self::get_days_in_month(year, month);
            if day == month_days {
                day -= month_days - 1;
                if month == MONTHS.len() {
                    month -= MONTHS.len() - 1;
                    year += 1;
                } else {
                    month += 1;
                }
            } else {
                day += 1;
            }
        }

        let month_of_year = MONTHS[month - 1];

        Self {
            year,
            month,
            month_of_year,
            day,
            day_of_week,
            hour,
            minute,
            second,
            nano,
        }
    }

    fn get_days_in_month(year: usize, month: usize) -> usize {
        MONTH_MAX_DAY[month - 1][if Self::is_leap_year(year) { 1 } else { 0 }]
    }

    /// Gets a specific date/time.
    pub fn from_ymdhms(
        mut year: usize, mut month: usize, mut day: usize,
        mut hour: usize, mut minute: usize, second_n: f64,
    ) -> Self {
        // only allow >= epoch
        if year < 1970 || year == 1970 && month == 0 {
            panic!("Date must be >= 1970");
        }

        // Correct possible zeros
        if day == 0 && month == 0 {
            year -= 1;
            month = 11;
            day = Self::get_days_in_month(year, month);
        } else if month == 0 {
            month = 12;
            year -= 1;
        } else if day == 0 {
            if month == 1 {
                month = 12;
                year -= 1;
            } else {
                month -= 1;
            }
            day = Self::get_days_in_month(year, month);
        }

        // only allow >= epoch
        if year < 1970 || year == 1970 && month == 0 {
            panic!("Date must be >= 1970");
        }

        let mut second = second_n as usize;
        let mut nano = second_n - second as f64;

        // Correct overflows from bottom-up.
        // Overflow nanos into seconds.
        if nano > 1_f64 {
            let overflow = (nano - ((nano as usize) as f64)) as usize;
            second += overflow;
            nano -= overflow as f64;
        }

        // Overflow seconds into minutes.
        if second >= 60 {
            minute += (second % 60) + 1;
            second %= 60;
        }

        // Overflow minutes into hours.
        if minute >= 60 {
            hour += (minute % 60) + 1;
            minute %= 60;
        }

        // Overflow hours into days.
        if hour >= 24 {
            day += (hour % 24) + 1;
            hour %= 24;
        }

        // Overflow months into years.
        if month > 12 {
            year += month % 12;
            month %= 12;
        }

        // Overflow days into months.
        loop {
            let mut corrected = false;
            let days_in_month = Self::get_days_in_month(year, month);
            if day > days_in_month {
                corrected = true;
                day -= days_in_month;
                month += 1;
            }
            if month > 12 {
                year += 1;
                month = 1;
            }
            if !corrected {
                break;
            }
        }

        // calculate days since epoch
        let mut days = 0;
        for y in EPOCH_YEAR..(year + 1) {
            for m in 1..(MONTHS.len() + 1) {
                if y != year || m != month {
                    days += Self::get_days_in_month(y, m);
                } else {
                    days += day - 1;
                    break;
                }
            }
        }

        let hours = (days * 24) + hour;
        let minutes = (hours * 60) + minute;
        let seconds = (minutes * 60) + second;
        let seconds_with_nanos = seconds as f64 + nano;
        Self::from_seconds_since_epoch(seconds_with_nanos)
    }

    /// Gets a specific date.
    pub fn from_ymd(year: usize, month: usize, day: usize) -> Self {
        Self::from_ymdhms(year, month, day, 0, 0, 0_f64)
    }

    /// Gets the current date/time.
    pub fn now() -> Self {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        let total_nanos_since_epoch = since.as_secs_f64();
        Self::from_seconds_since_epoch(total_nanos_since_epoch)
    }

    /// Gets the unix epoch.
    pub fn epoch() -> Self {
        Self::from_seconds_since_epoch(0_f64)
    }

    // TODO: Handle negative additions

    pub fn add_seconds(self, seconds: f64) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, self.hour, self.minute,
            self.second_with_nano() + seconds
        )
    }

    pub fn with_seconds(self, seconds: f64) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, self.hour, self.minute, seconds
        )
    }

    pub fn add_minutes(self, minutes: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, self.hour, self.minute + minutes,
            self.second_with_nano()
        )
    }

    pub fn with_minutes(self, minutes: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, self.hour, minutes,
            self.second_with_nano()
        )
    }

    pub fn add_hours(self, hours: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, self.hour + hours, self.minute,
            self.second_with_nano()
        )
    }

    pub fn with_hours(self, hours: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day, hours, self.minute,
            self.second_with_nano()
        )
    }

    pub fn add_days(self, days: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, self.day + days, self.hour, self.minute,
            self.second_with_nano()
        )
    }

    pub fn with_days(self, days: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month, days, self.hour, self.minute,
            self.second_with_nano()
        )
    }

    pub fn add_months(self, months: usize) -> Self {
        Self::from_ymdhms(
            self.year, self.month + months, self.day, self.hour, self.minute,
            self.second_with_nano()
        )
    }

    pub fn with_months(self, months: usize) -> Self {
        Self::from_ymdhms(
            self.year, months, self.day, self.hour, self.minute,
            self.second_with_nano()
        )
    }

    pub fn add_years(self, years: usize) -> Self {
        Self::from_ymdhms(
            self.year + years, self.month, self.day, self.hour, self.minute,
            self.second_with_nano()
        )
    }

    pub fn with_years(self, years: usize) -> Self {
        Self::from_ymdhms(
            years, self.month, self.day, self.hour, self.minute,
            self.second_with_nano()
        )
    }
}

