pub fn is_leap_year(year: u64) -> bool {
    let is_mod = |n| year % n == 0;
    is_mod(4) && !is_mod(100) || is_mod(400)
}
