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

use crate::types::{BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS, INT32, UINTN};

/// GUID for the [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EFI_GUID = EFI_GUID(
    0x387477C2,
    0x69D7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    TestString: EFI_TEXT_TEST_STRING,
    QueryMode: EFI_TEXT_QUERY_MODE,
    SetMode: EFI_TEXT_SET_MODE,
    SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
    ClearScreen: EFI_TEXT_CLEAR_SCREEN,
    SetCursorPosition: EFI_TEXT_SET_CURSOR_POSITION,
    EnableCursor: EFI_TEXT_ENABLE_CURSOR,
    /// Pointer to [`SIMPLE_TEXT_OUTPUT_MODE`] data.
    ///
    /// [`SIMPLE_TEXT_OUTPUT_MODE`]: crate::protocols::console::simple_text_output::SIMPLE_TEXT_OUTPUT_MODE
    pub Mode: *mut SIMPLE_TEXT_OUTPUT_MODE,
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
    /// [`Reset()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.Reset
    /// [`ExtendedVerification`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#extendedverification
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    pub unsafe fn Reset(&mut self, ExtendedVerification: BOOLEAN) -> EFI_STATUS {
        (self.Reset)(self, ExtendedVerification)
    }

    /// Writes a string to the output device.
    ///
    /// The [`OutputString()`] function writes a string to the output device. This is the most
    /// basic output mechanism on an output device. The String is displayed at the current cursor
    /// location on the output device(s) and the cursor is advanced according to the rules listed
    /// in EFI Cursor Location/Advance Rules.
    ///
    /// # Parameters
    ///
    /// ## `String`
    ///
    /// The null-terminated string to be displayed on the output device(s).
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the string was output to the device.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device reported an error while attempting to output the text.
    /// reset.
    ///
    /// [`EFI_UNSUPPORTED`] - the output device???s mode is not currently in a defined text mode.
    ///
    /// [`EFI_WARN_UNKNOWN_GLYPH`] - this warning code indicates that some of the characters in the
    /// string could not be rendered and were skipped.
    ///
    /// [`OutputString()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.OutputString
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    /// [`EFI_WARN_UNKNOWN_GLYPH`]: crate::status::EFI_WARN_UNKNOWN_GLYPH
    pub unsafe fn OutputString(&mut self, String: *mut CHAR16) -> EFI_STATUS {
        (self.OutputString)(self, String)
    }

    /// Verifies that all characters in a string can be output to the target device.
    ///
    /// The [`TestString()`] function verifies that all characters in a string can be output to the
    /// target device.
    ///
    /// This function provides a way to know if the desired character codes are supported for
    /// rendering on the output device(s). This allows the installation procedure (or EFI image) to
    /// at least select character codes that the output devices are capable of displaying. Since
    /// the output device(s) may be changed between boots, if the loader cannot adapt to such
    /// changes it is recommended that the loader call [`OutputString()`] with the text it has and
    /// ignore any `UNSUPPORTED` error codes. Devices that are capable of displaying the Unicode
    /// character codes will do so.
    ///
    /// # Parameters
    ///
    /// ## `String`
    ///
    /// The null-terminated string to be examined for the output device(s).
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the string was output to the device.
    ///
    /// [`EFI_UNSUPPORTED`] - the output device???s mode is not currently in a defined text mode.
    ///
    /// [`TestString()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.TestString
    /// [`OutputString()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.OutputString
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn TestString(&mut self, String: *mut CHAR16) -> EFI_STATUS {
        (self.TestString)(self, String)
    }

    /// Returns information for an available text mode that the output device(s) supports.
    ///
    /// The [`QueryMode()`] function returns information for an available text mode that the output
    /// device(s) supports.
    ///
    /// It is required that all output devices support at least 80x25 text mode. This mode is
    /// defined to be mode `0`. If the output devices support 80x50, that is defined to be mode `1`.
    /// All other text dimensions supported by the device will follow as modes `2` and above. If an
    /// output device supports modes `2` and above, but does not support `80x50`, then querying for
    /// mode `1` will return [`EFI_UNSUPPORTED`].
    ///
    /// # Parameters
    ///
    /// ## `ModeNumber`
    ///
    /// The mode number to return information on.
    ///
    /// ## `Columns`, `Rows`
    ///
    /// Returns the geometry of the text output device for the requested [`ModeNumber`].
    ///
    /// ## Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the requested mode information was returned.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request.
    ///
    /// [`EFI_UNSUPPORTED`] - the mode number was not valid.
    ///
    /// [`QueryMode()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.QueryMode
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    /// [`ModeNumber`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#modenumber
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    pub unsafe fn QueryMode(
        &mut self,
        ModeNumber: UINTN,
        Columns: *mut UINTN,
        Rows: *mut UINTN,
    ) -> EFI_STATUS {
        (self.QueryMode)(self, ModeNumber, Columns, Rows)
    }

    /// Sets the output device(s) to a specified mode.
    ///
    /// The [`SetMode()`] function sets the output device(s) to the requested mode. On success
    /// the device is in the geometry for the requested mode, and the device has been cleared to
    /// the current background color with the cursor at (0,0).
    ///
    /// # Parameters
    ///
    /// ## `ModeNumber`
    ///
    /// The text mode to set.
    ///
    /// ## Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the requested text mode was set.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request.
    ///
    /// [`EFI_UNSUPPORTED`] - the mode number was not valid.
    ///
    /// [`SetMode()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.SetMode
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn SetMode(&mut self, ModeNumber: UINTN) -> EFI_STATUS {
        (self.SetMode)(self, ModeNumber)
    }

    /// Sets the background and foreground colors for the [`OutputString()`], and [`ClearScreen()`]
    /// functions.
    ///
    /// The [`SetAttribute()`] function sets the background and foreground colors for the [`OutputString()`]
    /// and [`ClearScreen()`] functions.
    ///
    /// The color mask can be set even when the device is in an invalid text mode.
    ///
    /// Devices supporting a different number of text colors are required to emulate the above colors to the best of the device???s capabilities.
    ///
    /// [`SetAttribute()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.SetAttribute
    /// [`OutputString()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.OutputString
    /// [`ClearScreen()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.ClearScreen
    ///
    /// # Parameters
    ///
    /// ## `Attribute`
    ///
    /// The attribute to set. Bits `0..3` are the foreground color, and bits `4..6` are the background
    /// color. All other bits are reserved.
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the requested attributes were set.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request.
    ///
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    pub unsafe fn SetAttribute(&mut self, Attribute: UINTN) -> EFI_STATUS {
        (self.SetAttribute)(self, Attribute)
    }

    /// Clears the output device(s) display to the currently selected background color.
    ///
    /// The [`ClearScreen()`] function clears the output device(s) display to the currently
    /// selected background color. The cursor position is set to (0, 0).
    ///
    /// [`ClearScreen()`] ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.ClearScreen
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the requested attributes were set.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request.
    ///
    /// [`EFI_UNSUPPORTED`] - the output device is not in a valid text mode.
    ///
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn ClearScreen(&mut self) -> EFI_STATUS {
        (self.ClearScreen)(self)
    }

    /// Makes the cursor visible or invisible.
    ///
    /// The [`SetCursorPosition()`] function sets the current coordinates of the cursor position.
    /// The upper left corner of the screen is defined as coordinate (0, 0).
    ///
    /// # Parameters
    ///
    /// ## `Column`, `Row`
    ///
    /// The position to set the cursor to. Must greater than or equal to zero and less than the
    /// number of columns and rows returned by [`QueryMode()`].
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the operation completed successfully.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request.
    ///
    /// [`EFI_UNSUPPORTED`] - the output device is not in a valid text mode, or the cursor position
    /// is invalid for the current mode.
    ///
    /// [`QueryMode()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.QueryMode
    /// [`SetCursorPosition()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.SetCursorPosition
    ///
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn SetCursorPosition(&mut self, Column: UINTN, Row: UINTN) -> EFI_STATUS {
        (self.SetCursorPosition)(self, Column, Row)
    }

    /// Makes the cursor visible or invisible.
    ///
    /// The [`EnableCursor()`] function makes the cursor visible or invisible.
    ///
    /// # Parameters
    ///
    /// ## `Visible`
    ///
    /// If `TRUE`, the cursor is set to be visible. If `FALSE`, the cursor is set to be invisible.
    ///
    /// # Status Codes Returned
    ///
    /// [`EFI_SUCCESS`] - the operation completed successfully.
    ///
    /// [`EFI_DEVICE_ERROR`] - the device had an error and could not complete the request or the
    /// device does not support changing the cursor mode.
    ///
    /// [`EFI_UNSUPPORTED`] - the output device does not support visibility control of the cursor.
    ///
    /// [`EnableCursor()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.EnableCursor
    /// [`EFI_SUCCESS`]: crate::status::EFI_SUCCESS
    /// [`EFI_DEVICE_ERROR`]: crate::status::EFI_DEVICE_ERROR
    /// [`EFI_UNSUPPORTED`]: crate::status::EFI_UNSUPPORTED
    pub unsafe fn EnableCursor(&mut self, Visible: BOOLEAN) -> EFI_STATUS {
        (self.EnableCursor)(self, Visible)
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    /// The number of modes supported by [`QueryMode()`] and [`SetMode()`].
    ///
    /// [`QueryMode()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.QueryMode
    /// [`SetMode()`]: ./struct.EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.html#method.[`SetMode()`]
    pub MaxMode: INT32,
    /// The text mode of the output device(s).
    pub Mode: INT32,
    /// The current character output attribute.
    pub Attribute: INT32,
    /// The cursor???s column.
    pub CursorColumn: INT32,
    /// The cursor???s row.
    pub CursorRow: INT32,
    /// The cursor is currently visible or not.
    pub CursorVisible: BOOLEAN,
}

