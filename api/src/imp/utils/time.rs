use crate::imp::fs::Kstat;
use crate::ptr::{PtrWrapper, UserConstPtr, UserPtr};
use arceos_posix_api::{self as api, ctypes::timeval};
use axerrno::LinuxResult;
use axhal::time::{monotonic_time_nanos, nanos_to_ticks};
use core::ffi::c_char;
use starry_core::{ctypes::Tms, task::time_stat_output};

pub fn sys_clock_gettime(clock_id: i32, tp: UserPtr<api::ctypes::timespec>) -> LinuxResult<isize> {
    unsafe { Ok(api::sys_clock_gettime(clock_id, tp.get()?) as _) }
}

pub fn sys_get_time_of_day(ts: UserPtr<timeval>) -> LinuxResult<isize> {
    unsafe { Ok(api::sys_get_time_of_day(ts.get()?) as _) }
}

pub fn sys_times(tms: UserPtr<Tms>) -> LinuxResult<isize> {
    let (_, utime_us, _, stime_us) = time_stat_output();
    unsafe {
        *tms.get()? = Tms {
            tms_utime: utime_us,
            tms_stime: stime_us,
            tms_cutime: utime_us,
            tms_cstime: stime_us,
        }
    }
    Ok(nanos_to_ticks(monotonic_time_nanos()) as _)
}

pub fn sys_utimensat(
    _dirfd: i32,
    _path: UserConstPtr<c_char>,
    _times: UserConstPtr<Kstat>,
    _flags: i32,
) -> LinuxResult<isize> {
    warn!("[sys_utimensat] not implemented yet");
    Ok(0)
}
