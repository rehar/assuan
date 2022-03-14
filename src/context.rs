use std::{
    ffi::CStr,
    panic::{RefUnwindSafe, UnwindSafe},
    path::Path,
    ptr,
};

use crate::{callbacks::get_transaction_data_cb_ptr, types::NonNull};
use cstr_argument::CStrArgument;
use ffi::assuan_pid_t;
use gpg_error::{return_err, Error, Result};

use libc::{c_char, c_void, size_t};

use super::callbacks::{get_transact_inquire_cb_ptr, get_transact_status_cb_ptr};
// missing define in ffi
pub const ASSUAN_INVALID_PID: assuan_pid_t = -1;

#[derive(Debug, Clone)]
pub struct Context(NonNull<ffi::assuan_context_t>);

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::assuan_release(self.as_raw()) }
    }
}

impl Context {
    pub fn new() -> Result<Self> {
        unsafe {
            let mut ctx: ffi::assuan_context_t = ptr::null_mut();
            return_err!(ffi::assuan_new(&mut ctx));
            Ok(Context::from_raw(ctx))
        }
    }
    pub fn socket_connect(&mut self, path: &Path) -> Result<()> {
        let name = path.to_str().unwrap().into_cstr();
        unsafe {
            return_err!(ffi::assuan_socket_connect(
                self.as_raw(),
                name.as_ref().as_ptr(),
                ASSUAN_INVALID_PID,
                ffi::ASSUAN_SOCKET_CONNECT_FDPASSING
            ));
            Ok(())
        }
    }

    pub fn transact<D, I, S>(
        &mut self,
        cmd: &str,
        mut data: D,
        mut inquire: I,
        mut status: S,
    ) -> Result<()>
    where
        D: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
        I: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
        S: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
    {
        unsafe {
            return_err!(ffi::assuan_transact(
                self.as_raw(),
                cmd.into_cstr().as_ref().as_ptr(),
                Some(get_transaction_data_cb_ptr(&data)),
                &mut data as *mut _ as *mut c_void,
                Some(get_transact_inquire_cb_ptr(&inquire)),
                &mut inquire as *mut _ as *mut c_void,
                Some(get_transact_status_cb_ptr(&status)),
                &mut status as *mut _ as *mut c_void
            ));
            Ok(())
        }
    }
    pub fn write_line(&mut self, line: &str) -> Result<()> {
        let line = line.into_cstr();
        return_err!(unsafe { ffi::assuan_write_line(self.as_raw(), line.as_ptr()) });
        Ok(())
    }
    pub fn line(&mut self) -> Result<String> {
        let mut len: size_t = 0;
        let mut ptr = std::ptr::null_mut() as *mut c_char;
        return_err!(unsafe {
            ffi::assuan_read_line(self.as_raw(), &mut ptr as *mut *mut c_char, &mut len)
        });

        let buff = ptr as *const c_char;
        let str = unsafe { CStr::from_ptr(buff) }
            .to_string_lossy()
            .to_string();
        Ok(str)
    }

    #[inline]
    pub fn as_raw(&self) -> ffi::assuan_context_t {
        self.0.as_ptr()
    }

    #[inline]
    pub unsafe fn from_raw(raw: ffi::assuan_context_t) -> Self {
        Self(NonNull::<ffi::assuan_context_t>::new(raw).unwrap())
    }

    #[inline]
    pub fn into_raw(self) -> ffi::assuan_context_t {
        let raw = self.as_raw();
        ::std::mem::forget(self);
        raw
    }
}
