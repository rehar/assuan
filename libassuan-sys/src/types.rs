use libc::{c_char, c_int, c_uint, c_void, size_t, sockaddr, ssize_t};
#[cfg(not(windows))]
use libc::{pid_t, socklen_t};

use std::option::Option;

pub use libgpg_error_sys::{
    gpg_err_source_t as assuan_err_source_t, gpg_error_t as assuan_error_t,
};

#[cfg(not(windows))]
pub type assuan_pid_t = pid_t;

#[cfg(windows)]
pub type assuan_pid_t = c_int;


#[cfg(not(windows))]
pub type assuan_socklen_t = socklen_t;
#[cfg(windows)]
pub type assuan_socklen_t = c_int;

#[repr(C)]
pub struct assuan_context_s {
    _unused: [u8; 0],
}
pub type assuan_context_t = *mut assuan_context_s;

#[repr(C)]
pub struct assuan_sock_nonce_s {
    pub length: size_t,
}
pub type assuan_sock_nonce_t = assuan_sock_nonce_s;

#[repr(C)]
pub struct assuan_malloc_hooks {
    pub malloc: Option<unsafe extern "C" fn(cnt: size_t) -> *mut c_void>,
    pub realloc: Option<unsafe extern "C" fn(ptr: *mut c_void, cnt: size_t) -> *mut c_void>,
    pub free: Option<unsafe extern "C" fn(ptr: *mut c_void)>,
}
pub type assuan_log_cb_t = unsafe extern "C" fn(
    ctx: assuan_context_t,
    hook: *mut c_void,
    cat: c_uint,
    msg: *const c_char,
) -> c_int;

pub type assuan_io_monitor_t = unsafe extern "C" fn(
    ctx: assuan_context_t,
    hook: *mut c_void,
    inout: c_int,
    line: *const c_char,
    linelen: size_t,
) -> c_uint;

pub type assuan_malloc_hooks_t = *mut assuan_malloc_hooks;

pub type assuan_fd_t = c_int;

/// Definitions of flags for assuan_set_flag().
pub type assuan_flag_t = c_uint;

pub type assuan_handler_t =
    unsafe extern "C" fn(ctx: assuan_context_t, line: *mut c_char) -> assuan_error_t;

#[cfg(all(target_os = "windows"))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _assuan_peercred {
    pub _dummy: c_uint,
}

#[cfg(not(target_os = "windows"))]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _assuan_peercred {
    pub pid: assuan_pid_t,
    pub uid: libc::uid_t,
    pub gid: libc::gid_t,
}

pub type assuan_peercred_t = *mut _assuan_peercred;

pub type assuan_response_t = c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct assuan_system_hooks {
    pub version: c_int,
    pub usleep: Option<unsafe extern "C" fn(ctx: assuan_context_t, usec: c_uint)>,
    pub pipe: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            fd: *mut assuan_fd_t,
            inherit_idx: c_int,
        ) -> c_int,
    >,
    pub close: Option<unsafe extern "C" fn(ctx: assuan_context_t, fd: assuan_fd_t) -> c_int>,
    pub read: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            fd: assuan_fd_t,
            buffer: *mut c_void,
            size: size_t,
        ) -> ssize_t,
    >,
    pub write: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            fd: assuan_fd_t,
            buffer: *const c_void,
            size: size_t,
        ) -> ssize_t,
    >,
    pub recvmsg: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            fd: assuan_fd_t,
            msg: assuan_msghdr_t,
            flags: c_int,
        ) -> c_int,
    >,
    pub sendmsg: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            fd: assuan_fd_t,
            msg: assuan_msghdr_t,
            flags: c_int,
        ) -> c_int,
    >,
    pub spawn: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            r_pid: *mut assuan_pid_t,
            name: *const c_char,
            argv: *mut *const c_char,
            fd_in: assuan_fd_t,
            fd_out: assuan_fd_t,
            fd_child_list: *mut assuan_fd_t,
            atfork: Option<unsafe extern "C" fn(opaque: *mut c_void, reserved: c_int)>,
            atforkvalue: *mut c_void,
            flags: c_uint,
        ) -> c_int,
    >,
    pub waitpid: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            pid: assuan_pid_t,
            action: c_int,
            status: *mut c_int,
            options: c_int,
        ) -> assuan_pid_t,
    >,
    pub socketpair: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            _namespace: c_int,
            style: c_int,
            protocol: c_int,
            filedes: *mut assuan_fd_t,
        ) -> c_int,
    >,
    pub socket: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            _namespace: c_int,
            style: c_int,
            protocol: c_int,
        ) -> c_int,
    >,
    pub connect: Option<
        unsafe extern "C" fn(
            ctx: assuan_context_t,
            sock: c_int,
            addr: *mut sockaddr,
            length: assuan_socklen_t,
        ) -> c_int,
    >,
}
pub type assuan_system_hooks_t = *mut assuan_system_hooks;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct assuan_msghdr_s {
    _unused: [u8; 0],
}

pub type assuan_msghdr_t = *mut assuan_msghdr_s;

pub type assuan_transact_data_cb_t = unsafe extern "C" fn(
    opaque: *mut c_void,
    data: *const c_void,
    length: size_t,
) -> assuan_error_t;

pub type assuan_transact_inquire_cb_t =
    unsafe extern "C" fn(opaque: *mut c_void, name: *const c_char) -> assuan_error_t;

pub type assuan_transact_status_cb_t =
    unsafe extern "C" fn(opaque: *mut c_void, line: *const c_char) -> assuan_error_t;
