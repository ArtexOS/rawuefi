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

//! # UEFI Simple Text Output Protocol
//!
//! This module defines the Simple Text Output Protocol, also known as the [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
//!
//! [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL

use crate::types::{BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS};

/// GUID for the [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EFI_GUID = EFI_GUID(
    0x387477C2,
    0x69D7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
}

impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    /// Resets the text output device hardware.
    ///
    /// The [`Reset()`] function resets the text output device hardware. The cursor position is set
    /// to (0, 0), and the screen is cleared to the default background color for the output device.
    ///
    /// As part of initialization process, the firmware/device will make a quick but reasonable
    /// attempt to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE`
    /// the firmware may take an extended amount of time to verify the device is operating on reset.
    /// Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// # Parameters
    ///
    /// ## `ExtendedVerification`
    ///
    /// Indicates that the driver may perform a more exhaustive verification operation of the device
    /// during reset.
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the text output device was reset.
    ///
    /// [`EFI_DEVICE_ERROR`] - the text output device is not functioning correctly and could not be
    /// reset.
    ///
    /// [`ExtendedVerification`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#extendedverification
    pub unsafe fn Reset(&mut self, ExtendedVerification: BOOLEAN) -> EFI_STATUS {
        (self.Reset)(self, ExtendedVerification)
    }

    pub unsafe fn OutputString(&mut self, String: *mut CHAR16) -> EFI_STATUS {
        (self.OutputString)(self, String)
    }
}

type EFI_TEXT_RESET = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN
) -> EFI_STATUS;

type EFI_TEXT_STRING = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    String: *mut CHAR16,
) -> EFI_STATUS;
