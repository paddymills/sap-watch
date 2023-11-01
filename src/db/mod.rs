
//! database abstractions

mod api;
pub use api::{BurnedPart, MaterialData};

mod sn;
pub use sn::Sndb;


use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
pub(crate) type MssqlClient = Client<Compat<TcpStream>>;
