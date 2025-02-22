use std::arch::x86_64::{__m256i, _mm256_cmpeq_epi8, _mm256_loadu_si256, _mm256_movemask_epi8, _tzcnt_u32};
use std::cmp::{max, min};
use std::str;
use crate::simd::{compare_registers, from_u8x32_to_vector_register, register_to_i32, trailing_zero, u8x32, u8x32_mask};

#[cfg(test)]
mod tests;
mod simd;

const BYTES_LANE:usize=32;

// Find the longest line in a file.
// Implementation uses ternary operator with a hope that compiler will
// turn it into a CMOV instruction.
// The code inside the inner loop is equivalent to:
/*
if (s == '\n') {
  longestLine = std::max(curLineLength, longestLine);
  curLineLength = 0;
} else {
  curLineLength++;
}*/
pub fn solution(input_contents: &str) -> u32 {
    let mut longest_line = 0;
    let mut cur_line_length = 0;

    for s in input_contents.chars() {
        cur_line_length = if s == '\n' { 0 } else { cur_line_length + 1 };
        longest_line = max(cur_line_length, longest_line);
    }

    longest_line
}

pub fn solution_opt(input_contents: &str) -> u32 {
    //println!("opt\n\n");
    let mut longest_line = 0;
    let mut cur_line_length = 0;
    let mask:i64=0x0a0a0a0a0a0a0a0a;
    let content=input_contents.as_bytes();
    let len=input_contents.len();
    let mut current=0;
    while current+8<=len {
        //let strc=str::from_utf8(&content[current..current+8]).unwrap();
        //println!("content read={} len={}" ,strc,strc.chars().count());
        let input=i64::from_le_bytes(content[current..current+8].try_into().unwrap());
        let matching=input^mask;
        let value=matching-0x0101010101010101 & !matching & -9187201950435737472;
        //println!("{value:#b} value.trailing_zeros()={}",value.trailing_zeros());
        let pos=value.trailing_zeros()>>3;
        cur_line_length+=pos;
        longest_line = max(cur_line_length, longest_line);
        current+=if pos==8 {
            8usize
        } else {
            (pos+1) as usize
        };
        if pos!=8 {
            //println!("cur_line_length={} current={current}" ,cur_line_length);
            cur_line_length=0;
            //println!("newline");

        }


    }
    //println!("current={current} cur_line_length={cur_line_length} longest_line={longest_line}");
    while current<len {
        cur_line_length = if content[current] == 0xa { 0 } else { cur_line_length + 1 };
        //println!("longest_line={longest_line} cur_line_length={cur_line_length}");
        longest_line = max(cur_line_length, longest_line);
        current+=1;
        //println!("current={}",current);
    }

    longest_line

}
pub fn solution_simd(input_contents: &str) -> u32 {
    //https://www.youtube.com/watch?v=0WUihFxjzSE&list=PLRWO2AL1QAV6bJAU2kgB4xfodGID43Y5d
    let content=input_contents.as_bytes();
    let mask=u8x32_mask();
    let p_src = content.as_ptr();
    let input_len=content.len();
    let mut pos=0;
    let mut line_len=0;
    let mut longest_line = 0;
    while pos+BYTES_LANE<=input_len {
        let tmp;
        unsafe {
            tmp = p_src.offset(pos as isize) as *const u8x32;
        }
        let vector_register = from_u8x32_to_vector_register(tmp);
        let mask_register = compare_registers(vector_register, mask);
        let mask = register_to_i32(mask_register) as u32;
        let mask_pos = trailing_zero(mask);
        line_len += mask_pos;
        //println!("mask {mask:032b}");
        //println!("linelen {line_len} mask_pos {mask_pos}");
        longest_line = longest_line.max(line_len);

        pos+=if mask_pos==BYTES_LANE as u32 {
            BYTES_LANE
        } else {
            (mask_pos+1) as usize
        };
        if mask_pos!=BYTES_LANE as u32 {
            //println!("cur_line_length={} current={current}" ,cur_line_length);
            line_len=0;
            //println!("newline");

        }
    }
    //println!("after simd pos {pos}");
    while pos<input_len {
        line_len = if content[pos] == 0xa { 0 } else { line_len + 1 };
        //println!("longest_line={longest_line} cur_line_length={cur_line_length}");
        longest_line = max(line_len, longest_line);
        pos+=1;
        //println!("current={}",current);
    }
    longest_line
}

