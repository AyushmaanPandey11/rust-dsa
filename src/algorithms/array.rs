use std::i32;

pub fn largest_element_in_array( arr: &[i32] ) -> i32 { 
    if arr.len() == 0 {
        return 0;
    }
    let mut max = arr[0];
    for &element in arr {
        if element > max {
            max = element;
        }
    }
    return max;
}

pub fn second_largest_element( arr: &[i32] ) -> i32 {
    if arr.len() < 2 {
        return -1;
    }
    let  ( mut largest, mut second_largest ) = ( i32::MIN, i32::MIN );
    for &element in arr {
        if element > largest {
            second_largest = largest;
            largest = element;
        }
        else if element > second_largest && element != largest {
            second_largest = element
        }
    }
    return second_largest;
}

pub fn check_if_arr_sorted( arr: &[i32] ) -> bool {
    if arr.len() == 0 {
        return false;
    }
    for (idx,_) in arr.iter().enumerate() {
        if arr[idx] > arr[idx+1] {
            return false;
        }
    }
    return true;
}

pub fn remove_duplicate_array( arr:  & mut Vec<i32> ) -> usize {
    if arr.len() == 0 {
        return 0;
    }
    let mut first = 0;
    for idx in 1..arr.len() {
        if arr[first] != arr[idx] {
            first = first + 1;
            arr[first] = arr[idx];
        }
    }
    return first+1;
}