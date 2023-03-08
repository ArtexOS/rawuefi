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

//! # UEFI Runtime Services
//!
//! UEFI uses the EFI Runtime Services Table, which contains a table header and pointers to all of
//! the runtime services. The definition for this table is shown in the following code fragments.
//! Except for the table header, all elements in the EFI Runtime Services Tables are prototypes of
//! function pointers to functions as defined in Services — Runtime Services. Unlike the EFI Boot
//! Services Table, this table, and the function pointers it contains are valid after the UEFI OS
//! loader and OS have taken control of the platform with a call to [`EFI_BOOT_SERVICES.ExitBootServices()`].
//! If a call to [`SetVirtualAddressMap()`] is made by the OS, then the function pointers in this
//! table are fixed up to point to the new virtually mapped entry points.

use crate::tables::EFI_TABLE_HEADER;
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::types::{UINT32, UINT64};

/// Signature for the EFI Runtime Services Table.
pub const EFI_RUNTIME_SERVICES_SIGNATURE: UINT64 = 0x56524553544e5552;

/// The EFI Runtime Services Table revision.
pub const EFI_RUNTIME_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

/// Contains a table header and pointers to all of the runtime services.
#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
    /// The table header for the EFI Runtime Services Table. This header contains the
    /// [`EFI_RUNTIME_SERVICES_SIGNATURE`] and [`EFI_RUNTIME_SERVICES_REVISION`] values along with
    /// the size of the [`EFI_RUNTIME_SERVICES`] structure and a 32-bit CRC to verify that the
    /// contents of the EFI Runtime Services Table are valid.
    ///
    /// [`EFI_RUNTIME_SERVICES_SIGNATURE`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES_SIGNATURE
    /// [`EFI_RUNTIME_SERVICES_REVISION`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES_REVISION
    /// [`EFI_RUNTIME_SERVICES`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES
    pub Hdr: EFI_TABLE_HEADER,
}
