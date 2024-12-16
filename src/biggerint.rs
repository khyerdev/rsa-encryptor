#![allow(unused)]

use std::ops::*;

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq)]
pub struct BiggerUInt {
    digits: Vec<u8>
}

impl BiggerUInt {
    pub fn dbg_display(&self) -> String {
        let mut cloned = self.digits.clone();
        cloned.reverse();
        format!("{:?}", cloned)
    }

    fn new_empty() -> Self {
        Self { digits: Vec::new() }
    }
    pub fn from_u8(num: u8) -> Self {
        Self { digits: vec![num] }
    }
    pub fn from_u16(num: u16) -> Self {
        let insert = num;
        let mut digits = vec![0u8; 2];
        let divisor = (u8::MAX as u16) + 1;
        digits[1] = (insert / divisor) as u8;
        digits[0] = (insert % divisor) as u8;
        Self { digits }
    }
    pub fn from_u32(num: u32) -> Self {
        let mut insert = num;
        let mut digits = vec![0u8; 4];
        let mut pos = 3;
        while pos > 0 {
            let divisor = ((u8::MAX as u32) + 1).pow(pos as u32);
            digits[pos] = (insert / divisor) as u8;
            insert %= divisor;
            pos -= 1;
        }
        digits[0] = insert as u8;
        Self { digits }
    }
    pub fn from_u64(num: u64) -> Self {
        let mut insert = num;
        let mut digits = vec![0u8; 8];
        let mut pos = 7;
        while pos > 0 {
            let divisor = ((u8::MAX as u64) + 1).pow(pos as u32);
            digits[pos] = (insert / divisor) as u8;
            insert %= divisor;
            pos -= 1;
        }
        digits[0] = insert as u8;
        Self { digits }
    }
    pub fn from_u128(num: u128) -> Self {
        let mut insert = num;
        let mut digits = vec![0u8; 16];
        let mut pos = 15;
        while pos > 0 {
            let divisor = ((u8::MAX as u128) + 1).pow(pos as u32);
            digits[pos] = (insert / divisor) as u8;
            insert %= divisor;
            pos -= 1;
        }
        digits[0] = insert as u8;
        Self { digits }
    }
    pub fn from_base256(digits: Vec<u8>) -> Self {
        Self { digits }
    }
    pub fn from_base256_human_form(digits: Vec<u8>) -> Self {
        let mut digits = digits;
        digits.reverse();
        Self { digits }
    }

    fn push_digit(&mut self, digit: u8) {
        self.digits.push(digit);
    }

