#![allow(unused)]

use std::ops::*;

#[derive(Clone)]
pub struct BiggerUInt {
    digits: Vec<u8>
}

impl BiggerUInt {
    pub fn dbg_display(self) -> String {
        let mut cloned = self.digits.clone();
        cloned.reverse();
        format!("{:?}", cloned)
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
