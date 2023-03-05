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

//! # UEFI Protocols
//!
//! UEFI defines a variety of protocols that may be accessed, implemented by a certain UEFI driver
//! or occasionally by a certain UEFI application.
//!
//! Every module in this module categorizes UEFI protocols into its respective functionalities
//! and usage. See the module-level documentation for each module for more information on that
//! module and the protocols they define and include.

pub mod console;