    pub fn truncate_zeros(mut self) -> Self {
        self.digits.reverse();
        let mut idx = 0;
        loop {
            if let Some(x) = self.digits.get(idx) {
                if *x == 0 {
                    self.digits.remove(idx);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.digits.reverse();
        self
    }

    fn sub_underflow(&self, rhs: Self) -> (Self, bool) {
        let mut output: Vec<u8> = Vec::new();
        let mut borrows: Vec<i16> = vec![0, 2];
        let mut place = 0;

        while place < self.digits.len().max(rhs.digits.len()) {
            if let Some(lhs) = self.digits.get(place) {
                let mut result = (*lhs as i16 + borrows[place]) - (*rhs.digits.get(place).unwrap_or(&0)) as i16;

                if result < 0 {
                    if let Some(next) = self.digits.get(place + 1) {
                        borrows[place + 1] = -1;
                        result += (u8::MAX as i16) + 1;
                    } else {
                        return (Self::from_u8(0), true)
                    }
                }

                output.push(result as u8);
            } else {
                return (Self::from_u8(0), true)
            }

            place += 1;
            borrows.push(0);
        }

        (Self { digits: output }, false)
    }

    fn slice(&self, range: Range<usize>) -> (Self, bool) {
        if let Some(range) = self.digits.get(range) {
            ( Self { digits: range.to_vec() }, false)
        } else {
            ( Self::new_empty(), true )
        }
    }

    pub fn divide(&self, divisor: Self) -> (Self, Self) {
        let dividend = self.clone().truncate_zeros();
        let divisor = self.clone().truncate_zeros();

        if divisor == Self::new_empty() { panic!("Quotient of BiggerUInt is Infinity due to divisor being Zero. Please reconsider.") }
        if dividend == Self::new_empty() { return (Self::from_u8(0), Self::from_u8(0)) }

        todo!()
    }

    pub fn pow(&self, exponent: Self) -> Self { // https://en.wikipedia.org/wiki/Exponentiation_by_squaring
        let first_digit = *exponent.digits.get(0).unwrap_or(&0);
        if exponent.clone().truncate_zeros() == Self::new_empty() {
            Self::from_u8(1)
        } else if first_digit % 2 == 0 {
            Self::pow(&(self.clone() * self.clone()), exponent / Self::from_u8(2))
        } else {
            self.clone() * Self::pow(&(self.clone() * self.clone()), (exponent - Self::from_u8(1)) / Self::from_u8(2))
        }
    }
}

impl Add for BiggerUInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut output: Vec<u8> = Vec::new();
        let mut carries: Vec<u8> = Vec::new();
        let mut place = 0;

        while place < self.digits.len().max(rhs.digits.len()) {
            let mut result = (*self.digits.get(place).unwrap_or(&0)) as u16 + (*rhs.digits.get(place).unwrap_or(&0)) as u16;

            if place > 0 {
                result += carries[place - 1] as u16;
            }

            let divisor = (u8::MAX as u16) + 1;
            carries.push((result / divisor) as u8);
            output.push((result % divisor) as u8);

            place += 1;
        }
        if carries[place - 1] > 0 {
            output.push(carries[place - 1]);
        }

        Self { digits: output }
    }
}
impl AddAssign for BiggerUInt {
    fn add_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs + rhs;
    }
}

impl Sub for BiggerUInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let (result, overflow) = self.sub_underflow(rhs);
        if overflow { panic!("Subtraction underflow of a BiggerUInt. Please reconsider.") }
        result
    }
}
impl SubAssign for BiggerUInt {
    fn sub_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs - rhs;
    }
}

impl Mul for BiggerUInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.digits.len() == 0 || rhs.digits.len() == 0 { return Self::new_empty() }

        let mut addition_matrix: Vec<Self> = Vec::new();
        let mut carries: Vec<u8> = Vec::new();

        let mut bottom_place = 0;
        while bottom_place < rhs.digits.len() {
            addition_matrix.push(Self::new_empty());
            if bottom_place > 0 {
                for _ in 0..bottom_place {
                    addition_matrix[bottom_place].push_digit(0u8);
                }
            }
            
            let mut top_place = 0;
            while top_place < self.digits.len() {
                let mut result = self.digits[top_place] as u16 * rhs.digits[bottom_place] as u16;

                if top_place > 0 {
                    result += carries[top_place - 1] as u16;
                }

                let divisor = (u8::MAX as u16) + 1;
                carries.push((result / divisor) as u8);
                addition_matrix[bottom_place].push_digit((result % divisor) as u8);

                top_place += 1;
            }

            if carries[top_place - 1] > 0 {
                addition_matrix[bottom_place].push_digit(carries[top_place - 1]);
            }

            carries = Vec::new();
            bottom_place += 1;
        }

        addition_matrix.iter().fold(Self::new_empty(), |acc, x| acc + x.clone())
    }
}
impl MulAssign for BiggerUInt {
    fn mul_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs * rhs;
    }
}

impl Div for BiggerUInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let (quotient, remainder) = self.divide(rhs);
        quotient
    }
}
impl DivAssign for BiggerUInt {
    fn div_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs / rhs;
    }
}

impl Rem for BiggerUInt {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let (quotient, remainder) = self.divide(rhs);
        return remainder
    }
}
impl RemAssign for BiggerUInt {
    fn rem_assign(&mut self, rhs: Self) {
        let lhs = self.clone();
        *self = lhs % rhs;
    }
}
