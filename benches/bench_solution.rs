use aoc_2020::day1::solve_day1a;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_day1a(c: &mut Criterion) {
    c.bench_function("day1a", |b| b.iter(|| black_box(solve_day1a())));
}

criterion_group!(benches, bench_day1a);
criterion_main!(benches);
