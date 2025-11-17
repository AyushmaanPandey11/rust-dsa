use crate::algorithms::recursion;

mod algorithms; 

fn main() {
    let n = 5;
    let mut arr = vec![1,8,2,4,9,9];
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

    // recursion 
    println!("Sum using recursion: {}",recursion::sum_recursion(n, 0));
    println!("Factorial using recursion: {}",recursion::factorial(n));
    print!("Print descreasing number:");recursion::descreasing_order(n);
    println!();
    println!("Fibonacci 5th number value: {}",recursion::fibonacci_numbers(n));
    println!("Find palindrome of string Malayalam: {}",recursion::is_palindrome(String::from("Malayalam")));

    // array
    println!("largest element in the array is: {}",algorithms::array::largest_element_in_array( &arr)); 
    println!("second largest element in the array is: {}",algorithms::array::second_largest_element(&arr)); 
    println!("checking if the array is sorted: {}",algorithms::array::check_if_arr_sorted(&arr));
    print!("Removing duplicated elements from array: "); 
    let last_index = algorithms::array::remove_duplicate_array(&mut arr);
    for value in 0..last_index {
        print!("{} ", arr[value])
    } 

    println!();
    print!("rotating array by 2 places : "); 
    algorithms::array::rotate_array_d_places(&mut arr, 2);
    for value in arr {
        print!("{} ", value)
    }

    println!();
    let mut v = vec![1, 0, 2, 0, 5, 0];
    print!("moving zeroes to right end : "); 
    let v = algorithms::array::move_zeroes_to_end(&mut v);
    for value in v {
        print!("{} ", value)
    }
    println!();
    println!("maximum consecutive ones are: {}",algorithms::array::max_consecutive_ones(&vec![1,1,1,0,1,1,1,1,1]));    
    println!("Number with one count is: {}",algorithms::array::single_number(vec![1,1,1,0,1,1,1,1,1]));  
    println!("maxinum sub-array length with sum 10 is: {}",algorithms::array::largest_arr_with_given_sum(vec![2,4,4,1,9],10));  
    println!("is sum of two element is present is given array: {}", algorithms::array::two_sum_problem(vec![2,4,4,1,9],10))
}