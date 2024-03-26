use {
    utc::Utc,
    criterion:: { criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("now", |b| b.iter(|| Utc::now()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
