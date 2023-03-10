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

//! # UEFI Runtime Services
//!
//! UEFI uses the EFI Runtime Services Table, which contains a table header and pointers to all of
//! the runtime services. The definition for this table is shown in the following code fragments.
//! Except for the table header, all elements in the EFI Runtime Services Tables are prototypes of
//! function pointers to functions as defined in Services — Runtime Services. Unlike the EFI Boot
//! Services Table, this table, and the function pointers it contains are valid after the UEFI OS
//! loader and OS have taken control of the platform with a call to [`EFI_BOOT_SERVICES.ExitBootServices()`].
//! If a call to [`SetVirtualAddressMap()`] is made by the OS, then the function pointers in this
//! table are fixed up to point to the new virtually mapped entry points.

use crate::tables::EFI_TABLE_HEADER;
use crate::tables::system::EFI_SPECIFICATION_VERSION;
use crate::types::{BOOLEAN, EFI_STATUS, UINT16, UINT32, UINT64, UINT8};

/// Signature for the EFI Runtime Services Table.
pub const EFI_RUNTIME_SERVICES_SIGNATURE: UINT64 = 0x56524553544e5552;

/// The EFI Runtime Services Table revision.
pub const EFI_RUNTIME_SERVICES_REVISION: UINT32 = EFI_SPECIFICATION_VERSION;

/// Contains a table header and pointers to all of the runtime services.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_RUNTIME_SERVICES {
    /// The table header for the EFI Runtime Services Table. This header contains the
    /// [`EFI_RUNTIME_SERVICES_SIGNATURE`] and [`EFI_RUNTIME_SERVICES_REVISION`] values along with
    /// the size of the [`EFI_RUNTIME_SERVICES`] structure and a 32-bit CRC to verify that the
    /// contents of the EFI Runtime Services Table are valid.
    ///
    /// [`EFI_RUNTIME_SERVICES_SIGNATURE`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES_SIGNATURE
    /// [`EFI_RUNTIME_SERVICES_REVISION`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES_REVISION
    /// [`EFI_RUNTIME_SERVICES`]: crate::tables::runtime_services::EFI_RUNTIME_SERVICES
    pub Hdr: EFI_TABLE_HEADER,

    GetTime: EFI_GET_TIME,
    SetTime: EFI_SET_TIME,
}

impl EFI_RUNTIME_SERVICES {
    /// Returns the current time and date information, and the time-keeping capabilities of the
    /// hardware platform.
    ///
    /// The GetTime() function returns a time that was valid sometime during the call to the
    /// function. While the returned [`EFI_TIME`] structure contains [`TimeZone`] and [`Daylight`]
    /// savings time information, the actual clock does not maintain these values. The current time
    /// zone and daylight saving time information returned by [`GetTime()`] are the values that were
    /// last set via [`SetTime()`].
    ///
    /// The [`GetTime()`] function should take approximately the same amount of time to read the time
    /// each time it is called. All reported device capabilities are to be rounded up.
    ///
    /// During runtime, if a PC-AT CMOS device is present in the platform the caller must
    /// synchronize access to the device before calling [`GetTime()`].
    ///
    /// [`EFI_TIME`]: crate::tables::runtime_services::EFI_TIME
    /// [`TimeZone`]: ./struct.EFI_TIME.html#structfield.TimeZone
    /// [`Daylight`]: ./struct.EFI_TIME.html#structfield.Daylight
    /// [`GetTime()`]: ./struct.EFI_RUNTIME_SERVICES.html#method.GetTime
    /// [`SetTime()`]: ./struct.EFI_RUNTIME_SERVICES.html#method.SetTime
    pub unsafe fn GetTime(&self, Time: *mut EFI_TIME, Capabilities: *mut EFI_TIME_CAPABILITIES) -> EFI_STATUS {
        (self.GetTime)(Time, Capabilities)
    }
}

