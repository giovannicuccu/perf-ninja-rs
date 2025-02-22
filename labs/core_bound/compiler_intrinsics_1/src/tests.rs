use crate::{image_smoothing, init, InputVector, OutputVector, RADIUS};
use std::cmp::min;

#[test]
fn validate() {
    let in_a = init();

    let mut expected: OutputVector = vec![0; in_a.len()];
    let mut received: OutputVector = vec![0; in_a.len()];

    reference_solution(&in_a, RADIUS, &mut expected);
    image_smoothing(&in_a, RADIUS, &mut received);

    assert_eq!(expected.len(), received.len());
    assert_eq!(expected, received);
}

fn reference_solution(input: &InputVector, radius: usize, output: &mut OutputVector) {
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

#[test]
fn test_shift() {
    let mut arr1 = [1, 2, 3, 4, 5];
    {
        let len = arr1.len();
        arr1.copy_within(..len-1, 1);
        arr1[0] = 0;
    }
    println!("Using copy_within: {:?}", arr1);  // [0, 1, 2, 3, 4]

    // Example 2: Using rotate_right
    let mut arr2 = [1, 2, 3, 4, 5];
    arr2.rotate_right(1);
    arr2[0] = 0;
    println!("Using rotate_right: {:?}", arr2);  // [0, 1, 2, 3, 4]

    // Example 3: Manual implementation
    let mut arr3 = [1, 2, 3, 4, 5];
    for i in (1..arr3.len()).rev() {
        arr3[i] = arr3[i-1];
    }
    arr3[0] = 0;
    println!("Manual implementation: {:?}", arr3);
}
