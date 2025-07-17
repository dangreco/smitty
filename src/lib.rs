// SPDX-License-Identifier: GPL-3.0-only

mod error;
pub use error::Error;

pub mod lazy;
pub use lazy::LazyValue;

mod resolve;
pub use resolve::Resolve;

mod result;
pub use result::Result;

mod value;
pub use value::Value;
