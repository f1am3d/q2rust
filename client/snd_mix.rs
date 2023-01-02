#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]

extern "C" {
    fn sin(_: libc::c_double) -> libc::c_double;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut channels: [channel_t; 32];
    static mut paintedtime: libc::c_int;
    static mut s_rawend: libc::c_int;
    static mut dma: dma_t;
    static mut s_pendingplays: playsound_t;
    static mut s_rawsamples: [portable_samplepair_t; 8192];
    static mut s_volume: *mut cvar_t;
    static mut s_testsound: *mut cvar_t;
    fn S_LoadSound(s: *mut sfx_t) -> *mut sfxcache_t;
    fn S_IssuePlaysound(ps: *mut playsound_t);
}

pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;

pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;

pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct portable_samplepair_t {
    pub left: libc::c_int,
    pub right: libc::c_int,
}

pub type sfx_t = sfx_s;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct playsound_s {
    pub prev: *mut playsound_s,
    pub next: *mut playsound_s,
    pub sfx: *mut sfx_t,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub fixed_origin: qboolean,
    pub origin: vec3_t,
    pub begin: libc::c_uint,
}

pub type playsound_t = playsound_s;

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
pub struct channel_t {
    pub sfx: *mut sfx_t,
    pub leftvol: libc::c_int,
    pub rightvol: libc::c_int,
    pub end: libc::c_int,
    pub pos: libc::c_int,
    pub looping: libc::c_int,
    pub entnum: libc::c_int,
    pub entchannel: libc::c_int,
    pub origin: vec3_t,
    pub dist_mult: vec_t,
    pub master_vol: libc::c_int,
    pub fixed_origin: qboolean,
    pub autosound: qboolean,
}

#[no_mangle]
pub static mut paintbuffer: [portable_samplepair_t; 2048] = [portable_samplepair_t {
    left: 0,
    right: 0,
}; 2048];
#[no_mangle]
pub static mut snd_scaletable: [[libc::c_int; 256]; 32] = [[0; 256]; 32];
#[no_mangle]
pub static mut snd_p: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut snd_linear_count: libc::c_int = 0;
#[no_mangle]
pub static mut snd_vol: libc::c_int = 0;
#[no_mangle]
pub static mut snd_out: *mut libc::c_short = 0 as *const libc::c_short
    as *mut libc::c_short;

