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
//! The EFI System Table data structure is defined in this crate as the [`EFI_SYSTEM_TABLE`] structure.
//!
//! [`EFI_SYSTEM_TABLE`]: crate::tables::system::EFI_SYSTEM_TABLE

use crate::types::{UINT32, UINT64};

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

#[repr(C)]
pub struct EFI_SYSTEM_TABLE;
