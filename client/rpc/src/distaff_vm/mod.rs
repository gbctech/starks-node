// @file mod.rs
// @author ming (wmzhou@protonmail.com)
// @brief distaffvm rpc methods.
// @version 0.0.1
// @date 2020-10-03

// License Apache 2.0
// Copyright (C) 2020 Starks Network
// This file is part of the Starks Node program.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. 
// You should have received a copy of the Apache License 2.0
// along with this program. If not, see <https://www.apache.org/licenses/LICENSE-2.0>.

//! Distaff VM RPC API.

use self::error::Result;

use sp_core::{Bytes};
use distaff::{ self, StarkProof };

pub use sc_rpc_api::distaff_vm::*;
pub use self::gen_client::Client as DistaffVMClient;

/// Distaff API
#[derive(Debug)]
pub struct DistaffVM {

}

impl DistaffVM {
	/// Create new instance of Distaff VM API.
	pub fn new() -> Self{
		DistaffVM{

		}
	}
}

impl DistaffVMApi for DistaffVM {
	/// Distaff VM for proof verification
	fn distaffvm_verify(&self, program_hash: Bytes, public_inputs: Bytes, outputs : Bytes, proof : Bytes) -> Result<String>
	{
		let d_program_hash: [u8; 32] = bincode::deserialize(&program_hash).unwrap();
		let d_public_input: Vec<u128> = bincode::deserialize(&public_inputs).unwrap();
		let d_outputs: Vec<u128> = bincode::deserialize(&outputs).unwrap();
		let d_proof: StarkProof = bincode::deserialize(&proof).unwrap();
		//@todo : check the format and validity of input data
		match distaff::verify(&d_program_hash, &d_public_input, &d_outputs, &d_proof) {
			Ok(_) => Ok("verification passed".to_string().clone()),
			Err(_msg) => Ok("verification failed".to_string().clone())
		}
	}
}
