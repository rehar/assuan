use std::{ffi::CStr, panic::{self, UnwindSafe, RefUnwindSafe}, mem};
use libc::{c_void, size_t, c_char};
use gpg_error::{Result, Error};

pub(crate) fn get_transaction_data_cb_ptr<F>(_closure: &F) -> ffi::assuan_transact_data_cb_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    transact_data_cb::<F>
}

extern fn transact_data_cb<F>(opaque: *mut c_void, buffer: *const c_void, length: size_t) -> ffi::assuan_error_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    let handler = unsafe {*(opaque as *const F)};

    let buff = buffer as *const u8;

    match panic::catch_unwind( || {
        let mut data:Vec<u8> = Vec::with_capacity(length);

        for i in 0..length {
            let d = unsafe {*buff.offset(i as isize)};
            data.push(d);
        }

         handler(&data)
    }){
         Ok(r) => r.err().map_or(0, |e|e.raw()),
         Err(_) => Error::GENERAL.raw(),
    }
}

pub(crate) fn get_transact_inquire_cb_ptr<F>(_closure: &F) -> ffi::assuan_transact_inquire_cb_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    transact_inquire_cb::<F>
}

extern fn transact_inquire_cb<F>(opaque: *mut c_void, name: *const c_char, ) -> ffi::assuan_error_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    let handler = unsafe {&mut *(opaque as *mut F)};

    match panic::catch_unwind( || {
        let name = unsafe {CStr::from_ptr(name)};
         handler(name.to_bytes())
    }){
         Ok(r) => r.err().map_or(0, |e|e.raw()),
         Err(_) => Error::GENERAL.raw(),
    }
}
pub(crate) fn get_transact_status_cb_ptr<F>(_closure: &F) -> ffi::assuan_transact_status_cb_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    transact_inquire_cb::<F>
}
extern fn transact_statsus_cb<F>(opaque: *mut c_void, line: *const c_char, ) -> ffi::assuan_error_t
where
    F: UnwindSafe + Copy + RefUnwindSafe + FnOnce(&[u8]) -> Result<()>,
{
    let handler = unsafe {&mut *(opaque as *mut F)};

    match panic::catch_unwind( || {
        let line = unsafe {CStr::from_ptr(line)};
         handler(line.to_bytes())
    }){
         Ok(r) => r.err().map_or(0, |e|e.raw()),
         Err(_) => Error::GENERAL.raw(),
    }
}