pub mod context;
pub mod types;
pub mod callbacks;

use gpg_error::Result;
use once_cell::sync::Lazy;

#[cfg(windows)]
fn assuan_init() -> Result<()> {
    //need to initialize Windows socket layer
    use winapi::um::winsock2::{WSAStartup, LPWSADATA, WSAGetLastError, WSAVERNOTSUPPORTED, WSASYSNOTREADY, WSAEINPROGRESS, WSAEPROCLIM, WSAEFAULT, WSADATA};
    let mut data: WSADATA = WSADATA::default();
    let ret = unsafe {WSAStartup(winapi::um::winsock2::WINSOCK_VERSION,&mut data)};
    if ret != 0 {
        let winerr = unsafe {WSAGetLastError()};
        return Err(match winerr {
            WSASYSNOTREADY  => gpg_error::Error::EBUSY,
            WSAVERNOTSUPPORTED  => gpg_error::Error::NOT_SUPPORTED,
            WSAEINPROGRESS      => gpg_error::Error::EINPROGRESS,
            WSAEPROCLIM =>  gpg_error::Error::LIMIT_REACHED,
            WSAEFAULT   => gpg_error::Error::EINVAL, 
            _   => gpg_error::Error::UNKNOWN_ERRNO, 
        });
    }
    Ok(())    
}

#[cfg(not(windows))]
fn assuan_init() -> Result<()> {
    Ok(())      
}

pub fn init() -> Result<()> {
   static INIT_RESULT: Lazy<Result<()>> = Lazy::new(|| unsafe {
        let res = ffi::assuan_check_version(ffi::MIN_ASSUAN_VERSION.as_ptr().cast());
        
        if res.is_null() {
            panic!("The linked assuan library is not compatible. Requires version {} or higher.", ffi::MIN_ASSUAN_VERSION);
        }

        assuan_init()
    });
    
    *INIT_RESULT
}