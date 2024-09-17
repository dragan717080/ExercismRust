fn main() {
    let year1 = 1600;
    let year2 = 1980;
    let year3 = 1990;

    println!("{}", is_leap_year(year1));
    println!("{}", is_leap_year(year2));
    println!("{}", is_leap_year(year3));
}

pub fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