pub const BOXDRAW_HORIZONTAL: CHAR16 = 0x2500;
pub const BOXDRAW_VERTICAL: CHAR16 = 0x2502;
pub const BOXDRAW_DOWN_RIGHT: CHAR16 = 0x250C;
pub const BOXDRAW_DOWN_LEFT: CHAR16 = 0x2510;
pub const BOXDRAW_UP_RIGHT: CHAR16 = 0x2514;
pub const BOXDRAW_UP_LEFT: CHAR16 = 0x2518;
pub const BOXDRAW_VERTICAL_RIGHT: CHAR16 = 0x251C;
pub const BOXDRAW_VERTICAL_LEFT: CHAR16 = 0x2524;
pub const BOXDRAW_DOWN_HORIZONTAL: CHAR16 = 0x252C;
pub const BOXDRAW_UP_HORIZONTAL: CHAR16 = 0x2534;
pub const BOXDRAW_VERTICAL_HORIZONTAL: CHAR16 = 0x253C;

pub const BOXDRAW_DOUBLE_HORIZONTAL: CHAR16 = 0x2550;
pub const BOXDRAW_DOUBLE_VERTICAL: CHAR16 = 0x2551;
pub const BOXDRAW_DOWN_RIGHT_DOUBLE: CHAR16 = 0x2552;
pub const BOXDRAW_DOWN_DOUBLE_RIGHT: CHAR16 = 0x2553;
pub const BOXDRAW_DOUBLE_DOWN_RIGHT: CHAR16 = 0x2554;
pub const BOXDRAW_DOWN_LEFT_DOUBLE: CHAR16 = 0x2555;
pub const BOXDRAW_DOWN_DOUBLE_LEFT: CHAR16 = 0x2556;
pub const BOXDRAW_DOUBLE_DOWN_LEFT: CHAR16 = 0x2557;

