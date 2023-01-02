#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atof(__nptr: *const libc::c_char) -> libc::c_double;
    fn CopyString(in_0: *mut libc::c_char) -> *mut libc::c_char;
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn FS_ExecAutoexec();
    fn FS_SetGamedir(dir: *mut libc::c_char);
    fn Com_ServerState() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Info_SetValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
        value: *mut libc::c_char,
    );
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cvar_s {
    pub name: *mut libc::c_char,
    pub string: *mut libc::c_char,
    pub latched_string: *mut libc::c_char,
    pub flags: libc::c_int,
    pub modified: qboolean,
    pub value: libc::c_float,
    pub next: *mut cvar_s,
}
pub type cvar_t = cvar_s;
pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;
#[no_mangle]
pub static mut cvar_vars: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
unsafe extern "C" fn Cvar_InfoValidate(mut s: *mut libc::c_char) -> qboolean {
    if !(strstr(s, b"\\\0" as *const u8 as *const libc::c_char)).is_null() {
        return false_0;
    }
    if !(strstr(s, b"\"\0" as *const u8 as *const libc::c_char)).is_null() {
        return false_0;
    }
    if !(strstr(s, b";\0" as *const u8 as *const libc::c_char)).is_null() {
        return false_0;
    }
    return true_0;
}
unsafe extern "C" fn Cvar_FindVar(mut var_name: *mut libc::c_char) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = cvar_vars;
    while !var.is_null() {
        if strcmp(var_name, (*var).name) == 0 {
            return var;
        }
        var = (*var).next;
    }
    return 0 as *mut cvar_t;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableValue(
    mut var_name: *mut libc::c_char,
) -> libc::c_float {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return 0 as libc::c_int as libc::c_float;
    }
    return atof((*var).string) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_VariableString(
    mut var_name: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    return (*var).string;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_CompleteVariable(
    mut partial: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut cvar: *mut cvar_t = 0 as *mut cvar_t;
    let mut len: libc::c_int = 0;
    len = strlen(partial) as libc::c_int;
    if len == 0 {
        return 0 as *mut libc::c_char;
    }
    cvar = cvar_vars;
    while !cvar.is_null() {
        if strcmp(partial, (*cvar).name) == 0 {
            return (*cvar).name;
        }
        cvar = (*cvar).next;
    }
    cvar = cvar_vars;
    while !cvar.is_null() {
        if strncmp(partial, (*cvar).name, len as libc::c_ulong) == 0 {
            return (*cvar).name;
        }
        cvar = (*cvar).next;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Get(
    mut var_name: *mut libc::c_char,
    mut var_value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    if flags & (2 as libc::c_int | 4 as libc::c_int) != 0 {
        if Cvar_InfoValidate(var_name) as u64 == 0 {
            Com_Printf(
                b"invalid info cvar name\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut cvar_t;
        }
    }
    var = Cvar_FindVar(var_name);
    if !var.is_null() {
        (*var).flags |= flags;
        return var;
    }
    if var_value.is_null() {
        return 0 as *mut cvar_t;
    }
    if flags & (2 as libc::c_int | 4 as libc::c_int) != 0 {
        if Cvar_InfoValidate(var_value) as u64 == 0 {
            Com_Printf(
                b"invalid info cvar value\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return 0 as *mut cvar_t;
        }
    }
    var = Z_Malloc(::std::mem::size_of::<cvar_t>() as libc::c_ulong as libc::c_int)
        as *mut cvar_t;
    let ref mut fresh0 = (*var).name;
    *fresh0 = CopyString(var_name);
    let ref mut fresh1 = (*var).string;
    *fresh1 = CopyString(var_value);
    (*var).modified = true_0;
    (*var).value = atof((*var).string) as libc::c_float;
    let ref mut fresh2 = (*var).next;
    *fresh2 = cvar_vars;
    cvar_vars = var;
    (*var).flags = flags;
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set2(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut force: qboolean,
) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return Cvar_Get(var_name, value, 0 as libc::c_int);
    }
    if (*var).flags & (2 as libc::c_int | 4 as libc::c_int) != 0 {
        if Cvar_InfoValidate(value) as u64 == 0 {
            Com_Printf(
                b"invalid info cvar value\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return var;
        }
    }
    if force as u64 == 0 {
        if (*var).flags & 8 as libc::c_int != 0 {
            Com_Printf(
                b"%s is write protected.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                var_name,
            );
            return var;
        }
        if (*var).flags & 16 as libc::c_int != 0 {
            if !((*var).latched_string).is_null() {
                if strcmp(value, (*var).latched_string) == 0 as libc::c_int {
                    return var;
                }
                Z_Free((*var).latched_string as *mut libc::c_void);
            } else if strcmp(value, (*var).string) == 0 as libc::c_int {
                return var
            }
            if Com_ServerState() != 0 {
                Com_Printf(
                    b"%s will be changed for next game.\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    var_name,
                );
                let ref mut fresh3 = (*var).latched_string;
                *fresh3 = CopyString(value);
            } else {
                let ref mut fresh4 = (*var).string;
                *fresh4 = CopyString(value);
                (*var).value = atof((*var).string) as libc::c_float;
                if strcmp((*var).name, b"game\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    FS_SetGamedir((*var).string);
                    FS_ExecAutoexec();
                }
            }
            return var;
        }
    } else if !((*var).latched_string).is_null() {
        Z_Free((*var).latched_string as *mut libc::c_void);
        let ref mut fresh5 = (*var).latched_string;
        *fresh5 = 0 as *mut libc::c_char;
    }
    if strcmp(value, (*var).string) == 0 {
        return var;
    }
    (*var).modified = true_0;
    if (*var).flags & 2 as libc::c_int != 0 {
        userinfo_modified = true_0;
    }
    Z_Free((*var).string as *mut libc::c_void);
    let ref mut fresh6 = (*var).string;
    *fresh6 = CopyString(value);
    (*var).value = atof((*var).string) as libc::c_float;
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_ForceSet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> *mut cvar_t {
    return Cvar_Set2(var_name, value, true_0);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> *mut cvar_t {
    return Cvar_Set2(var_name, value, false_0);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_FullSet(
    mut var_name: *mut libc::c_char,
    mut value: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut cvar_t {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = Cvar_FindVar(var_name);
    if var.is_null() {
        return Cvar_Get(var_name, value, flags);
    }
    (*var).modified = true_0;
    if (*var).flags & 2 as libc::c_int != 0 {
        userinfo_modified = true_0;
    }
    Z_Free((*var).string as *mut libc::c_void);
    let ref mut fresh7 = (*var).string;
    *fresh7 = CopyString(value);
    (*var).value = atof((*var).string) as libc::c_float;
    (*var).flags = flags;
    return var;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_SetValue(
    mut var_name: *mut libc::c_char,
    mut value: libc::c_float,
) {
    let mut val: [libc::c_char; 32] = [0; 32];
    if value == value as libc::c_int as libc::c_float {
        Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value as libc::c_int,
        );
    } else {
        Com_sprintf(
            val.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong as libc::c_int,
            b"%f\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            value as libc::c_double,
        );
    }
    Cvar_Set(var_name, val.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_GetLatchedVars() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    var = cvar_vars;
    while !var.is_null() {
        if !((*var).latched_string).is_null() {
            Z_Free((*var).string as *mut libc::c_void);
            let ref mut fresh8 = (*var).string;
            *fresh8 = (*var).latched_string;
            let ref mut fresh9 = (*var).latched_string;
            *fresh9 = 0 as *mut libc::c_char;
            (*var).value = atof((*var).string) as libc::c_float;
            if strcmp((*var).name, b"game\0" as *const u8 as *const libc::c_char) == 0 {
                FS_SetGamedir((*var).string);
                FS_ExecAutoexec();
            }
        }
        var = (*var).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Command() -> qboolean {
    let mut v: *mut cvar_t = 0 as *mut cvar_t;
    v = Cvar_FindVar(Cmd_Argv(0 as libc::c_int));
    if v.is_null() {
        return false_0;
    }
    if Cmd_Argc() == 1 as libc::c_int {
        Com_Printf(
            b"\"%s\" is \"%s\"\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*v).name,
            (*v).string,
        );
        return true_0;
    }
    Cvar_Set((*v).name, Cmd_Argv(1 as libc::c_int));
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Set_f() {
    let mut c: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    c = Cmd_Argc();
    if c != 3 as libc::c_int && c != 4 as libc::c_int {
        Com_Printf(
            b"usage: set <variable> <value> [u / s]\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if c == 4 as libc::c_int {
        if strcmp(Cmd_Argv(3 as libc::c_int), b"u\0" as *const u8 as *const libc::c_char)
            == 0
        {
            flags = 2 as libc::c_int;
        } else if strcmp(
            Cmd_Argv(3 as libc::c_int),
            b"s\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            flags = 4 as libc::c_int;
        } else {
            Com_Printf(
                b"flags can only be 'u' or 's'\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
        Cvar_FullSet(Cmd_Argv(1 as libc::c_int), Cmd_Argv(2 as libc::c_int), flags);
    } else {
        Cvar_Set(Cmd_Argv(1 as libc::c_int), Cmd_Argv(2 as libc::c_int));
    };
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_WriteVariables(mut path: *mut libc::c_char) {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut buffer: [libc::c_char; 1024] = [0; 1024];
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(path, b"a\0" as *const u8 as *const libc::c_char);
    var = cvar_vars;
    while !var.is_null() {
        if (*var).flags & 1 as libc::c_int != 0 {
            Com_sprintf(
                buffer.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                    as libc::c_int,
                b"set %s \"%s\"\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*var).name,
                (*var).string,
            );
            fprintf(f, b"%s\0" as *const u8 as *const libc::c_char, buffer.as_mut_ptr());
        }
        var = (*var).next;
    }
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_List_f() {
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    var = cvar_vars;
    while !var.is_null() {
        if (*var).flags & 1 as libc::c_int != 0 {
            Com_Printf(b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            Com_Printf(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        if (*var).flags & 2 as libc::c_int != 0 {
            Com_Printf(b"U\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            Com_Printf(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        if (*var).flags & 4 as libc::c_int != 0 {
            Com_Printf(b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            Com_Printf(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        if (*var).flags & 8 as libc::c_int != 0 {
            Com_Printf(b"-\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else if (*var).flags & 16 as libc::c_int != 0 {
            Com_Printf(b"L\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        } else {
            Com_Printf(b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
        Com_Printf(
            b" %s \"%s\"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*var).name,
            (*var).string,
        );
        var = (*var).next;
        i += 1;
    }
    Com_Printf(
        b"%i cvars\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        i,
    );
}
#[no_mangle]
pub static mut userinfo_modified: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn Cvar_BitInfo(mut bit: libc::c_int) -> *mut libc::c_char {
    static mut info: [libc::c_char; 512] = [0; 512];
    let mut var: *mut cvar_t = 0 as *mut cvar_t;
    info[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    var = cvar_vars;
    while !var.is_null() {
        if (*var).flags & bit != 0 {
            Info_SetValueForKey(info.as_mut_ptr(), (*var).name, (*var).string);
        }
        var = (*var).next;
    }
    return info.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Userinfo() -> *mut libc::c_char {
    return Cvar_BitInfo(2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Serverinfo() -> *mut libc::c_char {
    return Cvar_BitInfo(4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Cvar_Init() {
    Cmd_AddCommand(
        b"set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cvar_Set_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"cvarlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(Cvar_List_f as unsafe extern "C" fn() -> ()),
    );
}
