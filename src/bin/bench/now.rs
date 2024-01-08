const REPEAT: u64 = 1000000;

fn main() {

    let durs = (0..REPEAT).into_iter()
        .map(|i| {
            let start = std::time::Instant::now();
            utc::Utc::now();
            (i, (std::time::Instant::now() - start).as_nanos() as u64)
        })
        .collect::<Vec<(u64, u64)>>();

    let avg = durs.iter().map(|d| d.1).sum::<u64>() / durs.len() as u64;

    let min = durs.iter().map(|d| d.1).min().unwrap();

    println!("Total Runs,Average (nS),Minimum (nS)");
    println!("{REPEAT},{avg},{min}");
}
