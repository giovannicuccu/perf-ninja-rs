use criterion::{criterion_group, criterion_main, Criterion};

use vectorization_2::{checksum, init, Blob, N, checksum_opt};

fn bench1(c: &mut Criterion) {
    let mut group = c.benchmark_group("lab");
    let mut blob: Blob = [0; N];
    init(&mut blob);

    group.bench_function("ref", |b| {
        b.iter(|| {
            let result = checksum(&blob);
            std::hint::black_box(result);
        });
    });
    group.bench_function("opt", |b| {
        b.iter(|| {
            let result = checksum_opt(&blob);
            std::hint::black_box(result);
        });
    });
    group.finish();
}

criterion_group!(benches, bench1);
criterion_main!(benches);
