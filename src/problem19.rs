/*
You are given the following information, but you may prefer to do some research for yourself.

1 Jan 1900 was a Monday.
Thirty days has September,
April, June and November.
All the rest have thirty-one,
Saving February alone,
Which has twenty-eight, rain or shine.
And on leap years, twenty-nine.
A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)? 
*/

pub fn problem19() -> usize {
    let mut count = 0;
    let mut date = (0, 0, 1_900); // day, month, year (days and months starting at 0)
    let mut day = 0; // day from 0 to 6

    while date != (0, 0, 2_001) {
        if day == 6 && date.0 == 0 && date.2 > 1_900 { count += 1; }

        let leap_year = date.2 % 4 == 0 && (date.2 % 100 != 0 || date.2 % 400 == 0);
        let end_month = match date.1 {
            8 | 3 | 5  | 10 => 30, // September, April, June and November
            1 => if leap_year { 29 } else { 28 },
            _ => 31
        };
        day = (day + 1) % 7;
        date.0 += 1;
        if date.0 == end_month {
            date.0 = 0;
            date.1 += 1;

            if date.1 == 12 {
                date.1 = 0;
                date.2 += 1
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem19() {
        assert_eq!(problem19(), 171);
    }
}