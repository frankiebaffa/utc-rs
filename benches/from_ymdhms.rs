use {
    utc::Utc,
    criterion:: { criterion_group, criterion_main, Criterion, },
};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("from_ymdhms", |b| b.iter(|| Utc::from_ymdhms(
        3000, 12, 31, 23, 59, 59.999999_f64
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
