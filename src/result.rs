// SPDX-License-Identifier: GPL-3.0-only

use crate::error::Error;

/// Wrapper around `std::result::Result` that uses `Error` as the error type.
pub type Result<T> = std::result::Result<T, Error>;
