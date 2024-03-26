pub(crate) const EPOCH_Y: usize = 1970;
pub(crate) const EPOCH_M: usize = 1;
pub(crate) const EPOCH_D: usize = 1;

pub(crate) const DIW: usize = 7;

const THU_N: &str = "Thursday";
const FRI_N: &str = "Friday";
const SAT_N: &str = "Saturday";
const SUN_N: &str = "Sunday";
const MON_N: &str = "Monday";
const TUE_N: &str = "Tuesday";
const WED_N: &str = "Wednesday";

pub(crate) const D: [&str; 7] = [
    THU_N, // unix epoch started on a Thursday
    FRI_N,
    SAT_N,
    SUN_N,
    MON_N,
    TUE_N,
    WED_N,
];

pub(crate) const MIY: usize = 12;

const JAN_N: &str = "January";
const FEB_N: &str = "February";
const MAR_N: &str = "March";
const APR_N: &str = "April";
const MAY_N: &str = "May";
const JUN_N: &str = "June";
const JUL_N: &str = "July";
const AUG_N: &str = "August";
const SEP_N: &str = "September";
const OCT_N: &str = "October";
const NOV_N: &str = "November";
const DEC_N: &str = "December";

pub(crate) const M: [&str; 12] = [
    JAN_N,
    FEB_N,
    MAR_N,
    APR_N,
    MAY_N,
    JUN_N,
    JUL_N,
    AUG_N,
    SEP_N,
    OCT_N,
    NOV_N,
    DEC_N,
];

pub(crate) const fn ly(year: usize) -> usize {
    (year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)) as usize
}

const JAN_D: usize = 31;
const FEB_D: usize = 28;
const FEB_L: usize = 29;
const MAR_D: usize = 31;
const APR_D: usize = 30;
const MAY_D: usize = 31;
const JUN_D: usize = 30;
const JUL_D: usize = 31;
const AUG_D: usize = 31;
const SEP_D: usize = 30;
const OCT_D: usize = 31;
const NOV_D: usize = 30;
const DEC_D: usize = 31;

pub(crate) const DIYC: usize = JAN_D + FEB_D + MAR_D + APR_D + MAY_D + JUN_D +
    JUL_D + AUG_D + SEP_D + OCT_D + NOV_D + DEC_D;

pub(crate) const DILC: usize = JAN_D + FEB_L + MAR_D + APR_D + MAY_D + JUN_D +
    JUL_D + AUG_D + SEP_D + OCT_D + NOV_D + DEC_D;

pub(crate) const DIY: [usize; 2] = [DIYC, DILC];

pub(crate) const DIM: [[usize; 2]; 12] = [
    [JAN_D, JAN_D],
    [FEB_D, FEB_L],
    [MAR_D, MAR_D],
    [APR_D, APR_D],
    [MAY_D, MAY_D],
    [JUN_D, JUN_D],
    [JUL_D, JUL_D],
    [AUG_D, AUG_D],
    [SEP_D, SEP_D],
    [OCT_D, OCT_D],
    [NOV_D, NOV_D],
    [DEC_D, DEC_D],
];

