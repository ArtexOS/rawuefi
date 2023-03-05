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

//! # UEFI Tables
//!
//! There are many UEFI tables that store various data or contain pointers to useful functions. The
//! `tables` module provides accurate bindings and representations of each fo these tables, and
//! detailed documentation will be provided on each.
//!
//! Each UEFI table has its own header. It is a data structure that precedes any UEFI table. It
//! includes a signature that is unique for each table type, a revision of the table that may be
//! updated as extensions are added to the EFI table types, and a 32-bit CRC so a consumer of an
//! EFI table type can validate the integrity of the contents of the EFI table. See
//! [Section 4.2 of the UEFI Specification] for more details. The table header data structure is
//! defined in this crate as the [`EFI_TABLE_HEADER`] structure.
//!
//! [`EFI_TABLE_HEADER`]: crate::tables::EFI_TABLE_HEADER
//!
//! [Section 4.2 of the UEFI Specification]: https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-table-header

use crate::types::{UINT32, UINT64};

pub mod system;

/// Data structure that precedes all of the standard EFI table types.
///
/// **Note 1:** The capabilities found in the EFI System Table, Runtime Services Table and Boot
/// Services Table may change over time. The first field in each of these tables is an [`EFI_TABLE_HEADER`].
/// This header’s [`Revision`] field is incremented when new capabilities and functions are added
/// to the functions in the table. When checking for capabilities, code should verify that [`Revision`]
/// is greater than or equal to the revision level of the table at the point when the capabilities
/// were added to the UEFI specification.
///
/// **Note 2:** Unless otherwise specified, UEFI uses a standard CCITT32 CRC algorithm with a seed
/// polynomial value of `0x04C11DB7` for its CRC calculations.
///
/// **Note 3:**  The size of the System Table, Runtime Services Table, and Boot Services Table may
/// increase over time. It is very important to always use the [`HeaderSize`] field of the
/// [`EFI_TABLE_HEADER`] to determine the size of these tables.
///
/// [`EFI_TABLE_HEADER`]: crate::tables::EFI_TABLE_HEADER
/// [`HeaderSize`]: ./struct.EFI_TABLE_HEADER.html#structfield.HeaderSize
/// [`Revision`]: ./struct.EFI_TABLE_HEADER.html#structfield.Revision
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_TABLE_HEADER {
    /// A 64-bit signature that identifies the type of table that follows. Unique signatures have
    /// been generated for the EFI System Table, the EFI Boot Services Table, and the
    /// EFI Runtime Services Table.
    pub Signature: UINT64,
    /// The revision of the EFI Specification to which this table conforms. The upper 16 bits of
    /// this field contain the major revision value, and the lower 16 bits contain the minor
    /// revision value. The minor revision values are binary coded decimals and are limited to the
    /// range of `00`..`99`.
    ///
    /// When printed or displayed UEFI spec revision is referred as:
    ///
    /// `(Major revision).(Minor revision upper decimal).(Minor revision lower decimal)`
    ///
    /// or
    ///
    /// `(Major revision).(Minor revision upper decimal)`
    ///
    /// in case Minor revision lower decimal is set to 0. For example:
    ///
    /// Specification with the revision value `((2<<16) | (30))` would be interpreted as `2.3`;
    ///
    /// A specification with the revision value `((2<<16) | (31))` would be referred as `2.3.1`.
    pub Revision: UINT32,
    /// The size, in bytes, of the entire table including the [`EFI_TABLE_HEADER`].
    ///
    /// [`EFI_TABLE_HEADER`]: crate::tables::EFI_TABLE_HEADER
    pub HeaderSize: UINT32,
    /// The 32-bit CRC for the entire table. This value is computed by setting this field to `0`,
    /// and computing the 32-bit CRC for HeaderSize bytes.
    pub Crc32: UINT32,
    /// Reserved field that must be set to `0`.
    pub Reserved: UINT32,
}
