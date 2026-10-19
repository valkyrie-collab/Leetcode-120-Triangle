fn min_triangle_path(triangle: &mut Vec<Vec<i32>>) {
    let rows: usize = triangle.len();
    
    for xr in 2..=rows {
        println!("{:?}", triangle);

        let cur_row: usize = rows - xr;
        let bel_row: usize = rows - (xr - 1);
        let cols: usize = triangle[cur_row].len();

        for xc in 0..cols {
            let num_one: i32 = triangle[cur_row][xc] + triangle[bel_row][xc];
            let num_two: i32 = triangle[cur_row][xc] + triangle[bel_row][xc + 1];
            let min_val: i32 = i32::min(num_one, num_two);
            triangle[cur_row][xc] = min_val;
        }

        println!("{:?}\n", triangle);
    }

}

fn main() {
    let mut triangle: Vec<Vec<i32>> = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
    min_triangle_path(&mut triangle);
    println!("min row, {}", triangle[0][0]);
}
