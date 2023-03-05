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

use crate::types::{BOOLEAN, EFI_STATUS};

/// The Simple Text Input protocol defines the minimum input required to support the ConsoleIn device.
///
/// This protocol is used to obtain input from the ConsoleIn device. The EFI specification requires
/// that the [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`] supports the same languages as the corresponding
/// [`EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL`].
///
/// [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL;

pub type EFI_INPUT_RESET = extern "efiapi" fn(
    This: *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;
