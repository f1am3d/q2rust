#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Z_Malloc(size: libc::c_int) -> *mut libc::c_void;
    static mut dma: dma_t;
    static mut s_loadas8bit: *mut cvar_t;
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfx_s {
    pub name: [libc::c_char; 64],
    pub registration_sequence: libc::c_int,
    pub cache: *mut sfxcache_t,
    pub truename: *mut libc::c_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sfxcache_t {
    pub length: libc::c_int,
    pub loopstart: libc::c_int,
    pub speed: libc::c_int,
    pub width: libc::c_int,
    pub stereo: libc::c_int,
    pub data: [byte; 1],
}

pub type sfx_t = sfx_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dma_t {
    pub channels: libc::c_int,
    pub samples: libc::c_int,
    pub submission_chunk: libc::c_int,
    pub samplepos: libc::c_int,
    pub samplebits: libc::c_int,
    pub speed: libc::c_int,
    pub buffer: *mut byte,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct wavinfo_t {
    pub rate: libc::c_int,
    pub width: libc::c_int,
    pub channels: libc::c_int,
    pub loopstart: libc::c_int,
    pub samples: libc::c_int,
    pub dataofs: libc::c_int,
}

#[no_mangle]
pub static mut cache_full_cycle: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn ResampleSfx(
    mut sfx: *mut sfx_t,
    mut inrate: libc::c_int,
    mut inwidth: libc::c_int,
    mut data: *mut byte,
) {
    let mut outcount: libc::c_int = 0;
    let mut srcsample: libc::c_int = 0;
    let mut stepscale: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut sample: libc::c_int = 0;
    let mut samplefrac: libc::c_int = 0;
    let mut fracstep: libc::c_int = 0;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    sc = (*sfx).cache;
    if sc.is_null() {
        return;
    }
    stepscale = inrate as libc::c_float / dma.speed as libc::c_float;
    outcount = ((*sc).length as libc::c_float / stepscale) as libc::c_int;
    (*sc).length = outcount;
    if (*sc).loopstart != -(1 as libc::c_int) {
        (*sc).loopstart = ((*sc).loopstart as libc::c_float / stepscale) as libc::c_int;
    }
    (*sc).speed = dma.speed;
    if (*s_loadas8bit).value != 0. {
        (*sc).width = 1 as libc::c_int;
    } else {
        (*sc).width = inwidth;
    }
    (*sc).stereo = 0 as libc::c_int;
    if stepscale == 1 as libc::c_int as libc::c_float && inwidth == 1 as libc::c_int
        && (*sc).width == 1 as libc::c_int
    {
        i = 0 as libc::c_int;
        while i < outcount {
            *(((*sc).data).as_mut_ptr() as *mut libc::c_schar)
                .offset(
                    i as isize,
                ) = (*data.offset(i as isize) as libc::c_int - 128 as libc::c_int)
                as libc::c_schar;
            i += 1;
        }
    } else {
        samplefrac = 0 as libc::c_int;
        fracstep = (stepscale * 256 as libc::c_int as libc::c_float) as libc::c_int;
        i = 0 as libc::c_int;
        while i < outcount {
            srcsample = samplefrac >> 8 as libc::c_int;
            samplefrac += fracstep;
            if inwidth == 2 as libc::c_int {
                sample = LittleShort(
                    *(data as *mut libc::c_short).offset(srcsample as isize),
                ) as libc::c_int;
            } else {
                sample = (*data.offset(srcsample as isize) as libc::c_int
                    - 128 as libc::c_int) << 8 as libc::c_int;
            }
            if (*sc).width == 2 as libc::c_int {
                *(((*sc).data).as_mut_ptr() as *mut libc::c_short)
                    .offset(i as isize) = sample as libc::c_short;
            } else {
                *(((*sc).data).as_mut_ptr() as *mut libc::c_schar)
                    .offset(i as isize) = (sample >> 8 as libc::c_int) as libc::c_schar;
            }
            i += 1;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn S_LoadSound(mut s: *mut sfx_t) -> *mut sfxcache_t {
    let mut namebuffer: [libc::c_char; 64] = [0; 64];
    let mut data: *mut byte = 0 as *mut byte;
    let mut info: wavinfo_t = wavinfo_t {
        rate: 0,
        width: 0,
        channels: 0,
        loopstart: 0,
        samples: 0,
        dataofs: 0,
    };
    let mut len: libc::c_int = 0;
    let mut stepscale: libc::c_float = 0.;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    let mut size: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*s).name[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
        return 0 as *mut sfxcache_t;
    }
    sc = (*s).cache;
    if !sc.is_null() {
        return sc;
    }
    if !((*s).truename).is_null() {
        name = (*s).truename;
    } else {
        name = ((*s).name).as_mut_ptr();
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32 {
        strcpy(namebuffer.as_mut_ptr(), &mut *name.offset(1 as libc::c_int as isize));
    } else {
        Com_sprintf(
            namebuffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"sound/%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
        );
    }
    size = FS_LoadFile(
        namebuffer.as_mut_ptr(),
        &mut data as *mut *mut byte as *mut *mut libc::c_void,
    );
    if data.is_null() {
        Com_DPrintf(
            b"Couldn't load %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            namebuffer.as_mut_ptr(),
        );
        return 0 as *mut sfxcache_t;
    }
    info = GetWavinfo(((*s).name).as_mut_ptr(), data, size);
    if info.channels != 1 as libc::c_int {
        Com_Printf(
            b"%s is a stereo sample\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*s).name).as_mut_ptr(),
        );
        FS_FreeFile(data as *mut libc::c_void);
        return 0 as *mut sfxcache_t;
    }
    stepscale = info.rate as libc::c_float / dma.speed as libc::c_float;
    len = (info.samples as libc::c_float / stepscale) as libc::c_int;
    len = len * info.width * info.channels;
    let ref mut fresh0 = (*s).cache;
    *fresh0 = Z_Malloc(
        (len as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<sfxcache_t>() as libc::c_ulong)
            as libc::c_int,
    ) as *mut sfxcache_t;
    sc = *fresh0;
    if sc.is_null() {
        FS_FreeFile(data as *mut libc::c_void);
        return 0 as *mut sfxcache_t;
    }
    (*sc).length = info.samples;
    (*sc).loopstart = info.loopstart;
    (*sc).speed = info.rate;
    (*sc).width = info.width;
    (*sc).stereo = info.channels;
    ResampleSfx(s, (*sc).speed, (*sc).width, data.offset(info.dataofs as isize));
    FS_FreeFile(data as *mut libc::c_void);
    return sc;
}

#[no_mangle]
pub static mut data_p: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut iff_end: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut last_chunk: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut iff_data: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub static mut iff_chunk_len: libc::c_int = 0;

#[no_mangle]
pub unsafe extern "C" fn GetLittleShort() -> libc::c_short {
    let mut val: libc::c_short = 0 as libc::c_int as libc::c_short;
    val = *data_p as libc::c_short;
    val = (val as libc::c_int
        + ((*data_p.offset(1 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int)) as libc::c_short;
    data_p = data_p.offset(2 as libc::c_int as isize);
    return val;
}

#[no_mangle]
pub unsafe extern "C" fn GetLittleLong() -> libc::c_int {
    let mut val: libc::c_int = 0 as libc::c_int;
    val = *data_p as libc::c_int;
    val = val
        + ((*data_p.offset(1 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int);
    val = val
        + ((*data_p.offset(2 as libc::c_int as isize) as libc::c_int)
        << 16 as libc::c_int);
    val = val
        + ((*data_p.offset(3 as libc::c_int as isize) as libc::c_int)
        << 24 as libc::c_int);
    data_p = data_p.offset(4 as libc::c_int as isize);
    return val;
}

#[no_mangle]
pub unsafe extern "C" fn FindNextChunk(mut name: *mut libc::c_char) {
    loop {
        data_p = last_chunk;
        if data_p >= iff_end {
            data_p = 0 as *mut byte;
            return;
        }
        data_p = data_p.offset(4 as libc::c_int as isize);
        iff_chunk_len = GetLittleLong();
        if iff_chunk_len < 0 as libc::c_int {
            data_p = 0 as *mut byte;
            return;
        }
        data_p = data_p.offset(-(8 as libc::c_int as isize));
        last_chunk = data_p
            .offset(8 as libc::c_int as isize)
            .offset((iff_chunk_len + 1 as libc::c_int & !(1 as libc::c_int)) as isize);
        if strncmp(
            data_p as *const libc::c_char,
            name,
            4 as libc::c_int as libc::c_ulong,
        ) == 0
        {
            return;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn FindChunk(mut name: *mut libc::c_char) {
    last_chunk = iff_data;
    FindNextChunk(name);
}

#[no_mangle]
pub unsafe extern "C" fn DumpChunks() {
    let mut str: [libc::c_char; 5] = [0; 5];
    str[4 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    data_p = iff_data;
    loop {
        memcpy(
            str.as_mut_ptr() as *mut libc::c_void,
            data_p as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        data_p = data_p.offset(4 as libc::c_int as isize);
        iff_chunk_len = GetLittleLong();
        Com_Printf(
            b"0x%x : %s (%d)\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            data_p.offset(-(4 as libc::c_int as isize)) as libc::c_int,
            str.as_mut_ptr(),
            iff_chunk_len,
        );
        data_p = data_p
            .offset((iff_chunk_len + 1 as libc::c_int & !(1 as libc::c_int)) as isize);
        if !(data_p < iff_end) {
            break;
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn GetWavinfo(
    mut name: *mut libc::c_char,
    mut wav: *mut byte,
    mut wavlength: libc::c_int,
) -> wavinfo_t {
    let mut info: wavinfo_t = wavinfo_t {
        rate: 0,
        width: 0,
        channels: 0,
        loopstart: 0,
        samples: 0,
        dataofs: 0,
    };
    let mut i: libc::c_int = 0;
    let mut format: libc::c_int = 0;
    let mut samples: libc::c_int = 0;
    memset(
        &mut info as *mut wavinfo_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<wavinfo_t>() as libc::c_ulong,
    );
    if wav.is_null() {
        return info;
    }
    iff_data = wav;
    iff_end = wav.offset(wavlength as isize);
    FindChunk(b"RIFF\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !(!data_p.is_null()
        && strncmp(
        data_p.offset(8 as libc::c_int as isize) as *const libc::c_char,
        b"WAVE\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0)
    {
        Com_Printf(
            b"Missing RIFF/WAVE chunks\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return info;
    }
    iff_data = data_p.offset(12 as libc::c_int as isize);
    FindChunk(b"fmt \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if data_p.is_null() {
        Com_Printf(
            b"Missing fmt chunk\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return info;
    }
    data_p = data_p.offset(8 as libc::c_int as isize);
    format = GetLittleShort() as libc::c_int;
    if format != 1 as libc::c_int {
        Com_Printf(
            b"Microsoft PCM format only\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return info;
    }
    info.channels = GetLittleShort() as libc::c_int;
    info.rate = GetLittleLong();
    data_p = data_p.offset((4 as libc::c_int + 2 as libc::c_int) as isize);
    info.width = GetLittleShort() as libc::c_int / 8 as libc::c_int;
    FindChunk(b"cue \0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if !data_p.is_null() {
        data_p = data_p.offset(32 as libc::c_int as isize);
        info.loopstart = GetLittleLong();
        FindNextChunk(
            b"LIST\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !data_p.is_null() {
            if strncmp(
                data_p.offset(28 as libc::c_int as isize) as *const libc::c_char,
                b"mark\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                data_p = data_p.offset(24 as libc::c_int as isize);
                i = GetLittleLong();
                info.samples = info.loopstart + i;
            }
        }
    } else {
        info.loopstart = -(1 as libc::c_int);
    }
    FindChunk(b"data\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if data_p.is_null() {
        Com_Printf(
            b"Missing data chunk\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return info;
    }
    data_p = data_p.offset(4 as libc::c_int as isize);
    samples = GetLittleLong() / info.width;
    if info.samples != 0 {
        if samples < info.samples {
            Com_Error(
                1 as libc::c_int,
                b"Sound %s has a bad loop length\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                name,
            );
        }
    } else {
        info.samples = samples;
    }
    info.dataofs = data_p.offset_from(wav) as libc::c_long as libc::c_int;
    return info;
}
