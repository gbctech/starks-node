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
use primitives_stark::{ self };

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
		let d_program_hash = bincode::deserialize(&program_hash);
		let d_program_hash = match d_program_hash {
			Ok(program_hash) => program_hash,
			Err(_error) => return Ok("parse program_hash failed".to_string().clone())
		};
		let d_public_input = bincode::deserialize(&public_inputs);
		let d_public_input = match d_public_input {
			Ok(public_input) => public_input,
			Err(_error) => return Ok("parse public_inputs failed".to_string().clone())
		};
		let d_outputs = bincode::deserialize(&outputs);
		let d_outputs = match d_outputs {
			Ok(output) => output,
			Err(_error) => return Ok("parse outputs failed".to_string().clone())
		};
		let d_proof =  bincode::deserialize(&proof);
		let d_proof = match d_proof {
			Ok(proof) => proof,
			Err(_error) => return Ok("parse proof failed".to_string().clone())
		};

		// for types infer.
		let t_public_input:Vec<u128> = d_public_input;
		let t_outputs:Vec<u128> = d_outputs;

		match primitives_stark::verify(&d_program_hash, &t_public_input, &t_outputs, &d_proof) {
			Ok(_) => return Ok("verification passed".to_string().clone()),
			Err(_msg) => Ok("verification failed".to_string().clone())
		}
	}
}
