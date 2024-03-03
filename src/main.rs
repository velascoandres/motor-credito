use chrono::NaiveDate;
use credit_dest::{credit_data::{load_data, Genre}, credit_handler::{get_credit, Nomine, Options}, financial::cal_opt_credit_line};


fn main() {
    let persons = [
        (Nomine::A, (2022, 6, 12), Genre::Female),
        (Nomine::B, (1993, 12, 30), Genre::Female),
        (Nomine::C, (2020, 9, 19), Genre::Male),
        (Nomine::D, (2019, 1, 15), Genre::Male),
    ];

    println!("Credits:");

    let credit_data: credit_dest::credit_data::CreditData = load_data();

    for person in persons.into_iter(){
        let option = Options {
            nomine: person.0,
            start_date: NaiveDate::from_ymd_opt(person.1.0, person.1.1, person.1.2).unwrap(),
            genre: person.2,
        };

        let min_credit = get_credit(&credit_data.min_credits, &option);
        let max_credit = get_credit(&credit_data.max_credits,  &option);
        let credit_line = cal_opt_credit_line(min_credit, max_credit);
    
        println!("Min: {:?} Max: {:?} Line: {:?}",min_credit, max_credit, credit_line)
    }
}
