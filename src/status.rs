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

//! # UEFI Status Codes
//!
//! This module defines every single documented UEFI Status Code. These can be found in the
//! UEFI Specification, typically [Appendix D].
//!
//! Each Status Code comes with its own brief description. These Status Codes will also be used
//! throughout the documentation of RawUEFI, stating clearly the Status Codes each function may
//! return, for your reference and convenience.
//!
//! [Appendix D]: https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html

use core::mem;

use crate::types::EFI_STATUS;

const ERROR_BIT: usize = 1 << (mem::size_of::<EFI_STATUS>() * 8 - 1);

/// The operation completed successfully.
pub const EFI_SUCCESS: EFI_STATUS = 0;

/// The image failed to load.
pub const EFI_LOAD_ERROR: EFI_STATUS = ERROR_BIT | 1;
/// A parameter was incorrect.
pub const EFI_INVALID_PARAMETER: EFI_STATUS = ERROR_BIT | 2;
/// The operation is not supported.
pub const EFI_UNSUPPORTED: EFI_STATUS = ERROR_BIT | 3;
/// The buffer was not the proper size for the request.
pub const EFI_BAD_BUFFER_SIZE: EFI_STATUS = ERROR_BIT | 4;
/// The buffer is not large enough to hold the requested data. The required buffer size is
/// returned in the appropriate parameter when this error occurs.
pub const EFI_BUFFER_TOO_SMALL: EFI_STATUS = ERROR_BIT | 5;
/// There is no data pending upon return.
pub const EFI_NOT_READY: EFI_STATUS = ERROR_BIT | 6;
/// The physical device reported an error while attempting the operation.
pub const EFI_DEVICE_ERROR: EFI_STATUS = ERROR_BIT | 7;
/// The device cannot be written to.
pub const EFI_WRITE_PROTECTED: EFI_STATUS = ERROR_BIT | 8;
/// A resource has run out.
pub const EFI_OUT_OF_RESOURCES: EFI_STATUS = ERROR_BIT | 9;
/// An inconstancy was detected on the file system causing the operation to fail.
pub const EFI_VOLUME_CORRUPTED: EFI_STATUS = ERROR_BIT | 10;
/// There is no more space on the file system.
pub const EFI_VOLUME_FULL: EFI_STATUS = ERROR_BIT | 11;
/// The device does not contain any medium to perform the operation.
pub const EFI_NO_MEDIA: EFI_STATUS = ERROR_BIT | 12;
/// The medium in the device has changed since the last access.
pub const EFI_MEDIA_CHANGED: EFI_STATUS = ERROR_BIT | 13;
/// The item was not found.
pub const EFI_NOT_FOUND: EFI_STATUS = ERROR_BIT | 14;
/// Access was denied.
pub const EFI_ACCESS_DENIED: EFI_STATUS = ERROR_BIT | 15;
/// The server was not found or did not respond to the request.
pub const EFI_NO_RESPONSE: EFI_STATUS = ERROR_BIT | 16;
/// A mapping to a device does not exist.
pub const EFI_NO_MAPPING: EFI_STATUS = ERROR_BIT | 17;
/// The timeout time expired.
pub const EFI_TIMEOUT: EFI_STATUS = ERROR_BIT | 18;
/// The protocol has not been started.
pub const EFI_NOT_STARTED: EFI_STATUS = ERROR_BIT | 19;
/// The protocol has already been started.
pub const EFI_ALREADY_STARTED: EFI_STATUS = ERROR_BIT | 20;
/// The operation was aborted.
pub const EFI_ABORTED: EFI_STATUS = ERROR_BIT | 21;
/// An ICMP error occurred during the network operation.
pub const EFI_ICMP_ERROR: EFI_STATUS = ERROR_BIT | 22;
/// A TFTP error occurred during the network operation.
pub const EFI_TFTP_ERROR: EFI_STATUS = ERROR_BIT | 23;
/// A protocol error occurred during the network operation.
pub const EFI_PROTOCOL_ERROR: EFI_STATUS = ERROR_BIT | 24;
/// The function encountered an internal version that was incompatible with a version requested by
/// the caller.
pub const EFI_INCOMPATIBLE_VERSION: EFI_STATUS = ERROR_BIT | 25;
/// The function was not performed due to a security violation.
pub const EFI_SECURITY_VIOLATION: EFI_STATUS = ERROR_BIT | 26;
/// A CRC error was detected.
pub const EFI_CRC_ERROR: EFI_STATUS = ERROR_BIT | 27;
/// Beginning or end of media was reached.
pub const EFI_END_OF_MEDIA: EFI_STATUS = ERROR_BIT | 28;
/// The end of the file was reached.
pub const EFI_END_OF_FILE: EFI_STATUS = ERROR_BIT | 31;
/// The language specified was invalid.
pub const EFI_INVALID_LANGUAGE: EFI_STATUS = ERROR_BIT | 32;
/// The security status of the data is unknown or compromised and the data must be updated or
/// replaced to restore a valid security status.
pub const EFI_COMPROMISED_DATA: EFI_STATUS = ERROR_BIT | 33;
/// There is an address conflict in address allocation.
pub const EFI_IP_ADDRESS_CONFLICT: EFI_STATUS = ERROR_BIT | 34;
/// A HTTP error occurred during the network operation.
pub const EFI_HTTP_ERROR: EFI_STATUS = ERROR_BIT | 35;

/// The string contained one or more characters that the device could not render and were skipped.
pub const EFI_WARN_UNKNOWN_GLYPH: EFI_STATUS = 1;
/// The handle was closed, but the file was not deleted.
pub const EFI_WARN_DELETE_FAILURE: EFI_STATUS = 2;
/// The handle was closed, but the data to the file was not flushed properly.
pub const EFI_WARN_WRITE_FAILURE: EFI_STATUS = 3;
/// The resulting buffer was too small, and the data was truncated to the buffer size.
pub const EFI_WARN_BUFFER_TOO_SMALL: EFI_STATUS = 4;
/// The data has not been updated within the timeframe set by local policy for this type of data.
pub const EFI_WARN_STALE_DATA: EFI_STATUS = 5;
/// The resulting buffer contains UEFI-compliant file system.
pub const EFI_WARN_FILE_SYSTEM: EFI_STATUS = 6;
/// The operation will be processed across a system reset.
pub const EFI_WARN_RESET_REQUIRED: EFI_STATUS = 7;
