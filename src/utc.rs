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

impl Utc {
    /// Gets the year part.
    pub fn year(&self) -> usize {
        self.year
    }

    /// Gets the month part.
    pub fn month(&self) -> usize {
        self.month
    }

    /// Gets the name of the month.
    pub fn month_of_year(&self) -> &'static str {
        self.month_of_year
    }

    /// Gets the day part.
    pub fn day(&self)-> usize {
        self.day
    }

    /// Gets the name of the day.
    pub fn day_of_week(&self) -> &'static str {
        self.day_of_week
    }

    /// Gets the hour part.
    pub fn hour(&self) -> usize {
        self.hour
    }

    /// Gets the minute part.
    pub fn minute(&self) -> usize {
        self.minute
    }

    /// Gets the second part without nano seconds.
    pub fn second(&self) -> usize {
        self.second
    }

    /// Gets the fractional nano seconds.
    pub fn nano(&self) -> f64 {
        self.nano
    }

    /// Gets the seconds with fractional nano seconds.
    pub fn second_with_nano(&self) -> f64 {
        self.second as f64 + self.nano
    }

    /// Gets the date/time in yyyy-MM-ddTHH:mm:ss format.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymdhms(2024, 1, 9, 16, 21, 0.5_f64);
    /// assert_eq!("2024-01-09T16:21:00", dt.as_rfc3339());
    /// ```
    pub fn as_rfc3339(&self) -> String {
        format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// Gets the date/time in yyyy-MM-ddTHH:mm:ss.f format with the
    /// specified precision.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymdhms(2024, 1, 9, 12, 59, 9.8_f64);
    /// assert_eq!("2024-01-09T12:59:09.8", dt.as_rfc3339_with_nano(1));
    /// ```
    pub fn as_rfc3339_with_nano(&self, precision: usize) -> String {
        let nano = &format!("{:.precision$}", self.nano)[2..];
        format!(
            "{:0>4}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}.{nano}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }

    /// Gets the date/time/ in yyyy-MM-ddTHH:mm:ss.ffffff format.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymdhms(2024, 1, 9, 12, 5, 57.2_f64);
    /// assert_eq!("2024-01-09T12:05:57.200000", dt.as_rfc3339_nano());
    /// ```
    pub fn as_rfc3339_nano(&self) -> String {
        self.as_rfc3339_with_nano(6)
    }

    /// Gets the date/time in Day, dd Mon yyyy HH:mm:ss Z format.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymdhms(2024, 1, 9, 12, 5, 57_f64);
    /// assert_eq!("Tue, 09 Jan 2024 12:05:57 GMT", dt.as_rfc7231());
    /// ```
    pub fn as_rfc7231(&self) -> String {
        format!(
            "{}, {:0>2} {} {:0>4} {:0>2}:{:0>2}:{:0>2} GMT",
            &self.day_of_week[0..3], self.day, &self.month_of_year[0..3],
            self.year, self.hour, self.minute, self.second
        )
    }

    fn from_seconds_since_epoch(seconds_with_nanos: f64) -> Self {
        let total_seconds_since_epoch = seconds_with_nanos as usize;
        let nano = seconds_with_nanos - total_seconds_since_epoch as f64;
        let mut minute = total_seconds_since_epoch / 60_usize;
        let second = total_seconds_since_epoch - (minute * 60_usize);
        let mut hour = minute / 60_usize;
        minute -= hour * 60_usize;
        let mut days = hour / 24_usize;
        hour -= days * 24_usize;

        let day_of_week = super::statics::D[days % super::statics::DIW];

        let mut month = super::statics::EPOCH_M;
        let mut year = super::statics::EPOCH_Y;

        let day;

        loop {
            let ly = super::statics::ly(year);

            let days_in_year = super::statics::DIY[ly];
            if days > days_in_year {
                days -= days_in_year;
                year += 1;
                continue;
            }

            let days_in_month = super::statics::DIM[month - 1][ly];
            if days >= days_in_month {
                days -= days_in_month;
                month += 1;
                if month > 12 {
                    month = 1;
                    year += 1;
                }
                continue;
            }

            day = days + super::statics::EPOCH_D;
            break;
        }

        let month_of_year = super::statics::M[month - 1];

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

    /// Gets the specified date/time.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymdhms(2024, 1, 9, 12, 57, 1.0_f64);
    /// assert_eq!(2024, dt.year());
    /// assert_eq!(1, dt.month());
    /// assert_eq!(9, dt.day());
    /// assert_eq!(12, dt.hour());
    /// assert_eq!(57, dt.minute());
    /// assert_eq!(1, dt.second());
    /// assert_eq!(0_f64, dt.nano());
    /// ```
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
            day = super::statics::DIM[month - 1][super::statics::ly(year)];
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
            day = super::statics::DIM[month - 1][super::statics::ly(year)];
        }

        // only allow >= epoch
        if year < 1970 {
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
            let days_in_month = super::statics::DIM[month - 1][super::statics::ly(year)];
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

        let mut days = (super::statics::EPOCH_Y..year)
            .into_iter()
            .map(|y| super::statics::DIY[super::statics::ly(y)])
            .sum::<usize>();

        for m in 1..(super::statics::MIY + 1) {
            if m != month {
                days += super::statics::DIM[m - 1][super::statics::ly(year)];
            } else {
                days += day - 1;
                break;
            }
        }

        let hours = (days * 24) + hour;
        let minutes = (hours * 60) + minute;
        let seconds = (minutes * 60) + second;
        let seconds_with_nanos = seconds as f64 + nano;
        Self::from_seconds_since_epoch(seconds_with_nanos)
    }

    /// Gets the specified date at the start of the day.
    ///
    /// # Examples
    ///
    /// ```
    /// let dt = utc::Utc::from_ymd(2024, 1, 9);
    /// assert_eq!(2024, dt.year());
    /// assert_eq!(1, dt.month());
    /// assert_eq!(9, dt.day());
    /// assert_eq!(0, dt.hour());
    /// assert_eq!(0, dt.minute());
    /// assert_eq!(0_f64, dt.second_with_nano());
    /// ```
    pub fn from_ymd(year: usize, month: usize, day: usize) -> Self {
        Self::from_ymdhms(year, month, day, 0, 0, 0_f64)
    }

    /// Gets the current date/time.
    ///
    /// # Examples
    ///
    /// ```
    /// let now = utc::Utc::now();
    /// println!("{now}");
    /// ```
    pub fn now() -> Self {
        Self::from_seconds_since_epoch(std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64()
        )
    }

    /// Gets the unix epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// let epoch = utc::Utc::epoch();
    /// assert_eq!("1970-01-01T00:00:00.000000", epoch.as_rfc3339_nano());
    /// ```
    pub fn epoch() -> Self {
        Self::from_seconds_since_epoch(0_f64)
    }
}

impl std::fmt::Display for Utc {
    fn fmt(&self, fmtr: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmtr.write_str(&self.as_rfc3339_nano())
    }
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

impl PartialOrd for Utc {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.year.cmp(&other.year) {
            std::cmp::Ordering::Equal => match self.month.cmp(&other.month) {
                std::cmp::Ordering::Equal => match self.day.cmp(&other.day) {
                    std::cmp::Ordering::Equal => match self.hour.cmp(&other.hour) {
                        std::cmp::Ordering::Equal => match self.minute.cmp(&other.minute) {
                            std::cmp::Ordering::Equal => match self.second.cmp(&other.second) {
                                std::cmp::Ordering::Equal => self.nano.partial_cmp(&other.nano),
                                o => Some(o),
                            },
                            o => Some(o),
                        },
                        o => Some(o),
                    },
                    o => Some(o),
                },
                o => Some(o),
            },
            o => Some(o),
        }
    }
}

