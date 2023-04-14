#![doc = include_str!("crate-doc.md")]
#![cfg_attr(doc, deny(rustdoc::all))]
#![allow(
    clippy::missing_errors_doc, // TODO clippy::missing_errors_doc
    clippy::option_if_let_else,
    clippy::module_name_repetitions,
)]
#[macro_use] extern crate log;

// NOTE : 트리의 차수를 여기서 정의
const DEGREE_YOU_WANT: usize = 2;


pub(crate) const PAGE_SIZE: usize = 4096;
pub(crate) const DEGREE: usize = (DEGREE_YOU_WANT * 2) - 1;

pub trait Index where Self: Send + Sync + Clone + Copy + BytesExtension + std::fmt::Debug + Default + std::cmp::PartialOrd {}
pub trait Value where Self: Send + Sync + Clone + Copy + BytesExtension + std::fmt::Debug + Default + std::cmp::PartialOrd {}

mod qlgl;


pub use qlgl::*;
