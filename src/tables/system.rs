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

//! # EFI System Table
//!
//! UEFI uses the EFI System Table, which contains pointers to the Runtime and Boot Services Tables.
//! The definition for this table is shown in the following code fragments. Except for the table
//! header, all elements in the service tables are pointers to functions as defined in Boot Services
//! and Runtime Services. Prior to a call to [`EFI_BOOT_SERVICES.ExitBootServices()`], all of the
//! fields of the EFI System Table are valid. After an operating system has taken control of the
//! platform with a call to [`ExitBootServices()`], only the [`Hdr`], [`FirmwareVendor`],
//! [`FirmwareRevision`], [`RuntimeServices`], [`NumberOfTableEntries`], and [`ConfigurationTable`]
//! fields are valid.
//!
//! See [Section 4.3 of the UEFI Specification] for more details. The EFI System Table data
//! structure is defined in this crate as the [`EFI_SYSTEM_TABLE`] structure.
//!
//! [`EFI_SYSTEM_TABLE`]: crate::tables::system::EFI_SYSTEM_TABLE
//! [`Hdr`]: ./struct.EFI_SYSTEM_TABLE.html#structfield.Hdr
//! [`FirmwareVendor`]: ./struct.EFI_SYSTEM_TABLE.html#structfield.FirmwareVendor
//! [`FirmwareRevision`]: ./struct.EFI_SYSTEM_TABLE.html#structfield.FirmwareRevision
//!
//! [Section 4.3 of the UEFI Specification]: https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-system-table-1

use crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
use crate::tables::EFI_TABLE_HEADER;
use crate::types::{CHAR16, EFI_HANDLE, UINT32, UINT64};

/// Signature for the EFI System Table.
pub const EFI_SYSTEM_TABLE_SIGNATURE: UINT64 = 0x5453595320494249;

/// Revision of the 2.10 EFI System Table.
pub const EFI_2_100_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 100;
/// Revision of the 2.9 EFI System Table.
pub const EFI_2_90_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 90;
/// Revision of the 2.8 EFI System Table.
pub const EFI_2_80_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 80;
/// Revision of the 2.7 EFI System Table.
pub const EFI_2_70_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 70;
/// Revision of the 2.6 EFI System Table.
pub const EFI_2_60_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 60;
/// Revision of the 2.5 EFI System Table.
pub const EFI_2_50_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 50;
/// Revision of the 2.4 EFI System Table.
pub const EFI_2_40_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 40;
/// Revision of the 2.3.1 EFI System Table.
pub const EFI_2_31_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 31;
/// Revision of the 2.3 EFI System Table.
pub const EFI_2_30_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 30;
/// Revision of the 2.2 EFI System Table.
pub const EFI_2_20_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 20;
/// Revision of the 2.1 EFI System Table.
pub const EFI_2_10_SYSTEM_TABLE_REVISION: UINT32 = (2 << 16) | 10;
/// Revision of the 2.0 EFI System Table.
pub const EFI_2_00_SYSTEM_TABLE_REVISION: UINT32 = 2 << 16;
/// Revision of the 1.1 EFI System Table.
pub const EFI_1_10_SYSTEM_TABLE_REVISION: UINT32 = (1 << 16) | 10;
/// Revision of the 1.0.2 EFI System Table.
pub const EFI_1_02_SYSTEM_TABLE_REVISION: UINT32 = (1 << 16) | 02;

/// The EFI specification version.
pub const EFI_SPECIFICATION_VERSION: UINT32 = EFI_SYSTEM_TABLE_REVISION;
/// The EFI System Table revision.
pub const EFI_SYSTEM_TABLE_REVISION: UINT32 = EFI_2_100_SYSTEM_TABLE_REVISION;

/// Contains pointers to the runtime and boot services tables.
#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    /// The table header for the EFI System Table. This header contains the
    /// [`EFI_SYSTEM_TABLE_SIGNATURE`] and [`EFI_SYSTEM_TABLE_REVISION`] values along with the size
    /// of the [`EFI_SYSTEM_TABLE`] structure and a 32-bit CRC to verify that the contents of the
    /// EFI System Table are valid.
    ///
    /// [`EFI_SYSTEM_TABLE_SIGNATURE`]: crate::tables::system::EFI_SYSTEM_TABLE_SIGNATURE
    /// [`EFI_SYSTEM_TABLE_REVISION`]: crate::tables::system::EFI_SYSTEM_TABLE_REVISION
    /// [`EFI_SYSTEM_TABLE`]: crate::tables::system::EFI_SYSTEM_TABLE
    pub Hdr: EFI_TABLE_HEADER,
    /// A pointer to a null terminated string that identifies the vendor that produces the system
    /// firmware for the platform.
    pub FirmwareVendor: *mut CHAR16,
    /// A firmware vendor specific value that identifies the revision of the system firmware for
    /// the platform.
    pub FirmwareRevision: UINT32,
    /// The handle for the active console input device. This handle must support [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]
    /// and [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`]. If there is no active console, these protocols
    /// must still be present.
    ///
    /// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    /// [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL
    pub ConsoleInHandle: EFI_HANDLE,
    /// A pointer to the [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`] interface that is associated with
    /// [`ConsoleInHandle`].
    ///
    /// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    /// [`ConsoleInHandle`]: ./struct.EFI_SYSTEM_TABLE.html#structfield.ConsoleInHandle
    pub ConIn: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
}
