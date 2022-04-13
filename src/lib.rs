pub mod database;
pub mod settings;
pub mod server;
pub mod router;
pub mod error;
pub mod telemetry;
pub mod utils;

use error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;