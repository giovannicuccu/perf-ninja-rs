use criterion::{criterion_group, criterion_main, Criterion};

use vectorization_1::{compute_alignment, compute_alignment_opt, init};

fn bench1(c: &mut Criterion) {
    let mut group = c.benchmark_group("lab");
    let (sequences1, sequences2) = init();
    group.bench_function("ref", |b| {
        b.iter(|| {
            let result = compute_alignment(&sequences1, &sequences2);
            std::hint::black_box(result);
        });
    });
    group.bench_function("opt", |b| {
        b.iter(|| {
            let result = compute_alignment_opt(&sequences1, &sequences2);
            std::hint::black_box(result);
        });
    });
    group.finish();
}

criterion_group!(benches, bench1);
criterion_main!(benches);
