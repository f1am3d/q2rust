#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_strcasecmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Sys_Mkdir(path: *mut libc::c_char);
    fn Sys_FindFirst(
        path: *mut libc::c_char,
        musthave: libc::c_uint,
        canthave: libc::c_uint,
    ) -> *mut libc::c_char;
    fn Sys_FindNext(musthave: libc::c_uint, canthave: libc::c_uint) -> *mut libc::c_char;
    fn Sys_FindClose();
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn CopyString(in_0: *mut libc::c_char) -> *mut libc::c_char;
    fn Cbuf_AddText(text: *mut libc::c_char);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_FullSet(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_VariableString(var_name: *mut libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn Com_BlockChecksum(buffer: *mut libc::c_void, length: libc::c_int) -> libc::c_uint;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    static mut dedicated: *mut cvar_t;
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    fn Z_Free(ptr: *mut libc::c_void);
    fn CDAudio_Stop();
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
pub type byte = libc::c_uchar;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpackfile_t {
    pub name: [libc::c_char; 56],
    pub filepos: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dpackheader_t {
    pub ident: libc::c_int,
    pub dirofs: libc::c_int,
    pub dirlen: libc::c_int,
}
pub type searchpath_t = searchpath_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct searchpath_s {
    pub filename: [libc::c_char; 128],
    pub pack: *mut pack_t,
    pub next: *mut searchpath_s,
}
pub type pack_t = pack_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pack_s {
    pub filename: [libc::c_char; 128],
    pub handle: *mut FILE,
    pub numfiles: libc::c_int,
    pub files: *mut packfile_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct packfile_t {
    pub name: [libc::c_char; 64],
    pub filepos: libc::c_int,
    pub filelen: libc::c_int,
}
pub type filelink_t = filelink_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filelink_s {
    pub next: *mut filelink_s,
    pub from: *mut libc::c_char,
    pub fromlength: libc::c_int,
    pub to: *mut libc::c_char,
}
#[no_mangle]
pub static mut fs_gamedir: [libc::c_char; 128] = [0; 128];
#[no_mangle]
pub static mut fs_basedir: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fs_cddir: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fs_gamedirvar: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fs_links: *mut filelink_t = 0 as *const filelink_t as *mut filelink_t;
#[no_mangle]
pub static mut fs_searchpaths: *mut searchpath_t = 0 as *const searchpath_t
    as *mut searchpath_t;
#[no_mangle]
pub static mut fs_base_searchpaths: *mut searchpath_t = 0 as *const searchpath_t
    as *mut searchpath_t;
#[no_mangle]
pub unsafe extern "C" fn FS_filelength(mut f: *mut FILE) -> libc::c_int {
    let mut pos: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    pos = ftell(f) as libc::c_int;
    fseek(f, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    end = ftell(f) as libc::c_int;
    fseek(f, pos as libc::c_long, 0 as libc::c_int);
    return end;
}
#[no_mangle]
pub unsafe extern "C" fn FS_CreatePath(mut path: *mut libc::c_char) {
    let mut ofs: *mut libc::c_char = 0 as *mut libc::c_char;
    ofs = path.offset(1 as libc::c_int as isize);
    while *ofs != 0 {
        if *ofs as libc::c_int == '/' as i32 {
            *ofs = 0 as libc::c_int as libc::c_char;
            Sys_Mkdir(path);
            *ofs = '/' as i32 as libc::c_char;
        }
        ofs = ofs.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FS_FCloseFile(mut f: *mut FILE) {
    fclose(f);
}
#[no_mangle]
pub unsafe extern "C" fn Developer_searchpath(mut who: libc::c_int) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    if who == 1 as libc::c_int {
        ch = 'x' as i32;
    } else if who == 2 as libc::c_int {
        ch = 'r' as i32;
    }
    search = fs_searchpaths;
    while !search.is_null() {
        if !(strstr(
            ((*search).filename).as_mut_ptr(),
            b"xatrix\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            return 1 as libc::c_int;
        }
        if !(strstr(
            ((*search).filename).as_mut_ptr(),
            b"rogue\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            return 2 as libc::c_int;
        }
        search = (*search).next;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut file_from_pak: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn FS_FOpenFile(
    mut filename: *mut libc::c_char,
    mut file: *mut *mut FILE,
) -> libc::c_int {
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut netpath: [libc::c_char; 128] = [0; 128];
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut i: libc::c_int = 0;
    let mut link: *mut filelink_t = 0 as *mut filelink_t;
    file_from_pak = 0 as libc::c_int;
    link = fs_links;
    while !link.is_null() {
        if strncmp(filename, (*link).from, (*link).fromlength as libc::c_ulong) == 0 {
            Com_sprintf(
                netpath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                    as libc::c_int,
                b"%s%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*link).to,
                filename.offset((*link).fromlength as isize),
            );
            *file = fopen(
                netpath.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if !(*file).is_null() {
                Com_DPrintf(
                    b"link file: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    netpath.as_mut_ptr(),
                );
                return FS_filelength(*file);
            }
            return -(1 as libc::c_int);
        }
        link = (*link).next;
    }
    search = fs_searchpaths;
    while !search.is_null() {
        if !((*search).pack).is_null() {
            pak = (*search).pack;
            i = 0 as libc::c_int;
            while i < (*pak).numfiles {
                if Q_strcasecmp(
                    ((*((*pak).files).offset(i as isize)).name).as_mut_ptr(),
                    filename,
                ) == 0
                {
                    file_from_pak = 1 as libc::c_int;
                    Com_DPrintf(
                        b"PackFile: %s : %s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        ((*pak).filename).as_mut_ptr(),
                        filename,
                    );
                    *file = fopen(
                        ((*pak).filename).as_mut_ptr(),
                        b"rb\0" as *const u8 as *const libc::c_char,
                    );
                    if (*file).is_null() {
                        Com_Error(
                            0 as libc::c_int,
                            b"Couldn't reopen %s\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            ((*pak).filename).as_mut_ptr(),
                        );
                    }
                    fseek(
                        *file,
                        (*((*pak).files).offset(i as isize)).filepos as libc::c_long,
                        0 as libc::c_int,
                    );
                    return (*((*pak).files).offset(i as isize)).filelen;
                }
                i += 1;
            }
        } else {
            Com_sprintf(
                netpath.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
                    as libc::c_int,
                b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*search).filename).as_mut_ptr(),
                filename,
            );
            *file = fopen(
                netpath.as_mut_ptr(),
                b"rb\0" as *const u8 as *const libc::c_char,
            );
            if !(*file).is_null() {
                Com_DPrintf(
                    b"FindFile: %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    netpath.as_mut_ptr(),
                );
                return FS_filelength(*file);
            }
        }
        search = (*search).next;
    }
    Com_DPrintf(
        b"FindFile: can't find %s\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        filename,
    );
    *file = 0 as *mut FILE;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn FS_Read(
    mut buffer: *mut libc::c_void,
    mut len: libc::c_int,
    mut f: *mut FILE,
) {
    let mut block: libc::c_int = 0;
    let mut remaining: libc::c_int = 0;
    let mut read: libc::c_int = 0;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut tries: libc::c_int = 0;
    buf = buffer as *mut byte;
    remaining = len;
    tries = 0 as libc::c_int;
    while remaining != 0 {
        block = remaining;
        if block > 0x10000 as libc::c_int {
            block = 0x10000 as libc::c_int;
        }
        read = fread(
            buf as *mut libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            block as libc::c_ulong,
            f,
        ) as libc::c_int;
        if read == 0 as libc::c_int {
            if tries == 0 {
                tries = 1 as libc::c_int;
                CDAudio_Stop();
            } else {
                Com_Error(
                    0 as libc::c_int,
                    b"FS_Read: 0 bytes read\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        if read == -(1 as libc::c_int) {
            Com_Error(
                0 as libc::c_int,
                b"FS_Read: -1 bytes read\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        remaining -= read;
        buf = buf.offset(read as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FS_LoadFile(
    mut path: *mut libc::c_char,
    mut buffer: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut h: *mut FILE = 0 as *mut FILE;
    let mut buf: *mut byte = 0 as *mut byte;
    let mut len: libc::c_int = 0;
    buf = 0 as *mut byte;
    len = FS_FOpenFile(path, &mut h);
    if h.is_null() {
        if !buffer.is_null() {
            *buffer = 0 as *mut libc::c_void;
        }
        return -(1 as libc::c_int);
    }
    if buffer.is_null() {
        fclose(h);
        return len;
    }
    buf = Z_Malloc(len) as *mut byte;
    *buffer = buf as *mut libc::c_void;
    FS_Read(buf as *mut libc::c_void, len, h);
    fclose(h);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn FS_FreeFile(mut buffer: *mut libc::c_void) {
    Z_Free(buffer);
}
#[no_mangle]
pub unsafe extern "C" fn FS_LoadPackFile(
    mut packfile: *mut libc::c_char,
) -> *mut pack_t {
    let mut header: dpackheader_t = dpackheader_t {
        ident: 0,
        dirofs: 0,
        dirlen: 0,
    };
    let mut i: libc::c_int = 0;
    let mut newfiles: *mut packfile_t = 0 as *mut packfile_t;
    let mut numpackfiles: libc::c_int = 0;
    let mut pack: *mut pack_t = 0 as *mut pack_t;
    let mut packhandle: *mut FILE = 0 as *mut FILE;
    let mut info: [dpackfile_t; 4096] = [dpackfile_t {
        name: [0; 56],
        filepos: 0,
        filelen: 0,
    }; 4096];
    let mut checksum: libc::c_uint = 0;
    packhandle = fopen(packfile, b"rb\0" as *const u8 as *const libc::c_char);
    if packhandle.is_null() {
        return 0 as *mut pack_t;
    }
    fread(
        &mut header as *mut dpackheader_t as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<dpackheader_t>() as libc::c_ulong,
        packhandle,
    );
    if LittleLong(header.ident)
        != (('K' as i32) << 24 as libc::c_int) + (('C' as i32) << 16 as libc::c_int)
            + (('A' as i32) << 8 as libc::c_int) + 'P' as i32
    {
        Com_Error(
            0 as libc::c_int,
            b"%s is not a packfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            packfile,
        );
    }
    header.dirofs = LittleLong(header.dirofs);
    header.dirlen = LittleLong(header.dirlen);
    numpackfiles = (header.dirlen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dpackfile_t>() as libc::c_ulong)
        as libc::c_int;
    if numpackfiles > 4096 as libc::c_int {
        Com_Error(
            0 as libc::c_int,
            b"%s has %i files\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            packfile,
            numpackfiles,
        );
    }
    newfiles = Z_Malloc(
        (numpackfiles as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<packfile_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut packfile_t;
    fseek(packhandle, header.dirofs as libc::c_long, 0 as libc::c_int);
    fread(
        info.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        header.dirlen as libc::c_ulong,
        packhandle,
    );
    checksum = Com_BlockChecksum(info.as_mut_ptr() as *mut libc::c_void, header.dirlen);
    i = 0 as libc::c_int;
    while i < numpackfiles {
        strcpy(
            ((*newfiles.offset(i as isize)).name).as_mut_ptr(),
            (info[i as usize].name).as_mut_ptr(),
        );
        (*newfiles.offset(i as isize)).filepos = LittleLong(info[i as usize].filepos);
        (*newfiles.offset(i as isize)).filelen = LittleLong(info[i as usize].filelen);
        i += 1;
    }
    pack = Z_Malloc(::std::mem::size_of::<pack_t>() as libc::c_ulong as libc::c_int)
        as *mut pack_t;
    strcpy(((*pack).filename).as_mut_ptr(), packfile);
    let ref mut fresh0 = (*pack).handle;
    *fresh0 = packhandle;
    (*pack).numfiles = numpackfiles;
    let ref mut fresh1 = (*pack).files;
    *fresh1 = newfiles;
    Com_Printf(
        b"Added packfile %s (%i files)\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        packfile,
        numpackfiles,
    );
    return pack;
}
#[no_mangle]
pub unsafe extern "C" fn FS_AddGameDirectory(mut dir: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut search: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut pak: *mut pack_t = 0 as *mut pack_t;
    let mut pakfile: [libc::c_char; 128] = [0; 128];
    strcpy(fs_gamedir.as_mut_ptr(), dir);
    search = Z_Malloc(
        ::std::mem::size_of::<searchpath_t>() as libc::c_ulong as libc::c_int,
    ) as *mut searchpath_t;
    strcpy(((*search).filename).as_mut_ptr(), dir);
    let ref mut fresh2 = (*search).next;
    *fresh2 = fs_searchpaths;
    fs_searchpaths = search;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        Com_sprintf(
            pakfile.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            b"%s/pak%i.pak\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dir,
            i,
        );
        pak = FS_LoadPackFile(pakfile.as_mut_ptr());
        if !pak.is_null() {
            search = Z_Malloc(
                ::std::mem::size_of::<searchpath_t>() as libc::c_ulong as libc::c_int,
            ) as *mut searchpath_t;
            let ref mut fresh3 = (*search).pack;
            *fresh3 = pak;
            let ref mut fresh4 = (*search).next;
            *fresh4 = fs_searchpaths;
            fs_searchpaths = search;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FS_Gamedir() -> *mut libc::c_char {
    return fs_gamedir.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FS_ExecAutoexec() {
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: [libc::c_char; 64] = [0; 64];
    dir = Cvar_VariableString(
        b"gamedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if *dir != 0 {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%s/%s/autoexec.cfg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*fs_basedir).string,
            dir,
        );
    } else {
        Com_sprintf(
            name.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%s/%s/autoexec.cfg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*fs_basedir).string,
            b"baseq2\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(Sys_FindFirst(
        name.as_mut_ptr(),
        0 as libc::c_int as libc::c_uint,
        (0x8 as libc::c_int | 0x2 as libc::c_int | 0x10 as libc::c_int) as libc::c_uint,
    ))
        .is_null()
    {
        Cbuf_AddText(
            b"exec autoexec.cfg\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Sys_FindClose();
}
#[no_mangle]
pub unsafe extern "C" fn FS_SetGamedir(mut dir: *mut libc::c_char) {
    let mut next: *mut searchpath_t = 0 as *mut searchpath_t;
    if !(strstr(dir, b"..\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"/\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b"\\\0" as *const u8 as *const libc::c_char)).is_null()
        || !(strstr(dir, b":\0" as *const u8 as *const libc::c_char)).is_null()
    {
        Com_Printf(
            b"Gamedir should be a single filename, not a path\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    while fs_searchpaths != fs_base_searchpaths {
        if !((*fs_searchpaths).pack).is_null() {
            fclose((*(*fs_searchpaths).pack).handle);
            Z_Free((*(*fs_searchpaths).pack).files as *mut libc::c_void);
            Z_Free((*fs_searchpaths).pack as *mut libc::c_void);
        }
        next = (*fs_searchpaths).next;
        Z_Free(fs_searchpaths as *mut libc::c_void);
        fs_searchpaths = next;
    }
    if !dedicated.is_null() && (*dedicated).value == 0. {
        Cbuf_AddText(
            b"vid_restart\nsnd_restart\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Com_sprintf(
        fs_gamedir.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (*fs_basedir).string,
        dir,
    );
    if strcmp(dir, b"baseq2\0" as *const u8 as *const libc::c_char) == 0
        || *dir as libc::c_int == 0 as libc::c_int
    {
        Cvar_FullSet(
            b"gamedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            4 as libc::c_int | 8 as libc::c_int,
        );
        Cvar_FullSet(
            b"game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            16 as libc::c_int | 4 as libc::c_int,
        );
    } else {
        Cvar_FullSet(
            b"gamedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dir,
            4 as libc::c_int | 8 as libc::c_int,
        );
        if *((*fs_cddir).string).offset(0 as libc::c_int as isize) != 0 {
            FS_AddGameDirectory(
                va(
                    b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    (*fs_cddir).string,
                    dir,
                ),
            );
        }
        FS_AddGameDirectory(
            va(
                b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*fs_basedir).string,
                dir,
            ),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_Link_f() {
    let mut l: *mut filelink_t = 0 as *mut filelink_t;
    let mut prev: *mut *mut filelink_t = 0 as *mut *mut filelink_t;
    if Cmd_Argc() != 3 as libc::c_int {
        Com_Printf(
            b"USAGE: link <from> <to>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    prev = &mut fs_links;
    l = fs_links;
    while !l.is_null() {
        if strcmp((*l).from, Cmd_Argv(1 as libc::c_int)) == 0 {
            Z_Free((*l).to as *mut libc::c_void);
            if strlen(Cmd_Argv(2 as libc::c_int)) == 0 {
                *prev = (*l).next;
                Z_Free((*l).from as *mut libc::c_void);
                Z_Free(l as *mut libc::c_void);
                return;
            }
            let ref mut fresh5 = (*l).to;
            *fresh5 = CopyString(Cmd_Argv(2 as libc::c_int));
            return;
        }
        prev = &mut (*l).next;
        l = (*l).next;
    }
    l = Z_Malloc(::std::mem::size_of::<filelink_t>() as libc::c_ulong as libc::c_int)
        as *mut filelink_t;
    let ref mut fresh6 = (*l).next;
    *fresh6 = fs_links;
    fs_links = l;
    let ref mut fresh7 = (*l).from;
    *fresh7 = CopyString(Cmd_Argv(1 as libc::c_int));
    (*l).fromlength = strlen((*l).from) as libc::c_int;
    let ref mut fresh8 = (*l).to;
    *fresh8 = CopyString(Cmd_Argv(2 as libc::c_int));
}
#[no_mangle]
pub unsafe extern "C" fn FS_ListFiles(
    mut findname: *mut libc::c_char,
    mut numfiles: *mut libc::c_int,
    mut musthave: libc::c_uint,
    mut canthave: libc::c_uint,
) -> *mut *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nfiles: libc::c_int = 0 as libc::c_int;
    let mut list: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    s = Sys_FindFirst(findname, musthave, canthave);
    while !s.is_null() {
        if *s
            .offset((strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '.' as i32
        {
            nfiles += 1;
        }
        s = Sys_FindNext(musthave, canthave);
    }
    Sys_FindClose();
    if nfiles == 0 {
        return 0 as *mut *mut libc::c_char;
    }
    nfiles += 1;
    *numfiles = nfiles;
    list = malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nfiles as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    memset(
        list as *mut libc::c_void,
        0 as libc::c_int,
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nfiles as libc::c_ulong),
    );
    s = Sys_FindFirst(findname, musthave, canthave);
    nfiles = 0 as libc::c_int;
    while !s.is_null() {
        if *s
            .offset((strlen(s)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int != '.' as i32
        {
            let ref mut fresh9 = *list.offset(nfiles as isize);
            *fresh9 = strdup(s);
            nfiles += 1;
        }
        s = Sys_FindNext(musthave, canthave);
    }
    Sys_FindClose();
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn FS_Dir_f() {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut findname: [libc::c_char; 1024] = [0; 1024];
    let mut wildcard: [libc::c_char; 1024] = *::std::mem::transmute::<
        &[u8; 1024],
        &mut [libc::c_char; 1024],
    >(
        b"*.*\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    );
    let mut dirnames: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut ndirs: libc::c_int = 0;
    if Cmd_Argc() != 1 as libc::c_int {
        strcpy(wildcard.as_mut_ptr(), Cmd_Argv(1 as libc::c_int));
    }
    loop {
        path = FS_NextPath(path);
        if path.is_null() {
            break;
        }
        let mut tmp: *mut libc::c_char = findname.as_mut_ptr();
        Com_sprintf(
            findname.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                as libc::c_int,
            b"%s/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            path,
            wildcard.as_mut_ptr(),
        );
        while *tmp as libc::c_int != 0 as libc::c_int {
            if *tmp as libc::c_int == '\\' as i32 {
                *tmp = '/' as i32 as libc::c_char;
            }
            tmp = tmp.offset(1);
        }
        Com_Printf(
            b"Directory of %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            findname.as_mut_ptr(),
        );
        Com_Printf(b"----\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        dirnames = FS_ListFiles(
            findname.as_mut_ptr(),
            &mut ndirs,
            0 as libc::c_int as libc::c_uint,
            0 as libc::c_int as libc::c_uint,
        );
        if !dirnames.is_null() {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < ndirs - 1 as libc::c_int {
                if !(strrchr(*dirnames.offset(i as isize), '/' as i32)).is_null() {
                    Com_Printf(
                        b"%s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        (strrchr(*dirnames.offset(i as isize), '/' as i32))
                            .offset(1 as libc::c_int as isize),
                    );
                } else {
                    Com_Printf(
                        b"%s\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        *dirnames.offset(i as isize),
                    );
                }
                free(*dirnames.offset(i as isize) as *mut libc::c_void);
                i += 1;
            }
            free(dirnames as *mut libc::c_void);
        }
        Com_Printf(b"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn FS_Path_f() {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut l: *mut filelink_t = 0 as *mut filelink_t;
    Com_Printf(
        b"Current search path:\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    s = fs_searchpaths;
    while !s.is_null() {
        if s == fs_base_searchpaths {
            Com_Printf(
                b"----------\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        if !((*s).pack).is_null() {
            Com_Printf(
                b"%s (%i files)\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*s).pack).filename).as_mut_ptr(),
                (*(*s).pack).numfiles,
            );
        } else {
            Com_Printf(
                b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ((*s).filename).as_mut_ptr(),
            );
        }
        s = (*s).next;
    }
    Com_Printf(b"\nLinks:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    l = fs_links;
    while !l.is_null() {
        Com_Printf(
            b"%s : %s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*l).from,
            (*l).to,
        );
        l = (*l).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn FS_NextPath(
    mut prevpath: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut searchpath_t = 0 as *mut searchpath_t;
    let mut prev: *mut libc::c_char = 0 as *mut libc::c_char;
    if prevpath.is_null() {
        return fs_gamedir.as_mut_ptr();
    }
    prev = fs_gamedir.as_mut_ptr();
    s = fs_searchpaths;
    while !s.is_null() {
        if ((*s).pack).is_null() {
            if prevpath == prev {
                return ((*s).filename).as_mut_ptr();
            }
            prev = ((*s).filename).as_mut_ptr();
        }
        s = (*s).next;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn FS_InitFilesystem() {
    Cmd_AddCommand(
        b"path\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(FS_Path_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"link\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(FS_Link_f as unsafe extern "C" fn() -> ()),
    );
    Cmd_AddCommand(
        b"dir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        Some(FS_Dir_f as unsafe extern "C" fn() -> ()),
    );
    fs_basedir = Cvar_Get(
        b"basedir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b".\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    fs_cddir = Cvar_Get(
        b"cddir\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    if *((*fs_cddir).string).offset(0 as libc::c_int as isize) != 0 {
        FS_AddGameDirectory(
            va(
                b"%s/baseq2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*fs_cddir).string,
            ),
        );
    }
    FS_AddGameDirectory(
        va(
            b"%s/baseq2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*fs_basedir).string,
        ),
    );
    fs_base_searchpaths = fs_searchpaths;
    fs_gamedirvar = Cvar_Get(
        b"game\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        16 as libc::c_int | 4 as libc::c_int,
    );
    if *((*fs_gamedirvar).string).offset(0 as libc::c_int as isize) != 0 {
        FS_SetGamedir((*fs_gamedirvar).string);
    }
}
