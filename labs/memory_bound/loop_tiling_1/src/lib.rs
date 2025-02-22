#[cfg(test)]
mod tests;

pub type MatrixOfDoubles = Vec<Vec<f64>>;

pub fn init_matrix(matrix: &mut MatrixOfDoubles) {
    let size = matrix.len();

    for i in 0..size {
        for j in 0..size {
            matrix[i][j] = ((i + j) % 1024) as f64
        }
    }
}

pub fn solution(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    let size = matrix_in.len();
    let block_size=8;
    let blocks=size/block_size;
    //println!("blocks {}", blocks);
    for row_block in 0..blocks {
        //println!("row from {} to {}", row_block*block_size,row_block*block_size + block_size);
        for col_block in 0..blocks {
            //println!("col from {} to {}", col_block*block_size,col_block*block_size + block_size);
            for i in row_block*block_size.. row_block*block_size + block_size {
                for j in col_block*block_size .. col_block*block_size + block_size {
                    matrix_out[i][j] = matrix_in[j][i];
                }
            }
        }
    }
    if blocks%block_size != 0 {
        let dim_covered=blocks*block_size;
        //println!("dim_covered {}", dim_covered);
        for i in 0..dim_covered {
            for j in dim_covered.. size {
                matrix_out[i][j] = matrix_in[j][i];
            }
        }
        for i in dim_covered..size {
            for j in 0..size {
                matrix_out[i][j] = matrix_in[j][i];
            }
        }
    }
}

pub fn solution_o(matrix_in: &MatrixOfDoubles, matrix_out: &mut MatrixOfDoubles) {
    let size = matrix_in.len();

    for i in 0..size {
        for j in 0..size {
            matrix_out[i][j] = matrix_in[j][i];
        }
    }
}
