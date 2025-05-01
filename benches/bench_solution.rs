use aoc_2020::day1::*;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_day1a(c: &mut Criterion) {
    c.bench_function("day1a", |b| b.iter(|| black_box(solve_day1a())));
}

fn bench_day1b(c: &mut Criterion) {
    c.bench_function("day1b", |b| b.iter(|| black_box(solve_day1b())));
}

criterion_group!(benches, bench_day1a, bench_day1b);
criterion_main!(benches);
