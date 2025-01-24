fn print_mat(matrix: &[Vec<i32>]) {
    for row in matrix {
        for &item in row {
            print!("{} ", item);
        }
        println!();
    }
    println!();
}

fn mat_mul(mat1:&[Vec<i32>], mat2:&[Vec<i32>]){
    let mat2_c: usize = mat2[0].iter().count();
    let mut temp: i32 = 0; // new element from summation
    let mut new_mat=vec![];
    let mut count_mul :i32 = 0;

    // first matrix row-wise multiplication
    // iterate over m1 rows, and creating new mat rows
    for row in mat1{
        let mut r2 : usize = 0;
        let mut c2 : usize = 0;

        let mut new_row = vec![];
        while c2 < mat2_c {
            for element in row{
                temp = temp + element*mat2[r2][c2];
                r2 +=1;
                count_mul +=1
            }
            new_row.push(temp);
            c2 +=1;
            r2 = 0;
            temp=0;
            
        }
       
        new_mat.push(new_row.clone());
        
    }

    print_mat(&new_mat);
    println!("{} multiplication required", count_mul)

}
fn main() {
    // 3x3 matrix
    let matrix = vec![
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3],
    ];
    //3x4 matrix
    let matrix2 = vec![
        vec![1, 2, 3, 4],
        vec![1, 2, 5, 4],
        vec![1, 2, 3, 5],
    ];
    
    print_mat(&matrix);
    print_mat(&matrix2);
    // matrix multiplication
    mat_mul(&matrix, &matrix2);
    
}
