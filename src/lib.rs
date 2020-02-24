pub fn is_leap_year(year: u64) -> bool {

    let cond1 = year % 4 == 0;
    let cond2 = year % 100 == 0;
    let cond3 = year % 400 == 0;

    if cond1 && (!cond2 || (cond2 && cond3)) {
        return true;
    } else {
        return false;
    }
}
