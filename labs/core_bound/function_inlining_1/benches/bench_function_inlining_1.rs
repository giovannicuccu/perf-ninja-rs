use criterion::{criterion_group, criterion_main, Criterion};

use function_inlining_1::{init, solution, solution_opt};

fn bench1(c: &mut Criterion) {
    let mut group = c.benchmark_group("lab");
    let arr = init();
    group.bench_function("ref", |b| {
        b.iter(|| {
            let mut copy = arr;
            solution(&mut copy);
            std::hint::black_box(copy);
        });
    });
    group.bench_function("opt", |b| {
        b.iter(|| {
            let mut copy = arr;
            solution_opt(&mut copy);
            std::hint::black_box(copy);
        });
    });
    group.finish();
}

criterion_group!(benches, bench1);
criterion_main!(benches);
