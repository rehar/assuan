pub const ASSUAN_LINELENGTH: u32 = 1002;
pub const ASSUAN_INVALID_FD: i32 = -1;

/* Categories for log messages.  */
pub const ASSUAN_LOG_INIT: u32 = 1;
pub const ASSUAN_LOG_CTX: u32 = 2;
pub const ASSUAN_LOG_ENGINE: u32 = 3;
pub const ASSUAN_LOG_DATA: u32 = 4;
pub const ASSUAN_LOG_SYSIO: u32 = 5;
pub const ASSUAN_LOG_CONTROL: u32 = 8;

/**
 *  When using a pipe server, by default Assuan will wait for the
 * forked process to die in assuan_release.  In certain cases this
 * is not desirable.  By setting this flag, the waitpid will be
 * skipped and the caller is responsible to cleanup a forked
 * process. 
 */
pub const ASSUAN_NO_WAITPID: u32 = 1;
/** 
 * This flag indicates whether Assuan logging is in confidential mode.
 *  You can use assuan_{begin,end}_condidential to change the mode.  
 */
pub const ASSUAN_CONFIDENTIAL: u32 = 2;
/** This flag suppresses fix up of signal handlers for pipes.  */
pub const ASSUAN_NO_FIXSIGNALS: u32 = 3;
/** 
 * This flag changes assuan_transact to return comment lines via the
 * status callback.  The default is to skip comment lines.  
 */
pub const ASSUAN_CONVEY_COMMENTS: u32 = 4;
/** This flag disables logging for one context.  */
pub const ASSUAN_NO_LOGGING: u32 = 5;
/** This flag forces a connection close.  */
pub const ASSUAN_FORCE_CLOSE: u32 = 6;

/* Direction values for assuan_set_io_monitor.  */
pub const ASSUAN_IO_FROM_PEER: u32 = 0;
pub const ASSUAN_IO_TO_PEER: u32 = 1;

/* Return flags of I/O monitor.  */
pub const ASSUAN_IO_MONITOR_NOLOG: u32 = 1;
pub const ASSUAN_IO_MONITOR_IGNORE: u32 = 2;


pub const ASSUAN_SYSTEM_HOOKS_VERSION: u32 = 2;
pub const ASSUAN_SPAWN_DETACHED: u32 = 128;


pub const ASSUAN_SOCKET_SERVER_FDPASSING: u32 = 1;
pub const ASSUAN_SOCKET_SERVER_ACCEPTED: u32 = 2;


pub const ASSUAN_PIPE_CONNECT_FDPASSING: u32 = 1;
pub const ASSUAN_PIPE_CONNECT_DETACHED: u32 = 128;
pub const ASSUAN_SOCKET_CONNECT_FDPASSING: u32 = 1;

/* Client interface.  */
pub const ASSUAN_RESPONSE_ERROR: u32 = 0;
pub const ASSUAN_RESPONSE_OK: u32 = 1;
pub const ASSUAN_RESPONSE_DATA: u32 = 2;
pub const ASSUAN_RESPONSE_INQUIRE: u32 = 3;
pub const ASSUAN_RESPONSE_STATUS: u32 = 4;
pub const ASSUAN_RESPONSE_END: u32 = 5;
pub const ASSUAN_RESPONSE_COMMENT: u32 = 6;

pub const ASSUAN_SOCK_SOCKS: u32 = 1;
pub const ASSUAN_SOCK_TOR: u32 = 2;