#[no_mangle]
pub unsafe extern "C" fn S_WriteLinearBlastStereo16() {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < snd_linear_count {
        val = *snd_p.offset(i as isize) >> 8 as libc::c_int;
        if val > 0x7fff as libc::c_int {
            *snd_out.offset(i as isize) = 0x7fff as libc::c_int as libc::c_short;
        } else if val < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            *snd_out.offset(i as isize) = 0x8000 as libc::c_int as libc::c_short;
        } else {
            *snd_out.offset(i as isize) = val as libc::c_short;
        }
        val = *snd_p.offset((i + 1 as libc::c_int) as isize) >> 8 as libc::c_int;
        if val > 0x7fff as libc::c_int {
            *snd_out
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = 0x7fff as libc::c_int as libc::c_short;
        } else if val < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
            *snd_out
                .offset(
                    (i + 1 as libc::c_int) as isize,
                ) = 0x8000 as libc::c_int as libc::c_short;
        } else {
            *snd_out.offset((i + 1 as libc::c_int) as isize) = val as libc::c_short;
        }
        i += 2 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_TransferStereo16(
    mut pbuf: *mut libc::c_ulong,
    mut endtime: libc::c_int,
) {
    let mut lpos: libc::c_int = 0;
    let mut lpaintedtime: libc::c_int = 0;
    snd_p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
    lpaintedtime = paintedtime;
    while lpaintedtime < endtime {
        lpos = lpaintedtime & (dma.samples >> 1 as libc::c_int) - 1 as libc::c_int;
        snd_out = (pbuf as *mut libc::c_short)
            .offset((lpos << 1 as libc::c_int) as isize);
        snd_linear_count = (dma.samples >> 1 as libc::c_int) - lpos;
        if lpaintedtime + snd_linear_count > endtime {
            snd_linear_count = endtime - lpaintedtime;
        }
        snd_linear_count <<= 1 as libc::c_int;
        S_WriteLinearBlastStereo16();
        snd_p = snd_p.offset(snd_linear_count as isize);
        lpaintedtime += snd_linear_count >> 1 as libc::c_int;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_TransferPaintBuffer(mut endtime: libc::c_int) {
    let mut out_idx: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut out_mask: libc::c_int = 0;
    let mut p: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut step: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut pbuf: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    pbuf = dma.buffer as *mut libc::c_ulong;
    if (*s_testsound).value != 0. {
        let mut i: libc::c_int = 0;
        let mut count_0: libc::c_int = 0;
        count_0 = endtime - paintedtime;
        i = 0 as libc::c_int;
        while i < count_0 {
            paintbuffer[i as usize]
                .right = (sin((paintedtime + i) as libc::c_double * 0.1f64)
                * 20000 as libc::c_int as libc::c_double
                * 256 as libc::c_int as libc::c_double) as libc::c_int;
            paintbuffer[i as usize].left = paintbuffer[i as usize].right;
            i += 1;
        }
    }
    if dma.samplebits == 16 as libc::c_int && dma.channels == 2 as libc::c_int {
        S_TransferStereo16(pbuf, endtime);
    } else {
        p = paintbuffer.as_mut_ptr() as *mut libc::c_int;
        count = (endtime - paintedtime) * dma.channels;
        out_mask = dma.samples - 1 as libc::c_int;
        out_idx = paintedtime * dma.channels & out_mask;
        step = 3 as libc::c_int - dma.channels;
        if dma.samplebits == 16 as libc::c_int {
            let mut out: *mut libc::c_short = pbuf as *mut libc::c_short;
            loop {
                let fresh0 = count;
                count = count - 1;
                if !(fresh0 != 0) {
                    break;
                }
                val = *p >> 8 as libc::c_int;
                p = p.offset(step as isize);
                if val > 0x7fff as libc::c_int {
                    val = 0x7fff as libc::c_int;
                } else if val < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                    val = 0x8000 as libc::c_int as libc::c_short as libc::c_int;
                }
                *out.offset(out_idx as isize) = val as libc::c_short;
                out_idx = out_idx + 1 as libc::c_int & out_mask;
            }
        } else if dma.samplebits == 8 as libc::c_int {
            let mut out_0: *mut libc::c_uchar = pbuf as *mut libc::c_uchar;
            loop {
                let fresh1 = count;
                count = count - 1;
                if !(fresh1 != 0) {
                    break;
                }
                val = *p >> 8 as libc::c_int;
                p = p.offset(step as isize);
                if val > 0x7fff as libc::c_int {
                    val = 0x7fff as libc::c_int;
                } else if val < 0x8000 as libc::c_int as libc::c_short as libc::c_int {
                    val = 0x8000 as libc::c_int as libc::c_short as libc::c_int;
                }
                *out_0
                    .offset(
                        out_idx as isize,
                    ) = ((val >> 8 as libc::c_int) + 128 as libc::c_int)
                    as libc::c_uchar;
                out_idx = out_idx + 1 as libc::c_int & out_mask;
            }
        }
    };
}

#[no_mangle]
pub unsafe extern "C" fn S_PaintChannels(mut endtime: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut end: libc::c_int = 0;
    let mut ch: *mut channel_t = 0 as *mut channel_t;
    let mut sc: *mut sfxcache_t = 0 as *mut sfxcache_t;
    let mut ltime: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut ps: *mut playsound_t = 0 as *mut playsound_t;
    snd_vol = ((*s_volume).value * 256 as libc::c_int as libc::c_float) as libc::c_int;
    while paintedtime < endtime {
        end = endtime;
        if endtime - paintedtime > 2048 as libc::c_int {
            end = paintedtime + 2048 as libc::c_int;
        }
        loop {
            ps = s_pendingplays.next;
            if ps == &mut s_pendingplays as *mut playsound_t {
                break;
            }
            if (*ps).begin <= paintedtime as libc::c_uint {
                S_IssuePlaysound(ps);
            } else {
                if (*ps).begin < end as libc::c_uint {
                    end = (*ps).begin as libc::c_int;
                }
                break;
            }
        }
        if s_rawend < paintedtime {
            memset(
                paintbuffer.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ((end - paintedtime) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<portable_samplepair_t>() as libc::c_ulong,
                    ),
            );
        } else {
            let mut s: libc::c_int = 0;
            let mut stop: libc::c_int = 0;
            stop = if end < s_rawend { end } else { s_rawend };
            i = paintedtime;
            while i < stop {
                s = i & 8192 as libc::c_int - 1 as libc::c_int;
                paintbuffer[(i - paintedtime) as usize] = s_rawsamples[s as usize];
                i += 1;
            }
            while i < end {
                paintbuffer[(i - paintedtime) as usize].right = 0 as libc::c_int;
                paintbuffer[(i - paintedtime) as usize]
                    .left = paintbuffer[(i - paintedtime) as usize].right;
                i += 1;
            }
        }
        ch = channels.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 32 as libc::c_int {
            ltime = paintedtime;
            while ltime < end {
                if ((*ch).sfx).is_null() || (*ch).leftvol == 0 && (*ch).rightvol == 0 {
                    break;
                }
                count = end - ltime;
                if (*ch).end - ltime < count {
                    count = (*ch).end - ltime;
                }
                sc = S_LoadSound((*ch).sfx);
                if sc.is_null() {
                    break;
                }
                if count > 0 as libc::c_int && !((*ch).sfx).is_null() {
                    if (*sc).width == 1 as libc::c_int {
                        S_PaintChannelFrom8(ch, sc, count, ltime - paintedtime);
                    } else {
                        S_PaintChannelFrom16(ch, sc, count, ltime - paintedtime);
                    }
                    ltime += count;
                }
                if ltime >= (*ch).end {
                    if (*ch).autosound as u64 != 0 {
                        (*ch).pos = 0 as libc::c_int;
                        (*ch).end = ltime + (*sc).length;
                    } else if (*sc).loopstart >= 0 as libc::c_int {
                        (*ch).pos = (*sc).loopstart;
                        (*ch).end = ltime + (*sc).length - (*ch).pos;
                    } else {
                        let ref mut fresh2 = (*ch).sfx;
                        *fresh2 = 0 as *mut sfx_t;
                    }
                }
            }
            i += 1;
            ch = ch.offset(1);
        }
        S_TransferPaintBuffer(end);
        paintedtime = end;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_InitScaletable() {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut scale: libc::c_int = 0;
    (*s_volume).modified = false_0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        scale = ((i * 8 as libc::c_int * 256 as libc::c_int) as libc::c_float
            * (*s_volume).value) as libc::c_int;
        j = 0 as libc::c_int;
        while j < 256 as libc::c_int {
            snd_scaletable[i
                as usize][j as usize] = j as libc::c_schar as libc::c_int * scale;
            j += 1;
        }
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn S_PaintChannelFrom8(
    mut ch: *mut channel_t,
    mut sc: *mut sfxcache_t,
    mut count: libc::c_int,
    mut offset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut lscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut rscale: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sfx: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut i: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t = 0 as *mut portable_samplepair_t;
    if (*ch).leftvol > 255 as libc::c_int {
        (*ch).leftvol = 255 as libc::c_int;
    }
    if (*ch).rightvol > 255 as libc::c_int {
        (*ch).rightvol = 255 as libc::c_int;
    }
    lscale = (snd_scaletable[((*ch).leftvol >> 11 as libc::c_int) as usize])
        .as_mut_ptr();
    rscale = (snd_scaletable[((*ch).rightvol >> 11 as libc::c_int) as usize])
        .as_mut_ptr();
    sfx = (((*sc).data).as_mut_ptr() as *mut libc::c_schar).offset((*ch).pos as isize)
        as *mut libc::c_uchar;
    samp = &mut *paintbuffer.as_mut_ptr().offset(offset as isize)
        as *mut portable_samplepair_t;
    i = 0 as libc::c_int;
    while i < count {
        data = *sfx.offset(i as isize) as libc::c_int;
        (*samp).left += *lscale.offset(data as isize);
        (*samp).right += *rscale.offset(data as isize);
        i += 1;
        samp = samp.offset(1);
    }
    (*ch).pos += count;
}

#[no_mangle]
pub unsafe extern "C" fn S_PaintChannelFrom16(
    mut ch: *mut channel_t,
    mut sc: *mut sfxcache_t,
    mut count: libc::c_int,
    mut offset: libc::c_int,
) {
    let mut data: libc::c_int = 0;
    let mut left: libc::c_int = 0;
    let mut right: libc::c_int = 0;
    let mut leftvol: libc::c_int = 0;
    let mut rightvol: libc::c_int = 0;
    let mut sfx: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut i: libc::c_int = 0;
    let mut samp: *mut portable_samplepair_t = 0 as *mut portable_samplepair_t;
    leftvol = (*ch).leftvol * snd_vol;
    rightvol = (*ch).rightvol * snd_vol;
    sfx = (((*sc).data).as_mut_ptr() as *mut libc::c_short).offset((*ch).pos as isize);
    samp = &mut *paintbuffer.as_mut_ptr().offset(offset as isize)
        as *mut portable_samplepair_t;
    i = 0 as libc::c_int;
    while i < count {
        data = *sfx.offset(i as isize) as libc::c_int;
        left = data * leftvol >> 8 as libc::c_int;
        right = data * rightvol >> 8 as libc::c_int;
        (*samp).left += left;
        (*samp).right += right;
        i += 1;
        samp = samp.offset(1);
    }
    (*ch).pos += count;
}
