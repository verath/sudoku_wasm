use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solver;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("solve");

    group.bench_function("SIMPLE", |b| {
        b.iter(|| solver::solve(black_box(solver::SIMPLE)))
    });
    group.bench_function("HARD", |b| {
        b.iter(|| solver::solve(black_box(solver::HARD)))
    });
    group.bench_function("EXTREME", |b| {
        b.iter(|| solver::solve(black_box(solver::EXTREME)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
