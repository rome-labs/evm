#![allow(clippy::cast_possible_truncation)]

use crate::U256;
use crate::utils::{Sign, I256};

/// Signed less-than comparison
pub fn slt(op1: U256, op2: U256) -> U256 {
	let op1: I256 = op1.into();
	let op2: I256 = op2.into();

	if op1.lt(&op2) {
		U256::one()
	} else {
		U256::zero()
	}
}

/// Signed greater-than comparison
pub fn sgt(op1: U256, op2: U256) -> U256 {
	let op1: I256 = op1.into();
	let op2: I256 = op2.into();

	if op1.gt(&op2) {
		U256::one()
	} else {
		U256::zero()
	}
}

/// Is-zero comparison
pub fn iszero(op1: U256) -> U256 {
	if op1 == U256::zero() {
		U256::one()
	} else {
		U256::zero()
	}
}

/// Bitwise NOT operation
pub fn not(op1: U256) -> U256 {
	!op1
}

/// Retrieve single byte from word
pub fn byte(op1: U256, op2: U256) -> U256 {
	let i = op1.as_usize();
	if i >= 32 {
		U256::zero()
	} else {
		let mut buf = [0u8; 32];
		op2.to_big_endian(&mut buf);
		U256::from(buf[i])
	}
}

/// Left shift operation
pub fn shl(shift: U256, value: U256) -> U256 {
	if value == U256::zero() || shift >= U256::from(256) {
		U256::zero()
	} else {
		value << shift.as_usize()
	}
}

/// Right shift operation
pub fn shr(shift: U256, value: U256) -> U256 {
	if value == U256::zero() || shift >= U256::from(256) {
		U256::zero()
	} else {
		value >> shift.as_usize()
	}
}

/// Arithmetic (signed) right shift operation
pub fn sar(shift: U256, value: U256) -> U256 {
	let value = I256::from(value);

	if value == I256::zero() || shift >= U256::from(256) {
		let I256(sign, _) = value;
		match sign {
			// value is 0 or >=1, pushing 0
			Sign::Plus | Sign::NoSign => U256::zero(),
			// value is <0, pushing -1
			Sign::Minus => I256(Sign::Minus, U256::one()).into(),
		}
	} else {

		match value.0 {
			Sign::Plus | Sign::NoSign => value.1 >> shift.as_usize(),
			Sign::Minus => {
				let shifted = ((value.1.overflowing_sub(U256::one()).0) >> shift.as_usize())
					.overflowing_add(U256::one()).0;
				I256(Sign::Minus, shifted).into()
			}
		}
	}
}
