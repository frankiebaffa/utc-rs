struct DateTime;

const YEAR: usize = 1970;
const MONTH: usize = 1;
const DAY: usize = 1;
const HOUR: usize = 0;
const MINUTE: usize = 0;
const SECOND: usize = 0;

impl DateTime {
    pub fn now() {
        let since = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap();
        let mut seconds = since.as_secs_f64();
        let mut minutes = seconds / 60_f64;
        seconds -= minutes;
        let mut hours = minutes / 60_f64;
    }
}
