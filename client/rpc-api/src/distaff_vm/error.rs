// @file mod.rs
// @author ming (wmzhou@protonmail.com)
// @brief distaffvm rpc errors.
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
 