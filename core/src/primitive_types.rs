#![allow(clippy::use_self)]

use std::io::{Read, Write};
use fixed_hash::{construct_fixed_hash, impl_fixed_hash_conversions};
use uint::{construct_uint};


construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 20 bytes (160 bits) size.
	pub struct H160(20);
}
construct_fixed_hash! {
	/// Fixed-size uninterpreted hash type with 32 bytes (256 bits) size.
	pub struct H256(32);
}

impl_fixed_hash_conversions!(H256, H160);


construct_uint! {
	/// 256-bit unsigned integer.
	pub struct U256(4);
}

construct_uint! {
	/// 512-bit unsigned integer.
	pub struct U512(8);
}

impl_rlp::impl_uint_rlp!(U256, 4);
impl_rlp::impl_fixed_hash_rlp!(H160, 20);
impl_rlp::impl_fixed_hash_rlp!(H256, 32);

impl_serde::impl_uint_serde!(U256, 4);
impl_serde::impl_fixed_hash_serde!(H160, 20);
impl_serde::impl_fixed_hash_serde!(H256, 32);


impl From<U256> for U512 {
	fn from(value: U256) -> U512 {
		let U256(ref arr) = value;
		let mut ret = [0; 8];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		U512(ret)
	}
}

/// Error type for conversion.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
	/// Overflow encountered.
	Overflow,
}

impl core::convert::TryFrom<U512> for U256 {
	type Error = Error;

	fn try_from(value: U512) -> Result<U256, Error> {
		let U512(ref arr) = value;
		if arr[4] | arr[5] | arr[6] | arr[7] != 0 {
			return Err(Error::Overflow);
		}
		let mut ret = [0; 4];
		ret[0] = arr[0];
		ret[1] = arr[1];
		ret[2] = arr[2];
		ret[3] = arr[3];
		Ok(U256(ret))
	}
}


/// Add Serde serialization support to a fixed-sized hash type created by `construct_fixed_hash!`.
#[macro_export]
macro_rules! impl_fixed_hash_borsh {
	($name: ident) => {
		impl borsh::BorshSerialize for $name {
			fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        		writer.write_all(&self.0)?;

				Ok(())
    		}
		}

		impl borsh::BorshDeserialize for $name {

			fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {
				const LEN: usize = core::mem::size_of::<$name>();
				let mut buf = [0_u8; LEN];
				let read_len = reader.read(&mut buf)?;

				if read_len < LEN {
					return Err(borsh::io::Error::new(
						borsh::io::ErrorKind::InvalidInput,
						"ERROR_UNEXPECTED_LENGTH_OF_INPUT"
					));
				}

				Ok(Self::from_slice(&buf))
    		}
		}
	};
}

impl_fixed_hash_borsh!(H160);
impl_fixed_hash_borsh!(H256);

impl borsh::BorshSerialize for U256 {
	fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
		let buffer: [u8; 32] = unsafe { core::mem::transmute_copy(self) };
		writer.write_all(&buffer)?;

		Ok(())
	}
}

impl borsh::BorshDeserialize for U256 {
	fn deserialize_reader<R: Read>(reader: &mut R) -> std::io::Result<Self> {

		const LEN: usize = core::mem::size_of::<U256>();
		let mut buf = [0_u8; LEN];
		let read_len = reader.read(&mut buf)?;

		if read_len < LEN {
			return Err(borsh::io::Error::new(
				borsh::io::ErrorKind::InvalidInput,
				"ERROR_UNEXPECTED_LENGTH_OF_INPUT"
			));
		}

		let value: U256 = unsafe { core::mem::transmute(buf) };

		Ok(value)
	}


}


impl U256 {
	pub fn into_big_endian_fast(self, buffer: &mut [u8]) {
		let (low, high): (u128, u128) = unsafe { core::mem::transmute(self) };

		buffer[..16].copy_from_slice(&high.to_be_bytes());
		buffer[16..].copy_from_slice(&low.to_be_bytes());
	}

	#[must_use]
	pub fn from_big_endian_fast(buffer: &[u8]) -> U256 {
		assert!(32 >= buffer.len());

		let mut data = [0_u8; 32];
		data[32 - buffer.len()..32].copy_from_slice(buffer);
		
		unsafe {
			let (high, low): (u128, u128) =  core::mem::transmute(data);
			core::mem::transmute((u128::from_be(low), u128::from_be(high)))
		}
	}
}

impl From<U256> for H256 {
	fn from(value: U256) -> H256 {
		let mut h = H256::default();
		value.into_big_endian_fast(&mut h[..]);
		h
	}
}

impl From<U256> for H160 {
	fn from(value: U256) -> H160 {
		H256::from(value).into()
	}
}
