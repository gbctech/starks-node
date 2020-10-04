/**
 * @file mod.rs
 * @author ming (wmzhou@protonmail.com)
 * @brief distaffvm API.
 * @version 1.00
 * @date 2020-10-03
 *
 * @copyright Copyright@gbctech. All Rights Reserved.
 *
 */

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
 