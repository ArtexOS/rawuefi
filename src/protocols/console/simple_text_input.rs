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

//! # UEFI Simple Text Input Protocol
//!
//! This module defines the Simple Text Input Protocol, also known as the [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`].
//!
//! [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL

use crate::types::{BOOLEAN, CHAR16, EFI_EVENT, EFI_STATUS, UINT16};

/// The Simple Text Input protocol defines the minimum input required to support the ConsoleIn device.
///
/// This protocol is used to obtain input from the ConsoleIn device. The EFI specification requires
/// that the [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`] supports the same languages as the corresponding
/// [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    Reset: EFI_INPUT_RESET,
    ReadKeyStroke: EFI_INPUT_READ_KEY_STROKE,
    /// Event to use with [`EFI_BOOT_SERVICES.WaitForEvent()`] to wait for a key to be available.
    pub WaitForKey: EFI_EVENT,
}

impl EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    /// Resets the input device hardware.
    ///
    /// The [`Reset()`] function resets the input device hardware.
    ///
    /// The implementation of `Reset` is required to clear the contents of any input queues resident in
    /// memory used for buffering keystroke data and put the input stream in a known empty state.
    ///
    /// As part of initialization process, the firmware/device will make a quick but reasonable attempt
    /// to verify that the device is functioning. If the `ExtendedVerification` flag is `TRUE` the
    /// firmware may take an extended amount of time to verify the device is operating on reset.
    /// Otherwise the reset operation is to occur as quickly as possible.
    ///
    /// The hardware verification process is not defined by this specification and is left up to the
    /// platform firmware or driver to implement.
    ///
    /// ## Parameters
    ///
    /// ### `ExtendedVerification`
    ///
    /// Indicates that the driver may perform a more exhaustive verification operation of the device
    /// during reset.
    ///
    /// ## Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the device was reset.
    /// [`EFI_DEVICE_ERROR`] - the device is not functioning correctly and could not be reset.
    ///
    /// [`Reset()`]: ./struct.EFI_SIMPLE_TEXT_INPUT_PROTOCOL.html#method.Reset
    /// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    pub unsafe fn Reset(&mut self, ExtendedVerification: BOOLEAN) -> EFI_STATUS {
        (self.Reset)(self, ExtendedVerification)
    }

    pub unsafe fn ReadKeyStroke(&mut self, Key: *mut EFI_INPUT_KEY) -> EFI_STATUS {
        (self.ReadKeyStroke)(self, Key)
    }
}

#[repr(C)]
pub struct EFI_INPUT_KEY {
    pub ScanCode: UINT16,
    pub UnicodeChar: CHAR16,
}

type EFI_INPUT_RESET = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;

type EFI_INPUT_READ_KEY_STROKE = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    Key: *mut EFI_INPUT_KEY,
) -> EFI_STATUS;
