const REPEAT: u64 = 10000;

fn main() {

    let durs = std::thread::scope(|scope| {
        let mut threads = Vec::new();
        for i in 0..REPEAT {
            match std::thread::Builder::new()
                .name(format!("utc_dur_test_{i}"))
                .spawn_scoped(scope, move || {
                    let start = std::time::Instant::now();
                    utc::Utc::now();
                    let end = std::time::Instant::now();
                    (i, (end - start).as_nanos() as u64)
                })
            {
                Ok(t) => threads.push(t),
                Err(_) => {},
            }
        }
        threads.into_iter()
            .map(|t| t.join())
            .collect::<Vec<std::thread::Result<(u64, u64)>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<(u64, u64)>>()
    });

    let avg = durs.iter().map(|d| d.1).sum::<u64>() / durs.len() as u64;

    let min = durs.iter().map(|d| d.1).min().unwrap();
    let min_run = durs.iter().filter(|d| d.1.eq(&min)).nth(0).unwrap().0;

    let max = durs.iter().map(|d| d.1).max().unwrap();
    let max_run = durs.iter().filter(|d| d.1.eq(&max)).nth(0).unwrap().0;

    println!("Over {} successful runs:", durs.len());
    println!("\tAverage: {avg} nano-seconds");
    println!("\tMinimum: {min} nano seconds on {min_run}th run");
    println!("\tMaximum: {max} nano seconds on {max_run}th run");
}
