/*
   on every year that is evenly divisible by 4
   except every year that is evenly divisible by 100
   unless the year is also evenly divisible by 400
*/

fn is_divisible_by(year: u32, case: u32) -> bool {
    if case == 0 {
        return false;
    }
    year % case == 0
}

fn leap_year_excpetion(year: u32) -> bool {
    if is_divisible_by(year, 100) && is_divisible_by(year, 400){
        true
    }else if is_divisible_by(year, 100) && !is_divisible_by(year, 400){
        false
    } else {
        true
    }
}

pub fn is_leap_year(year:  u32) -> bool {
  is_divisible_by(year, 4) && leap_year_excpetion(year)
}
