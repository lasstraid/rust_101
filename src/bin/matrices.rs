fn main() {
    // como crear una matriz
    let m: usize = 5;
    let n: usize = 6;

    let matrix: Vec<Vec<i32>> = (0..m).map(|x| vec![x as i32; n]).collect();

    println!("{:?}", matrix);

    // otra forma de crear
    let another_matrix: Vec<Vec<i32>> = vec![vec![0; 4]; 5];
    println!("{:?}", another_matrix);

    // recorrer
    for i in 0..m {
        for j in 0..n {
            println!("{}", matrix[i][j]);
        }
    }
}