/// Represents current time information.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_TIME {
    /// The year of the current local date.
    pub Year: UINT16,
    /// The month of the current local date.
    pub Month: UINT8,
    /// The day of the current local date.
    pub Day: UINT8,
    /// The hour of the current local time.
    pub Hour: UINT8,
    /// The minute of the current local time.
    pub Minute: UINT8,
    /// The second of the current local time.
    pub Second: UINT8,
    Pad1: UINT8,
    /// Nanoseconds report the current fraction of a second in the device.
    pub Nanosecond: UINT32,
    /// The time’s offset in minutes from UTC. If the value is [`EFI_UNSPECIFIED_TIMEZONE`], then the
    /// time is interpreted as a local time. The [`TimeZone`] is the number of minutes that the
    /// local time is relative to UTC. To calculate the [`TimeZone`] value, follow this equation:
    /// Localtime = UTC - TimeZone.
    ///
    /// [`EFI_UNSPECIFIED_TIMEZONE`]: crate::tables::runtime_services::EFI_UNSPECIFIED_TIMEZONE
    /// [`TimeZone`]: ./struct.EFI_TIME.html#structfield.TimeZone
    pub TimeZone: UINT16,
    /// A bitmask containing the daylight savings time information for the time.
    ///
    /// The [`EFI_TIME_ADJUST_DAYLIGHT`] bit indicates if the time is affected by daylight savings
    /// time or not. This value does not indicate that the time has been adjusted for daylight
    /// savings time. It indicates only that it should be adjusted when the [`EFI_TIME`] enters
    /// daylight savings time.
    ///
    /// If [`EFI_TIME_IN_DAYLIGHT`] is set, the time has been adjusted for daylight savings time.
    ///
    /// All other bits must be zero.
    ///
    /// [`EFI_TIME_ADJUST_DAYLIGHT`]: crate::tables::runtime_services::EFI_TIME_ADJUST_DAYLIGHT
    /// [`EFI_TIME`]: crate::tables::runtime_services::EFI_TIME
    /// [`EFI_TIME_IN_DAYLIGHT`]: crate::tables::runtime_services::EFI_TIME_IN_DAYLIGHT
    pub Daylight: UINT8,
    Pad2: UINT8,
}

/// This provides the capabilities of the real time clock device as exposed through EFI.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct EFI_TIME_CAPABILITIES {
    /// Provides the reporting resolution of the real-time clock device in counts per second. For a
    /// normal PC-AT CMOS RTC device, this value would be 1 Hz, or 1, to indicate that the device
    /// only reports the time to the resolution of 1 second.
    pub Resolution: UINT32,
    /// Provides the timekeeping accuracy of the real-time clock in an error rate of 1E-6 parts
    /// per million. For a clock with an accuracy of 50 parts per million, the value in this
    /// field would be `50_000_000`.
    pub Accuracy: UINT32,
    /// A `TRUE` indicates that a time set operation clears the device’s time below the [`Resolution`]
    /// reporting level. A `FALSE` indicates that the state below the [`Resolution`] level of the
    /// device is not cleared when the time is set. Normal PC-AT CMOS RTC devices set this value to
    /// `FALSE`.
    ///
    ///  [`Resolution`]: ./struct.EFI_TIME_CAPABILITIES.html#structfield.Resolution
    pub SetsToZero: BOOLEAN,
}

pub const EFI_UNSPECIFIED_TIMEZONE: UINT16 = 0x07FF;

pub const EFI_TIME_ADJUST_DAYLIGHT: UINT8 = 0x01;
pub const EFI_TIME_IN_DAYLIGHT: UINT8 = 0x02;

type EFI_GET_TIME = extern "efiapi" fn(
    Time: *mut EFI_TIME,
    Capabilities: *mut EFI_TIME_CAPABILITIES,
) -> EFI_STATUS;

type EFI_SET_TIME = extern "efiapi" fn(
    Time: *mut EFI_TIME,
) -> EFI_STATUS;
