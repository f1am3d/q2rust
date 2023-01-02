#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type edict_s;
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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn BoxOnPlaneSide(
        emins: *mut vec_t,
        emaxs: *mut vec_t,
        plane: *mut cplane_s,
    ) -> libc::c_int;
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
    fn LittleFloat(l: libc::c_float) -> libc::c_float;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn FS_FreeFile(buffer: *mut libc::c_void);
    fn FS_Read(buffer: *mut libc::c_void, len: libc::c_int, f: *mut FILE);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_BlockChecksum(buffer: *mut libc::c_void, length: libc::c_int) -> libc::c_uint;
    fn FS_LoadFile(
        path: *mut libc::c_char,
        buffer: *mut *mut libc::c_void,
    ) -> libc::c_int;
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
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
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
pub type cplane_t = cplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmodel_s {
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub origin: vec3_t,
    pub headnode: libc::c_int,
}
pub type cmodel_t = cmodel_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csurface_s {
    pub name: [libc::c_char; 16],
    pub flags: libc::c_int,
    pub value: libc::c_int,
}
pub type csurface_t = csurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mapsurface_s {
    pub c: csurface_t,
    pub rname: [libc::c_char; 32],
}
pub type mapsurface_t = mapsurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace_t {
    pub allsolid: qboolean,
    pub startsolid: qboolean,
    pub fraction: libc::c_float,
    pub endpos: vec3_t,
    pub plane: cplane_t,
    pub surface: *mut csurface_t,
    pub contents: libc::c_int,
    pub ent: *mut edict_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lump_t {
    pub fileofs: libc::c_int,
    pub filelen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dheader_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub lumps: [lump_t; 19],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmodel_t {
    pub mins: [libc::c_float; 3],
    pub maxs: [libc::c_float; 3],
    pub origin: [libc::c_float; 3],
    pub headnode: libc::c_int,
    pub firstface: libc::c_int,
    pub numfaces: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dplane_t {
    pub normal: [libc::c_float; 3],
    pub dist: libc::c_float,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnode_t {
    pub planenum: libc::c_int,
    pub children: [libc::c_int; 2],
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstface: libc::c_ushort,
    pub numfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct texinfo_s {
    pub vecs: [[libc::c_float; 4]; 2],
    pub flags: libc::c_int,
    pub value: libc::c_int,
    pub texture: [libc::c_char; 32],
    pub nexttexinfo: libc::c_int,
}
pub type texinfo_t = texinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dleaf_t {
    pub contents: libc::c_int,
    pub cluster: libc::c_short,
    pub area: libc::c_short,
    pub mins: [libc::c_short; 3],
    pub maxs: [libc::c_short; 3],
    pub firstleafface: libc::c_ushort,
    pub numleaffaces: libc::c_ushort,
    pub firstleafbrush: libc::c_ushort,
    pub numleafbrushes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbrushside_t {
    pub planenum: libc::c_ushort,
    pub texinfo: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dbrush_t {
    pub firstside: libc::c_int,
    pub numsides: libc::c_int,
    pub contents: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dvis_t {
    pub numclusters: libc::c_int,
    pub bitofs: [[libc::c_int; 2]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dareaportal_t {
    pub portalnum: libc::c_int,
    pub otherarea: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct darea_t {
    pub numareaportals: libc::c_int,
    pub firstareaportal: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct carea_t {
    pub numareaportals: libc::c_int,
    pub firstareaportal: libc::c_int,
    pub floodnum: libc::c_int,
    pub floodvalid: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cnode_t {
    pub plane: *mut cplane_t,
    pub children: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbrushside_t {
    pub plane: *mut cplane_t,
    pub surface: *mut mapsurface_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cleaf_t {
    pub contents: libc::c_int,
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstleafbrush: libc::c_ushort,
    pub numleafbrushes: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbrush_t {
    pub contents: libc::c_int,
    pub numsides: libc::c_int,
    pub firstbrushside: libc::c_int,
    pub checkcount: libc::c_int,
}
#[no_mangle]
pub static mut checkcount: libc::c_int = 0;
#[no_mangle]
pub static mut map_name: [libc::c_char; 64] = [0; 64];
#[no_mangle]
pub static mut numbrushsides: libc::c_int = 0;
#[no_mangle]
pub static mut map_brushsides: [cbrushside_t; 65536] = [cbrushside_t {
    plane: 0 as *const cplane_t as *mut cplane_t,
    surface: 0 as *const mapsurface_t as *mut mapsurface_t,
}; 65536];
#[no_mangle]
pub static mut numtexinfo: libc::c_int = 0;
#[no_mangle]
pub static mut map_surfaces: [mapsurface_t; 8192] = [mapsurface_t {
    c: csurface_t {
        name: [0; 16],
        flags: 0,
        value: 0,
    },
    rname: [0; 32],
}; 8192];
#[no_mangle]
pub static mut numplanes: libc::c_int = 0;
#[no_mangle]
pub static mut map_planes: [cplane_t; 65542] = [cplane_t {
    normal: [0.; 3],
    dist: 0.,
    type_0: 0,
    signbits: 0,
    pad: [0; 2],
}; 65542];
#[no_mangle]
pub static mut numnodes: libc::c_int = 0;
#[no_mangle]
pub static mut map_nodes: [cnode_t; 65542] = [cnode_t {
    plane: 0 as *const cplane_t as *mut cplane_t,
    children: [0; 2],
}; 65542];
#[no_mangle]
pub static mut numleafs: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut map_leafs: [cleaf_t; 65536] = [cleaf_t {
    contents: 0,
    cluster: 0,
    area: 0,
    firstleafbrush: 0,
    numleafbrushes: 0,
}; 65536];
#[no_mangle]
pub static mut emptyleaf: libc::c_int = 0;
#[no_mangle]
pub static mut solidleaf: libc::c_int = 0;
#[no_mangle]
pub static mut numleafbrushes: libc::c_int = 0;
#[no_mangle]
pub static mut map_leafbrushes: [libc::c_ushort; 65536] = [0; 65536];
#[no_mangle]
pub static mut numcmodels: libc::c_int = 0;
#[no_mangle]
pub static mut map_cmodels: [cmodel_t; 1024] = [cmodel_t {
    mins: [0.; 3],
    maxs: [0.; 3],
    origin: [0.; 3],
    headnode: 0,
}; 1024];
#[no_mangle]
pub static mut numbrushes: libc::c_int = 0;
#[no_mangle]
pub static mut map_brushes: [cbrush_t; 8192] = [cbrush_t {
    contents: 0,
    numsides: 0,
    firstbrushside: 0,
    checkcount: 0,
}; 8192];
#[no_mangle]
pub static mut numvisibility: libc::c_int = 0;
#[no_mangle]
pub static mut map_visibility: [byte; 1048576] = [0; 1048576];
#[no_mangle]
pub static mut map_vis: *mut dvis_t = unsafe {
    map_visibility.as_ptr() as *mut _ as *mut dvis_t
};
#[no_mangle]
pub static mut numentitychars: libc::c_int = 0;
#[no_mangle]
pub static mut map_entitystring: [libc::c_char; 262144] = [0; 262144];
#[no_mangle]
pub static mut numareas: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut map_areas: [carea_t; 256] = [carea_t {
    numareaportals: 0,
    firstareaportal: 0,
    floodnum: 0,
    floodvalid: 0,
}; 256];
#[no_mangle]
pub static mut numareaportals: libc::c_int = 0;
#[no_mangle]
pub static mut map_areaportals: [dareaportal_t; 1024] = [dareaportal_t {
    portalnum: 0,
    otherarea: 0,
}; 1024];
#[no_mangle]
pub static mut numclusters: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub static mut nullsurface: mapsurface_t = mapsurface_t {
    c: csurface_t {
        name: [0; 16],
        flags: 0,
        value: 0,
    },
    rname: [0; 32],
};
#[no_mangle]
pub static mut floodvalid: libc::c_int = 0;
#[no_mangle]
pub static mut portalopen: [qboolean; 1024] = [false_0; 1024];
#[no_mangle]
pub static mut map_noareas: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut c_pointcontents: libc::c_int = 0;
#[no_mangle]
pub static mut c_traces: libc::c_int = 0;
#[no_mangle]
pub static mut c_brush_traces: libc::c_int = 0;
#[no_mangle]
pub static mut cmod_base: *mut byte = 0 as *const byte as *mut byte;
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadSubmodels(mut l: *mut lump_t) {
    let mut in_0: *mut dmodel_t = 0 as *mut dmodel_t;
    let mut out: *mut cmodel_t = 0 as *mut cmodel_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dmodel_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dmodel_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dmodel_t>() as libc::c_ulong) as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map with no models\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 1024 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many models\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    numcmodels = count;
    i = 0 as libc::c_int;
    while i < count {
        out = &mut *map_cmodels.as_mut_ptr().offset(i as isize) as *mut cmodel_t;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out)
                .mins[j
                as usize] = LittleFloat((*in_0).mins[j as usize])
                - 1 as libc::c_int as libc::c_float;
            (*out)
                .maxs[j
                as usize] = LittleFloat((*in_0).maxs[j as usize])
                + 1 as libc::c_int as libc::c_float;
            (*out).origin[j as usize] = LittleFloat((*in_0).origin[j as usize]);
            j += 1;
        }
        (*out).headnode = LittleLong((*in_0).headnode);
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadSurfaces(mut l: *mut lump_t) {
    let mut in_0: *mut texinfo_t = 0 as *mut texinfo_t;
    let mut out: *mut mapsurface_t = 0 as *mut mapsurface_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut texinfo_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<texinfo_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<texinfo_t>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map with no surfaces\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 8192 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many surfaces\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    numtexinfo = count;
    out = map_surfaces.as_mut_ptr();
    i = 0 as libc::c_int;
    while i < count {
        strncpy(
            ((*out).c.name).as_mut_ptr(),
            ((*in_0).texture).as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        strncpy(
            ((*out).rname).as_mut_ptr(),
            ((*in_0).texture).as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        (*out).c.flags = LittleLong((*in_0).flags);
        (*out).c.value = LittleLong((*in_0).value);
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadNodes(mut l: *mut lump_t) {
    let mut in_0: *mut dnode_t = 0 as *mut dnode_t;
    let mut child: libc::c_int = 0;
    let mut out: *mut cnode_t = 0 as *mut cnode_t;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dnode_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dnode_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dnode_t>() as libc::c_ulong) as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has no nodes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 65536 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many nodes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_nodes.as_mut_ptr();
    numnodes = count;
    i = 0 as libc::c_int;
    while i < count {
        let ref mut fresh0 = (*out).plane;
        *fresh0 = map_planes.as_mut_ptr().offset(LittleLong((*in_0).planenum) as isize);
        j = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            child = LittleLong((*in_0).children[j as usize]);
            (*out).children[j as usize] = child;
            j += 1;
        }
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadBrushes(mut l: *mut lump_t) {
    let mut in_0: *mut dbrush_t = 0 as *mut dbrush_t;
    let mut out: *mut cbrush_t = 0 as *mut cbrush_t;
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dbrush_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dbrush_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dbrush_t>() as libc::c_ulong) as libc::c_int;
    if count > 8192 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many brushes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_brushes.as_mut_ptr();
    numbrushes = count;
    i = 0 as libc::c_int;
    while i < count {
        (*out).firstbrushside = LittleLong((*in_0).firstside);
        (*out).numsides = LittleLong((*in_0).numsides);
        (*out).contents = LittleLong((*in_0).contents);
        i += 1;
        out = out.offset(1);
        in_0 = in_0.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadLeafs(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut cleaf_t = 0 as *mut cleaf_t;
    let mut in_0: *mut dleaf_t = 0 as *mut dleaf_t;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dleaf_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dleaf_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dleaf_t>() as libc::c_ulong) as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map with no leafs\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 65536 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many planes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_leafs.as_mut_ptr();
    numleafs = count;
    numclusters = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < count {
        (*out).contents = LittleLong((*in_0).contents);
        (*out).cluster = LittleShort((*in_0).cluster) as libc::c_int;
        (*out).area = LittleShort((*in_0).area) as libc::c_int;
        (*out)
            .firstleafbrush = LittleShort((*in_0).firstleafbrush as libc::c_short)
            as libc::c_ushort;
        (*out)
            .numleafbrushes = LittleShort((*in_0).numleafbrushes as libc::c_short)
            as libc::c_ushort;
        if (*out).cluster >= numclusters {
            numclusters = (*out).cluster + 1 as libc::c_int;
        }
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
    if map_leafs[0 as libc::c_int as usize].contents != 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map leaf 0 is not CONTENTS_SOLID\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    solidleaf = 0 as libc::c_int;
    emptyleaf = -(1 as libc::c_int);
    i = 1 as libc::c_int;
    while i < numleafs {
        if map_leafs[i as usize].contents == 0 {
            emptyleaf = i;
            break;
        } else {
            i += 1;
        }
    }
    if emptyleaf == -(1 as libc::c_int) {
        Com_Error(
            1 as libc::c_int,
            b"Map does not have an empty leaf\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadPlanes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut cplane_t = 0 as *mut cplane_t;
    let mut in_0: *mut dplane_t = 0 as *mut dplane_t;
    let mut count: libc::c_int = 0;
    let mut bits: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut dplane_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dplane_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dplane_t>() as libc::c_ulong) as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map with no planes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 65536 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many planes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_planes.as_mut_ptr();
    numplanes = count;
    i = 0 as libc::c_int;
    while i < count {
        bits = 0 as libc::c_int;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            (*out).normal[j as usize] = LittleFloat((*in_0).normal[j as usize]);
            if (*out).normal[j as usize] < 0 as libc::c_int as libc::c_float {
                bits |= (1 as libc::c_int) << j;
            }
            j += 1;
        }
        (*out).dist = LittleFloat((*in_0).dist);
        (*out).type_0 = LittleLong((*in_0).type_0) as byte;
        (*out).signbits = bits as byte;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadLeafBrushes(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut in_0: *mut libc::c_ushort = 0 as *mut libc::c_ushort;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut libc::c_ushort;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<libc::c_ushort>() as libc::c_ulong)
        as libc::c_int;
    if count < 1 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map with no planes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if count > 65536 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many leafbrushes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_leafbrushes.as_mut_ptr();
    numleafbrushes = count;
    i = 0 as libc::c_int;
    while i < count {
        *out = LittleShort(*in_0 as libc::c_short) as libc::c_ushort;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadBrushSides(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut out: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut in_0: *mut dbrushside_t = 0 as *mut dbrushside_t;
    let mut count: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut dbrushside_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dbrushside_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dbrushside_t>() as libc::c_ulong)
        as libc::c_int;
    if count > 65536 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many planes\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_brushsides.as_mut_ptr();
    numbrushsides = count;
    i = 0 as libc::c_int;
    while i < count {
        num = LittleShort((*in_0).planenum as libc::c_short) as libc::c_int;
        let ref mut fresh1 = (*out).plane;
        *fresh1 = &mut *map_planes.as_mut_ptr().offset(num as isize) as *mut cplane_t;
        j = LittleShort((*in_0).texinfo) as libc::c_int;
        if j >= numtexinfo {
            Com_Error(
                1 as libc::c_int,
                b"Bad brushside texinfo\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        let ref mut fresh2 = (*out).surface;
        *fresh2 = &mut *map_surfaces.as_mut_ptr().offset(j as isize)
            as *mut mapsurface_t;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadAreas(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut carea_t = 0 as *mut carea_t;
    let mut in_0: *mut darea_t = 0 as *mut darea_t;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void as *mut darea_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<darea_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<darea_t>() as libc::c_ulong) as libc::c_int;
    if count > 256 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many areas\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_areas.as_mut_ptr();
    numareas = count;
    i = 0 as libc::c_int;
    while i < count {
        (*out).numareaportals = LittleLong((*in_0).numareaportals);
        (*out).firstareaportal = LittleLong((*in_0).firstareaportal);
        (*out).floodvalid = 0 as libc::c_int;
        (*out).floodnum = 0 as libc::c_int;
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadAreaPortals(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    let mut out: *mut dareaportal_t = 0 as *mut dareaportal_t;
    let mut in_0: *mut dareaportal_t = 0 as *mut dareaportal_t;
    let mut count: libc::c_int = 0;
    in_0 = cmod_base.offset((*l).fileofs as isize) as *mut libc::c_void
        as *mut dareaportal_t;
    if ((*l).filelen as libc::c_ulong)
        .wrapping_rem(::std::mem::size_of::<dareaportal_t>() as libc::c_ulong) != 0
    {
        Com_Error(
            1 as libc::c_int,
            b"MOD_LoadBmodel: funny lump size\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    count = ((*l).filelen as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<dareaportal_t>() as libc::c_ulong)
        as libc::c_int;
    if count > 256 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too many areas\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    out = map_areaportals.as_mut_ptr();
    numareaportals = count;
    i = 0 as libc::c_int;
    while i < count {
        (*out).portalnum = LittleLong((*in_0).portalnum);
        (*out).otherarea = LittleLong((*in_0).otherarea);
        i += 1;
        in_0 = in_0.offset(1);
        out = out.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadVisibility(mut l: *mut lump_t) {
    let mut i: libc::c_int = 0;
    numvisibility = (*l).filelen;
    if (*l).filelen > 0x100000 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too large visibility lump\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memcpy(
        map_visibility.as_mut_ptr() as *mut libc::c_void,
        cmod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
    (*map_vis).numclusters = LittleLong((*map_vis).numclusters);
    i = 0 as libc::c_int;
    while i < (*map_vis).numclusters {
        (*map_vis)
            .bitofs[i
            as usize][0 as libc::c_int
            as usize] = LittleLong(
            (*map_vis).bitofs[i as usize][0 as libc::c_int as usize],
        );
        (*map_vis)
            .bitofs[i
            as usize][1 as libc::c_int
            as usize] = LittleLong(
            (*map_vis).bitofs[i as usize][1 as libc::c_int as usize],
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CMod_LoadEntityString(mut l: *mut lump_t) {
    numentitychars = (*l).filelen;
    if (*l).filelen > 0x40000 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"Map has too large entity lump\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    memcpy(
        map_entitystring.as_mut_ptr() as *mut libc::c_void,
        cmod_base.offset((*l).fileofs as isize) as *const libc::c_void,
        (*l).filelen as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CM_LoadMap(
    mut name: *mut libc::c_char,
    mut clientload: qboolean,
    mut checksum: *mut libc::c_uint,
) -> *mut cmodel_t {
    let mut buf: *mut libc::c_uint = 0 as *mut libc::c_uint;
    let mut i: libc::c_int = 0;
    let mut header: dheader_t = dheader_t {
        ident: 0,
        version: 0,
        lumps: [lump_t { fileofs: 0, filelen: 0 }; 19],
    };
    let mut length: libc::c_int = 0;
    static mut last_checksum: libc::c_uint = 0;
    map_noareas = Cvar_Get(
        b"map_noareas\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    if strcmp(map_name.as_mut_ptr(), name) == 0
        && (clientload as libc::c_uint != 0
            || Cvar_VariableValue(
                b"flushmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) == 0.)
    {
        *checksum = last_checksum;
        if clientload as u64 == 0 {
            memset(
                portalopen.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<[qboolean; 1024]>() as libc::c_ulong,
            );
            FloodAreaConnections();
        }
        return &mut *map_cmodels.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut cmodel_t;
    }
    numplanes = 0 as libc::c_int;
    numnodes = 0 as libc::c_int;
    numleafs = 0 as libc::c_int;
    numcmodels = 0 as libc::c_int;
    numvisibility = 0 as libc::c_int;
    numentitychars = 0 as libc::c_int;
    map_entitystring[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    map_name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if name.is_null() || *name.offset(0 as libc::c_int as isize) == 0 {
        numleafs = 1 as libc::c_int;
        numclusters = 1 as libc::c_int;
        numareas = 1 as libc::c_int;
        *checksum = 0 as libc::c_int as libc::c_uint;
        return &mut *map_cmodels.as_mut_ptr().offset(0 as libc::c_int as isize)
            as *mut cmodel_t;
    }
    length = FS_LoadFile(
        name,
        &mut buf as *mut *mut libc::c_uint as *mut *mut libc::c_void,
    );
    if buf.is_null() {
        Com_Error(
            1 as libc::c_int,
            b"Couldn't load %s\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    last_checksum = LittleLong(
        Com_BlockChecksum(buf as *mut libc::c_void, length) as libc::c_int,
    ) as libc::c_uint;
    *checksum = last_checksum;
    header = *(buf as *mut dheader_t);
    i = 0 as libc::c_int;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<dheader_t>() as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong)
    {
        *(&mut header as *mut dheader_t as *mut libc::c_int)
            .offset(
                i as isize,
            ) = LittleLong(
            *(&mut header as *mut dheader_t as *mut libc::c_int).offset(i as isize),
        );
        i += 1;
    }
    if header.version != 38 as libc::c_int {
        Com_Error(
            1 as libc::c_int,
            b"CMod_LoadBrushModel: %s has wrong version number (%i should be %i)\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
            name,
            header.version,
            38 as libc::c_int,
        );
    }
    cmod_base = buf as *mut byte;
    CMod_LoadSurfaces(
        &mut *(header.lumps).as_mut_ptr().offset(5 as libc::c_int as isize),
    );
    CMod_LoadLeafs(&mut *(header.lumps).as_mut_ptr().offset(8 as libc::c_int as isize));
    CMod_LoadLeafBrushes(
        &mut *(header.lumps).as_mut_ptr().offset(10 as libc::c_int as isize),
    );
    CMod_LoadPlanes(&mut *(header.lumps).as_mut_ptr().offset(1 as libc::c_int as isize));
    CMod_LoadBrushes(
        &mut *(header.lumps).as_mut_ptr().offset(14 as libc::c_int as isize),
    );
    CMod_LoadBrushSides(
        &mut *(header.lumps).as_mut_ptr().offset(15 as libc::c_int as isize),
    );
    CMod_LoadSubmodels(
        &mut *(header.lumps).as_mut_ptr().offset(13 as libc::c_int as isize),
    );
    CMod_LoadNodes(&mut *(header.lumps).as_mut_ptr().offset(4 as libc::c_int as isize));
    CMod_LoadAreas(&mut *(header.lumps).as_mut_ptr().offset(17 as libc::c_int as isize));
    CMod_LoadAreaPortals(
        &mut *(header.lumps).as_mut_ptr().offset(18 as libc::c_int as isize),
    );
    CMod_LoadVisibility(
        &mut *(header.lumps).as_mut_ptr().offset(3 as libc::c_int as isize),
    );
    CMod_LoadEntityString(
        &mut *(header.lumps).as_mut_ptr().offset(0 as libc::c_int as isize),
    );
    FS_FreeFile(buf as *mut libc::c_void);
    CM_InitBoxHull();
    memset(
        portalopen.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[qboolean; 1024]>() as libc::c_ulong,
    );
    FloodAreaConnections();
    strcpy(map_name.as_mut_ptr(), name);
    return &mut *map_cmodels.as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut cmodel_t;
}
#[no_mangle]
pub unsafe extern "C" fn CM_InlineModel(mut name: *mut libc::c_char) -> *mut cmodel_t {
    let mut num: libc::c_int = 0;
    if name.is_null()
        || *name.offset(0 as libc::c_int as isize) as libc::c_int != '*' as i32
    {
        Com_Error(
            1 as libc::c_int,
            b"CM_InlineModel: bad name\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    num = atoi(name.offset(1 as libc::c_int as isize));
    if num < 1 as libc::c_int || num >= numcmodels {
        Com_Error(
            1 as libc::c_int,
            b"CM_InlineModel: bad number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return &mut *map_cmodels.as_mut_ptr().offset(num as isize) as *mut cmodel_t;
}
#[no_mangle]
pub unsafe extern "C" fn CM_NumClusters() -> libc::c_int {
    return numclusters;
}
#[no_mangle]
pub unsafe extern "C" fn CM_NumInlineModels() -> libc::c_int {
    return numcmodels;
}
#[no_mangle]
pub unsafe extern "C" fn CM_EntityString() -> *mut libc::c_char {
    return map_entitystring.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CM_LeafContents(mut leafnum: libc::c_int) -> libc::c_int {
    if leafnum < 0 as libc::c_int || leafnum >= numleafs {
        Com_Error(
            1 as libc::c_int,
            b"CM_LeafContents: bad number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return map_leafs[leafnum as usize].contents;
}
#[no_mangle]
pub unsafe extern "C" fn CM_LeafCluster(mut leafnum: libc::c_int) -> libc::c_int {
    if leafnum < 0 as libc::c_int || leafnum >= numleafs {
        Com_Error(
            1 as libc::c_int,
            b"CM_LeafCluster: bad number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return map_leafs[leafnum as usize].cluster;
}
#[no_mangle]
pub unsafe extern "C" fn CM_LeafArea(mut leafnum: libc::c_int) -> libc::c_int {
    if leafnum < 0 as libc::c_int || leafnum >= numleafs {
        Com_Error(
            1 as libc::c_int,
            b"CM_LeafArea: bad number\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return map_leafs[leafnum as usize].area;
}
#[no_mangle]
pub static mut box_planes: *mut cplane_t = 0 as *const cplane_t as *mut cplane_t;
#[no_mangle]
pub static mut box_headnode: libc::c_int = 0;
#[no_mangle]
pub static mut box_brush: *mut cbrush_t = 0 as *const cbrush_t as *mut cbrush_t;
#[no_mangle]
pub static mut box_leaf: *mut cleaf_t = 0 as *const cleaf_t as *mut cleaf_t;
#[no_mangle]
pub unsafe extern "C" fn CM_InitBoxHull() {
    let mut i: libc::c_int = 0;
    let mut side: libc::c_int = 0;
    let mut c: *mut cnode_t = 0 as *mut cnode_t;
    let mut p: *mut cplane_t = 0 as *mut cplane_t;
    let mut s: *mut cbrushside_t = 0 as *mut cbrushside_t;
    box_headnode = numnodes;
    box_planes = &mut *map_planes.as_mut_ptr().offset(numplanes as isize)
        as *mut cplane_t;
    if numnodes + 6 as libc::c_int > 65536 as libc::c_int
        || numbrushes + 1 as libc::c_int > 8192 as libc::c_int
        || numleafbrushes + 1 as libc::c_int > 65536 as libc::c_int
        || numbrushsides + 6 as libc::c_int > 65536 as libc::c_int
        || numplanes + 12 as libc::c_int > 65536 as libc::c_int
    {
        Com_Error(
            1 as libc::c_int,
            b"Not enough room for box tree\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    box_brush = &mut *map_brushes.as_mut_ptr().offset(numbrushes as isize)
        as *mut cbrush_t;
    (*box_brush).numsides = 6 as libc::c_int;
    (*box_brush).firstbrushside = numbrushsides;
    (*box_brush).contents = 0x2000000 as libc::c_int;
    box_leaf = &mut *map_leafs.as_mut_ptr().offset(numleafs as isize) as *mut cleaf_t;
    (*box_leaf).contents = 0x2000000 as libc::c_int;
    (*box_leaf).firstleafbrush = numleafbrushes as libc::c_ushort;
    (*box_leaf).numleafbrushes = 1 as libc::c_int as libc::c_ushort;
    map_leafbrushes[numleafbrushes as usize] = numbrushes as libc::c_ushort;
    i = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        side = i & 1 as libc::c_int;
        s = &mut *map_brushsides.as_mut_ptr().offset((numbrushsides + i) as isize)
            as *mut cbrushside_t;
        let ref mut fresh3 = (*s).plane;
        *fresh3 = map_planes
            .as_mut_ptr()
            .offset((numplanes + i * 2 as libc::c_int + side) as isize);
        let ref mut fresh4 = (*s).surface;
        *fresh4 = &mut nullsurface;
        c = &mut *map_nodes.as_mut_ptr().offset((box_headnode + i) as isize)
            as *mut cnode_t;
        let ref mut fresh5 = (*c).plane;
        *fresh5 = map_planes
            .as_mut_ptr()
            .offset((numplanes + i * 2 as libc::c_int) as isize);
        (*c).children[side as usize] = -(1 as libc::c_int) - emptyleaf;
        if i != 5 as libc::c_int {
            (*c)
                .children[(side ^ 1 as libc::c_int)
                as usize] = box_headnode + i + 1 as libc::c_int;
        } else {
            (*c)
                .children[(side ^ 1 as libc::c_int)
                as usize] = -(1 as libc::c_int) - numleafs;
        }
        p = &mut *box_planes.offset((i * 2 as libc::c_int) as isize) as *mut cplane_t;
        (*p).type_0 = (i >> 1 as libc::c_int) as byte;
        (*p).signbits = 0 as libc::c_int as byte;
        let ref mut fresh6 = (*p).normal[2 as libc::c_int as usize];
        *fresh6 = 0 as libc::c_int as vec_t;
        let ref mut fresh7 = (*p).normal[1 as libc::c_int as usize];
        *fresh7 = *fresh6;
        (*p).normal[0 as libc::c_int as usize] = *fresh7;
        (*p).normal[(i >> 1 as libc::c_int) as usize] = 1 as libc::c_int as vec_t;
        p = &mut *box_planes.offset((i * 2 as libc::c_int + 1 as libc::c_int) as isize)
            as *mut cplane_t;
        (*p).type_0 = (3 as libc::c_int + (i >> 1 as libc::c_int)) as byte;
        (*p).signbits = 0 as libc::c_int as byte;
        let ref mut fresh8 = (*p).normal[2 as libc::c_int as usize];
        *fresh8 = 0 as libc::c_int as vec_t;
        let ref mut fresh9 = (*p).normal[1 as libc::c_int as usize];
        *fresh9 = *fresh8;
        (*p).normal[0 as libc::c_int as usize] = *fresh9;
        (*p).normal[(i >> 1 as libc::c_int) as usize] = -(1 as libc::c_int) as vec_t;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CM_HeadnodeForBox(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> libc::c_int {
    (*box_planes.offset(0 as libc::c_int as isize))
        .dist = *maxs.offset(0 as libc::c_int as isize);
    (*box_planes.offset(1 as libc::c_int as isize))
        .dist = -*maxs.offset(0 as libc::c_int as isize);
    (*box_planes.offset(2 as libc::c_int as isize))
        .dist = *mins.offset(0 as libc::c_int as isize);
    (*box_planes.offset(3 as libc::c_int as isize))
        .dist = -*mins.offset(0 as libc::c_int as isize);
    (*box_planes.offset(4 as libc::c_int as isize))
        .dist = *maxs.offset(1 as libc::c_int as isize);
    (*box_planes.offset(5 as libc::c_int as isize))
        .dist = -*maxs.offset(1 as libc::c_int as isize);
    (*box_planes.offset(6 as libc::c_int as isize))
        .dist = *mins.offset(1 as libc::c_int as isize);
    (*box_planes.offset(7 as libc::c_int as isize))
        .dist = -*mins.offset(1 as libc::c_int as isize);
    (*box_planes.offset(8 as libc::c_int as isize))
        .dist = *maxs.offset(2 as libc::c_int as isize);
    (*box_planes.offset(9 as libc::c_int as isize))
        .dist = -*maxs.offset(2 as libc::c_int as isize);
    (*box_planes.offset(10 as libc::c_int as isize))
        .dist = *mins.offset(2 as libc::c_int as isize);
    (*box_planes.offset(11 as libc::c_int as isize))
        .dist = -*mins.offset(2 as libc::c_int as isize);
    return box_headnode;
}
#[no_mangle]
pub unsafe extern "C" fn CM_PointLeafnum_r(
    mut p: *mut vec_t,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut d: libc::c_float = 0.;
    let mut node: *mut cnode_t = 0 as *mut cnode_t;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    while num >= 0 as libc::c_int {
        node = map_nodes.as_mut_ptr().offset(num as isize);
        plane = (*node).plane;
        if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
            d = *p.offset((*plane).type_0 as isize) - (*plane).dist;
        } else {
            d = (*plane).normal[0 as libc::c_int as usize]
                * *p.offset(0 as libc::c_int as isize)
                + (*plane).normal[1 as libc::c_int as usize]
                    * *p.offset(1 as libc::c_int as isize)
                + (*plane).normal[2 as libc::c_int as usize]
                    * *p.offset(2 as libc::c_int as isize) - (*plane).dist;
        }
        if d < 0 as libc::c_int as libc::c_float {
            num = (*node).children[1 as libc::c_int as usize];
        } else {
            num = (*node).children[0 as libc::c_int as usize];
        }
    }
    c_pointcontents += 1;
    return -(1 as libc::c_int) - num;
}
#[no_mangle]
pub unsafe extern "C" fn CM_PointLeafnum(mut p: *mut vec_t) -> libc::c_int {
    if numplanes == 0 {
        return 0 as libc::c_int;
    }
    return CM_PointLeafnum_r(p, 0 as libc::c_int);
}
#[no_mangle]
pub static mut leaf_count: libc::c_int = 0;
#[no_mangle]
pub static mut leaf_maxcount: libc::c_int = 0;
#[no_mangle]
pub static mut leaf_list: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
#[no_mangle]
pub static mut leaf_mins: *mut libc::c_float = 0 as *const libc::c_float
    as *mut libc::c_float;
#[no_mangle]
pub static mut leaf_maxs: *mut libc::c_float = 0 as *const libc::c_float
    as *mut libc::c_float;
#[no_mangle]
pub static mut leaf_topnode: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn CM_BoxLeafnums_r(mut nodenum: libc::c_int) {
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut node: *mut cnode_t = 0 as *mut cnode_t;
    let mut s: libc::c_int = 0;
    loop {
        if nodenum < 0 as libc::c_int {
            if leaf_count >= leaf_maxcount {
                return;
            }
            let fresh10 = leaf_count;
            leaf_count = leaf_count + 1;
            *leaf_list.offset(fresh10 as isize) = -(1 as libc::c_int) - nodenum;
            return;
        }
        node = &mut *map_nodes.as_mut_ptr().offset(nodenum as isize) as *mut cnode_t;
        plane = (*node).plane;
        s = if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
            if (*plane).dist <= *leaf_mins.offset((*plane).type_0 as isize) {
                1 as libc::c_int
            } else if (*plane).dist >= *leaf_maxs.offset((*plane).type_0 as isize) {
                2 as libc::c_int
            } else {
                3 as libc::c_int
            }
        } else {
            BoxOnPlaneSide(leaf_mins, leaf_maxs, plane)
        };
        if s == 1 as libc::c_int {
            nodenum = (*node).children[0 as libc::c_int as usize];
        } else if s == 2 as libc::c_int {
            nodenum = (*node).children[1 as libc::c_int as usize];
        } else {
            if leaf_topnode == -(1 as libc::c_int) {
                leaf_topnode = nodenum;
            }
            CM_BoxLeafnums_r((*node).children[0 as libc::c_int as usize]);
            nodenum = (*node).children[1 as libc::c_int as usize];
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxLeafnums_headnode(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut list: *mut libc::c_int,
    mut listsize: libc::c_int,
    mut headnode: libc::c_int,
    mut topnode: *mut libc::c_int,
) -> libc::c_int {
    leaf_list = list;
    leaf_count = 0 as libc::c_int;
    leaf_maxcount = listsize;
    leaf_mins = mins;
    leaf_maxs = maxs;
    leaf_topnode = -(1 as libc::c_int);
    CM_BoxLeafnums_r(headnode);
    if !topnode.is_null() {
        *topnode = leaf_topnode;
    }
    return leaf_count;
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxLeafnums(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut list: *mut libc::c_int,
    mut listsize: libc::c_int,
    mut topnode: *mut libc::c_int,
) -> libc::c_int {
    return CM_BoxLeafnums_headnode(
        mins,
        maxs,
        list,
        listsize,
        map_cmodels[0 as libc::c_int as usize].headnode,
        topnode,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CM_PointContents(
    mut p: *mut vec_t,
    mut headnode: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    if numnodes == 0 {
        return 0 as libc::c_int;
    }
    l = CM_PointLeafnum_r(p, headnode);
    return map_leafs[l as usize].contents;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TransformedPointContents(
    mut p: *mut vec_t,
    mut headnode: libc::c_int,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) -> libc::c_int {
    let mut p_l: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut l: libc::c_int = 0;
    p_l[0 as libc::c_int
        as usize] = *p.offset(0 as libc::c_int as isize)
        - *origin.offset(0 as libc::c_int as isize);
    p_l[1 as libc::c_int
        as usize] = *p.offset(1 as libc::c_int as isize)
        - *origin.offset(1 as libc::c_int as isize);
    p_l[2 as libc::c_int
        as usize] = *p.offset(2 as libc::c_int as isize)
        - *origin.offset(2 as libc::c_int as isize);
    if headnode != box_headnode
        && (*angles.offset(0 as libc::c_int as isize) != 0.
            || *angles.offset(1 as libc::c_int as isize) != 0.
            || *angles.offset(2 as libc::c_int as isize) != 0.)
    {
        AngleVectors(angles, forward.as_mut_ptr(), right.as_mut_ptr(), up.as_mut_ptr());
        temp[0 as libc::c_int as usize] = p_l[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] = p_l[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] = p_l[2 as libc::c_int as usize];
        p_l[0 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        p_l[1 as libc::c_int
            as usize] = -(temp[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * right[2 as libc::c_int as usize]);
        p_l[2 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
    }
    l = CM_PointLeafnum_r(p_l.as_mut_ptr(), headnode);
    return map_leafs[l as usize].contents;
}
#[no_mangle]
pub static mut trace_start: vec3_t = [0.; 3];
#[no_mangle]
pub static mut trace_end: vec3_t = [0.; 3];
#[no_mangle]
pub static mut trace_mins: vec3_t = [0.; 3];
#[no_mangle]
pub static mut trace_maxs: vec3_t = [0.; 3];
#[no_mangle]
pub static mut trace_extents: vec3_t = [0.; 3];
#[no_mangle]
pub static mut trace_trace: trace_t = trace_t {
    allsolid: false_0,
    startsolid: false_0,
    fraction: 0.,
    endpos: [0.; 3],
    plane: cplane_t {
        normal: [0.; 3],
        dist: 0.,
        type_0: 0,
        signbits: 0,
        pad: [0; 2],
    },
    surface: 0 as *const csurface_t as *mut csurface_t,
    contents: 0,
    ent: 0 as *const edict_s as *mut edict_s,
};
#[no_mangle]
pub static mut trace_contents: libc::c_int = 0;
#[no_mangle]
pub static mut trace_ispoint: qboolean = false_0;
#[no_mangle]
pub unsafe extern "C" fn CM_ClipBoxToBrush(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut p1: *mut vec_t,
    mut p2: *mut vec_t,
    mut trace: *mut trace_t,
    mut brush: *mut cbrush_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut clipplane: *mut cplane_t = 0 as *mut cplane_t;
    let mut dist: libc::c_float = 0.;
    let mut enterfrac: libc::c_float = 0.;
    let mut leavefrac: libc::c_float = 0.;
    let mut ofs: vec3_t = [0.; 3];
    let mut d1: libc::c_float = 0.;
    let mut d2: libc::c_float = 0.;
    let mut getout: qboolean = false_0;
    let mut startout: qboolean = false_0;
    let mut f: libc::c_float = 0.;
    let mut side: *mut cbrushside_t = 0 as *mut cbrushside_t;
    let mut leadside: *mut cbrushside_t = 0 as *mut cbrushside_t;
    enterfrac = -(1 as libc::c_int) as libc::c_float;
    leavefrac = 1 as libc::c_int as libc::c_float;
    clipplane = 0 as *mut cplane_t;
    if (*brush).numsides == 0 {
        return;
    }
    c_brush_traces += 1;
    getout = false_0;
    startout = false_0;
    leadside = 0 as *mut cbrushside_t;
    i = 0 as libc::c_int;
    while i < (*brush).numsides {
        side = &mut *map_brushsides
            .as_mut_ptr()
            .offset(((*brush).firstbrushside + i) as isize) as *mut cbrushside_t;
        plane = (*side).plane;
        if trace_ispoint as u64 == 0 {
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                if (*plane).normal[j as usize] < 0 as libc::c_int as libc::c_float {
                    ofs[j as usize] = *maxs.offset(j as isize);
                } else {
                    ofs[j as usize] = *mins.offset(j as isize);
                }
                j += 1;
            }
            dist = ofs[0 as libc::c_int as usize]
                * (*plane).normal[0 as libc::c_int as usize]
                + ofs[1 as libc::c_int as usize]
                    * (*plane).normal[1 as libc::c_int as usize]
                + ofs[2 as libc::c_int as usize]
                    * (*plane).normal[2 as libc::c_int as usize];
            dist = (*plane).dist - dist;
        } else {
            dist = (*plane).dist;
        }
        d1 = *p1.offset(0 as libc::c_int as isize)
            * (*plane).normal[0 as libc::c_int as usize]
            + *p1.offset(1 as libc::c_int as isize)
                * (*plane).normal[1 as libc::c_int as usize]
            + *p1.offset(2 as libc::c_int as isize)
                * (*plane).normal[2 as libc::c_int as usize] - dist;
        d2 = *p2.offset(0 as libc::c_int as isize)
            * (*plane).normal[0 as libc::c_int as usize]
            + *p2.offset(1 as libc::c_int as isize)
                * (*plane).normal[1 as libc::c_int as usize]
            + *p2.offset(2 as libc::c_int as isize)
                * (*plane).normal[2 as libc::c_int as usize] - dist;
        if d2 > 0 as libc::c_int as libc::c_float {
            getout = true_0;
        }
        if d1 > 0 as libc::c_int as libc::c_float {
            startout = true_0;
        }
        if d1 > 0 as libc::c_int as libc::c_float && d2 >= d1 {
            return;
        }
        if !(d1 <= 0 as libc::c_int as libc::c_float
            && d2 <= 0 as libc::c_int as libc::c_float)
        {
            if d1 > d2 {
                f = ((d1 as libc::c_double - 0.03125f64) / (d1 - d2) as libc::c_double)
                    as libc::c_float;
                if f > enterfrac {
                    enterfrac = f;
                    clipplane = plane;
                    leadside = side;
                }
            } else {
                f = ((d1 as libc::c_double + 0.03125f64) / (d1 - d2) as libc::c_double)
                    as libc::c_float;
                if f < leavefrac {
                    leavefrac = f;
                }
            }
        }
        i += 1;
    }
    if startout as u64 == 0 {
        (*trace).startsolid = true_0;
        if getout as u64 == 0 {
            (*trace).allsolid = true_0;
        }
        return;
    }
    if enterfrac < leavefrac {
        if enterfrac > -(1 as libc::c_int) as libc::c_float
            && enterfrac < (*trace).fraction
        {
            if enterfrac < 0 as libc::c_int as libc::c_float {
                enterfrac = 0 as libc::c_int as libc::c_float;
            }
            (*trace).fraction = enterfrac;
            (*trace).plane = *clipplane;
            let ref mut fresh11 = (*trace).surface;
            *fresh11 = &mut (*(*leadside).surface).c;
            (*trace).contents = (*brush).contents;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CM_TestBoxInBrush(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut p1: *mut vec_t,
    mut trace: *mut trace_t,
    mut brush: *mut cbrush_t,
) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut dist: libc::c_float = 0.;
    let mut ofs: vec3_t = [0.; 3];
    let mut d1: libc::c_float = 0.;
    let mut side: *mut cbrushside_t = 0 as *mut cbrushside_t;
    if (*brush).numsides == 0 {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*brush).numsides {
        side = &mut *map_brushsides
            .as_mut_ptr()
            .offset(((*brush).firstbrushside + i) as isize) as *mut cbrushside_t;
        plane = (*side).plane;
        j = 0 as libc::c_int;
        while j < 3 as libc::c_int {
            if (*plane).normal[j as usize] < 0 as libc::c_int as libc::c_float {
                ofs[j as usize] = *maxs.offset(j as isize);
            } else {
                ofs[j as usize] = *mins.offset(j as isize);
            }
            j += 1;
        }
        dist = ofs[0 as libc::c_int as usize]
            * (*plane).normal[0 as libc::c_int as usize]
            + ofs[1 as libc::c_int as usize] * (*plane).normal[1 as libc::c_int as usize]
            + ofs[2 as libc::c_int as usize]
                * (*plane).normal[2 as libc::c_int as usize];
        dist = (*plane).dist - dist;
        d1 = *p1.offset(0 as libc::c_int as isize)
            * (*plane).normal[0 as libc::c_int as usize]
            + *p1.offset(1 as libc::c_int as isize)
                * (*plane).normal[1 as libc::c_int as usize]
            + *p1.offset(2 as libc::c_int as isize)
                * (*plane).normal[2 as libc::c_int as usize] - dist;
        if d1 > 0 as libc::c_int as libc::c_float {
            return;
        }
        i += 1;
    }
    let ref mut fresh12 = (*trace).allsolid;
    *fresh12 = true_0;
    (*trace).startsolid = *fresh12;
    (*trace).fraction = 0 as libc::c_int as libc::c_float;
    (*trace).contents = (*brush).contents;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TraceToLeaf(mut leafnum: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut leaf: *mut cleaf_t = 0 as *mut cleaf_t;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    leaf = &mut *map_leafs.as_mut_ptr().offset(leafnum as isize) as *mut cleaf_t;
    if (*leaf).contents & trace_contents == 0 {
        return;
    }
    k = 0 as libc::c_int;
    while k < (*leaf).numleafbrushes as libc::c_int {
        brushnum = map_leafbrushes[((*leaf).firstleafbrush as libc::c_int + k) as usize]
            as libc::c_int;
        b = &mut *map_brushes.as_mut_ptr().offset(brushnum as isize) as *mut cbrush_t;
        if !((*b).checkcount == checkcount) {
            (*b).checkcount = checkcount;
            if !((*b).contents & trace_contents == 0) {
                CM_ClipBoxToBrush(
                    trace_mins.as_mut_ptr(),
                    trace_maxs.as_mut_ptr(),
                    trace_start.as_mut_ptr(),
                    trace_end.as_mut_ptr(),
                    &mut trace_trace,
                    b,
                );
                if trace_trace.fraction == 0. {
                    return;
                }
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CM_TestInLeaf(mut leafnum: libc::c_int) {
    let mut k: libc::c_int = 0;
    let mut brushnum: libc::c_int = 0;
    let mut leaf: *mut cleaf_t = 0 as *mut cleaf_t;
    let mut b: *mut cbrush_t = 0 as *mut cbrush_t;
    leaf = &mut *map_leafs.as_mut_ptr().offset(leafnum as isize) as *mut cleaf_t;
    if (*leaf).contents & trace_contents == 0 {
        return;
    }
    k = 0 as libc::c_int;
    while k < (*leaf).numleafbrushes as libc::c_int {
        brushnum = map_leafbrushes[((*leaf).firstleafbrush as libc::c_int + k) as usize]
            as libc::c_int;
        b = &mut *map_brushes.as_mut_ptr().offset(brushnum as isize) as *mut cbrush_t;
        if !((*b).checkcount == checkcount) {
            (*b).checkcount = checkcount;
            if !((*b).contents & trace_contents == 0) {
                CM_TestBoxInBrush(
                    trace_mins.as_mut_ptr(),
                    trace_maxs.as_mut_ptr(),
                    trace_start.as_mut_ptr(),
                    &mut trace_trace,
                    b,
                );
                if trace_trace.fraction == 0. {
                    return;
                }
            }
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CM_RecursiveHullCheck(
    mut num: libc::c_int,
    mut p1f: libc::c_float,
    mut p2f: libc::c_float,
    mut p1: *mut vec_t,
    mut p2: *mut vec_t,
) {
    let mut node: *mut cnode_t = 0 as *mut cnode_t;
    let mut plane: *mut cplane_t = 0 as *mut cplane_t;
    let mut t1: libc::c_float = 0.;
    let mut t2: libc::c_float = 0.;
    let mut offset: libc::c_float = 0.;
    let mut frac: libc::c_float = 0.;
    let mut frac2: libc::c_float = 0.;
    let mut idist: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    let mut mid: vec3_t = [0.; 3];
    let mut side: libc::c_int = 0;
    let mut midf: libc::c_float = 0.;
    if trace_trace.fraction <= p1f {
        return;
    }
    if num < 0 as libc::c_int {
        CM_TraceToLeaf(-(1 as libc::c_int) - num);
        return;
    }
    node = map_nodes.as_mut_ptr().offset(num as isize);
    plane = (*node).plane;
    if ((*plane).type_0 as libc::c_int) < 3 as libc::c_int {
        t1 = *p1.offset((*plane).type_0 as isize) - (*plane).dist;
        t2 = *p2.offset((*plane).type_0 as isize) - (*plane).dist;
        offset = trace_extents[(*plane).type_0 as usize];
    } else {
        t1 = (*plane).normal[0 as libc::c_int as usize]
            * *p1.offset(0 as libc::c_int as isize)
            + (*plane).normal[1 as libc::c_int as usize]
                * *p1.offset(1 as libc::c_int as isize)
            + (*plane).normal[2 as libc::c_int as usize]
                * *p1.offset(2 as libc::c_int as isize) - (*plane).dist;
        t2 = (*plane).normal[0 as libc::c_int as usize]
            * *p2.offset(0 as libc::c_int as isize)
            + (*plane).normal[1 as libc::c_int as usize]
                * *p2.offset(1 as libc::c_int as isize)
            + (*plane).normal[2 as libc::c_int as usize]
                * *p2.offset(2 as libc::c_int as isize) - (*plane).dist;
        if trace_ispoint as u64 != 0 {
            offset = 0 as libc::c_int as libc::c_float;
        } else {
            offset = (fabs(
                (trace_extents[0 as libc::c_int as usize]
                    * (*plane).normal[0 as libc::c_int as usize]) as libc::c_double,
            )
                + fabs(
                    (trace_extents[1 as libc::c_int as usize]
                        * (*plane).normal[1 as libc::c_int as usize]) as libc::c_double,
                )
                + fabs(
                    (trace_extents[2 as libc::c_int as usize]
                        * (*plane).normal[2 as libc::c_int as usize]) as libc::c_double,
                )) as libc::c_float;
        }
    }
    if t1 >= offset && t2 >= offset {
        CM_RecursiveHullCheck(
            (*node).children[0 as libc::c_int as usize],
            p1f,
            p2f,
            p1,
            p2,
        );
        return;
    }
    if t1 < -offset && t2 < -offset {
        CM_RecursiveHullCheck(
            (*node).children[1 as libc::c_int as usize],
            p1f,
            p2f,
            p1,
            p2,
        );
        return;
    }
    if t1 < t2 {
        idist = (1.0f64 / (t1 - t2) as libc::c_double) as libc::c_float;
        side = 1 as libc::c_int;
        frac2 = (((t1 + offset) as libc::c_double + 0.03125f64)
            * idist as libc::c_double) as libc::c_float;
        frac = (((t1 - offset) as libc::c_double + 0.03125f64) * idist as libc::c_double)
            as libc::c_float;
    } else if t1 > t2 {
        idist = (1.0f64 / (t1 - t2) as libc::c_double) as libc::c_float;
        side = 0 as libc::c_int;
        frac2 = (((t1 - offset) as libc::c_double - 0.03125f64)
            * idist as libc::c_double) as libc::c_float;
        frac = (((t1 + offset) as libc::c_double + 0.03125f64) * idist as libc::c_double)
            as libc::c_float;
    } else {
        side = 0 as libc::c_int;
        frac = 1 as libc::c_int as libc::c_float;
        frac2 = 0 as libc::c_int as libc::c_float;
    }
    if frac < 0 as libc::c_int as libc::c_float {
        frac = 0 as libc::c_int as libc::c_float;
    }
    if frac > 1 as libc::c_int as libc::c_float {
        frac = 1 as libc::c_int as libc::c_float;
    }
    midf = p1f + (p2f - p1f) * frac;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        mid[i
            as usize] = *p1.offset(i as isize)
            + frac * (*p2.offset(i as isize) - *p1.offset(i as isize));
        i += 1;
    }
    CM_RecursiveHullCheck(
        (*node).children[side as usize],
        p1f,
        midf,
        p1,
        mid.as_mut_ptr(),
    );
    if frac2 < 0 as libc::c_int as libc::c_float {
        frac2 = 0 as libc::c_int as libc::c_float;
    }
    if frac2 > 1 as libc::c_int as libc::c_float {
        frac2 = 1 as libc::c_int as libc::c_float;
    }
    midf = p1f + (p2f - p1f) * frac2;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        mid[i
            as usize] = *p1.offset(i as isize)
            + frac2 * (*p2.offset(i as isize) - *p1.offset(i as isize));
        i += 1;
    }
    CM_RecursiveHullCheck(
        (*node).children[(side ^ 1 as libc::c_int) as usize],
        midf,
        p2f,
        mid.as_mut_ptr(),
        p2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CM_BoxTrace(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut headnode: libc::c_int,
    mut brushmask: libc::c_int,
) -> trace_t {
    let mut i: libc::c_int = 0;
    checkcount += 1;
    c_traces += 1;
    memset(
        &mut trace_trace as *mut trace_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<trace_t>() as libc::c_ulong,
    );
    trace_trace.fraction = 1 as libc::c_int as libc::c_float;
    trace_trace.surface = &mut nullsurface.c;
    if numnodes == 0 {
        return trace_trace;
    }
    trace_contents = brushmask;
    trace_start[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    trace_start[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    trace_start[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    trace_end[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize);
    trace_end[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize);
    trace_end[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize);
    trace_mins[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
    trace_mins[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
    trace_mins[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
    trace_maxs[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
    trace_maxs[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
    trace_maxs[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
    if *start.offset(0 as libc::c_int as isize) == *end.offset(0 as libc::c_int as isize)
        && *start.offset(1 as libc::c_int as isize)
            == *end.offset(1 as libc::c_int as isize)
        && *start.offset(2 as libc::c_int as isize)
            == *end.offset(2 as libc::c_int as isize)
    {
        let mut leafs: [libc::c_int; 1024] = [0; 1024];
        let mut i_0: libc::c_int = 0;
        let mut numleafs_0: libc::c_int = 0;
        let mut c1: vec3_t = [0.; 3];
        let mut c2: vec3_t = [0.; 3];
        let mut topnode: libc::c_int = 0;
        c1[0 as libc::c_int
            as usize] = *start.offset(0 as libc::c_int as isize)
            + *mins.offset(0 as libc::c_int as isize);
        c1[1 as libc::c_int
            as usize] = *start.offset(1 as libc::c_int as isize)
            + *mins.offset(1 as libc::c_int as isize);
        c1[2 as libc::c_int
            as usize] = *start.offset(2 as libc::c_int as isize)
            + *mins.offset(2 as libc::c_int as isize);
        c2[0 as libc::c_int
            as usize] = *start.offset(0 as libc::c_int as isize)
            + *maxs.offset(0 as libc::c_int as isize);
        c2[1 as libc::c_int
            as usize] = *start.offset(1 as libc::c_int as isize)
            + *maxs.offset(1 as libc::c_int as isize);
        c2[2 as libc::c_int
            as usize] = *start.offset(2 as libc::c_int as isize)
            + *maxs.offset(2 as libc::c_int as isize);
        i_0 = 0 as libc::c_int;
        while i_0 < 3 as libc::c_int {
            c1[i_0 as usize] -= 1 as libc::c_int as libc::c_float;
            c2[i_0 as usize] += 1 as libc::c_int as libc::c_float;
            i_0 += 1;
        }
        numleafs_0 = CM_BoxLeafnums_headnode(
            c1.as_mut_ptr(),
            c2.as_mut_ptr(),
            leafs.as_mut_ptr(),
            1024 as libc::c_int,
            headnode,
            &mut topnode,
        );
        i_0 = 0 as libc::c_int;
        while i_0 < numleafs_0 {
            CM_TestInLeaf(leafs[i_0 as usize]);
            if trace_trace.allsolid as u64 != 0 {
                break;
            }
            i_0 += 1;
        }
        trace_trace
            .endpos[0 as libc::c_int
            as usize] = *start.offset(0 as libc::c_int as isize);
        trace_trace
            .endpos[1 as libc::c_int
            as usize] = *start.offset(1 as libc::c_int as isize);
        trace_trace
            .endpos[2 as libc::c_int
            as usize] = *start.offset(2 as libc::c_int as isize);
        return trace_trace;
    }
    if *mins.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *mins.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *mins.offset(2 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *maxs.offset(0 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *maxs.offset(1 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
        && *maxs.offset(2 as libc::c_int as isize) == 0 as libc::c_int as libc::c_float
    {
        trace_ispoint = true_0;
        trace_extents[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
        trace_extents[1 as libc::c_int
            as usize] = trace_extents[2 as libc::c_int as usize];
        trace_extents[0 as libc::c_int
            as usize] = trace_extents[1 as libc::c_int as usize];
    } else {
        trace_ispoint = false_0;
        trace_extents[0 as libc::c_int
            as usize] = if -*mins.offset(0 as libc::c_int as isize)
            > *maxs.offset(0 as libc::c_int as isize)
        {
            -*mins.offset(0 as libc::c_int as isize)
        } else {
            *maxs.offset(0 as libc::c_int as isize)
        };
        trace_extents[1 as libc::c_int
            as usize] = if -*mins.offset(1 as libc::c_int as isize)
            > *maxs.offset(1 as libc::c_int as isize)
        {
            -*mins.offset(1 as libc::c_int as isize)
        } else {
            *maxs.offset(1 as libc::c_int as isize)
        };
        trace_extents[2 as libc::c_int
            as usize] = if -*mins.offset(2 as libc::c_int as isize)
            > *maxs.offset(2 as libc::c_int as isize)
        {
            -*mins.offset(2 as libc::c_int as isize)
        } else {
            *maxs.offset(2 as libc::c_int as isize)
        };
    }
    CM_RecursiveHullCheck(
        headnode,
        0 as libc::c_int as libc::c_float,
        1 as libc::c_int as libc::c_float,
        start,
        end,
    );
    if trace_trace.fraction == 1 as libc::c_int as libc::c_float {
        trace_trace
            .endpos[0 as libc::c_int as usize] = *end.offset(0 as libc::c_int as isize);
        trace_trace
            .endpos[1 as libc::c_int as usize] = *end.offset(1 as libc::c_int as isize);
        trace_trace
            .endpos[2 as libc::c_int as usize] = *end.offset(2 as libc::c_int as isize);
    } else {
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            trace_trace
                .endpos[i
                as usize] = *start.offset(i as isize)
                + trace_trace.fraction
                    * (*end.offset(i as isize) - *start.offset(i as isize));
            i += 1;
        }
    }
    return trace_trace;
}
#[no_mangle]
pub unsafe extern "C" fn CM_TransformedBoxTrace(
    mut start: *mut vec_t,
    mut end: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut headnode: libc::c_int,
    mut brushmask: libc::c_int,
    mut origin: *mut vec_t,
    mut angles: *mut vec_t,
) -> trace_t {
    let mut trace: trace_t = trace_t {
        allsolid: false_0,
        startsolid: false_0,
        fraction: 0.,
        endpos: [0.; 3],
        plane: cplane_t {
            normal: [0.; 3],
            dist: 0.,
            type_0: 0,
            signbits: 0,
            pad: [0; 2],
        },
        surface: 0 as *const csurface_t as *mut csurface_t,
        contents: 0,
        ent: 0 as *const edict_s as *mut edict_s,
    };
    let mut start_l: vec3_t = [0.; 3];
    let mut end_l: vec3_t = [0.; 3];
    let mut a: vec3_t = [0.; 3];
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut temp: vec3_t = [0.; 3];
    let mut rotated: qboolean = false_0;
    start_l[0 as libc::c_int
        as usize] = *start.offset(0 as libc::c_int as isize)
        - *origin.offset(0 as libc::c_int as isize);
    start_l[1 as libc::c_int
        as usize] = *start.offset(1 as libc::c_int as isize)
        - *origin.offset(1 as libc::c_int as isize);
    start_l[2 as libc::c_int
        as usize] = *start.offset(2 as libc::c_int as isize)
        - *origin.offset(2 as libc::c_int as isize);
    end_l[0 as libc::c_int
        as usize] = *end.offset(0 as libc::c_int as isize)
        - *origin.offset(0 as libc::c_int as isize);
    end_l[1 as libc::c_int
        as usize] = *end.offset(1 as libc::c_int as isize)
        - *origin.offset(1 as libc::c_int as isize);
    end_l[2 as libc::c_int
        as usize] = *end.offset(2 as libc::c_int as isize)
        - *origin.offset(2 as libc::c_int as isize);
    if headnode != box_headnode
        && (*angles.offset(0 as libc::c_int as isize) != 0.
            || *angles.offset(1 as libc::c_int as isize) != 0.
            || *angles.offset(2 as libc::c_int as isize) != 0.)
    {
        rotated = true_0;
    } else {
        rotated = false_0;
    }
    if rotated as u64 != 0 {
        AngleVectors(angles, forward.as_mut_ptr(), right.as_mut_ptr(), up.as_mut_ptr());
        temp[0 as libc::c_int as usize] = start_l[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] = start_l[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] = start_l[2 as libc::c_int as usize];
        start_l[0 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        start_l[1 as libc::c_int
            as usize] = -(temp[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * right[2 as libc::c_int as usize]);
        start_l[2 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
        temp[0 as libc::c_int as usize] = end_l[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] = end_l[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] = end_l[2 as libc::c_int as usize];
        end_l[0 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        end_l[1 as libc::c_int
            as usize] = -(temp[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * right[2 as libc::c_int as usize]);
        end_l[2 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
    }
    trace = CM_BoxTrace(
        start_l.as_mut_ptr(),
        end_l.as_mut_ptr(),
        mins,
        maxs,
        headnode,
        brushmask,
    );
    if rotated as libc::c_uint != 0 && trace.fraction as libc::c_double != 1.0f64 {
        a[0 as libc::c_int as usize] = -*angles.offset(0 as libc::c_int as isize);
        a[1 as libc::c_int as usize] = -*angles.offset(1 as libc::c_int as isize);
        a[2 as libc::c_int as usize] = -*angles.offset(2 as libc::c_int as isize);
        AngleVectors(
            a.as_mut_ptr(),
            forward.as_mut_ptr(),
            right.as_mut_ptr(),
            up.as_mut_ptr(),
        );
        temp[0 as libc::c_int as usize] = trace.plane.normal[0 as libc::c_int as usize];
        temp[1 as libc::c_int as usize] = trace.plane.normal[1 as libc::c_int as usize];
        temp[2 as libc::c_int as usize] = trace.plane.normal[2 as libc::c_int as usize];
        trace
            .plane
            .normal[0 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize]
            * forward[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * forward[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * forward[2 as libc::c_int as usize];
        trace
            .plane
            .normal[1 as libc::c_int
            as usize] = -(temp[0 as libc::c_int as usize]
            * right[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * right[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * right[2 as libc::c_int as usize]);
        trace
            .plane
            .normal[2 as libc::c_int
            as usize] = temp[0 as libc::c_int as usize] * up[0 as libc::c_int as usize]
            + temp[1 as libc::c_int as usize] * up[1 as libc::c_int as usize]
            + temp[2 as libc::c_int as usize] * up[2 as libc::c_int as usize];
    }
    trace
        .endpos[0 as libc::c_int
        as usize] = *start.offset(0 as libc::c_int as isize)
        + trace.fraction
            * (*end.offset(0 as libc::c_int as isize)
                - *start.offset(0 as libc::c_int as isize));
    trace
        .endpos[1 as libc::c_int
        as usize] = *start.offset(1 as libc::c_int as isize)
        + trace.fraction
            * (*end.offset(1 as libc::c_int as isize)
                - *start.offset(1 as libc::c_int as isize));
    trace
        .endpos[2 as libc::c_int
        as usize] = *start.offset(2 as libc::c_int as isize)
        + trace.fraction
            * (*end.offset(2 as libc::c_int as isize)
                - *start.offset(2 as libc::c_int as isize));
    return trace;
}
#[no_mangle]
pub unsafe extern "C" fn CM_DecompressVis(mut in_0: *mut byte, mut out: *mut byte) {
    let mut c: libc::c_int = 0;
    let mut out_p: *mut byte = 0 as *mut byte;
    let mut row: libc::c_int = 0;
    row = numclusters + 7 as libc::c_int >> 3 as libc::c_int;
    out_p = out;
    if in_0.is_null() || numvisibility == 0 {
        while row != 0 {
            let fresh13 = out_p;
            out_p = out_p.offset(1);
            *fresh13 = 0xff as libc::c_int as byte;
            row -= 1;
        }
        return;
    }
    loop {
        if *in_0 != 0 {
            let fresh14 = in_0;
            in_0 = in_0.offset(1);
            let fresh15 = out_p;
            out_p = out_p.offset(1);
            *fresh15 = *fresh14;
        } else {
            c = *in_0.offset(1 as libc::c_int as isize) as libc::c_int;
            in_0 = in_0.offset(2 as libc::c_int as isize);
            if out_p.offset_from(out) as libc::c_long + c as libc::c_long
                > row as libc::c_long
            {
                c = (row as libc::c_long - out_p.offset_from(out) as libc::c_long)
                    as libc::c_int;
                Com_DPrintf(
                    b"warning: Vis decompression overrun\n\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            while c != 0 {
                let fresh16 = out_p;
                out_p = out_p.offset(1);
                *fresh16 = 0 as libc::c_int as byte;
                c -= 1;
            }
        }
        if !((out_p.offset_from(out) as libc::c_long) < row as libc::c_long) {
            break;
        }
    };
}
#[no_mangle]
pub static mut pvsrow: [byte; 8192] = [0; 8192];
#[no_mangle]
pub static mut phsrow: [byte; 8192] = [0; 8192];
#[no_mangle]
pub unsafe extern "C" fn CM_ClusterPVS(mut cluster: libc::c_int) -> *mut byte {
    if cluster == -(1 as libc::c_int) {
        memset(
            pvsrow.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (numclusters + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
        );
    } else {
        CM_DecompressVis(
            map_visibility
                .as_mut_ptr()
                .offset(
                    (*map_vis).bitofs[cluster as usize][0 as libc::c_int as usize]
                        as isize,
                ),
            pvsrow.as_mut_ptr(),
        );
    }
    return pvsrow.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CM_ClusterPHS(mut cluster: libc::c_int) -> *mut byte {
    if cluster == -(1 as libc::c_int) {
        memset(
            phsrow.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            (numclusters + 7 as libc::c_int >> 3 as libc::c_int) as libc::c_ulong,
        );
    } else {
        CM_DecompressVis(
            map_visibility
                .as_mut_ptr()
                .offset(
                    (*map_vis).bitofs[cluster as usize][1 as libc::c_int as usize]
                        as isize,
                ),
            phsrow.as_mut_ptr(),
        );
    }
    return phsrow.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn FloodArea_r(mut area: *mut carea_t, mut floodnum: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut p: *mut dareaportal_t = 0 as *mut dareaportal_t;
    if (*area).floodvalid == floodvalid {
        if (*area).floodnum == floodnum {
            return;
        }
        Com_Error(
            1 as libc::c_int,
            b"FloodArea_r: reflooded\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    (*area).floodnum = floodnum;
    (*area).floodvalid = floodvalid;
    p = &mut *map_areaportals.as_mut_ptr().offset((*area).firstareaportal as isize)
        as *mut dareaportal_t;
    i = 0 as libc::c_int;
    while i < (*area).numareaportals {
        if portalopen[(*p).portalnum as usize] as u64 != 0 {
            FloodArea_r(
                &mut *map_areas.as_mut_ptr().offset((*p).otherarea as isize),
                floodnum,
            );
        }
        i += 1;
        p = p.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn FloodAreaConnections() {
    let mut i: libc::c_int = 0;
    let mut area: *mut carea_t = 0 as *mut carea_t;
    let mut floodnum: libc::c_int = 0;
    floodvalid += 1;
    floodnum = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i < numareas {
        area = &mut *map_areas.as_mut_ptr().offset(i as isize) as *mut carea_t;
        if !((*area).floodvalid == floodvalid) {
            floodnum += 1;
            FloodArea_r(area, floodnum);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CM_SetAreaPortalState(
    mut portalnum: libc::c_int,
    mut open: qboolean,
) {
    if portalnum > numareaportals {
        Com_Error(
            1 as libc::c_int,
            b"areaportal > numareaportals\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    portalopen[portalnum as usize] = open;
    FloodAreaConnections();
}
#[no_mangle]
pub unsafe extern "C" fn CM_AreasConnected(
    mut area1: libc::c_int,
    mut area2: libc::c_int,
) -> qboolean {
    if (*map_noareas).value != 0. {
        return true_0;
    }
    if area1 > numareas || area2 > numareas {
        Com_Error(
            1 as libc::c_int,
            b"area > numareas\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if map_areas[area1 as usize].floodnum == map_areas[area2 as usize].floodnum {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CM_WriteAreaBits(
    mut buffer: *mut byte,
    mut area: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut floodnum: libc::c_int = 0;
    let mut bytes: libc::c_int = 0;
    bytes = numareas + 7 as libc::c_int >> 3 as libc::c_int;
    if (*map_noareas).value != 0. {
        memset(buffer as *mut libc::c_void, 255 as libc::c_int, bytes as libc::c_ulong);
    } else {
        memset(buffer as *mut libc::c_void, 0 as libc::c_int, bytes as libc::c_ulong);
        floodnum = map_areas[area as usize].floodnum;
        i = 0 as libc::c_int;
        while i < numareas {
            if map_areas[i as usize].floodnum == floodnum || area == 0 {
                let ref mut fresh17 = *buffer.offset((i >> 3 as libc::c_int) as isize);
                *fresh17 = (*fresh17 as libc::c_int
                    | (1 as libc::c_int) << (i & 7 as libc::c_int)) as byte;
            }
            i += 1;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn CM_WritePortalState(mut f: *mut FILE) {
    fwrite(
        portalopen.as_mut_ptr() as *const libc::c_void,
        ::std::mem::size_of::<[qboolean; 1024]>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CM_ReadPortalState(mut f: *mut FILE) {
    FS_Read(
        portalopen.as_mut_ptr() as *mut libc::c_void,
        ::std::mem::size_of::<[qboolean; 1024]>() as libc::c_ulong as libc::c_int,
        f,
    );
    FloodAreaConnections();
}
#[no_mangle]
pub unsafe extern "C" fn CM_HeadnodeVisible(
    mut nodenum: libc::c_int,
    mut visbits: *mut byte,
) -> qboolean {
    let mut leafnum: libc::c_int = 0;
    let mut cluster: libc::c_int = 0;
    let mut node: *mut cnode_t = 0 as *mut cnode_t;
    if nodenum < 0 as libc::c_int {
        leafnum = -(1 as libc::c_int) - nodenum;
        cluster = map_leafs[leafnum as usize].cluster;
        if cluster == -(1 as libc::c_int) {
            return false_0;
        }
        if *visbits.offset((cluster >> 3 as libc::c_int) as isize) as libc::c_int
            & (1 as libc::c_int) << (cluster & 7 as libc::c_int) != 0
        {
            return true_0;
        }
        return false_0;
    }
    node = &mut *map_nodes.as_mut_ptr().offset(nodenum as isize) as *mut cnode_t;
    if CM_HeadnodeVisible((*node).children[0 as libc::c_int as usize], visbits) as u64
        != 0
    {
        return true_0;
    }
    return CM_HeadnodeVisible((*node).children[1 as libc::c_int as usize], visbits);
}
