// RawUEFI: Idiomatic Raw Bindings to UEFI
//
// Copyright (C) 2023 HTGAzureX1212.
//
// RawUEFI is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// RawUEFI is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with RawUEFI.  If not, see <https://www.gnu.org/licenses/>.

//! # UEFI Data Types
//!
//! The `types` module provides common data types defined in the UEFI Specification. These data
//! types are used throughout RawUEFI, and therefore, `types` is a commonly used module.
//!
//! The data types are defined as type aliases, and are listed as follows:
//!
//! | Data Type   |  Type in Rust  |
//! | ----------- | -------------- |
//! | [`BOOLEAN`] | [`u8`]         |
//! | [`INTN`]    | [`isize`]      |
//! | [`UINTN`]   | [`usize`]      |
//! | [`INT8`]    | [`i8`]         |
//! | [`UINT8`]   | [`u8`]         |
//! | [`INT16`]   | [`i16`]        |
//! | [`UINT16`]  | [`u16`]        |
//! | [`INT32`]   | [`i32`]        |
//! | [`UINT32`]  | [`u32`]        |
//! | [`INT64`]   | [`i64`]        |
//! | [`UINT64`]  | [`u64`]        |
//! | [`INT128`]  | [`i128`]       |
//! | [`UINT128`] | [`u128`]       |
//! | [`CHAR8`]   | [`u8`]         |
//! | [`CHAR16`]  | [`u16`]        |
//! | [`VOID`]    | [`VOID`]        |
//!
//! [`BOOLEAN`]: crate::types::BOOLEAN
//! [`INTN`]: crate::types::INTN
//! [`UINTN`]: crate::types::UINTN
//! [`INT8`]: crate::types::INT8
//! [`UINT8`]: crate::types::UINT8
//! [`INT16`]: crate::types::INT16
//! [`UINT16`]: crate::types::UINT16
//! [`INT32`]: crate::types::INT32
//! [`UINT32`]: crate::types::UINT32
//! [`INT64`]: crate::types::INT64
//! [`UINT64`]: crate::types::UINT64
//! [`INT128`]: crate::types::INT128
//! [`UINT128`]: crate::types::UINT128
//! [`CHAR8`]: crate::types::INT128
//! [`CHAR16`]: crate::types::UINT128
//! [`VOID`]: crate::types::VOID
//!
//! [`u8`]: https://doc.rust-lang.org/nightly/std/primitive.u8.html
//! [`isize`]: https://doc.rust-lang.org/nightly/std/primitive.isize.html
//! [`usize`]: https://doc.rust-lang.org/nightly/std/primitive.usize.html
//! [`i8`]: https://doc.rust-lang.org/nightly/std/primitive.i8.html
//! [`i16`]: https://doc.rust-lang.org/nightly/std/primitive.i16.html
//! [`u16`]: https://doc.rust-lang.org/nightly/std/primitive.u16.html
//! [`i32`]: https://doc.rust-lang.org/nightly/std/primitive.i32.html
//! [`u32`]: https://doc.rust-lang.org/nightly/std/primitive.u32.html
//! [`i64`]: https://doc.rust-lang.org/nightly/std/primitive.i64.html
//! [`u64`]: https://doc.rust-lang.org/nightly/std/primitive.u64.html
//! [`i128`]: https://doc.rust-lang.org/nightly/std/primitive.i128.html
//! [`u128`]: https://doc.rust-lang.org/nightly/std/primitive.u128.html

/// Logical Boolean. 1-byte value containing a `0` for `FALSE` and `1` for `TRUE`. Any other values are *undefined*.
pub type BOOLEAN = u8;
/// Signed value of native width. (4 bytes on supported 32-bit processor instructions, 8 bytes on supported 64-bit processor instructions, 16 bytes on supported 128-bit processor instructions)
pub type INTN = isize;
/// Unsigned value of native width. (4 bytes on supported 32-bit processor instructions, 8 bytes on supported 64-bit processor instructions, 16 bytes on supported 128-bit processor instructions)
pub type UINTN = usize;
/// 1-byte signed value.
pub type INT8 = i8;
/// 1-byte unsigned value.
pub type UINT8 = u8;
/// 2-byte signed value.
pub type INT16 = i16;
/// 2-byte unsigned value.
pub type UINT16 = u16;
/// 4-byte signed value.
pub type INT32 = i32;
/// 4-byte unsigned value.
pub type UINT32 = u32;
/// 8-byte signed value.
pub type INT64 = i64;
/// 8-byte unsigned value.
pub type UINT64 = u64;
/// 16-byte signed value.
pub type INT128 = i128;
/// 16-byte unsigned value.
pub type UINT128 = u128;
/// 1-byte character. Unless otherwise specified, all 1-byte or ASCII characters and strings are stored in 8-bit ASCII encoding format, using the ISO-Latin-1 character set.
pub type CHAR8 = u8;
/// 2-byte Character. Unless otherwise specified all characters and strings are stored in the UCS-2 encoding format as defined by Unicode 2.1 and ISO/IEC 10646 standards.
pub type CHAR16 = u16;
/// Undeclared type.
pub enum VOID {}
