#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn COM_Parse(data_p: *mut *mut libc::c_char) -> *mut libc::c_char;
    fn Q_strcasecmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Clear(buf: *mut sizebuf_t);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn COM_Argc() -> libc::c_int;
    fn COM_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn COM_ClearArgv(arg: libc::c_int);
    fn CopyString(in_0: *mut libc::c_char) -> *mut libc::c_char;
    fn Z_Free(ptr: *mut libc::c_void);
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Cmd_ForwardToServer();
    fn Cvar_Command() -> qboolean;
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sizebuf_s {
    pub allowoverflow: qboolean,
    pub overflowed: qboolean,
    pub data: *mut byte,
    pub maxsize: libc::c_int,
    pub cursize: libc::c_int,
    pub readcount: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
pub type cmdalias_t = cmdalias_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmdalias_s {
    pub next: *mut cmdalias_s,
    pub name: [libc::c_char; 32],
    pub value: *mut libc::c_char,
}
pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;
pub type cmd_function_t = cmd_function_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmd_function_s {
    pub next: *mut cmd_function_s,
    pub name: *mut libc::c_char,
    pub function: xcommand_t,
}
#[no_mangle]
pub static mut cmd_alias: *mut cmdalias_t = 0 as *const cmdalias_t as *mut cmdalias_t;
#[no_mangle]
pub static mut cmd_wait: qboolean = false_0;
#[no_mangle]
pub static mut alias_count: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn Cmd_Wait_f() {
    cmd_wait = true_0;
}
#[no_mangle]
pub static mut cmd_text: sizebuf_t = sizebuf_t {
    allowoverflow: false_0,
    overflowed: false_0,
    data: 0 as *const byte as *mut byte,
    maxsize: 0,
    cursize: 0,
    readcount: 0,
};
#[no_mangle]
pub static mut cmd_text_buf: [byte; 8192] = [0; 8192];
#[no_mangle]
pub static mut defer_text_buf: [byte; 8192] = [0; 8192];
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Init() {
    SZ_Init(
        &mut cmd_text,
        cmd_text_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 8192]>() as libc::c_ulong as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddText(mut text: *mut libc::c_char) {
    let mut l: libc::c_int = 0;
    l = strlen(text) as libc::c_int;
    if cmd_text.cursize + l >= cmd_text.maxsize {
        Com_Printf(
            b"Cbuf_AddText: overflow\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    SZ_Write(&mut cmd_text, text as *mut libc::c_void, strlen(text) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_InsertText(mut text: *mut libc::c_char) {
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut templen: libc::c_int = 0;
    templen = cmd_text.cursize;
    if templen != 0 {
        temp = Z_Malloc(templen) as *mut libc::c_char;
        memcpy(
            temp as *mut libc::c_void,
            cmd_text.data as *const libc::c_void,
            templen as libc::c_ulong,
        );
        SZ_Clear(&mut cmd_text);
    } else {
        temp = 0 as *mut libc::c_char;
    }
    Cbuf_AddText(text);
    if templen != 0 {
        SZ_Write(&mut cmd_text, temp as *mut libc::c_void, templen);
        Z_Free(temp as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_CopyToDefer() {
    memcpy(
        defer_text_buf.as_mut_ptr() as *mut libc::c_void,
        cmd_text_buf.as_mut_ptr() as *const libc::c_void,
        cmd_text.cursize as libc::c_ulong,
    );
    defer_text_buf[cmd_text.cursize as usize] = 0 as libc::c_int as byte;
    cmd_text.cursize = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_InsertFromDefer() {
    Cbuf_InsertText(defer_text_buf.as_mut_ptr() as *mut libc::c_char);
    defer_text_buf[0 as libc::c_int as usize] = 0 as libc::c_int as byte;
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_ExecuteText(
    mut exec_when: libc::c_int,
    mut text: *mut libc::c_char,
) {
    match exec_when {
        0 => {
            Cmd_ExecuteString(text);
        }
        1 => {
            Cbuf_InsertText(text);
        }
        2 => {
            Cbuf_AddText(text);
        }
        _ => {
            Com_Error(
                0 as libc::c_int,
                b"Cbuf_ExecuteText: bad exec_when\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_Execute() {
    let mut i: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut quotes: libc::c_int = 0;
    alias_count = 0 as libc::c_int;
    while cmd_text.cursize != 0 {
        text = cmd_text.data as *mut libc::c_char;
        quotes = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < cmd_text.cursize {
            if *text.offset(i as isize) as libc::c_int == '"' as i32 {
                quotes += 1;
            }
            if quotes & 1 as libc::c_int == 0
                && *text.offset(i as isize) as libc::c_int == ';' as i32
            {
                break;
            }
            if *text.offset(i as isize) as libc::c_int == '\n' as i32 {
                break;
            }
            i += 1;
        }
        memcpy(
            line.as_mut_ptr() as *mut libc::c_void,
            text as *const libc::c_void,
            i as libc::c_ulong,
        );
        line[i as usize] = 0 as libc::c_int as libc::c_char;
        if i == cmd_text.cursize {
            cmd_text.cursize = 0 as libc::c_int;
        } else {
            i += 1;
            cmd_text.cursize -= i;
            memmove(
                text as *mut libc::c_void,
                text.offset(i as isize) as *const libc::c_void,
                cmd_text.cursize as libc::c_ulong,
            );
        }
        Cmd_ExecuteString(line.as_mut_ptr());
        if !(cmd_wait as u64 != 0) {
            continue;
        }
        cmd_wait = false_0;
        break;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddEarlyCommands(mut clear: qboolean) {
    let mut i: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < COM_Argc() {
        s = COM_Argv(i);
        if !(strcmp(s, b"+set\0" as *const u8 as *const libc::c_char) != 0) {
            Cbuf_AddText(
                va(
                    b"set %s %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    COM_Argv(i + 1 as libc::c_int),
                    COM_Argv(i + 2 as libc::c_int),
                ),
            );
            if clear as u64 != 0 {
                COM_ClearArgv(i);
                COM_ClearArgv(i + 1 as libc::c_int);
                COM_ClearArgv(i + 2 as libc::c_int);
            }
            i += 2 as libc::c_int;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cbuf_AddLateCommands() -> qboolean {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut text: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut build: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    let mut argc: libc::c_int = 0;
    let mut ret: qboolean = false_0;
    s = 0 as libc::c_int;
    argc = COM_Argc();
    i = 1 as libc::c_int;
    while i < argc {
        s = (s as libc::c_ulong)
            .wrapping_add(
                (strlen(COM_Argv(i))).wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int as libc::c_int;
        i += 1;
    }
    if s == 0 {
        return false_0;
    }
    text = Z_Malloc(s + 1 as libc::c_int) as *mut libc::c_char;
    *text.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    i = 1 as libc::c_int;
    while i < argc {
        strcat(text, COM_Argv(i));
        if i != argc - 1 as libc::c_int {
            strcat(text, b" \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    build = Z_Malloc(s + 1 as libc::c_int) as *mut libc::c_char;
    *build.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < s - 1 as libc::c_int {
        if *text.offset(i as isize) as libc::c_int == '+' as i32 {
            i += 1;
            j = i;
            while *text.offset(j as isize) as libc::c_int != '+' as i32
                && *text.offset(j as isize) as libc::c_int != '-' as i32
                && *text.offset(j as isize) as libc::c_int != 0 as libc::c_int
            {
                j += 1;
            }
            c = *text.offset(j as isize);
            *text.offset(j as isize) = 0 as libc::c_int as libc::c_char;
            strcat(build, text.offset(i as isize));
            strcat(build, b"\n\0" as *const u8 as *const libc::c_char);
            *text.offset(j as isize) = c;
            i = j - 1 as libc::c_int;
        }
        i += 1;
    }
    ret = (*build.offset(0 as libc::c_int as isize) as libc::c_int != 0 as libc::c_int)
        as libc::c_int as qboolean;
    if ret as u64 != 0 {
        Cbuf_AddText(build);
    }
    Z_Free(text as *mut libc::c_void);
    Z_Free(build as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Exec_f() {
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    if Cmd_Argc() != 2 as libc::c_int {
        Com_Printf(
            b"exec <filename> : execute a script file\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    len = FS_LoadFile(
        Cmd_Argv(1 as libc::c_int),
        &mut f as *mut *mut libc::c_char as *mut *mut libc::c_void,
    );
    if f.is_null() {
        Com_Printf(
            b"couldn't exec %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            Cmd_Argv(1 as libc::c_int),
        );
        return;
    }
    Com_Printf(
        b"execing %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Cmd_Argv(1 as libc::c_int),
    );
    f2 = Z_Malloc(len + 1 as libc::c_int) as *mut libc::c_char;
    memcpy(f2 as *mut libc::c_void, f as *const libc::c_void, len as libc::c_ulong);
    *f2.offset(len as isize) = 0 as libc::c_int as libc::c_char;
    Cbuf_InsertText(f2);
    Z_Free(f2 as *mut libc::c_void);
    FS_FreeFile(f as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Echo_f() {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < Cmd_Argc() {
        Com_Printf(
            b"%s \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            Cmd_Argv(i),
        );
        i += 1;
    }
    Com_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Alias_f() {
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    let mut cmd: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if Cmd_Argc() == 1 as libc::c_int {
        Com_Printf(
            b"Current alias commands:\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        a = cmd_alias;
        while !a.is_null() {
            Com_Printf(
                b"%s : %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*a).name).as_mut_ptr(),
                (*a).value,
            );
            a = (*a).next;
        }
        return;
    }
    s = Cmd_Argv(1 as libc::c_int);
    if strlen(s) >= 32 as libc::c_int as libc::c_ulong {
        Com_Printf(
            b"Alias name is too long\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    a = cmd_alias;
    while !a.is_null() {
        if strcmp(s, ((*a).name).as_mut_ptr()) == 0 {
            Z_Free((*a).value as *mut libc::c_void);
            break;
        } else {
            a = (*a).next;
        }
    }
    if a.is_null() {
        a = Z_Malloc(::std::mem::size_of::<cmdalias_t>() as libc::c_ulong as libc::c_int)
            as *mut cmdalias_t;
        let ref mut fresh0 = (*a).next;
        *fresh0 = cmd_alias;
        cmd_alias = a;
    }
    strcpy(((*a).name).as_mut_ptr(), s);
    cmd[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    c = Cmd_Argc();
    i = 2 as libc::c_int;
    while i < c {
        strcat(cmd.as_mut_ptr(), Cmd_Argv(i));
        if i != c - 1 as libc::c_int {
            strcat(cmd.as_mut_ptr(), b" \0" as *const u8 as *const libc::c_char);
        }
        i += 1;
    }
    strcat(cmd.as_mut_ptr(), b"\n\0" as *const u8 as *const libc::c_char);
    let ref mut fresh1 = (*a).value;
    *fresh1 = CopyString(cmd.as_mut_ptr());
}
static mut cmd_argc: libc::c_int = 0;
static mut cmd_argv: [*mut libc::c_char; 80] = [0 as *const libc::c_char
    as *mut libc::c_char; 80];
static mut cmd_null_string: *mut libc::c_char = b"\0" as *const u8 as *const libc::c_char
    as *mut libc::c_char;
static mut cmd_args: [libc::c_char; 1024] = [0; 1024];
static mut cmd_functions: *mut cmd_function_t = 0 as *const cmd_function_t
    as *mut cmd_function_t;
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argc() -> libc::c_int {
    return cmd_argc;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Argv(mut arg: libc::c_int) -> *mut libc::c_char {
    if arg as libc::c_uint >= cmd_argc as libc::c_uint {
        return cmd_null_string;
    }
    return cmd_argv[arg as usize];
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Args() -> *mut libc::c_char {
    return cmd_args.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_MacroExpandString(
    mut text: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut inquote: qboolean = false_0;
    let mut scan: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut expanded: [libc::c_char; 1024] = [0; 1024];
    let mut temporary: [libc::c_char; 1024] = [0; 1024];
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
    inquote = false_0;
    scan = text;
    len = strlen(scan) as libc::c_int;
    if len >= 1024 as libc::c_int {
        Com_Printf(
            b"Line exceeded %i chars, discarded.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            1024 as libc::c_int,
        );
        return 0 as *mut libc::c_char;
    }
    count = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        if *scan.offset(i as isize) as libc::c_int == '"' as i32 {
            inquote = ::std::mem::transmute::<
                libc::c_uint,
                qboolean,
            >(inquote as libc::c_uint ^ 1 as libc::c_int as libc::c_uint);
        }
        if !(inquote as u64 != 0) {
            if !(*scan.offset(i as isize) as libc::c_int != '$' as i32) {
                start = scan.offset(i as isize).offset(1 as libc::c_int as isize);
                token = COM_Parse(&mut start);
                if !start.is_null() {
                    token = Cvar_VariableString(token);
                    j = strlen(token) as libc::c_int;
                    len += j;
                    if len >= 1024 as libc::c_int {
                        Com_Printf(
                            b"Expanded line exceeded %i chars, discarded.\n\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            1024 as libc::c_int,
                        );
                        return 0 as *mut libc::c_char;
                    }
                    strncpy(temporary.as_mut_ptr(), scan, i as libc::c_ulong);
                    strcpy(temporary.as_mut_ptr().offset(i as isize), token);
                    strcpy(
                        temporary.as_mut_ptr().offset(i as isize).offset(j as isize),
                        start,
                    );
                    strcpy(expanded.as_mut_ptr(), temporary.as_mut_ptr());
                    scan = expanded.as_mut_ptr();
                    i -= 1;
                    count += 1;
                    if count == 100 as libc::c_int {
                        Com_Printf(
                            b"Macro expansion loop, discarded.\n\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        return 0 as *mut libc::c_char;
                    }
                }
            }
        }
        i += 1;
    }
    if inquote as u64 != 0 {
        Com_Printf(
            b"Line has unmatched quote, discarded.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return 0 as *mut libc::c_char;
    }
    return scan;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_TokenizeString(
    mut text: *mut libc::c_char,
    mut macroExpand: qboolean,
) {
    let mut i: libc::c_int = 0;
    let mut com_token: *mut libc::c_char = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while i < cmd_argc {
        Z_Free(cmd_argv[i as usize] as *mut libc::c_void);
        i += 1;
    }
    cmd_argc = 0 as libc::c_int;
    cmd_args[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if macroExpand as u64 != 0 {
        text = Cmd_MacroExpandString(text);
    }
    if text.is_null() {
        return;
    }
    loop {
        while *text as libc::c_int != 0 && *text as libc::c_int <= ' ' as i32
            && *text as libc::c_int != '\n' as i32
        {
            text = text.offset(1);
        }
        if *text as libc::c_int == '\n' as i32 {
            text = text.offset(1);
            break;
        } else {
            if *text == 0 {
                return;
            }
            if cmd_argc == 1 as libc::c_int {
                let mut l: libc::c_int = 0;
                strcpy(cmd_args.as_mut_ptr(), text);
                l = (strlen(cmd_args.as_mut_ptr()))
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
                while l >= 0 as libc::c_int {
                    if !(cmd_args[l as usize] as libc::c_int <= ' ' as i32) {
                        break;
                    }
                    cmd_args[l as usize] = 0 as libc::c_int as libc::c_char;
                    l -= 1;
                }
            }
            com_token = COM_Parse(&mut text);
            if text.is_null() {
                return;
            }
            if cmd_argc < 80 as libc::c_int {
                cmd_argv[cmd_argc
                    as usize] = Z_Malloc(
                    (strlen(com_token)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int,
                ) as *mut libc::c_char;
                strcpy(cmd_argv[cmd_argc as usize], com_token);
                cmd_argc += 1;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_AddCommand(
    mut cmd_name: *mut libc::c_char,
    mut function: xcommand_t,
) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    if *(Cvar_VariableString(cmd_name)).offset(0 as libc::c_int as isize) != 0 {
        Com_Printf(
            b"Cmd_AddCommand: %s already defined as a var\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            cmd_name,
        );
        return;
    }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if strcmp(cmd_name, (*cmd).name) == 0 {
            Com_Printf(
                b"Cmd_AddCommand: %s already defined\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                cmd_name,
            );
            return;
        }
        cmd = (*cmd).next;
    }
    cmd = Z_Malloc(
        ::std::mem::size_of::<cmd_function_t>() as libc::c_ulong as libc::c_int,
    ) as *mut cmd_function_t;
    let ref mut fresh2 = (*cmd).name;
    *fresh2 = cmd_name;
    let ref mut fresh3 = (*cmd).function;
    *fresh3 = function;
    let ref mut fresh4 = (*cmd).next;
    *fresh4 = cmd_functions;
    cmd_functions = cmd;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_RemoveCommand(mut cmd_name: *mut libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut back: *mut *mut cmd_function_t = 0 as *mut *mut cmd_function_t;
    back = &mut cmd_functions;
    loop {
        cmd = *back;
        if cmd.is_null() {
            Com_Printf(
                b"Cmd_RemoveCommand: %s not added\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                cmd_name,
            );
            return;
        }
        if strcmp(cmd_name, (*cmd).name) == 0 {
            *back = (*cmd).next;
            Z_Free(cmd as *mut libc::c_void);
            return;
        }
        back = &mut (*cmd).next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Exists(mut cmd_name: *mut libc::c_char) -> qboolean {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    cmd = cmd_functions;
    while !cmd.is_null() {
        if strcmp(cmd_name, (*cmd).name) == 0 {
            return true_0;
        }
        cmd = (*cmd).next;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_CompleteCommand(
    mut partial: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut len: libc::c_int = 0;
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    len = strlen(partial) as libc::c_int;
    if len == 0 {
        return 0 as *mut libc::c_char;
    }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if strcmp(partial, (*cmd).name) == 0 {
            return (*cmd).name;
        }
        cmd = (*cmd).next;
    }
    a = cmd_alias;
    while !a.is_null() {
        if strcmp(partial, ((*a).name).as_mut_ptr()) == 0 {
            return ((*a).name).as_mut_ptr();
        }
        a = (*a).next;
    }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if strncmp(partial, (*cmd).name, len as libc::c_ulong) == 0 {
            return (*cmd).name;
        }
        cmd = (*cmd).next;
    }
    a = cmd_alias;
    while !a.is_null() {
        if strncmp(partial, ((*a).name).as_mut_ptr(), len as libc::c_ulong) == 0 {
            return ((*a).name).as_mut_ptr();
        }
        a = (*a).next;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_ExecuteString(mut text: *mut libc::c_char) {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut a: *mut cmdalias_t = 0 as *mut cmdalias_t;
    Cmd_TokenizeString(text, true_0);
    if Cmd_Argc() == 0 {
        return;
    }
    cmd = cmd_functions;
    while !cmd.is_null() {
        if Q_strcasecmp(cmd_argv[0 as libc::c_int as usize], (*cmd).name) == 0 {
            if ((*cmd).function).is_none() {
                Cmd_ExecuteString(
                    va(
                        b"cmd %s\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        text,
                    ),
                );
            } else {
                ((*cmd).function).expect("non-null function pointer")();
            }
            return;
        }
        cmd = (*cmd).next;
    }
    a = cmd_alias;
    while !a.is_null() {
        if Q_strcasecmp(cmd_argv[0 as libc::c_int as usize], ((*a).name).as_mut_ptr())
            == 0
        {
            alias_count += 1;
            if alias_count == 16 as libc::c_int {
                Com_Printf(
                    b"ALIAS_LOOP_COUNT\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                return;
            }
            Cbuf_InsertText((*a).value);
            return;
        }
        a = (*a).next;
    }
    if Cvar_Command() as u64 != 0 {
        return;
    }
    Cmd_ForwardToServer();
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_List_f() {
    let mut cmd: *mut cmd_function_t = 0 as *mut cmd_function_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    cmd = cmd_functions;
    while !cmd.is_null() {
        Com_Printf(
            b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*cmd).name,
        );
        cmd = (*cmd).next;
        i += 1;
    }
    Com_Printf(
        b"%i commands\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        i,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Cmd_Init() {
    Cmd_AddCommand(
        b"cmdlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cmd_List_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"exec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cmd_Exec_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"echo\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cmd_Echo_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"alias\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cmd_Alias_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"wait\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cmd_Wait_f as unsafe extern "C" fn() -> ()),
    );
}
