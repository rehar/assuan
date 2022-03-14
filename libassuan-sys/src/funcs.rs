use libc::{c_char, c_int, c_uchar, c_uint, c_ushort, c_void, size_t, sockaddr, FILE};
use std::option::Option;

use crate::types::*;

extern "C" {
    pub fn assuan_fdopen(fd: libc::c_int) -> assuan_fd_t;

    /// Release all resources associated with the given context.  
    pub fn assuan_release(ctx: assuan_context_t);

    /// Release the memory at PTR using the allocation handler of the
    /// context CTX.  This is a convenience function.  
    pub fn assuan_free(ctx: assuan_context_t, ptr: *mut c_void);

    /// Set user-data in a context.
    pub fn assuan_set_pointer(ctx: assuan_context_t, pointer: *mut c_void);

    /// Get user-data in a context.  */
    pub fn assuan_get_pointer(ctx: assuan_context_t) -> *mut c_void;

    /// For context CTX, set the flag FLAG to VALUE.  Values for flags
    /// are usually 1 or 0 but certain flags might allow for other values;
    /// see the description of the type assuan_flag_t for details.
    pub fn assuan_set_flag(ctx: assuan_context_t, flag: assuan_flag_t, value: c_int);

    /// Same as assuan_set_flag (ctx, ASSUAN_CONFIDENTIAL, 1).
    pub fn assuan_begin_confidential(ctx: assuan_context_t);

    /// Same as assuan_set_flag (ctx, ASSUAN_CONFIDENTIAL, 0).
    pub fn assuan_end_confidential(ctx: assuan_context_t);

    /// Set the IO monitor function.
    pub fn assuan_set_io_monitor(
        ctx: assuan_context_t,
        io_monitor: Option<assuan_io_monitor_t>,
        hook_data: *mut c_void,
    );

    pub fn assuan_check_version(req_version: *const c_char) -> *const c_char;
    pub fn assuan_set_malloc_hooks(malloc_hooks: assuan_malloc_hooks_t);
    pub fn assuan_get_malloc_hooks() -> assuan_malloc_hooks_t;
    pub fn assuan_set_log_cb(log_cb: Option<assuan_log_cb_t>, log_cb_data: *mut c_void);
    pub fn assuan_get_log_cb(log_cb: *mut Option<assuan_log_cb_t>, log_cb_data: *mut *mut c_void);

    /* Configuration of the default log handler.  */

    /**
     * Set the prefix to be used at the start of a line emitted by assuan
     * on the log stream.  The default is the empty string.  Note, that
     * this function is not thread-safe and should in general be used
     * right at startup.
     */
    pub fn assuan_set_assuan_log_prefix(text: *const c_char);

    /** Return a prefix to be used at the start of a line emitted by assuan
     *  on the log stream.  The default implementation returns the empty
     * string, i.e. ""  
     */
    pub fn assuan_get_assuan_log_prefix() -> *const c_char;

    /// Global default log stream.
    pub fn assuan_set_assuan_log_stream(fp: *mut FILE);

    /// Set the per context log stream for the default log handler.
    pub fn assuan_set_log_stream(ctx: assuan_context_t, fp: *mut FILE);

    /// Set the default gpg error source.
    pub fn assuan_set_gpg_err_source(errsource: assuan_err_source_t);

    /// Get the default gpg error source.
    pub fn assuan_get_gpg_err_source() -> assuan_err_source_t;

    /**
     * Create a new Assuan context.  The initial parameters are all needed
     * in the creation of the context.  
     */
    pub fn assuan_new_ext(
        ctx: *mut assuan_context_t,
        errsource: assuan_err_source_t,
        malloc_hooks: assuan_malloc_hooks_t,
        log_cb: Option<assuan_log_cb_t>,
        log_cb_data: *mut c_void,
    ) -> assuan_error_t;

    /// Create a new context with default arguments.
    pub fn assuan_new(ctx: *mut assuan_context_t) -> assuan_error_t;

    pub fn assuan_register_command(
        ctx: assuan_context_t,
        cmd_string: *const c_char,
        handler: Option<assuan_handler_t>,
        help_string: *const c_char,
    ) -> assuan_error_t;

    pub fn assuan_register_pre_cmd_notify(
        ctx: assuan_context_t,
        fnc: Option<
            unsafe extern "C" fn(ctx: assuan_context_t, cmd: *const c_char) -> assuan_error_t,
        >,
    ) -> assuan_error_t;

    pub fn assuan_register_post_cmd_notify(
        ctx: assuan_context_t,
        fnc: Option<unsafe extern "C" fn(ctx: assuan_context_t, error: assuan_error_t)>,
    ) -> assuan_error_t;

    pub fn assuan_register_bye_notify(
        ctx: assuan_context_t,
        handler: Option<assuan_handler_t>,
    ) -> assuan_error_t;

    pub fn assuan_register_reset_notify(
        ctx: assuan_context_t,
        handler: Option<assuan_handler_t>,
    ) -> assuan_error_t;

    pub fn assuan_register_cancel_notify(
        ctx: assuan_context_t,
        handler: Option<assuan_handler_t>,
    ) -> assuan_error_t;

    pub fn assuan_register_input_notify(
        ctx: assuan_context_t,
        handler: Option<assuan_handler_t>,
    ) -> assuan_error_t;

    pub fn assuan_register_output_notify(
        ctx: assuan_context_t,
        handler: Option<assuan_handler_t>,
    ) -> assuan_error_t;

    pub fn assuan_register_option_handler(
        ctx: assuan_context_t,
        fnc: Option<
            unsafe extern "C" fn(
                ctx: assuan_context_t,
                arg2: *const c_char,
                arg3: *const c_char,
            ) -> assuan_error_t,
        >,
    ) -> assuan_error_t;

    pub fn assuan_process(ctx: assuan_context_t) -> assuan_error_t;
    pub fn assuan_process_next(ctx: assuan_context_t, done: *mut c_int) -> assuan_error_t;

    pub fn assuan_process_done(ctx: assuan_context_t, rc: assuan_error_t) -> assuan_error_t;

    pub fn assuan_get_active_fds(
        ctx: assuan_context_t,
        what: c_int,
        fdarray: *mut assuan_fd_t,
        fdarraysize: c_int,
    ) -> c_int;

    pub fn assuan_get_command_name(ctx: assuan_context_t) -> *const c_char;

    pub fn assuan_get_data_fp(ctx: assuan_context_t) -> *mut FILE;

    pub fn assuan_set_okay_line(ctx: assuan_context_t, line: *const c_char) -> assuan_error_t;

    pub fn assuan_write_status(
        ctx: assuan_context_t,
        keyword: *const c_char,
        text: *const c_char,
    ) -> assuan_error_t;

    /**
     *  Negotiate a file descriptor.  If LINE contains "FD=N", returns N
     *  assuming a local file descriptor.  If LINE contains "FD" reads a
     *  file descriptor via CTX and stores it in *RDF (the CTX must be
     *  capable of passing file descriptors).  Under W32 the returned FD is
     * a libc-type one.  
     */
    pub fn assuan_command_parse_fd(
        ctx: assuan_context_t,
        line: *mut c_char,
        rfd: *mut assuan_fd_t,
    ) -> assuan_error_t;

    pub fn assuan_set_hello_line(ctx: assuan_context_t, line: *const c_char) -> assuan_error_t;

    /*-- assuan-listen.c --*/
    pub fn assuan_accept(ctx: assuan_context_t) -> assuan_error_t;
    pub fn assuan_get_input_fd(ctx: assuan_context_t) -> assuan_fd_t;
    pub fn assuan_get_output_fd(ctx: assuan_context_t) -> assuan_fd_t;
    pub fn assuan_close_input_fd(ctx: assuan_context_t) -> assuan_error_t;
    pub fn assuan_close_output_fd(ctx: assuan_context_t) -> assuan_error_t;

    /*-- assuan-pipe-server.c --*/
    pub fn assuan_init_pipe_server(
        ctx: assuan_context_t,
        filedes: *mut assuan_fd_t,
    ) -> assuan_error_t;

    /*-- assuan-socket-server.c --*/
    pub fn assuan_init_socket_server(
        ctx: assuan_context_t,
        listen_fd: assuan_fd_t,
        flags: c_uint,
    ) -> assuan_error_t;
    pub fn assuan_set_sock_nonce(ctx: assuan_context_t, nonce: *mut assuan_sock_nonce_t);

    /*-- assuan-pipe-connect.c --*/
    pub fn assuan_pipe_connect(
        ctx: assuan_context_t,
        name: *const c_char,
        argv: *mut *const c_char,
        fd_child_list: *mut assuan_fd_t,
        atfork: Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: c_int)>,
        atforkvalue: *mut c_void,
        flags: c_uint,
    ) -> assuan_error_t;

    /*-- assuan-socket-connect.c --*/
    pub fn assuan_socket_connect(
        ctx: assuan_context_t,
        name: *const c_char,
        server_pid: assuan_pid_t,
        flags: c_uint,
    ) -> assuan_error_t;
    pub fn assuan_socket_connect_fd(
        ctx: assuan_context_t,
        fd: c_int,
        flags: c_uint,
    ) -> assuan_error_t;

    pub fn assuan_get_pid(ctx: assuan_context_t) -> assuan_pid_t;

    pub fn assuan_get_peercred(
        ctx: assuan_context_t,
        peercred: *mut assuan_peercred_t,
    ) -> assuan_error_t;

    pub fn assuan_client_read_response(
        ctx: assuan_context_t,
        line: *mut *mut c_char,
        linelen: *mut c_int,
    ) -> assuan_error_t;

    pub fn assuan_client_parse_response(
        ctx: assuan_context_t,
        line: *mut c_char,
        linelen: c_int,
        response: *mut assuan_response_t,
        off: *mut c_int,
    ) -> assuan_error_t;

    pub fn assuan_transact(
        ctx: assuan_context_t,
        command: *const c_char,
        data_cb: Option<assuan_transact_data_cb_t>,
        data_cb_arg: *mut c_void,
        inquire_cb: Option<assuan_transact_inquire_cb_t>,
        inquire_cb_arg: *mut c_void,
        status_cb: Option<assuan_transact_status_cb_t>,
        status_cb_arg: *mut c_void,
    ) -> assuan_error_t;

    pub fn assuan_inquire(
        ctx: assuan_context_t,
        keyword: *const c_char,
        r_buffer: *mut *mut c_uchar,
        r_length: *mut size_t,
        maxlen: size_t,
    ) -> assuan_error_t;

    pub fn assuan_inquire_ext(
        ctx: assuan_context_t,
        keyword: *const c_char,
        maxlen: size_t,
        cb: Option<
            unsafe extern "C" fn(
                cb_data: *mut c_void,
                rc: assuan_error_t,
                buf: *mut c_uchar,
                buf_len: size_t,
            ) -> assuan_error_t,
        >,
        cb_data: *mut c_void,
    ) -> assuan_error_t;

    pub fn assuan_read_line(
        ctx: assuan_context_t,
        line: *mut *mut c_char,
        linelen: *mut size_t,
    ) -> assuan_error_t;

    pub fn assuan_pending_line(ctx: assuan_context_t) -> c_int;

    pub fn assuan_write_line(ctx: assuan_context_t, line: *const c_char) -> assuan_error_t;

    pub fn assuan_send_data(
        ctx: assuan_context_t,
        buffer: *const c_void,
        length: size_t,
    ) -> assuan_error_t;

    pub fn assuan_sendfd(ctx: assuan_context_t, fd: assuan_fd_t) -> assuan_error_t;

    pub fn assuan_receivefd(ctx: assuan_context_t, fd: *mut assuan_fd_t) -> assuan_error_t;

    pub fn assuan_set_error(
        ctx: assuan_context_t,
        err: assuan_error_t,
        text: *const c_char,
    ) -> assuan_error_t;

    /* These are socket wrapper functions to support an emulation of Unix
    domain sockets on Windows W32.  */
    pub fn assuan_sock_init() -> assuan_error_t;
    pub fn assuan_sock_deinit();
    pub fn assuan_sock_close(fd: assuan_fd_t) -> c_int;
    pub fn assuan_sock_new(domain: c_int, type_: c_int, proto: c_int) -> assuan_fd_t;
    pub fn assuan_sock_set_flag(sockfd: assuan_fd_t, name: *const c_char, value: c_int) -> c_int;
    pub fn assuan_sock_get_flag(
        sockfd: assuan_fd_t,
        name: *const c_char,
        r_value: *mut c_int,
    ) -> c_int;
    pub fn assuan_sock_connect(sockfd: assuan_fd_t, addr: *mut sockaddr, addrlen: c_int) -> c_int;
    pub fn assuan_sock_connect_byname(
        host: *const c_char,
        port: c_ushort,
        reserved: c_int,
        credentials: *const c_char,
        flags: c_uint,
    ) -> assuan_fd_t;
    pub fn assuan_sock_bind(sockfd: assuan_fd_t, addr: *mut sockaddr, addrlen: c_int) -> c_int;
    pub fn assuan_sock_set_sockaddr_un(
        fname: *const c_char,
        addr: *mut sockaddr,
        r_redirected: *mut c_int,
    ) -> c_int;
    pub fn assuan_sock_get_nonce(
        addr: *mut sockaddr,
        addrlen: c_int,
        nonce: *mut assuan_sock_nonce_t,
    ) -> c_int;
    pub fn assuan_sock_check_nonce(fd: assuan_fd_t, nonce: *mut assuan_sock_nonce_t) -> c_int;
    pub fn assuan_sock_set_system_hooks(system_hooks: assuan_system_hooks_t);
    pub fn assuan_set_system_hooks(system_hooks: assuan_system_hooks_t);
    pub fn assuan_ctx_set_system_hooks(ctx: assuan_context_t, system_hooks: assuan_system_hooks_t);
}
