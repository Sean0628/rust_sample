fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res: [[i32; 3]; 3] = [[0;3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            res[j][i] = matrix[i][j];
        }
    }

    return res;
}

fn prity_print(matrix: &[[i32; 3]; 3]) {
    for i in 0..3 {
        for j in 0..3 {
            print!("{} ", matrix[i][j]);
        }
        println!("");
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    prity_print(&matrix);

    let transpose = transpose(matrix);
    println!("transposed:");
    prity_print(&transpose);
}
