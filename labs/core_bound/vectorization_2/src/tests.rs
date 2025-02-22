#![cfg(test)]

use crate::{checksum, init, Blob, N};
use std::num::Wrapping;

fn original_checksum(blob: &Blob) -> u16 {
    let mut acc = Wrapping(0);
    for value in blob {
        acc += value;
        acc += (acc.0 < *value) as u16; // add carry
    }
    acc.0
}

#[test]
fn validate() {
    let mut blob: Blob = [0; N];
    init(&mut blob);

    let original_result = original_checksum(&blob);
    let result = checksum(&blob);

    assert_eq!(original_result, result);
}

#[test]
fn should_handle_overflow() {
    let mut blob: Blob = [0; N];
    blob[0]=u16::MAX;
    blob[1]=u16::MAX;
    blob[2]=2;
    blob[2]=2;

    let result = original_checksum(&blob);
    assert_eq!(4,result);
}

#[test]
fn should_handle_overflow_wrapping() {
    let mut acc = Wrapping(0);
    acc+=u16::MAX;
    acc+=u16::MAX;
    acc+=1;
    acc+=1;
    assert_eq!(4_u16,acc.0);

}
