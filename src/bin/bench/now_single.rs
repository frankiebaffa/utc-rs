fn main() {
    let start = std::time::Instant::now();
    utc::Utc::now();
    let end = std::time::Instant::now();
    let dur = (end - start).as_nanos();

    println!("{dur} nano seconds");
}
