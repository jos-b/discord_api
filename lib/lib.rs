//! A library for fetching various structures from the Discord API. This is not a fully fledged Discord wrapper
//! nor does it attempt to be. It contains some structures deemed useful.
//!
//! This library should maintain feature parity with the included binary, available at [https://github.com/jos-b/discord](https://github.com/jos-b/discord)
//! in that all new structures added to this library should be accessible in some way through the binary.
//!
//! # Examples
//!
//! Fetch a guild name from an invite code
//! ```
//! # #[tokio::main]
//! # async fn main() -> Result<(), reqwest::Error> {
//! use discord_api::get_invite;
//! let invite = get_invite("python").await?;
//!
//! if let Some(guild) = invite.guild {
//!   println!("Invite for: {}", guild.name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! Fetch a guild invite and list the features
//!
//! ```
//! # #[tokio::main]
//! # async fn main() -> Result<(), reqwest::Error> {
//! use discord_api::get_invite;
//!
//! let invite = get_invite("python").await?;
//!
//! if let Some(guild) = invite.guild {
//!   for feature in guild.features {
//!     println!("{}", feature);
//!   }
//! }
//! # Ok(())
//! # }
//! ```

#[macro_use]
extern crate log;

pub mod http;
pub mod models;

pub use http::get_invite;
