//! # Dockers
//!
//! Dockers is a docker api wrapper for rust, it's mainly focused on ease of use, asyncronous by default and exposing a low level api.
//!
//! ## Example
//!
//! ```rust
//! extern crate dockers;
//!
//! use dockers::Container;
//! use dockers::Image;
//!
//! fn main () {
//!     let img = Image::pull("rabbitmq".to_owned(), None)
//!         .expect("Cannot pull image");
//!
//!     let cont = Container::new(None, Some("rabbitmq".to_owned()))
//!         .create(Some("my_rabbitmq".to_owned()), None)
//!         .expect("Cannot create container");
//!
//!     // Do your things...
//!
//!     cont.remove();
//!     img.remove();
//! }
//! ```
#[macro_use]
extern crate quick_error;

#[macro_use]
extern crate serde_derive;

extern crate curl;
extern crate serde;
extern crate serde_json;

pub mod containers;
pub mod docker;
pub mod images;

pub use containers::Container;
pub use images::Image;
