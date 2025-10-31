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

pub fn star_pyramid(n: i32){
    for row in 0..n {
        for _ in 1..n - row {
            print!(" ");
        }

        for _ in 0..2*row+1 {
            print!("*");
        }

        for _ in 0..n - row {
            print!(" ");
        }
        println!();
    }
}

pub fn opposite_star_pyramid(n: i32){
    for row in 0..n {
        for _ in 0..row {
            print!(" ");
        }

        for _ in 0..2*(n -1 - row)+1 {
            print!("*");
        }

        for _ in 0..row {
            print!(" ");
        }
        println!();
    }
}

pub fn combined_star_pyramid(n : i32){
    for row in 0..n {
        for _ in 1..n - row {
            print!(" ");
        }

        for _ in 0..2*row+1 {
            print!("*");
        }

        for _ in 0..n - row {
            print!(" ");
        }
        println!();
    }
    for row in 0..n {
        for _ in 0..row {
            print!(" ");
        }

        for _ in 0..2*(n -1 - row)+1 {
            print!("*");
        }

        for _ in 0..row {
            print!(" ");
        }
        println!();
    }
}

pub fn side_triangle(n : i32){
    for row in 0..n+1 {
        for _ in 0..row+1 {
            print!("* ");
        }
        println!();
    }
    for row in 0..n+1 {
        for _ in 0..n - row +1 {
            print!("* ");
        }
        println!();
    }
}

pub fn number_base(n : i32){
    for row in 0..n{
        for col in 0..row {
            print!("{}",col+1);
        }
        for _ in 0..2*(n-row-1) {
            print!(" ");
        }
        for col in 0..row {
            print!("{}",row-col);
        }
        println!();
    }
}

pub fn left_triangle_count(n : i32){
    let mut  _count = 1;
    for row in 0..n+1{
        for _ in 0..row {
            print!("{} ",_count);
            _count = _count+1;
        }
        println!();
    }
}

pub fn left_triangle_alphabet(n : i32){
    let mut  _count = 0;
    for row in 0..n+1{
        for _ in 0..row {
            print!("{} ",(b'A' + _count) as char );
            _count = _count+1;
        }
        println!();
    }
}

pub fn opposite_left_triangle_alphabet(n : i32){
    for row in 0..n+1{
        for col in 0..n - row {
            print!("{} ",(b'A' + col as u8) as char );
        }
        println!();
    }
}

pub fn same_line_alphabet_left_triangle(n : i32){
    for row in 0..n{
        for _ in 0..row+1 {
            print!("{} ",(b'A' + row as u8) as char );
        }
        println!();
    }
}