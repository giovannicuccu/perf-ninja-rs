#[cfg(test)]
mod tests;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::{_mm_prefetch, _MM_HINT_T0};
pub const HASH_MAP_SIZE: usize = 32 * 1024 * 1024 - 5;
const NUMBER_OF_LOOKUPS: usize = 1024 * 1024;

const UNUSED: i32 = i32::MAX;
pub struct HashMapT {
    m_vector: Vec<i32>,
    n_buckets: usize,
}
impl HashMapT {
    pub fn new(size: usize) -> HashMapT {
        HashMapT {
            m_vector: vec![UNUSED; size],
            n_buckets: size,
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let bucket: usize = val as usize % self.n_buckets;
        if self.m_vector[bucket] == UNUSED {
            self.m_vector[bucket] = val;
            true
        } else {
            false
        }
    }

    pub fn find(&self, val: i32) -> bool {
        let bucket = val as usize % self.n_buckets;
        self.m_vector[bucket] != UNUSED
    }

    pub fn prefetch (&self, val: i32) {
        let bucket = val as usize % self.n_buckets;
        unsafe {
            _mm_prefetch::<_MM_HINT_T0>(self.m_vector.as_ptr().add(bucket) as *const i8);
        }
    }
}

fn get_sum_of_digits(mut n: i32) -> i32 {
    let mut sum = 0;
    while n != 0 {
        sum += n % 10;
        n /= 10;
    }
    sum
}

const PREFETCH_SIZE: i32 = 64;
pub fn solution(hash_map: &HashMapT, lookups: &[i32]) -> i32 {
    let mut result = 0;

    for i in 0 .. (lookups.len()-PREFETCH_SIZE as usize) {
    //for &val in lookups {
        let val=lookups[i];
        if hash_map.find(val) {
            result += get_sum_of_digits(val);
            hash_map.prefetch(val+PREFETCH_SIZE);
        }
    }

    for i in (lookups.len()-PREFETCH_SIZE as usize) .. lookups.len() {
        //for &val in lookups {
        let val=lookups[i];
        if hash_map.find(val) {
            result += get_sum_of_digits(val);
        }
    }

    result
}

pub fn init(hash_map: &mut HashMapT) -> Vec<i32> {
    use rand::prelude::*;
    let mut generator = thread_rng();

    for _ in 0..HASH_MAP_SIZE {
        hash_map.insert(generator.gen());
    }

    let mut lookups = Vec::with_capacity(NUMBER_OF_LOOKUPS);
    lookups.resize_with(NUMBER_OF_LOOKUPS, || generator.gen());

    lookups
}
