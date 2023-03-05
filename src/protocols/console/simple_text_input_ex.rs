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
    ReadKeyStrokeEx: EFI_INPUT_READ_KEY_EX,
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
    /// # Parameters
    ///
    /// ## `ExtendedVerification`
    ///
    /// Indicates that the driver may perform a more exhaustive verification operation of the device
    /// during reset.
    ///
    /// # Status Codes Returned
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

    /// Reads the next keystroke from the input device.
    ///
    /// The [`ReadKeyStrokeEx()`] function reads the next keystroke from the input device. If there
    /// is no pending keystroke the function returns [`EFI_NOT_READY`]. If there is a pending
    /// keystroke, then [`KeyData.Key.ScanCode`] is the EFI scan code defined in EFI Scan Codes
    /// for [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]. The [`KeyData.Key.UnicodeChar`] is the actual
    /// printable character or is zero if the key does not represent a printable character (control
    /// key, function key, etc.). The [`KeyData.KeyState`] is the modifier shift state for the
    /// character reflected in [`KeyData.Key.UnicodeChar`] or [`KeyData.Key.ScanCode`]. This
    /// function mirrors the behavior of [`ReadKeyStroke()`] in the Simple Input Protocol in that a
    /// keystroke will only be returned when [`KeyData.Key`] has data within it.
    ///
    /// When interpreting the data from this function, it should be noted that if a class of
    /// printable characters that are normally adjusted by shift modifiers (e.g. `Shift` Key + `F`
    /// Key) would be presented solely as a [`KeyData.Key.UnicodeChar`] without the associated shift
    /// state. So in the previous example of a `Shift` Key + `F` Key being pressed, the only
    /// pertinent data returned would be [`KeyData.Key.UnicodeChar`] with the value of `F` This of
    /// course would not typically be the case for non-printable characters such as the pressing of
    /// the `Right Shift` Key + `F10` Key since the corresponding returned data would be reflected
    /// both in the [`KeyData.KeyState.KeyShiftState`] and [`KeyData.Key.ScanCode`] values.
    ///
    /// UEFI drivers which implement the [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`] are required to
    /// return [`KeyData.Key`] and [`KeyData.KeyState`] values. These drivers must always return
    /// the most current state of [`KeyData.KeyState.KeyShiftState`] and [`KeyData.KeyState.KeyToggleState`].
    /// It should also be noted that certain input devices may not be able to produce shift or
    /// toggle state information, and in those cases the high order bit in the respective
    /// Toggle and Shift state fields should not be active.
    ///
    /// If the [`EFI_KEY_STATE_EXPOSED`] bit is turned on, then this instance of the [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`]
    /// supports the ability to return partial keystrokes. With [`EFI_KEY_STATE_EXPOSED`] bit enabled,
    /// the `ReadKeyStrokeEx` function will allow the return of incomplete keystrokes such as the
    /// holding down of certain keys which are expressed as a part of `KeyState` when there is no
    /// `Key` data.
    ///
    /// # Parameters
    ///
    /// ## `KeyData`
    ///
    /// A pointer to a buffer that is filled in with the keystroke state data for the key that was
    /// pressed.
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the keystroke information was returned.
    ///
    /// [`EFI_NOT_READY`] - there was no keystroke data available.
    ///
    /// [`EFI_DEVICE_ERROR`] - the keystroke information was not returned due to hardware errors.
    ///
    /// [`EFI_UNSUPPORTED`] - the device does not support the ability to read keystroke data.
    ///
    /// [`ReadKeyStrokeEx()`]: ./struct.EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.html#method.ReadKeyStrokeEx
    /// [`EFI_NOT_READY`]: crate::status::EFI_NOT_READY
    /// [`KeyData.Key.ScanCode`]: ../../../protocols/console/simple_text_input/struct.EFI_INPUT_KEY.html#structfield.ScanCode
    /// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
    /// [`KeyData.Key.UnicodeChar`]: ../../../protocols/console/simple_text_input/struct.EFI_INPUT_KEY.html#structfield.UnicodeChar
    /// [`KeyData.KeyState`]: ./struct.EFI_KEY_DATA.html#structfield.KeyState
    /// [`ReadKeyStroke()`]: ../../../protocols/console/simple_text_input/struct.EFI_SIMPLE_TEXT_INPUT_PROTOCOL.html#method.ReadKeyStroke
    /// [`KeyData.KeyState.KeyShiftState`]: ./struct.EFI_KEY_STATE.html#structfield.KeyShiftState
    /// [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL
    /// [`KeyData.Key`]: ./struct.EFI_KEY_DATA.html#structfield.Key
    /// [`KeyData.KeyState.KeyToggleState`]: ./struct.EFI_KEY_STATE.html#structfield.KeyToggleState
    /// [`EFI_KEY_STATE_EXPOSED`]: crate::protocols::console::simple_text_input_ex::EFI_KEY_STATE_EXPOSED
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn ReadKeyStrokeEx(&mut self, KeyData: *mut EFI_KEY_DATA) -> EFI_STATUS {
        (self.ReadKeyStrokeEx)(self, KeyData)
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

pub const EFI_SHIFT_STATE_VALID: UINT32 = 0x80000000;
pub const EFI_RIGHT_SHIFT_PRESSED: UINT32 = 0x00000001;
pub const EFI_LEFT_SHIFT_PRESSED: UINT32 = 0x00000002;
pub const EFI_RIGHT_CONTROL_PRESSED: UINT32 = 0x00000004;
pub const EFI_LEFT_CONTROL_PRESSED: UINT32 = 0x00000008;
pub const EFI_RIGHT_ALT_PRESSED: UINT32 = 0x00000010;
pub const EFI_LEFT_ALT_PRESSED: UINT32 = 0x00000020;
pub const EFI_RIGHT_LOGO_PRESSED: UINT32 = 0x00000040;
pub const EFI_LEFT_LOGO_PRESSED: UINT32 = 0x00000080;
pub const EFI_MENU_KEY_PRESSED: UINT32 = 0x00000100;
pub const EFI_SYS_REQ_PRESSED: UINT32 = 0x00000200;

pub const EFI_TOGGLE_STATE_VALID: EFI_KEY_TOGGLE_STATE = 0x80;
pub const EFI_KEY_STATE_EXPOSED: EFI_KEY_TOGGLE_STATE = 0x40;
pub const EFI_SCROLL_LOCK_ACTIVE: EFI_KEY_TOGGLE_STATE = 0x01;
pub const EFI_NUM_LOCK_ACTIVE: EFI_KEY_TOGGLE_STATE = 0x02;
pub const EFI_CAPS_LOCK_ACTIVE: EFI_KEY_TOGGLE_STATE = 0x04;

type EFI_INPUT_RESET_EX = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;

type EFI_INPUT_READ_KEY_EX = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyData: *mut EFI_KEY_DATA,
) -> EFI_STATUS;
