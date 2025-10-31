pub fn box_pattern(n : i32){
    for _ in 0..n {
        for _ in 0..n {
            print!("* ");
        }
        println!();
    }
}

pub fn triangle_pattern(n : i32){
    for row in 1..n+1 {
        for _ in 0..row {
            print!("* ");
        }
        println!();
    }
}

pub fn number_triangle(n : i32){
    for row in 1..n+1{
        for col in 1..row+1 {
            print!("{} ",col);
        }
        println!();
    }
}

pub fn same_number_triangle(n : i32){
    for row in 1..n+1 {
        for _ in 1..row+1 {
            print!("{} ",row);
        }
        println!();
    }
}

pub fn opposite_triangle_star(n: i32){
    for row in 0..n {
        for _ in 0..n - row{
            print!("* ");
        }
        println!();
    }
}

pub fn opposite_number_triangle(n : i32){
    for row in 0..n {
        for col in 1..n + 1 - row{
            print!("{} ", col);
        }
        println!();
    }
}
