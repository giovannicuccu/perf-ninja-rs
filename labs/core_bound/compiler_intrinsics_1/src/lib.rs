use std::cmp::min;

#[cfg(test)]
mod tests;

pub type InputVector = Vec<u8>;
pub type OutputVector = Vec<u16>;
pub const RADIUS: usize = 13; // assume diameter (2 * radius + 1) to be less
                              // than 256 so results fits in uint16_t

pub const BATCH_SIZE: usize = 8;
pub fn image_smoothing(input: &InputVector, radius: usize, output: &mut OutputVector) {
    let mut pos = 0;
    let mut current_sum: u16 = 0;
    let size = input.len();


    // 1. left border - time spend in this loop can be ignored, no need to
    // optimize it
    for i in 0..min(size, radius) {
        current_sum += input[i] as u16;
    }

    let mut limit = min(radius + 1, size.saturating_sub(radius));
    while pos < limit {
        current_sum += input[pos + radius] as u16;
        output[pos] = current_sum;
        pos += 1;
    }

    // 2. main loop.
    limit = size.saturating_sub(radius);

    let mut diff=[0_16;BATCH_SIZE];

    while pos+BATCH_SIZE < limit {

        for (i,val) in diff.iter_mut().enumerate() {
            *val = -(input[pos + i  - radius - 1] as i16) +
                input[pos + i + radius] as i16;
        }

        let shift=shift_right_1(&diff);
        for (dv,sv) in diff.iter_mut().zip(shift) {
            *dv+=sv;
        }

        let shift=shift_right_2(&diff);
        for (dv,sv) in diff.iter_mut().zip(shift) {
            *dv+=sv;
        }

        let shift=shift_right_4(&diff);
        for ((out_v,dv),sv) in output[pos..pos+BATCH_SIZE].iter_mut().zip(diff).zip(shift) {
            *out_v=(current_sum as i16+dv+sv) as u16;
        }
/*        for i in 0 .. 8 {
            output[pos+i]= (current_sum as i16 + diff[i]+shift[i]) as u16;
        }*/
/*
        for i in 0 .. 8 {
            println!("i {} cs {} first {} second {} diff {}",i, current_sum,
                     input[pos + i - radius - 1],input[pos + i + radius], diff[i]);
            current_sum -= input[pos + i - radius - 1] as u16;
            current_sum += input[pos + i + radius] as u16;
            output_ref[pos+i] = current_sum;

            if output_ref[pos+i]!=output[pos+i] {
                println!("inside i {} cs {} first {} second {} diff {}",i, current_sum,
                         input[pos + i - radius - 1],input[pos + i + radius], diff[i]);
                panic!("error at pos {} expected {} found {}", pos, output_ref[pos+1],output[pos+1]);
            }
        }
*/

        current_sum=output[pos+BATCH_SIZE-1];
        pos += BATCH_SIZE;
    }

    while pos < limit {
        current_sum -= input[pos - radius - 1] as u16;
        current_sum += input[pos + radius] as u16;
        output[pos] = current_sum;
        pos += 1;
    }

    // 3. special case, executed only if size <= 2*radius + 1
    limit = min(radius + 1, size);
    while pos < limit {
        output[pos] = current_sum;
        pos += 1;
    }

    // 4. right border - time spend in this loop can be ignored, no need to
    // optimize it
    while pos < size {
        current_sum -= input[pos - radius - 1] as u16;
        output[pos] = current_sum;
        pos += 1;
    }
}



fn shift_right_1<const N: usize>(input: &[i16; N]) -> [i16; N] {
    let mut result = [0; N];
    //if N > 1 {
        result[1..].copy_from_slice(&input[..N-1]);
    //}
    result
}

fn shift_right_2<const N: usize>(input: &[i16; N]) -> [i16; N] {
    let mut result = [0; N];
    //if N > 2 {
        result[2..].copy_from_slice(&input[..N-2]);
    //}
    result
}

fn shift_right_4<const N: usize>(input: &[i16; N]) -> [i16; N] {
    let mut result = [0; N];
    //if N > 4 {
        result[4..].copy_from_slice(&input[..N-4]);
    //}
    result
}

pub fn image_smoothing_orig(input: &InputVector, radius: usize, output: &mut OutputVector) {
    let mut pos = 0;
    let mut current_sum: u16 = 0;
    let size = input.len();

    // 1. left border - time spend in this loop can be ignored, no need to
    // optimize it
    for i in 0..min(size, radius) {
        current_sum += input[i] as u16;
    }

    let mut limit = min(radius + 1, size.saturating_sub(radius));
    while pos < limit {
        current_sum += input[pos + radius] as u16;
        output[pos] = current_sum;
        pos += 1;
    }

    // 2. main loop.
    limit = size.saturating_sub(radius);
    while pos < limit {
        current_sum -= input[pos - radius - 1] as u16;
        current_sum += input[pos + radius] as u16;
        output[pos] = current_sum;
        pos += 1;
    }

    // 3. special case, executed only if size <= 2*radius + 1
    limit = min(radius + 1, size);
    while pos < limit {
        output[pos] = current_sum;
        pos += 1;
    }

    // 4. right border - time spend in this loop can be ignored, no need to
    // optimize it
    while pos < size {
        current_sum -= input[pos - radius - 1] as u16;
        output[pos] = current_sum;
        pos += 1;
    }
}

const N: usize = 40_000;

pub fn init() -> InputVector {
    use rand::prelude::*;
    let mut generator = thread_rng();
    let mut data = Vec::with_capacity(N);
    data.resize_with(N, || generator.gen::<u8>());
    data
}
