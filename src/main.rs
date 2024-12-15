fn main() {
    let int1 = BiggerUInt::from_u128(5738940257894325432342595293);
    let int2 = BiggerUInt::from_u128(58195321852371409132849018403218402137);
    let int3 = BiggerUInt::from_u128(889021711111155);
    let int4 = BiggerUInt::from_u128(9999999999999999999999999999999999);
    let int5 = BiggerUInt::from_u128(299999999999999999999999999999999999999);
    println!("{}", (int1 + int2 + int3 + int4 + int5).dbg_display());

    let mut added_int = BiggerUInt::from_u16(7249);
    added_int += BiggerUInt::from_u32(552594992);
    println!("{}", added_int.dbg_display());
}

mod biggerint;
use biggerint::*;
