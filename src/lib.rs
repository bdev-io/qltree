#![doc = include_str!("crate-doc.md")]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]
#[macro_use] extern crate log;

pub(crate) const PAGE_SIZE: usize = 4096;
pub(crate) const DEGREE: usize = 3;

pub trait Index where Self: Send + Sync + Clone + Copy + BytesExtension + std::cmp::PartialOrd {}
pub trait Value where Self: Send + Sync + Clone + Copy + BytesExtension + std::cmp::PartialOrd {}

mod qlgl;


pub use qlgl::*;
