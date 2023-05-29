use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

use beancount_parser_2::parse;

const SAMPLE: &str = include_str!("../tests/samples/official.beancount");

pub fn run_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse official example");
    group.significance_level(0.01);
    group.throughput(Throughput::Bytes(SAMPLE.len() as u64));
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("parse::<&str, f64>", |b| {
        b.iter(|| parse::<&str, f64>(SAMPLE))
    });
    group.bench_function("parse::<&str, rust_decimal::Decimal>", |b| {
        b.iter(|| parse::<&str, rust_decimal::Decimal>(SAMPLE))
    });
    group.bench_function("parse::<String, f64>", |b| {
        b.iter(|| parse::<String, f64>(SAMPLE))
    });
    group.finish();
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
