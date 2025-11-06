use std::{cmp, collections::HashMap, i32};

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

pub fn rotate_array_d_places(arr: &mut Vec<i32>, d: i32) {
    if arr.len() == 0 {
        return ;
    }
    let len = arr.len();
    let k = (d as usize) % len ;

    arr[..len-k].reverse();
    arr[len-k..].reverse();
    arr.reverse();
}

pub fn move_zeroes_to_end( arr:  &mut Vec<i32> ) -> &mut Vec<i32> {
    if arr.len() == 0 {
        return arr;
    }
    let mut zeroidx = 0;
    for idx in 0..arr.len() {
        if arr[idx] != 0 {
            arr.swap(zeroidx,idx);
            zeroidx = zeroidx + 1;
        }
    }
    return arr;
}

pub fn max_consecutive_ones(arr: &[i32]) -> i32 {
    if arr.len() == 0 {
        return 0;
    }
    let mut ans = i32::MIN;
    let mut count = 0;

    for &value in arr {
        if value == 1 {
            count = count +1;
        }
        else if value != 1 {
            count = 0;
        }
        ans = cmp::max(ans, count);
    }
    return ans;
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut count_map = HashMap::new();
    for value in nums {
        *count_map.entry(value).or_insert(0) += 1;
    }
    for (num,value) in count_map {
        if value == 1 {
            return num;
        }
    }        
    return -1;
}

pub fn largest_arr_with_given_sum( nums: Vec<i32>, k: i32 ) -> i32{
    let (mut left,mut right) = (0,0);
    let mut sum= nums[0];
    let mut max_len = 0;

    while right < nums.len() {
        while left <= right && sum > k {
            sum -= nums[left];
            left += 1;
        }
        if sum == k {
            max_len  = cmp::max(max_len, right - left +1);
        }
        right = right+1;
        if right < nums.len() {
            sum = sum + nums[right];
        }
    }
    max_len as i32
}