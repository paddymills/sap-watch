
#![feature(lazy_cell)]

#![warn(missing_docs)]

//! SAP error pre-cogi watching and handling

#[macro_use] extern crate anyhow;
#[macro_use] extern crate serde;

pub mod api;
pub mod db;
pub mod excel;
pub mod logging;