use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

use compiler_intrinsics_2::{solution, solution_opt, solution_simd, solution_simd_1};

fn bench1(c: &mut Criterion) {
    let inputs = vec![
        /*"counter-example.txt" // input where sequential solution is faster*/
        "inputs/LoopVectorize.txt", // a large C++ file from the LLVM compiler.
        "inputs/MarkTwain-TomSawyer.txt", // a typical text file with long lines.
    ];

    let mut input_contents = Vec::with_capacity(inputs.len());
    for input in &inputs {
        let input_content = read_to_string(input).unwrap();
        input_contents.push(input_content);
    }
    let mut group = c.benchmark_group("std");
    group.bench_function("lab", |b| {
        b.iter(|| {
            for input_content in &input_contents {
                let output = solution(&input_content);
                std::hint::black_box(output);
            }
        });
    });
    group.bench_function("swar", |b| {
        b.iter(|| {
            for input_content in &input_contents {
                let output = solution_opt(&input_content);
                std::hint::black_box(output);
            }
        });
    });
    group.bench_function("simd", |b| {
        b.iter(|| {
            for input_content in &input_contents {
                let output = solution_simd(&input_content);
                std::hint::black_box(output);
            }
        });
    });
    group.bench_function("simd_evo", |b| {
        b.iter(|| {
            for input_content in &input_contents {
                let output = solution_simd_1(&input_content);
                std::hint::black_box(output);
            }
        });
    });
    group.finish();
}

criterion_group!(benches, bench1);
criterion_main!(benches);
