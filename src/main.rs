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
    algorithms::pattern::number_base(n);
    algorithms::pattern::left_triangle_count(n);
    algorithms::pattern::left_triangle_alphabet(n);
    algorithms::pattern::opposite_left_triangle_alphabet(n);
    algorithms::pattern::same_line_alphabet_left_triangle(n);
    algorithms::pattern::opposite_desc_triangle(n);
    algorithms::pattern::star_box_pyramid(n);
    algorithms::pattern::two_pyramid_combined_tip(n);
    algorithms::pattern::mid_gap_box(n);
    algorithms::pattern::number_ring(n);
}