pub const BOXDRAW_UP_RIGHT_DOUBLE: CHAR16 = 0x2558;
pub const BOXDRAW_UP_DOUBLE_RIGHT: CHAR16 = 0x2559;
pub const BOXDRAW_DOUBLE_UP_RIGHT: CHAR16 = 0x255A;
pub const BOXDRAW_UP_LEFT_DOUBLE: CHAR16 = 0x255B;
pub const BOXDRAW_UP_DOUBLE_LEFT: CHAR16 = 0x255C;
pub const BOXDRAW_DOUBLE_UP_LEFT: CHAR16 = 0x255D;

pub const BOXDRAW_VERTICAL_RIGHT_DOUBLE: CHAR16 = 0x255E;
pub const BOXDRAW_VERTICAL_DOUBLE_RIGHT: CHAR16 = 0x255F;
pub const BOXDRAW_DOUBLE_VERTICAL_RIGHT: CHAR16 = 0x2560;

pub const BOXDRAW_VERTICAL_LEFT_DOUBLE: CHAR16 = 0x2561;
pub const BOXDRAW_VERTICAL_DOUBLE_LEFT: CHAR16 = 0x2562;
pub const BOXDRAW_DOUBLE_VERTICAL_LEFT: CHAR16 = 0x2563;

pub const BOXDRAW_DOWN_HORIZONTAL_DOUBLE: CHAR16 = 0x2564;
pub const BOXDRAW_DOWN_DOUBLE_HORIZONTAL: CHAR16 = 0x2565;
pub const BOXDRAW_DOUBLE_DOWN_HORIZONTAL: CHAR16 = 0x2566;

