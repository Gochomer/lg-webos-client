//! WebOS TV websocket client allowing to communicate in request-response manner.
//!
//! # Example
//!
//! Create client and send command
//! ```rust
//! use lg_webos_client::client::*;
//! use lg_webos_client::command::Command;
//!
//! #[tokio::main]
//! async fn main() {
//!     env_logger::init();
//!     // Note: We must specify the ws protocol, and if we do not have the key, we must use a blank str.
//!     let config = WebOsClientConfig::new("ws://192.168.1.62:3000/", None);
//!
//!     let client = WebosClient::new(config).await.unwrap();
//!     let resp = client.send_command(Command::GetChannelList).await.unwrap();
//!     println!("Got response {:?}", resp.payload);
//! }
//! ```

pub mod client;
pub mod command;
