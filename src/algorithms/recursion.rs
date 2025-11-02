pub fn sum_recursion(n: i32, sum: i32) -> i32{
    if n < 1 {
        return sum;
    } 
    return sum_recursion(n-1, sum+n);
}

pub fn factorial (n: i32) -> i32 {
    if n == 1 {
        return n;
    }
    return n*factorial(n-1);
}

pub fn descreasing_order(n: i32){
    if n < 1 {
        return;
    }
    print!("{} ",n);

    descreasing_order(n-1);
}

pub fn fibonacci_numbers(n: i32) -> i32{
    if n <= 1 {
        return n;
    }
    let last = fibonacci_numbers(n-1);
    let second_last = fibonacci_numbers(n-2);
    return last+second_last;
}

pub fn is_palindrome(s: String) -> bool {
        let s: Vec<char> = s.chars().map(|c| c.to_ascii_lowercase()).collect();
        if s.is_empty() {
            return true;
        }
        let mut start = 0;
        let mut end = s.len() -1 ;
        while start < end {
            while !s[start].is_alphanumeric() {
                start = start + 1;
            }
            while !s[end].is_alphanumeric() {
                end = end - 1;
            }
            if start >= end {
                return false;
            }
            if s[start] != s[end] {
                return false;
            }
            start = start + 1;
            end = end -1;
        }
        return true;
}