pub const BOXDRAW_UP_HORIZONTAL_DOUBLE: CHAR16 = 0x2567;
pub const BOXDRAW_UP_DOUBLE_HORIZONTAL: CHAR16 = 0x2568;
pub const BOXDRAW_DOUBLE_UP_HORIZONTAL: CHAR16 = 0x2569;

pub const BOXDRAW_VERTICAL_HORIZONTAL_DOUBLE: CHAR16 = 0x256A;
pub const BOXDRAW_VERTICAL_DOUBLE_HORIZONTAL: CHAR16 = 0x256B;
pub const BOXDRAW_DOUBLE_VERTICAL_HORIZONTAL: CHAR16 = 0x256C;

pub const BLOCKELEMENT_FULL_BLOCK: CHAR16 = 0x2588;
pub const BLOCKELEMENT_LIGHT_SHADE: CHAR16 = 0x2591;

pub const GEOMETRICSHAPE_UP_TRIANGLE: CHAR16 = 0x25B2;
pub const GEOMETRICSHAPE_RIGHT_TRIANGLE: CHAR16 = 0x25BA;
pub const GEOMETRICSHAPE_DOWN_TRIANGLE: CHAR16 = 0x25BC;
pub const GEOMETRICSHAPE_LEFT_TRIANGLE: CHAR16 = 0x25C4;

pub const ARROW_UP: CHAR16 = 0x2191;
pub const ARROW_DOWN: CHAR16 = 0x2193;

pub const EFI_BLACK: UINTN = 0x00;
pub const EFI_BLUE: UINTN = 0x01;
pub const EFI_GREEN: UINTN = 0x02;
pub const EFI_CYAN: UINTN = 0x03;
pub const EFI_RED: UINTN = 0x04;
pub const EFI_MAGENTA: UINTN = 0x05;
pub const EFI_BROWN: UINTN = 0x06;
pub const EFI_LIGHTGRAY: UINTN = 0x07;
pub const EFI_BRIGHT: UINTN = 0x08;
pub const EFI_DARKGRAY: UINTN = 0x08;
pub const EFI_LIGHTBLUE: UINTN = 0x09;
pub const EFI_LIGHTGREEN: UINTN = 0x0A;
pub const EFI_LIGHTCYAN: UINTN = 0x0B;
pub const EFI_LIGHTRED: UINTN = 0x0C;
pub const EFI_LIGHTMAGENTA: UINTN = 0x0D;
pub const EFI_YELLOW: UINTN = 0x0E;
pub const EFI_WHITE: UINTN = 0x0F;

pub const EFI_BACKGROUND_BLACK: UINTN = 0x00;
pub const EFI_BACKGROUND_BLUE: UINTN = 0x10;
pub const EFI_BACKGROUND_GREEN: UINTN = 0x20;
pub const EFI_BACKGROUND_CYAN: UINTN = 0x30;
pub const EFI_BACKGROUND_RED: UINTN = 0x40;
pub const EFI_BACKGROUND_MAGENTA: UINTN = 0x50;
pub const EFI_BACKGROUND_BROWN: UINTN = 0x60;
pub const EFI_BACKGROUND_LIGHTGRAY: UINTN = 0x70;

type EFI_TEXT_RESET = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;

type EFI_TEXT_STRING = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    String: *mut CHAR16,
) -> EFI_STATUS;

type EFI_TEXT_TEST_STRING = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    String: *mut CHAR16,
) -> EFI_STATUS;

type EFI_TEXT_QUERY_MODE = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    ModeNumber: UINTN,
    Columns: *mut UINTN,
    Rows: *mut UINTN,
) -> EFI_STATUS;

type EFI_TEXT_SET_MODE =
    extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: UINTN) -> EFI_STATUS;

type EFI_TEXT_SET_ATTRIBUTE =
    extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Attribute: UINTN) -> EFI_STATUS;

type EFI_TEXT_CLEAR_SCREEN =
    extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;

type EFI_TEXT_SET_CURSOR_POSITION = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    Column: UINTN,
    Row: UINTN,
) -> EFI_STATUS;

type EFI_TEXT_ENABLE_CURSOR =
    extern "efiapi" fn(This: *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Visible: BOOLEAN) -> EFI_STATUS;
