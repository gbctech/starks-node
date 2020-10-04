/**
 * @file error.rs
 * @author ming (wmzhou@protonmail.com)
 * @brief Distaff RPC errors.
 * @version 1.00
 * @date 2020-10-03
 *
 * @copyright Copyright@gbctech. All Rights Reserved.
 *
 */
 use jsonrpc_core as rpc;

 /// Distaff RPC Result type.
 pub type Result<T> = std::result::Result<T, Error>;
 
 /// Distaff RPC errors.
 #[derive(Debug, derive_more::Display, derive_more::From)]
 pub enum Error {
   /// Call to an unsafe Distaff was denied.
   UnsafeRpcCalled(crate::policy::UnsafeRpcError),
 }
 
 impl std::error::Error for Error {
   fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
     match self {
       Self::UnsafeRpcCalled(err) => Some(err),
       _ => None,
     }
   }
 }
 
 impl From<Error> for rpc::Error {
   fn from(e: Error) -> Self {
     match e {
       Error::UnsafeRpcCalled(e) => e.into(),
     }
   }
 }
 