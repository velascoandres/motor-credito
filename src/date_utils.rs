use chrono::{offset::Local, NaiveDate};


pub fn get_month_diff(date: NaiveDate) -> i64 {

    let current_date = Local::now().date_naive();

    let diff: chrono::TimeDelta = current_date - date;
    let year_diff: i64 = diff.num_days() / 365;

    year_diff * 12
}