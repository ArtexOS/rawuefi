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

use crate::types::EFI_GUID;

/// GUID for the [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EFI_GUID = EFI_GUID(
    0x387477C2,
    0x69D7,
    0x11D2,
    [0x8E, 0x39, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;