pub fn solution_simd_1(input_contents: &str) -> u32 {
    //https://www.youtube.com/watch?v=0WUihFxjzSE&list=PLRWO2AL1QAV6bJAU2kgB4xfodGID43Y5d
    let content=input_contents.as_bytes();
    let mask=u8x32_mask();
    let p_src = content.as_ptr();
    let input_len=content.len();
    let mut pos=0;
    let mut line_len=0;
    let mut longest_line = 0;
    while pos+BYTES_LANE<=input_len {
        let tmp;
        unsafe {
            tmp = p_src.offset(pos as isize) as *const u8x32;
        }
        let vector_register = from_u8x32_to_vector_register(tmp);
        let mask_register = compare_registers(vector_register, mask);
        let mut mask = register_to_i32(mask_register) as u32;
        let mask_pos = trailing_zero(mask);
        line_len += mask_pos;
        //println!("mask {mask:032b}");
        //println!("linelen {line_len} mask_pos {mask_pos}");
        longest_line = longest_line.max(line_len);
        let mut remaining:u32=BYTES_LANE as u32;
        //println!("line_len={line_len}");
        if mask_pos<BYTES_LANE as u32 {
            line_len=0;
            remaining=remaining - (mask_pos+1);
            //println!("mask pos {mask_pos}");
            mask>>=(mask_pos+1).min(31);
            //println!("mask {mask:032b}");
            while mask>0 {
                let mask_pos = trailing_zero(mask);
                line_len += mask_pos;
                //println!("inside linelen {line_len} mask_pos {mask_pos}");
                longest_line = longest_line.max(line_len);
                line_len=0;
                remaining=remaining - (mask_pos+1);
                mask>>=mask_pos+1;
                //println!("inside mask {mask:032b}");
            }
            line_len+=remaining;
        }
        pos+=32;
    }
    //println!("after simd pos {pos}");
    while pos<input_len {
        line_len = if content[pos] == 0xa { 0 } else { line_len + 1 };
        //println!("longest_line={longest_line} cur_line_length={cur_line_length}");
        longest_line = max(line_len, longest_line);
        pos+=1;
        //println!("current={}",current);
    }
    longest_line
}

pub fn pos(content: &[u8]) -> u32 {
    let mut current=0;
    let mask:i64=0x0a0a0a0a0a0a0a0a;
    let mut cur_line_length = 0;
    let len=content.len();
    while current+8<=len {
        let input=i64::from_le_bytes(content[current..current+8].try_into().unwrap());
        let matching=input^mask;
        let mut value=matching-0x0101010101010101 & !matching & -9187201950435737472;
        //println!("{value:#b} value.trailing_zeros()={}",value.trailing_zeros());
        let pos=value.trailing_zeros()>>3;
        println!("pos={}",pos);
        cur_line_length+=pos;
        current+=(pos+1) as usize;
    }
    cur_line_length
}


mod test {
    use crate::{pos, solution_opt, solution_simd};

    #[test]
    fn should_return_1_with_simd() {
        unsafe { assert_eq!(30, solution_simd("1\n345678901234567890123456789012")); }
    }

    #[test]
    fn should_return_32_with_simd() {
        unsafe { assert_eq!(32, solution_simd("12345678901234567890123456789012\n")); }
    }

    #[test]
    fn should_return_34_with_simd() {
        unsafe { assert_eq!(34, solution_simd("1234567890123456789012345678901212\n45678901234567890123456789012")); }
    }

    #[test]
    fn should_return_22_with_simd() {
        unsafe { assert_eq!(22, solution_simd("1234567890123456789012\n456789012\n")); }
    }

    #[test]
    fn should_return_22_multiple_match_with_simd() {
        unsafe { assert_eq!(22, solution_simd("1\n3\n5678901234567890123456\n89012\n")); }
    }
    #[test]
    fn should_return_7_with_simd_initial_line_break() {
        unsafe { assert_eq!(7, solution_simd("\n2345678")); }
    }

    #[test]
    fn should_return_7_with_simd() {
        unsafe { assert_eq!(7, solution_simd("1234567\n")); }
    }

    #[test]
    fn should_return_8_with_simd() {
        unsafe { assert_eq!(8, solution_simd("12345678")); }
    }

    #[test]
    fn should_return_41_with_simd() {
        unsafe { assert_eq!(41, solution_simd("This line is 41 chracters long...........\n1234")); }
    }
}
