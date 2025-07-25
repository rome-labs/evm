use alloc::vec::Vec;
use crate::{Capture, Stack, ExitError, Opcode,
			Machine, ExitReason,
			H160, H256, U256};
use evm_core::{Context, CreateScheme, ExitFatal, Transfer};

/// EVM context handler.
pub trait Handler {
	/// Type of `CREATE` interrupt.
	type CreateInterrupt;
	/// Feedback value for `CREATE` interrupt.
	type CreateFeedback;
	/// Type of `CALL` interrupt.
	type CallInterrupt;
	/// Feedback value of `CALL` interrupt.
	type CallFeedback;

	/// Get keccak hash from data.
	fn keccak256_h256(&self, data: &[u8]) -> H256;

	/// Get account nonce
	fn nonce(&self, address: H160) -> U256;
	/// Get balance of address.
	fn balance(&self, address: H160) -> U256;
	/// Get code size of address.
	fn code_size(&self, address: H160) -> U256;
	/// Get code hash of address.
	fn code_hash(&self, address: H160) -> H256;
	/// Get code of address.
	fn code(&self, address: H160) -> Vec<u8>;
	/// Get valids of address.
	fn valids(&self, address: H160) -> Vec<u8>;
	/// Get storage value of address at index.
	fn storage(&self, address: H160, index: U256) -> U256;
	fn transient_storage(&self, address: H160, index: U256) -> U256;

	/// Get the gas left value.
	fn gas_left(&self) -> U256;
	/// Get the gas price value.
	fn gas_price(&self) -> U256;
	/// Get execution origin.
	fn origin(&self) -> H160;
	/// Get environmental block hash.
	fn block_hash(&self, number: U256) -> H256;
	/// Get environmental block number.
	fn block_number(&self) -> U256;
	/// Get environmental coinbase.
	fn block_coinbase(&self) -> H160;
	/// Get environmental block timestamp.
	fn block_timestamp(&self) -> U256;
	/// Get environmental block difficulty.
	fn block_difficulty(&self) -> U256;
	/// Get environmental gas limit.
	fn block_gas_limit(&self) -> U256;
	/// Get environmental chain ID.
	fn chain_id(&self) -> U256;
	/// Set storage value of address at index.
	fn set_storage(&mut self, address: H160, index: U256, value: U256) -> Result<(), ExitError>;
	/// Set transient storage value of address at index.
	fn set_transient_storage(&mut self, address: H160, index: U256, value: U256) -> Result<(), ExitError>;
	/// Create a log owned by address with given topics and data.
	fn log(&mut self, address: H160, topcis: Vec<H256>, data: Vec<u8>) -> Result<(), ExitError>;
	/// Mark an address to be deleted, with funds transferred to target.
	fn mark_delete(&mut self, address: H160, target: H160) -> Result<(), ExitError>;
	/// Invoke a create operation.
	fn create(
		&mut self,
		caller: H160,
		scheme: CreateScheme,
		value: U256,
		init_code: Vec<u8>,
		target_gas: Option<u64>,
	) -> Capture<(ExitReason, Option<H160>, Vec<u8>), Self::CreateInterrupt>;
	/// Feed in create feedback.
	fn create_feedback(
		&mut self,
		_feedback: Self::CreateFeedback
	) -> Result<(), ExitError> {
		Ok(())
	}
	/// Invoke a call operation.
	fn call(
		&mut self,
		code_address: H160,
		transfer: Option<Transfer>,
		input: Vec<u8>,
		target_gas: Option<u64>,
		is_static: bool,
		context: Context,
	) -> Capture<(ExitReason, Vec<u8>), Self::CallInterrupt>;
	/// Feed in call feedback.
	fn call_feedback(
		&mut self,
		_feedback: Self::CallFeedback
	) -> Result<(), ExitError> {
		Ok(())
	}

	/// Pre-validation step for the runtime.
	fn pre_validate(
		&mut self,
		context: &Context,
		opcode: Opcode,
		stack: &Stack
	) -> Result<(), ExitError>;
	/// Handle other unknown external opcodes.
	fn other(
		&mut self,
		_opcode: Opcode,
		_stack: &mut Machine
	) -> Result<(), ExitFatal>;
}
