fn number_of_days_in_month(year: i32, month_i: i32) -> i32
{
    match month_i {
        1 => 31,
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {29} else {28},
        _ => 0,
    }
}

#[test]
fn test() {
    assert_eq!(number_of_days_in_month(1999, 2), 28);
    assert_eq!(number_of_days_in_month(1996, 2), 29);
    assert_eq!(number_of_days_in_month(1900, 2), 28);
    assert_eq!(number_of_days_in_month(2000, 2), 29);
}

fn main() {
    let mut sundays = 0;
    let mut first_day_in_month = -1;
    for year in 1900..2000 {
        for month in 1..12 {
            first_day_in_month += number_of_days_in_month(year, month);
            if first_day_in_month % 7 == 0 && year > 1900 {
                sundays += 1;
            }
        }
    }
    println!("{}", sundays);
}
