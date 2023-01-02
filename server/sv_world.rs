#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut ge: *mut game_export_t;
    static mut sv: server_t;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn CM_LeafArea(leafnum: libc::c_int) -> libc::c_int;
    fn CM_LeafCluster(leafnum: libc::c_int) -> libc::c_int;
    fn CM_BoxLeafnums(
        mins: *mut vec_t,
        maxs: *mut vec_t,
        list: *mut libc::c_int,
        listsize: libc::c_int,
        topnode: *mut libc::c_int,
    ) -> libc::c_int;
    fn CM_TransformedBoxTrace(
        start: *mut vec_t,
        end: *mut vec_t,
        mins: *mut vec_t,
        maxs: *mut vec_t,
        headnode: libc::c_int,
        brushmask: libc::c_int,
        origin: *mut vec_t,
        angles: *mut vec_t,
    ) -> trace_t;
    fn CM_BoxTrace(
        start: *mut vec_t,
        end: *mut vec_t,
        mins: *mut vec_t,
        maxs: *mut vec_t,
        headnode: libc::c_int,
        brushmask: libc::c_int,
    ) -> trace_t;
    fn CM_TransformedPointContents(
        p: *mut vec_t,
        headnode: libc::c_int,
        origin: *mut vec_t,
        angles: *mut vec_t,
    ) -> libc::c_int;
    fn CM_PointContents(p: *mut vec_t, headnode: libc::c_int) -> libc::c_int;
    fn CM_HeadnodeForBox(mins: *mut vec_t, maxs: *mut vec_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut vec3_origin: vec3_t;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn fabs(_: libc::c_double) -> libc::c_double;
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
pub struct edict_s {
    pub s: entity_state_t,
    pub client: *mut gclient_s,
    pub inuse: qboolean,
    pub linkcount: libc::c_int,
    pub area: link_t,
    pub num_clusters: libc::c_int,
    pub clusternums: [libc::c_int; 16],
    pub headnode: libc::c_int,
    pub areanum: libc::c_int,
    pub areanum2: libc::c_int,
    pub svflags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub absmin: vec3_t,
    pub absmax: vec3_t,
    pub size: vec3_t,
    pub solid: solid_t,
    pub clipmask: libc::c_int,
    pub owner: *mut edict_t,
}
pub type edict_t = edict_s;
pub type solid_t = libc::c_uint;
pub const SOLID_BSP: solid_t = 3;
pub const SOLID_BBOX: solid_t = 2;
pub const SOLID_TRIGGER: solid_t = 1;
pub const SOLID_NOT: solid_t = 0;
pub type link_t = link_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct link_s {
    pub prev: *mut link_s,
    pub next: *mut link_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gclient_s {
    pub ps: player_state_t,
    pub ping: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct player_state_t {
    pub pmove: pmove_state_t,
    pub viewangles: vec3_t,
    pub viewoffset: vec3_t,
    pub kick_angles: vec3_t,
    pub gunangles: vec3_t,
    pub gunoffset: vec3_t,
    pub gunindex: libc::c_int,
    pub gunframe: libc::c_int,
    pub blend: [libc::c_float; 4],
    pub fov: libc::c_float,
    pub rdflags: libc::c_int,
    pub stats: [libc::c_short; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmove_state_t {
    pub pm_type: pmtype_t,
    pub origin: [libc::c_short; 3],
    pub velocity: [libc::c_short; 3],
    pub pm_flags: byte,
    pub pm_time: byte,
    pub gravity: libc::c_short,
    pub delta_angles: [libc::c_short; 3],
}
pub type pmtype_t = libc::c_uint;
pub const PM_FREEZE: pmtype_t = 4;
pub const PM_GIB: pmtype_t = 3;
pub const PM_DEAD: pmtype_t = 2;
pub const PM_SPECTATOR: pmtype_t = 1;
pub const PM_NORMAL: pmtype_t = 0;
pub type entity_state_t = entity_state_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_state_s {
    pub number: libc::c_int,
    pub origin: vec3_t,
    pub angles: vec3_t,
    pub old_origin: vec3_t,
    pub modelindex: libc::c_int,
    pub modelindex2: libc::c_int,
    pub modelindex3: libc::c_int,
    pub modelindex4: libc::c_int,
    pub frame: libc::c_int,
    pub skinnum: libc::c_int,
    pub effects: libc::c_uint,
    pub renderfx: libc::c_int,
    pub solid: libc::c_int,
    pub sound: libc::c_int,
    pub event: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct usercmd_s {
    pub msec: byte,
    pub buttons: byte,
    pub angles: [libc::c_short; 3],
    pub forwardmove: libc::c_short,
    pub sidemove: libc::c_short,
    pub upmove: libc::c_short,
    pub impulse: byte,
    pub lightlevel: byte,
}
pub type usercmd_t = usercmd_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_export_t {
    pub apiversion: libc::c_int,
    pub Init: Option::<unsafe extern "C" fn() -> ()>,
    pub Shutdown: Option::<unsafe extern "C" fn() -> ()>,
    pub SpawnEntities: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            *mut libc::c_char,
        ) -> (),
    >,
    pub WriteGame: Option::<unsafe extern "C" fn(*mut libc::c_char, qboolean) -> ()>,
    pub ReadGame: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub WriteLevel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub ReadLevel: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub ClientConnect: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> qboolean,
    >,
    pub ClientBegin: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientUserinfoChanged: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> (),
    >,
    pub ClientDisconnect: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientCommand: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub ClientThink: Option::<unsafe extern "C" fn(*mut edict_t, *mut usercmd_t) -> ()>,
    pub RunFrame: Option::<unsafe extern "C" fn() -> ()>,
    pub ServerCommand: Option::<unsafe extern "C" fn() -> ()>,
    pub edicts: *mut edict_s,
    pub edict_size: libc::c_int,
    pub num_edicts: libc::c_int,
    pub max_edicts: libc::c_int,
}
pub type server_state_t = libc::c_uint;
pub const ss_pic: server_state_t = 5;
pub const ss_demo: server_state_t = 4;
pub const ss_cinematic: server_state_t = 3;
pub const ss_game: server_state_t = 2;
pub const ss_loading: server_state_t = 1;
pub const ss_dead: server_state_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct server_t {
    pub state: server_state_t,
    pub attractloop: qboolean,
    pub loadgame: qboolean,
    pub time: libc::c_uint,
    pub framenum: libc::c_int,
    pub name: [libc::c_char; 64],
    pub models: [*mut cmodel_s; 256],
    pub configstrings: [[libc::c_char; 64]; 2080],
    pub baselines: [entity_state_t; 1024],
    pub multicast: sizebuf_t,
    pub multicast_buf: [byte; 1400],
    pub demofile: *mut FILE,
    pub timedemo: qboolean,
}
pub type areanode_t = areanode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct areanode_s {
    pub axis: libc::c_int,
    pub dist: libc::c_float,
    pub children: [*mut areanode_s; 2],
    pub trigger_edicts: link_t,
    pub solid_edicts: link_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moveclip_t {
    pub boxmins: vec3_t,
    pub boxmaxs: vec3_t,
    pub mins: *mut libc::c_float,
    pub maxs: *mut libc::c_float,
    pub mins2: vec3_t,
    pub maxs2: vec3_t,
    pub start: *mut libc::c_float,
    pub end: *mut libc::c_float,
    pub trace: trace_t,
    pub passedict: *mut edict_t,
    pub contentmask: libc::c_int,
}
#[no_mangle]
pub static mut sv_areanodes: [areanode_t; 32] = [areanode_t {
    axis: 0,
    dist: 0.,
    children: [0 as *const areanode_s as *mut areanode_s; 2],
    trigger_edicts: link_t {
        prev: 0 as *const link_s as *mut link_s,
        next: 0 as *const link_s as *mut link_s,
    },
    solid_edicts: link_t {
        prev: 0 as *const link_s as *mut link_s,
        next: 0 as *const link_s as *mut link_s,
    },
}; 32];
#[no_mangle]
pub static mut sv_numareanodes: libc::c_int = 0;
#[no_mangle]
pub static mut area_mins: *mut libc::c_float = 0 as *const libc::c_float
    as *mut libc::c_float;
#[no_mangle]
pub static mut area_maxs: *mut libc::c_float = 0 as *const libc::c_float
    as *mut libc::c_float;
#[no_mangle]
pub static mut area_list: *mut *mut edict_t = 0 as *const *mut edict_t
    as *mut *mut edict_t;
#[no_mangle]
pub static mut area_count: libc::c_int = 0;
#[no_mangle]
pub static mut area_maxcount: libc::c_int = 0;
#[no_mangle]
pub static mut area_type: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn ClearLink(mut l: *mut link_t) {
    let ref mut fresh0 = (*l).next;
    *fresh0 = l;
    let ref mut fresh1 = (*l).prev;
    *fresh1 = *fresh0;
}
#[no_mangle]
pub unsafe extern "C" fn RemoveLink(mut l: *mut link_t) {
    let ref mut fresh2 = (*(*l).next).prev;
    *fresh2 = (*l).prev;
    let ref mut fresh3 = (*(*l).prev).next;
    *fresh3 = (*l).next;
}
#[no_mangle]
pub unsafe extern "C" fn InsertLinkBefore(mut l: *mut link_t, mut before: *mut link_t) {
    let ref mut fresh4 = (*l).next;
    *fresh4 = before;
    let ref mut fresh5 = (*l).prev;
    *fresh5 = (*before).prev;
    let ref mut fresh6 = (*(*l).prev).next;
    *fresh6 = l;
    let ref mut fresh7 = (*(*l).next).prev;
    *fresh7 = l;
}
#[no_mangle]
pub unsafe extern "C" fn SV_CreateAreaNode(
    mut depth: libc::c_int,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) -> *mut areanode_t {
    let mut anode: *mut areanode_t = 0 as *mut areanode_t;
    let mut size: vec3_t = [0.; 3];
    let mut mins1: vec3_t = [0.; 3];
    let mut maxs1: vec3_t = [0.; 3];
    let mut mins2: vec3_t = [0.; 3];
    let mut maxs2: vec3_t = [0.; 3];
    anode = &mut *sv_areanodes.as_mut_ptr().offset(sv_numareanodes as isize)
        as *mut areanode_t;
    sv_numareanodes += 1;
    ClearLink(&mut (*anode).trigger_edicts);
    ClearLink(&mut (*anode).solid_edicts);
    if depth == 4 as libc::c_int {
        (*anode).axis = -(1 as libc::c_int);
        let ref mut fresh8 = (*anode).children[1 as libc::c_int as usize];
        *fresh8 = 0 as *mut areanode_s;
        let ref mut fresh9 = (*anode).children[0 as libc::c_int as usize];
        *fresh9 = *fresh8;
        return anode;
    }
    size[0 as libc::c_int
        as usize] = *maxs.offset(0 as libc::c_int as isize)
        - *mins.offset(0 as libc::c_int as isize);
    size[1 as libc::c_int
        as usize] = *maxs.offset(1 as libc::c_int as isize)
        - *mins.offset(1 as libc::c_int as isize);
    size[2 as libc::c_int
        as usize] = *maxs.offset(2 as libc::c_int as isize)
        - *mins.offset(2 as libc::c_int as isize);
    if size[0 as libc::c_int as usize] > size[1 as libc::c_int as usize] {
        (*anode).axis = 0 as libc::c_int;
    } else {
        (*anode).axis = 1 as libc::c_int;
    }
    (*anode)
        .dist = (0.5f64
        * (*maxs.offset((*anode).axis as isize) + *mins.offset((*anode).axis as isize))
            as libc::c_double) as libc::c_float;
    mins1[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
    mins1[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
    mins1[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
    mins2[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
    mins2[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
    mins2[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
    maxs1[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
    maxs1[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
    maxs1[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
    maxs2[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
    maxs2[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
    maxs2[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
    mins2[(*anode).axis as usize] = (*anode).dist;
    maxs1[(*anode).axis as usize] = mins2[(*anode).axis as usize];
    let ref mut fresh10 = (*anode).children[0 as libc::c_int as usize];
    *fresh10 = SV_CreateAreaNode(
        depth + 1 as libc::c_int,
        mins2.as_mut_ptr(),
        maxs2.as_mut_ptr(),
    );
    let ref mut fresh11 = (*anode).children[1 as libc::c_int as usize];
    *fresh11 = SV_CreateAreaNode(
        depth + 1 as libc::c_int,
        mins1.as_mut_ptr(),
        maxs1.as_mut_ptr(),
    );
    return anode;
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClearWorld() {
    memset(
        sv_areanodes.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[areanode_t; 32]>() as libc::c_ulong,
    );
    sv_numareanodes = 0 as libc::c_int;
    SV_CreateAreaNode(
        0 as libc::c_int,
        ((*sv.models[1 as libc::c_int as usize]).mins).as_mut_ptr(),
        ((*sv.models[1 as libc::c_int as usize]).maxs).as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn SV_UnlinkEdict(mut ent: *mut edict_t) {
    if ((*ent).area.prev).is_null() {
        return;
    }
    RemoveLink(&mut (*ent).area);
    let ref mut fresh12 = (*ent).area.next;
    *fresh12 = 0 as *mut link_s;
    let ref mut fresh13 = (*ent).area.prev;
    *fresh13 = *fresh12;
}
#[no_mangle]
pub unsafe extern "C" fn SV_LinkEdict(mut ent: *mut edict_t) {
    let mut node: *mut areanode_t = 0 as *mut areanode_t;
    let mut leafs: [libc::c_int; 128] = [0; 128];
    let mut clusters: [libc::c_int; 128] = [0; 128];
    let mut num_leafs: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut area: libc::c_int = 0;
    let mut topnode: libc::c_int = 0;
    if !((*ent).area.prev).is_null() {
        SV_UnlinkEdict(ent);
    }
    if ent == (*ge).edicts {
        return;
    }
    if (*ent).inuse as u64 == 0 {
        return;
    }
    (*ent)
        .size[0 as libc::c_int
        as usize] = (*ent).maxs[0 as libc::c_int as usize]
        - (*ent).mins[0 as libc::c_int as usize];
    (*ent)
        .size[1 as libc::c_int
        as usize] = (*ent).maxs[1 as libc::c_int as usize]
        - (*ent).mins[1 as libc::c_int as usize];
    (*ent)
        .size[2 as libc::c_int
        as usize] = (*ent).maxs[2 as libc::c_int as usize]
        - (*ent).mins[2 as libc::c_int as usize];
    if (*ent).solid as libc::c_uint == SOLID_BBOX as libc::c_int as libc::c_uint
        && (*ent).svflags & 0x2 as libc::c_int == 0
    {
        i = ((*ent).maxs[0 as libc::c_int as usize] / 8 as libc::c_int as libc::c_float)
            as libc::c_int;
        if i < 1 as libc::c_int {
            i = 1 as libc::c_int;
        }
        if i > 31 as libc::c_int {
            i = 31 as libc::c_int;
        }
        j = (-(*ent).mins[2 as libc::c_int as usize] / 8 as libc::c_int as libc::c_float)
            as libc::c_int;
        if j < 1 as libc::c_int {
            j = 1 as libc::c_int;
        }
        if j > 31 as libc::c_int {
            j = 31 as libc::c_int;
        }
        k = (((*ent).maxs[2 as libc::c_int as usize]
            + 32 as libc::c_int as libc::c_float) / 8 as libc::c_int as libc::c_float)
            as libc::c_int;
        if k < 1 as libc::c_int {
            k = 1 as libc::c_int;
        }
        if k > 63 as libc::c_int {
            k = 63 as libc::c_int;
        }
        (*ent).s.solid = k << 10 as libc::c_int | j << 5 as libc::c_int | i;
    } else if (*ent).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint {
        (*ent).s.solid = 31 as libc::c_int;
    } else {
        (*ent).s.solid = 0 as libc::c_int;
    }
    if (*ent).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint
        && ((*ent).s.angles[0 as libc::c_int as usize] != 0.
            || (*ent).s.angles[1 as libc::c_int as usize] != 0.
            || (*ent).s.angles[2 as libc::c_int as usize] != 0.)
    {
        let mut max: libc::c_float = 0.;
        let mut v: libc::c_float = 0.;
        let mut i_0: libc::c_int = 0;
        max = 0 as libc::c_int as libc::c_float;
        i_0 = 0 as libc::c_int;
        while i_0 < 3 as libc::c_int {
            v = fabs((*ent).mins[i_0 as usize] as libc::c_double) as libc::c_float;
            if v > max {
                max = v;
            }
            v = fabs((*ent).maxs[i_0 as usize] as libc::c_double) as libc::c_float;
            if v > max {
                max = v;
            }
            i_0 += 1;
        }
        i_0 = 0 as libc::c_int;
        while i_0 < 3 as libc::c_int {
            (*ent).absmin[i_0 as usize] = (*ent).s.origin[i_0 as usize] - max;
            (*ent).absmax[i_0 as usize] = (*ent).s.origin[i_0 as usize] + max;
            i_0 += 1;
        }
    } else {
        (*ent)
            .absmin[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize]
            + (*ent).mins[0 as libc::c_int as usize];
        (*ent)
            .absmin[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize]
            + (*ent).mins[1 as libc::c_int as usize];
        (*ent)
            .absmin[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize]
            + (*ent).mins[2 as libc::c_int as usize];
        (*ent)
            .absmax[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize]
            + (*ent).maxs[0 as libc::c_int as usize];
        (*ent)
            .absmax[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize]
            + (*ent).maxs[1 as libc::c_int as usize];
        (*ent)
            .absmax[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize]
            + (*ent).maxs[2 as libc::c_int as usize];
    }
    let ref mut fresh14 = (*ent).absmin[0 as libc::c_int as usize];
    *fresh14 -= 1 as libc::c_int as libc::c_float;
    let ref mut fresh15 = (*ent).absmin[1 as libc::c_int as usize];
    *fresh15 -= 1 as libc::c_int as libc::c_float;
    let ref mut fresh16 = (*ent).absmin[2 as libc::c_int as usize];
    *fresh16 -= 1 as libc::c_int as libc::c_float;
    let ref mut fresh17 = (*ent).absmax[0 as libc::c_int as usize];
    *fresh17 += 1 as libc::c_int as libc::c_float;
    let ref mut fresh18 = (*ent).absmax[1 as libc::c_int as usize];
    *fresh18 += 1 as libc::c_int as libc::c_float;
    let ref mut fresh19 = (*ent).absmax[2 as libc::c_int as usize];
    *fresh19 += 1 as libc::c_int as libc::c_float;
    (*ent).num_clusters = 0 as libc::c_int;
    (*ent).areanum = 0 as libc::c_int;
    (*ent).areanum2 = 0 as libc::c_int;
    num_leafs = CM_BoxLeafnums(
        ((*ent).absmin).as_mut_ptr(),
        ((*ent).absmax).as_mut_ptr(),
        leafs.as_mut_ptr(),
        128 as libc::c_int,
        &mut topnode,
    );
    i = 0 as libc::c_int;
    while i < num_leafs {
        clusters[i as usize] = CM_LeafCluster(leafs[i as usize]);
        area = CM_LeafArea(leafs[i as usize]);
        if area != 0 {
            if (*ent).areanum != 0 && (*ent).areanum != area {
                if (*ent).areanum2 != 0 && (*ent).areanum2 != area
                    && sv.state as libc::c_uint
                        == ss_loading as libc::c_int as libc::c_uint
                {
                    Com_DPrintf(
                        b"Object touching 3 areas at %f %f %f\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        (*ent).absmin[0 as libc::c_int as usize] as libc::c_double,
                        (*ent).absmin[1 as libc::c_int as usize] as libc::c_double,
                        (*ent).absmin[2 as libc::c_int as usize] as libc::c_double,
                    );
                }
                (*ent).areanum2 = area;
            } else {
                (*ent).areanum = area;
            }
        }
        i += 1;
    }
    if num_leafs >= 128 as libc::c_int {
        (*ent).num_clusters = -(1 as libc::c_int);
        (*ent).headnode = topnode;
    } else {
        (*ent).num_clusters = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < num_leafs {
            if !(clusters[i as usize] == -(1 as libc::c_int)) {
                j = 0 as libc::c_int;
                while j < i {
                    if clusters[j as usize] == clusters[i as usize] {
                        break;
                    }
                    j += 1;
                }
                if j == i {
                    if (*ent).num_clusters == 16 as libc::c_int {
                        (*ent).num_clusters = -(1 as libc::c_int);
                        (*ent).headnode = topnode;
                        break;
                    } else {
                        let ref mut fresh20 = (*ent).num_clusters;
                        let fresh21 = *fresh20;
                        *fresh20 = *fresh20 + 1;
                        (*ent).clusternums[fresh21 as usize] = clusters[i as usize];
                    }
                }
            }
            i += 1;
        }
    }
    if (*ent).linkcount == 0 {
        (*ent)
            .s
            .old_origin[0 as libc::c_int
            as usize] = (*ent).s.origin[0 as libc::c_int as usize];
        (*ent)
            .s
            .old_origin[1 as libc::c_int
            as usize] = (*ent).s.origin[1 as libc::c_int as usize];
        (*ent)
            .s
            .old_origin[2 as libc::c_int
            as usize] = (*ent).s.origin[2 as libc::c_int as usize];
    }
    let ref mut fresh22 = (*ent).linkcount;
    *fresh22 += 1;
    if (*ent).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
        return;
    }
    node = sv_areanodes.as_mut_ptr();
    while !((*node).axis == -(1 as libc::c_int)) {
        if (*ent).absmin[(*node).axis as usize] > (*node).dist {
            node = (*node).children[0 as libc::c_int as usize];
        } else {
            if !((*ent).absmax[(*node).axis as usize] < (*node).dist) {
                break;
            }
            node = (*node).children[1 as libc::c_int as usize];
        }
    }
    if (*ent).solid as libc::c_uint == SOLID_TRIGGER as libc::c_int as libc::c_uint {
        InsertLinkBefore(&mut (*ent).area, &mut (*node).trigger_edicts);
    } else {
        InsertLinkBefore(&mut (*ent).area, &mut (*node).solid_edicts);
    };
}
#[no_mangle]
pub unsafe extern "C" fn SV_AreaEdicts_r(mut node: *mut areanode_t) {
    let mut l: *mut link_t = 0 as *mut link_t;
    let mut next: *mut link_t = 0 as *mut link_t;
    let mut start: *mut link_t = 0 as *mut link_t;
    let mut check: *mut edict_t = 0 as *mut edict_t;
    let mut count: libc::c_int = 0;
    count = 0 as libc::c_int;
    if area_type == 1 as libc::c_int {
        start = &mut (*node).solid_edicts;
    } else {
        start = &mut (*node).trigger_edicts;
    }
    l = (*start).next;
    while l != start {
        next = (*l).next;
        check = (l as *mut byte)
            .offset(
                -(&mut (*(0 as *mut edict_t)).area as *mut link_t as libc::c_int
                    as isize),
            ) as *mut edict_t;
        if !((*check).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint)
        {
            if !((*check).absmin[0 as libc::c_int as usize]
                > *area_maxs.offset(0 as libc::c_int as isize)
                || (*check).absmin[1 as libc::c_int as usize]
                    > *area_maxs.offset(1 as libc::c_int as isize)
                || (*check).absmin[2 as libc::c_int as usize]
                    > *area_maxs.offset(2 as libc::c_int as isize)
                || (*check).absmax[0 as libc::c_int as usize]
                    < *area_mins.offset(0 as libc::c_int as isize)
                || (*check).absmax[1 as libc::c_int as usize]
                    < *area_mins.offset(1 as libc::c_int as isize)
                || (*check).absmax[2 as libc::c_int as usize]
                    < *area_mins.offset(2 as libc::c_int as isize))
            {
                if area_count == area_maxcount {
                    Com_Printf(
                        b"SV_AreaEdicts: MAXCOUNT\n\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    return;
                }
                let ref mut fresh23 = *area_list.offset(area_count as isize);
                *fresh23 = check;
                area_count += 1;
            }
        }
        l = next;
    }
    if (*node).axis == -(1 as libc::c_int) {
        return;
    }
    if *area_maxs.offset((*node).axis as isize) > (*node).dist {
        SV_AreaEdicts_r((*node).children[0 as libc::c_int as usize]);
    }
    if *area_mins.offset((*node).axis as isize) < (*node).dist {
        SV_AreaEdicts_r((*node).children[1 as libc::c_int as usize]);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_AreaEdicts(
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut list: *mut *mut edict_t,
    mut maxcount: libc::c_int,
    mut areatype: libc::c_int,
) -> libc::c_int {
    area_mins = mins;
    area_maxs = maxs;
    area_list = list;
    area_count = 0 as libc::c_int;
    area_maxcount = maxcount;
    area_type = areatype;
    SV_AreaEdicts_r(sv_areanodes.as_mut_ptr());
    return area_count;
}
#[no_mangle]
pub unsafe extern "C" fn SV_PointContents(mut p: *mut vec_t) -> libc::c_int {
    let mut touch: [*mut edict_t; 1024] = [0 as *mut edict_t; 1024];
    let mut hit: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut contents: libc::c_int = 0;
    let mut c2: libc::c_int = 0;
    let mut headnode: libc::c_int = 0;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    contents = CM_PointContents(p, (*sv.models[1 as libc::c_int as usize]).headnode);
    num = SV_AreaEdicts(p, p, touch.as_mut_ptr(), 1024 as libc::c_int, 1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < num {
        hit = touch[i as usize];
        headnode = SV_HullForEntity(hit);
        angles = ((*hit).s.angles).as_mut_ptr();
        if (*hit).solid as libc::c_uint != SOLID_BSP as libc::c_int as libc::c_uint {
            angles = vec3_origin.as_mut_ptr();
        }
        c2 = CM_TransformedPointContents(
            p,
            headnode,
            ((*hit).s.origin).as_mut_ptr(),
            ((*hit).s.angles).as_mut_ptr(),
        );
        contents |= c2;
        i += 1;
    }
    return contents;
}
#[no_mangle]
pub unsafe extern "C" fn SV_HullForEntity(mut ent: *mut edict_t) -> libc::c_int {
    let mut model: *mut cmodel_t = 0 as *mut cmodel_t;
    if (*ent).solid as libc::c_uint == SOLID_BSP as libc::c_int as libc::c_uint {
        model = sv.models[(*ent).s.modelindex as usize];
        if model.is_null() {
            Com_Error(
                0 as libc::c_int,
                b"MOVETYPE_PUSH with a non bsp model\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        return (*model).headnode;
    }
    return CM_HeadnodeForBox(((*ent).mins).as_mut_ptr(), ((*ent).maxs).as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn SV_ClipMoveToEntities(mut clip: *mut moveclip_t) {
    let mut i: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut touchlist: [*mut edict_t; 1024] = [0 as *mut edict_t; 1024];
    let mut touch: *mut edict_t = 0 as *mut edict_t;
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
        surface: 0 as *mut csurface_t,
        contents: 0,
        ent: 0 as *mut edict_s,
    };
    let mut headnode: libc::c_int = 0;
    let mut angles: *mut libc::c_float = 0 as *mut libc::c_float;
    num = SV_AreaEdicts(
        ((*clip).boxmins).as_mut_ptr(),
        ((*clip).boxmaxs).as_mut_ptr(),
        touchlist.as_mut_ptr(),
        1024 as libc::c_int,
        1 as libc::c_int,
    );
    let mut current_block_21: u64;
    i = 0 as libc::c_int;
    while i < num {
        touch = touchlist[i as usize];
        if !((*touch).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint)
        {
            if !(touch == (*clip).passedict) {
                if (*clip).trace.allsolid as u64 != 0 {
                    return;
                }
                if !((*clip).passedict).is_null() {
                    if (*touch).owner == (*clip).passedict {
                        current_block_21 = 735147466149431745;
                    } else if (*(*clip).passedict).owner == touch {
                        current_block_21 = 735147466149431745;
                    } else {
                        current_block_21 = 12349973810996921269;
                    }
                } else {
                    current_block_21 = 12349973810996921269;
                }
                match current_block_21 {
                    735147466149431745 => {}
                    _ => {
                        if !((*clip).contentmask & 0x4000000 as libc::c_int == 0
                            && (*touch).svflags & 0x2 as libc::c_int != 0)
                        {
                            headnode = SV_HullForEntity(touch);
                            angles = ((*touch).s.angles).as_mut_ptr();
                            if (*touch).solid as libc::c_uint
                                != SOLID_BSP as libc::c_int as libc::c_uint
                            {
                                angles = vec3_origin.as_mut_ptr();
                            }
                            if (*touch).svflags & 0x4 as libc::c_int != 0 {
                                trace = CM_TransformedBoxTrace(
                                    (*clip).start,
                                    (*clip).end,
                                    ((*clip).mins2).as_mut_ptr(),
                                    ((*clip).maxs2).as_mut_ptr(),
                                    headnode,
                                    (*clip).contentmask,
                                    ((*touch).s.origin).as_mut_ptr(),
                                    angles,
                                );
                            } else {
                                trace = CM_TransformedBoxTrace(
                                    (*clip).start,
                                    (*clip).end,
                                    (*clip).mins,
                                    (*clip).maxs,
                                    headnode,
                                    (*clip).contentmask,
                                    ((*touch).s.origin).as_mut_ptr(),
                                    angles,
                                );
                            }
                            if trace.allsolid as libc::c_uint != 0
                                || trace.startsolid as libc::c_uint != 0
                                || trace.fraction < (*clip).trace.fraction
                            {
                                trace.ent = touch;
                                if (*clip).trace.startsolid as u64 != 0 {
                                    (*clip).trace = trace;
                                    (*clip).trace.startsolid = true_0;
                                } else {
                                    (*clip).trace = trace;
                                }
                            } else if trace.startsolid as u64 != 0 {
                                (*clip).trace.startsolid = true_0;
                            }
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_TraceBounds(
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut boxmins: *mut vec_t,
    mut boxmaxs: *mut vec_t,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if *end.offset(i as isize) > *start.offset(i as isize) {
            *boxmins
                .offset(
                    i as isize,
                ) = *start.offset(i as isize) + *mins.offset(i as isize)
                - 1 as libc::c_int as libc::c_float;
            *boxmaxs
                .offset(
                    i as isize,
                ) = *end.offset(i as isize) + *maxs.offset(i as isize)
                + 1 as libc::c_int as libc::c_float;
        } else {
            *boxmins
                .offset(
                    i as isize,
                ) = *end.offset(i as isize) + *mins.offset(i as isize)
                - 1 as libc::c_int as libc::c_float;
            *boxmaxs
                .offset(
                    i as isize,
                ) = *start.offset(i as isize) + *maxs.offset(i as isize)
                + 1 as libc::c_int as libc::c_float;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SV_Trace(
    mut start: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
    mut end: *mut vec_t,
    mut passedict: *mut edict_t,
    mut contentmask: libc::c_int,
) -> trace_t {
    let mut clip: moveclip_t = moveclip_t {
        boxmins: [0.; 3],
        boxmaxs: [0.; 3],
        mins: 0 as *mut libc::c_float,
        maxs: 0 as *mut libc::c_float,
        mins2: [0.; 3],
        maxs2: [0.; 3],
        start: 0 as *mut libc::c_float,
        end: 0 as *mut libc::c_float,
        trace: trace_t {
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
            surface: 0 as *mut csurface_t,
            contents: 0,
            ent: 0 as *mut edict_s,
        },
        passedict: 0 as *mut edict_t,
        contentmask: 0,
    };
    if mins.is_null() {
        mins = vec3_origin.as_mut_ptr();
    }
    if maxs.is_null() {
        maxs = vec3_origin.as_mut_ptr();
    }
    memset(
        &mut clip as *mut moveclip_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<moveclip_t>() as libc::c_ulong,
    );
    clip.trace = CM_BoxTrace(start, end, mins, maxs, 0 as libc::c_int, contentmask);
    clip.trace.ent = (*ge).edicts;
    if clip.trace.fraction == 0 as libc::c_int as libc::c_float {
        return clip.trace;
    }
    clip.contentmask = contentmask;
    clip.start = start;
    clip.end = end;
    clip.mins = mins;
    clip.maxs = maxs;
    clip.passedict = passedict;
    clip.mins2[0 as libc::c_int as usize] = *mins.offset(0 as libc::c_int as isize);
    clip.mins2[1 as libc::c_int as usize] = *mins.offset(1 as libc::c_int as isize);
    clip.mins2[2 as libc::c_int as usize] = *mins.offset(2 as libc::c_int as isize);
    clip.maxs2[0 as libc::c_int as usize] = *maxs.offset(0 as libc::c_int as isize);
    clip.maxs2[1 as libc::c_int as usize] = *maxs.offset(1 as libc::c_int as isize);
    clip.maxs2[2 as libc::c_int as usize] = *maxs.offset(2 as libc::c_int as isize);
    SV_TraceBounds(
        start,
        (clip.mins2).as_mut_ptr(),
        (clip.maxs2).as_mut_ptr(),
        end,
        (clip.boxmins).as_mut_ptr(),
        (clip.boxmaxs).as_mut_ptr(),
    );
    SV_ClipMoveToEntities(&mut clip);
    return clip.trace;
}
