mod algorithms; 

fn main() {
    let n = 5;
    println!("Pattern 1 ({}x{}):", n, n);
    algorithms::pattern::box_pattern(n);
    algorithms::pattern::triangle_pattern(n);
    algorithms::pattern::number_triangle(n);
    algorithms::pattern::same_number_triangle(n);
    algorithms::pattern::opposite_triangle_star(n);
    algorithms::pattern::opposite_number_triangle(n);
    algorithms::pattern::star_pyramid(n);
    algorithms::pattern::opposite_star_pyramid(n);
    algorithms::pattern::combined_star_pyramid(n);
    algorithms::pattern::side_triangle(n);
}