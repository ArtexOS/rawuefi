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

//! # UEFI Console Protocols
//!
//! This module defines console support protocols, including Simple Text Input, Simple Text Output,
//! Simple Pointer, Serial IO, and Graphics Output protocols. These are correspondingly defined in
//! this module as follows:
//!
//! | Protocol             | Definition in Module                  |
//! | -------------------- | ------------------------------------- |
//! | Simple Text Input    | [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]    |
//! | Simple Text Input Ex | [`EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL`] |
//!
//! [`EFI_SIMPLE_TEXT_INPUT_PROTOCOL`]: crate::protocols::console::EFI_SIMPLE_TEXT_INPUT_PROTOCOL

pub mod simple_text_input;
pub mod simple_text_input_ex;
pub mod simple_text_output;

pub use simple_text_input::EFI_SIMPLE_TEXT_INPUT_PROTOCOL;
pub use simple_text_input_ex::EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL;
pub use simple_text_output::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL;
