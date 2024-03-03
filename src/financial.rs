fn calc_p1_amount(min: i32, max: i32) -> f32{
    min as f32 + f32::sqrt((max - min) as f32)
}

fn calc_p2_amount(min: i32, max: i32) -> f32{
    min as f32  + 0.0175 * (max - min) as f32
}

pub fn cal_opt_credit_line(min: i32, max: i32) -> f32 {
    let p1 = calc_p1_amount(min, max);
    let p2 = calc_p2_amount(min, max);

    f32::max(p1, p2)
}
