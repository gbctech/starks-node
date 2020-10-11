// @file mod.rs
// @author ming (wmzhou@protonmail.com)
// @brief distaffvm rpc api
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

 pub mod error;
 use jsonrpc_derive::rpc;
 use self::error::Result as DistaffVMResult;
 
 pub use self::gen_client::Client as DistaffVMClient;
 
 use sp_core::{Bytes};
 
 /// Substrate Distaff VM RPC API
 #[rpc]
 pub trait DistaffVMApi {
   /// distaff vm rpc api
   #[rpc(name = "distaffvm_verify")]
   fn distaffvm_verify(&self, program_hash: Bytes, public_inputs: Bytes, outputs : Bytes, proof : Bytes) -> DistaffVMResult<String>;
 }
 