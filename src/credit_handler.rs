use chrono::NaiveDate;

use crate::{credit_data::{GeneralCreditHash, Genre}, date_utils::get_month_diff};

pub enum Nomine {
   A,B,C,D
}

pub struct Options {
    pub genre: Genre,
    pub nomine: Nomine,
    pub start_date: NaiveDate
}

pub fn get_credit(data: &GeneralCreditHash, options: &Options) -> i32 {
    let credits_by_genre = data.get(&options.genre);
    
    let month_diff = get_month_diff(options.start_date);

    if credits_by_genre.is_none() {
        return 0
    }
    

    let mut credit_key = 0;

    let mut ordered_keys: Vec<i32> = credits_by_genre.unwrap().keys().copied().collect();
    ordered_keys.sort();

    let total = ordered_keys.len();

    for (index, credit_mk) in ordered_keys.iter().enumerate(){                
        if index == 0 && month_diff <= *credit_mk as i64  {
            credit_key = *credit_mk;

            break;
        }
        if index < total - 1 &&  month_diff == *credit_mk as i64 {
            credit_key = *credit_mk;

            break;
        }        

       if month_diff >= *credit_mk as i64 {
           credit_key = *credit_mk;

           break;
       }
    }

    if credit_key > 0 {
       let credit_nomine = credits_by_genre.unwrap().get(&credit_key).unwrap();

       return match options.nomine {
           Nomine::A => credit_nomine.a,
           Nomine::B => credit_nomine.b,
           Nomine::C => credit_nomine.c,
           Nomine::D => credit_nomine.d,
       };
    }    

    0
}