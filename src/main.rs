fn main() {
    let begin = std::time::Instant::now();

    // let int1 = BiggerUInt::from_u128(5738940257894325432342595293);
    // let int2 = BiggerUInt::from_u128(58195321852371409132849018403218402137);
    // let int3 = BiggerUInt::from_u128(889021711111155);
    // let int4 = BiggerUInt::from_u128(9999999999999999999999999999999999);
    // let int5 = BiggerUInt::from_u128(299999999999999999999999999999999999999);
    // let biggest_int = BiggerUInt::from_base256(vec![255; 50]);
    // println!("{}", (int1 + int2 + int3 - int4 + int5 + biggest_int).dbg_display());
    //
    // let mut added_int = BiggerUInt::from_u16(7249);
    // added_int += BiggerUInt::from_u32(552594992);
    // println!("{}", added_int.dbg_display());

    // let int6 = BiggerUInt::from_base256_human_form(vec![5, 2, 50]);
    // let int7 = BiggerUInt::from_base256_human_form(vec![3, 200]);
    // println!("{}", (int6 - int7).dbg_display());
    //
    // let mut subbed_int = BiggerUInt::from_u16(2958);
    // subbed_int -= BiggerUInt::from_u8(114);
    // println!("{}", subbed_int.dbg_display());

    // let int8 = BiggerUInt::from_u128(889021711111155);
    // let int9 = BiggerUInt::from_u128(9999999999777777222222229999999999);
    // let mut product = ((int8.clone() * int9.clone()) * (int8 * int9)).truncate_zeros();
    // println!("{}", product.dbg_display());
    //
    // product *= BiggerUInt::from_u8(2);
    // println!("{}", product.dbg_display());

    // TODO: division
    //
    // let inta = BiggerUInt::from_u8(25);
    // let intb = BiggerUInt::from_u16(3425);
    // println!("{}", (intb / inta).dbg_display());
    //
    // TODO: modulation

    // TODO: exponentiation
    //
    // let int40 = BiggerUInt::from_u8(25);
    // let int41 = BiggerUInt::from_u8(6);
    // println!("{}", (int40.pow(int41)).truncate_zeros().dbg_display());

    let time = begin.elapsed().as_micros();
    println!("time: {}μs", time);
}

mod biggerint;
use biggerint::*;
