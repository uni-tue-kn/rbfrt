/* Copyright 2023-present University of Tuebingen, Chair of Communication Networks
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Thrift client helpers
//!
//! This module provides convenience functions for connecting to Thrift services.
//!
//! # Example - Using Timestamp Service
//! ```no_run
//! use rbfrt::thrift_client;
//! use rbfrt::thrift_generated::ts::{TsSyncClient, TTsSyncClient};
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create Thrift client with all protocol setup done
//! let mut ts_client: TsSyncClient<_, _> = thrift_client::connect("localhost:9090")?;
//!
//! // Use ALL Thrift APIs directly
//! ts_client.ts_global_ts_value_set(0, 1_000_000_000)?;
//! # Ok(())
//! # }
//! ```

use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{
    ReadHalf, TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel, WriteHalf,
};

use crate::error::RBFRTError;

/// Type alias for Thrift input protocol
pub type ThriftInputProtocol = TBinaryInputProtocol<TFramedReadTransport<ReadHalf<TTcpChannel>>>;

/// Type alias for Thrift output protocol
pub type ThriftOutputProtocol =
    TBinaryOutputProtocol<TFramedWriteTransport<WriteHalf<TTcpChannel>>>;

/// Connect to a Thrift service and return protocol layers
///
/// This function handles all the boilerplate of creating a TCP connection
/// and setting up the Thrift protocol layers.
///
/// # Arguments
/// * `address` - Server address in format "host:port" (e.g., "localhost:9090")
///
/// # Returns
/// Returns a tuple of (InputProtocol, OutputProtocol) that can be passed to any Thrift client's new() method
///
/// # Example
/// ```no_run
/// use rbfrt::thrift_client;
/// use rbfrt::thrift_generated::ts::{TsSyncClient, TTsSyncClient};
///
/// # fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Create timestamp client
/// let (i_prot, o_prot) = thrift_client::connect("localhost:9090")?;
/// let mut ts_client = TsSyncClient::new(i_prot, o_prot);
/// ts_client.ts_global_ts_value_set(0, 1_000_000_000)?;
///
/// // Create port manager client
/// use rbfrt::thrift_generated::port_mgr::PortMgrSyncClient;
/// let (i_prot, o_prot) = thrift_client::connect("localhost:9090")?;
/// let mut port_client = PortMgrSyncClient::new(i_prot, o_prot);
/// port_client.port_mgr_mtu_set(0, 128, 9000, 9000)?;
/// # Ok(())
/// # }
/// ```
pub fn connect(address: &str) -> Result<(ThriftInputProtocol, ThriftOutputProtocol), RBFRTError> {
    // Create TCP connection
    let mut channel = TTcpChannel::new();
    channel
        .open(address)
        .map_err(|e| RBFRTError::GenericError {
            message: format!("Thrift connection to {} failed: {}", address, e),
        })?;

    // Split channel for bidirectional communication
    let (i_chan, o_chan) = channel.split().map_err(|e| RBFRTError::GenericError {
        message: format!("Failed to split channel: {}", e),
    })?;

    // Create protocol layers
    let i_prot = TBinaryInputProtocol::new(TFramedReadTransport::new(i_chan), true);
    let o_prot = TBinaryOutputProtocol::new(TFramedWriteTransport::new(o_chan), true);

    Ok((i_prot, o_prot))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::thrift_generated::ts::TsSyncClient;

    #[test]
    fn test_connect_signature() {
        // This test ensures the types compile correctly
        // Actual connection would require a running Thrift server
        fn _example() -> Result<(), RBFRTError> {
            let (i_prot, o_prot) = connect("localhost:9090")?;
            let _client = TsSyncClient::new(i_prot, o_prot);
            Ok(())
        }
    }
}
