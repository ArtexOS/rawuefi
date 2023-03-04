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

//! # RawUEFI
//!
//! RawUEFI is a Rust crate providing idiomatic raw Rust bindings for the Unified Extensible
//! Firmware Interface (UEFI). RawUEFI is to maintain a low-level API over UEFI and aims to be as
//! simple as possible.
//!
//! Links to the corresponding portions of the UEFI Specification will be included in the
//! corresponding portions of the crate documentation for your reference, in case the crate
//! documentation is not clear enough. The UEFI Specification should be the ultimate reference as
//! it is the most accurate in all cases.
//!
//! The module-level documentation of individual modules provide in-depth documentation about the
//! individual modules within RawUEFI. They explain the main purpose and specifies usage of the
//! module in detail. The module names should also give a rough idea of the functionality of
//! individual modules.

#![no_std]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod types;
