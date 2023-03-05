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

//! ## UEFI Simple Text Input Ex Protocol
//!
//! This module defines the Simple Text Input Ex Protocol, also known as the
//! [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`].

use crate::protocols::console::simple_text_input::EFI_INPUT_KEY;
use crate::types::{BOOLEAN, EFI_STATUS, UINT32, UINT8};

/// The Simple Text Input Ex protocol defines an extension to the Simple Text Input protocol
/// which enables various new capabilities
///
/// This protocol is used to obtain input from the `ConsoleIn` device. The EFI specification
/// requires that the [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`] supports the same languages as the
/// corresponding [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    Reset: EFI_INPUT_RESET_EX,
}

impl EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
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
    ///
    /// [`EFI_DEVICE_ERROR`] - the device is not functioning correctly and could not be reset.
    ///
    /// [`Reset()`]: ./struct.EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.html#method.Reset
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    pub unsafe fn Reset(&mut self, ExtendedVerification: BOOLEAN) -> EFI_STATUS {
        (self.Reset)(self, ExtendedVerification)
    }
}

/// Keystroke state data for the key that was pressed.
pub struct EFI_KEY_DATA {
    /// The EFI scan code and Unicode value returned from the input device.
    pub Key: EFI_INPUT_KEY,
    /// The current state of various toggled attributes as well as input modifier values.
    pub KeyState: EFI_KEY_STATE,
}

/// Current state of various toggled attributes as well as input modifier values.
pub struct EFI_KEY_STATE {
    /// Reflects the currently pressed shift modifiers for the input device. The returned value is
    /// valid only if the high order bit has been set.
    pub KeyShiftState: UINT32,
    /// Reflects the current internal state of various toggled attributes. The returned value is valid only if the high order bit has been set.
    pub KeyToggleState: EFI_KEY_TOGGLE_STATE,
}

/// Current internal state of various toggled attributes.
pub type EFI_KEY_TOGGLE_STATE = UINT8;

type EFI_INPUT_RESET_EX = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    ExtendedVerification: BOOLEAN
) -> EFI_STATUS;

type EFI_INPUT_READ_KEY_EX = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyData: *mut EFI_KEY_DATA,
) -> EFI_STATUS;
