#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    static mut paintedtime: libc::c_int;
    static mut dma: dma_t;
    static mut s_khz: *mut cvar_t;
    static mut s_primary: *mut cvar_t;
    fn SNDDMA_InitDirect() -> qboolean;
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
pub struct dma_t {
    pub channels: libc::c_int,
    pub samples: libc::c_int,
    pub submission_chunk: libc::c_int,
    pub samplepos: libc::c_int,
    pub samplebits: libc::c_int,
    pub speed: libc::c_int,
    pub buffer: *mut byte,
}
pub type sndinitstat = libc::c_uint;
pub const SIS_NOTAVAIL: sndinitstat = 2;
pub const SIS_FAILURE: sndinitstat = 1;
pub const SIS_SUCCESS: sndinitstat = 0;
#[no_mangle]
pub static mut s_wavonly: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut dsound_init: qboolean = false_0;
static mut wav_init: qboolean = false_0;
static mut snd_firsttime: qboolean = true_0;
static mut snd_isdirect: qboolean = false_0;
static mut snd_iswave: qboolean = false_0;
static mut primary_format_set: qboolean = false_0;
static mut snd_buffer_count: libc::c_int = 0 as libc::c_int;
static mut sample16: libc::c_int = 0;
static mut snd_sent: libc::c_int = 0;
static mut snd_completed: libc::c_int = 0;
#[no_mangle]
pub static mut hData: libc::c_int = 0;
#[no_mangle]
pub static mut lpData: libc::c_int = 0;
#[no_mangle]
pub static mut hWaveHdr: libc::c_int = 0;
#[no_mangle]
pub static mut lpWaveHdr: libc::c_int = 0;
#[no_mangle]
pub static mut hWaveOut: libc::c_int = 0;
#[no_mangle]
pub static mut wavecaps: libc::c_int = 0;
#[no_mangle]
pub static mut gSndBufSize: libc::c_int = 0;
#[no_mangle]
pub static mut mmstarttime: libc::c_int = 0;
#[no_mangle]
pub static mut pDS: libc::c_int = 0;
#[no_mangle]
pub static mut pDSBuf: libc::c_int = 0;
#[no_mangle]
pub static mut hInstDS: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Init() -> libc::c_int {
    let mut stat: sndinitstat = SIS_SUCCESS;
    memset(
        &mut dma as *mut dma_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<dma_t>() as libc::c_ulong,
    );
    s_wavonly = Cvar_Get(
        b"s_wavonly\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    wav_init = false_0;
    dsound_init = wav_init;
    stat = SIS_FAILURE;
    if (*s_wavonly).value == 0. {
        if snd_firsttime as libc::c_uint != 0 || snd_isdirect as libc::c_uint != 0 {
            stat = SNDDMA_InitDirect() as sndinitstat;
            if stat as libc::c_uint == SIS_SUCCESS as libc::c_int as libc::c_uint {
                snd_isdirect = true_0;
                if snd_firsttime as u64 != 0 {
                    Com_Printf(
                        b"dsound init succeeded\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
            } else {
                snd_isdirect = false_0;
                Com_Printf(
                    b"*** dsound init failed ***\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    if dsound_init as u64 == 0
        && stat as libc::c_uint != SIS_NOTAVAIL as libc::c_int as libc::c_uint
    {
        if snd_firsttime as libc::c_uint != 0 || snd_iswave as libc::c_uint != 0 {
            snd_iswave = SNDDMA_InitWav();
            if snd_iswave as u64 != 0 {
                if snd_firsttime as u64 != 0 {
                    Com_Printf(
                        b"Wave sound init succeeded\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
            } else {
                Com_Printf(
                    b"Wave sound init failed\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
    }
    snd_firsttime = false_0;
    snd_buffer_count = 1 as libc::c_int;
    if dsound_init as u64 == 0 && wav_init as u64 == 0 {
        if snd_firsttime as u64 != 0 {
            Com_Printf(
                b"*** No sound device initialized ***\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_GetDMAPos() -> libc::c_int {
    let mut s: libc::c_int = 0;
    if !(dsound_init as u64 != 0) {
        if wav_init as u64 != 0 {
            s = snd_sent * 0x400 as libc::c_int;
        }
    }
    s >>= sample16;
    s &= dma.samples - 1 as libc::c_int;
    return s;
}
#[no_mangle]
pub static mut locksize: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn SNDDMA_Shutdown() {
    FreeSound();
}
