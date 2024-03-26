use utc::Utc;

fn main() {
    println!("{}", Utc::now().as_rfc3339_with_nano(6));
}
