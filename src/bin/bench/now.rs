const REPEAT: u128 = 1000000;

fn main() {
    let durs = (0..REPEAT).into_iter()
        .map(|_| {
            let start = std::time::Instant::now();
            utc::Utc::now();
            (std::time::Instant::now() - start).as_nanos()
        })
        .collect::<Vec<u128>>();

    let avg = durs.iter().map(|d| d).sum::<u128>() / durs.len() as u128;

    let min = durs.iter().map(|d| d).min().unwrap();

    let max = durs.iter().map(|d| d).max().unwrap();

    println!("Total Runs,Average (ns),Minimum (ns),Maximum (ns)");
    println!("{REPEAT},{avg},{min},{max}");
}
