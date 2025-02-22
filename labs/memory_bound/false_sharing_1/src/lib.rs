#![feature(thread_id_value)]

#[cfg(test)]
mod tests;

use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;

#[repr(align(64))]
struct CacheAligned(AtomicU32);

pub fn solution(data: &[u32], thread_count: usize) -> i32 {
    // Using std::atomic counters to disallow compiler to promote `target`
    // memory location into a register. This way we ensure that the store
    // to `target` stays inside the loop.

    let mut accumulators = Vec::with_capacity(thread_count);
    accumulators.resize_with(thread_count, || CacheAligned::new(0));
    let chunks = data.chunks(data.len() / thread_count);
    thread::scope(|s| {
        // C++ uses `#pragma omp for` which splits into chunks, just like this
        for (idx, chunk) in chunks.enumerate() {
            let target_acc = &accumulators[idx % thread_count];
            s.spawn(move || {
                let mut acc=0_u32;
                for v in chunk {
                    // Perform computation on each input
                    let mut item = *v;
                    item += 1000;
                    item ^= 0xADEDAE;
                    item |= item >> 24;

                    // Write result to accumulator
                    acc+=item%13;
                }
                target_acc.fetch_add(acc, Ordering::SeqCst);
            });
        }
    });

    accumulators
        .iter()
        .map(|v| v.load(Ordering::SeqCst) as usize)
        .sum::<usize>()
        .try_into()
        .unwrap()
}
