//! Computes UTC datetimes after 1970-01-01T00:00:00 using only the
//! [time](`std::time`) module.

#[cfg(test)]
mod test;

mod statics;
mod utc;

pub use crate::utc::Utc;

