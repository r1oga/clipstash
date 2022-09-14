pub mod data;
pub use data::DataError;

pub mod domain;
pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::Time;

pub mod service;

pub mod web;