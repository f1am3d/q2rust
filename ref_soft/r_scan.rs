#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut vid: viddef_t;
    static mut r_refdef: oldrefdef_t;
    static mut r_newrefdef: refdef_t;
    static mut intsintable: [libc::c_int; 1280];
    static mut r_warpbuffer: [byte; 76800];
    static mut d_sdivzstepu: libc::c_float;
    static mut d_tdivzstepu: libc::c_float;
    static mut d_zistepu: libc::c_float;
    static mut d_sdivzstepv: libc::c_float;
    static mut d_tdivzstepv: libc::c_float;
    static mut d_zistepv: libc::c_float;
    static mut d_sdivzorigin: libc::c_float;
    static mut d_tdivzorigin: libc::c_float;
    static mut d_ziorigin: libc::c_float;
    static mut sadjust: fixed16_t;
    static mut tadjust: fixed16_t;
    static mut bbextents: fixed16_t;
    static mut bbextentt: fixed16_t;
    static mut cachewidth: libc::c_int;
    static mut r_screenwidth: libc::c_int;
    static mut d_viewbuffer: *mut pixel_t;
    static mut cacheblock: *mut pixel_t;
    static mut d_zwidth: libc::c_uint;
    static mut d_pzbuffer: *mut libc::c_short;
    static mut sintable: [libc::c_int; 1280];
    static mut blanktable: [libc::c_int; 1280];
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type fixed16_t = libc::c_int;
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
pub struct dvis_t {
    pub numclusters: libc::c_int,
    pub bitofs: [[libc::c_int; 2]; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct entity_s {
    pub model: *mut model_s,
    pub angles: [libc::c_float; 3],
    pub origin: [libc::c_float; 3],
    pub frame: libc::c_int,
    pub oldorigin: [libc::c_float; 3],
    pub oldframe: libc::c_int,
    pub backlerp: libc::c_float,
    pub skinnum: libc::c_int,
    pub lightstyle: libc::c_int,
    pub alpha: libc::c_float,
    pub skin: *mut image_s,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct image_s {
    pub name: [libc::c_char; 64],
    pub type_0: imagetype_t,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub transparent: qboolean,
    pub registration_sequence: libc::c_int,
    pub pixels: [*mut byte; 4],
}
pub type imagetype_t = libc::c_uint;
pub const it_sky: imagetype_t = 4;
pub const it_pic: imagetype_t = 3;
pub const it_wall: imagetype_t = 2;
pub const it_sprite: imagetype_t = 1;
pub const it_skin: imagetype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct model_s {
    pub name: [libc::c_char; 64],
    pub registration_sequence: libc::c_int,
    pub type_0: modtype_t,
    pub numframes: libc::c_int,
    pub flags: libc::c_int,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub clipbox: qboolean,
    pub clipmins: vec3_t,
    pub clipmaxs: vec3_t,
    pub firstmodelsurface: libc::c_int,
    pub nummodelsurfaces: libc::c_int,
    pub numsubmodels: libc::c_int,
    pub submodels: *mut dmodel_t,
    pub numplanes: libc::c_int,
    pub planes: *mut mplane_t,
    pub numleafs: libc::c_int,
    pub leafs: *mut mleaf_t,
    pub numvertexes: libc::c_int,
    pub vertexes: *mut mvertex_t,
    pub numedges: libc::c_int,
    pub edges: *mut medge_t,
    pub numnodes: libc::c_int,
    pub firstnode: libc::c_int,
    pub nodes: *mut mnode_t,
    pub numtexinfo: libc::c_int,
    pub texinfo: *mut mtexinfo_t,
    pub numsurfaces: libc::c_int,
    pub surfaces: *mut msurface_t,
    pub numsurfedges: libc::c_int,
    pub surfedges: *mut libc::c_int,
    pub nummarksurfaces: libc::c_int,
    pub marksurfaces: *mut *mut msurface_t,
    pub vis: *mut dvis_t,
    pub lightdata: *mut byte,
    pub skins: [*mut image_t; 32],
    pub extradata: *mut libc::c_void,
    pub extradatasize: libc::c_int,
}
pub type image_t = image_s;
pub type msurface_t = msurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msurface_s {
    pub visframe: libc::c_int,
    pub dlightframe: libc::c_int,
    pub dlightbits: libc::c_int,
    pub plane: *mut mplane_t,
    pub flags: libc::c_int,
    pub firstedge: libc::c_int,
    pub numedges: libc::c_int,
    pub cachespots: [*mut surfcache_s; 4],
    pub texturemins: [libc::c_short; 2],
    pub extents: [libc::c_short; 2],
    pub texinfo: *mut mtexinfo_t,
    pub styles: [byte; 4],
    pub samples: *mut byte,
    pub nextalphasurface: *mut msurface_s,
}
pub type mtexinfo_t = mtexinfo_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtexinfo_s {
    pub vecs: [[libc::c_float; 4]; 2],
    pub mipadjust: libc::c_float,
    pub image: *mut image_t,
    pub flags: libc::c_int,
    pub numframes: libc::c_int,
    pub next: *mut mtexinfo_s,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct surfcache_s {
    pub next: *mut surfcache_s,
    pub owner: *mut *mut surfcache_s,
    pub lightadj: [libc::c_int; 4],
    pub dlight: libc::c_int,
    pub size: libc::c_int,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub mipscale: libc::c_float,
    pub image: *mut image_t,
    pub data: [byte; 4],
}
pub type mplane_t = mplane_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mplane_s {
    pub normal: vec3_t,
    pub dist: libc::c_float,
    pub type_0: byte,
    pub signbits: byte,
    pub pad: [byte; 2],
}
pub type mnode_t = mnode_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_short; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_t,
    pub children: [*mut mnode_s; 2],
    pub firstsurface: libc::c_ushort,
    pub numsurfaces: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct medge_t {
    pub v: [libc::c_ushort; 2],
    pub cachededgeoffset: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mvertex_t {
    pub position: vec3_t,
}
pub type mleaf_t = mleaf_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mleaf_s {
    pub contents: libc::c_int,
    pub visframe: libc::c_int,
    pub minmaxs: [libc::c_short; 6],
    pub parent: *mut mnode_s,
    pub cluster: libc::c_int,
    pub area: libc::c_int,
    pub firstmarksurface: *mut *mut msurface_t,
    pub nummarksurfaces: libc::c_int,
    pub key: libc::c_int,
}
pub type modtype_t = libc::c_uint;
pub const mod_alias: modtype_t = 3;
pub const mod_sprite: modtype_t = 2;
pub const mod_brush: modtype_t = 1;
pub const mod_bad: modtype_t = 0;
pub type entity_t = entity_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dlight_t {
    pub origin: vec3_t,
    pub color: vec3_t,
    pub intensity: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct particle_t {
    pub origin: vec3_t,
    pub color: libc::c_int,
    pub alpha: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lightstyle_t {
    pub rgb: [libc::c_float; 3],
    pub white: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refdef_t {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub fov_x: libc::c_float,
    pub fov_y: libc::c_float,
    pub vieworg: [libc::c_float; 3],
    pub viewangles: [libc::c_float; 3],
    pub blend: [libc::c_float; 4],
    pub time: libc::c_float,
    pub rdflags: libc::c_int,
    pub areabits: *mut byte,
    pub lightstyles: *mut lightstyle_t,
    pub num_entities: libc::c_int,
    pub entities: *mut entity_t,
    pub num_dlights: libc::c_int,
    pub dlights: *mut dlight_t,
    pub num_particles: libc::c_int,
    pub particles: *mut particle_t,
}
pub type pixel_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vrect_s {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub pnext: *mut vrect_s,
}
pub type vrect_t = vrect_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct viddef_t {
    pub buffer: *mut pixel_t,
    pub colormap: *mut pixel_t,
    pub alphamap: *mut pixel_t,
    pub rowbytes: libc::c_int,
    pub width: libc::c_int,
    pub height: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldrefdef_t {
    pub vrect: vrect_t,
    pub aliasvrect: vrect_t,
    pub vrectright: libc::c_int,
    pub vrectbottom: libc::c_int,
    pub aliasvrectright: libc::c_int,
    pub aliasvrectbottom: libc::c_int,
    pub vrectrightedge: libc::c_float,
    pub fvrectx: libc::c_float,
    pub fvrecty: libc::c_float,
    pub fvrectx_adj: libc::c_float,
    pub fvrecty_adj: libc::c_float,
    pub vrect_x_adj_shift20: libc::c_int,
    pub vrectright_adj_shift20: libc::c_int,
    pub fvrectright_adj: libc::c_float,
    pub fvrectbottom_adj: libc::c_float,
    pub fvrectright: libc::c_float,
    pub fvrectbottom: libc::c_float,
    pub horizontalFieldOfView: libc::c_float,
    pub xOrigin: libc::c_float,
    pub yOrigin: libc::c_float,
    pub vieworg: vec3_t,
    pub viewangles: vec3_t,
    pub ambientlight: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct espan_s {
    pub u: libc::c_int,
    pub v: libc::c_int,
    pub count: libc::c_int,
    pub pnext: *mut espan_s,
}
pub type espan_t = espan_s;
#[no_mangle]
pub static mut r_turb_pbase: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut r_turb_pdest: *mut libc::c_uchar = 0 as *const libc::c_uchar
    as *mut libc::c_uchar;
#[no_mangle]
pub static mut r_turb_s: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_t: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_sstep: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_tstep: fixed16_t = 0;
#[no_mangle]
pub static mut r_turb_turb: *mut libc::c_int = 0 as *const libc::c_int
    as *mut libc::c_int;
#[no_mangle]
pub static mut r_turb_spancount: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn D_WarpScreen() {
    let mut w: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut u2: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut dest: *mut byte = 0 as *mut byte;
    let mut turb: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut col: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut row: *mut *mut byte = 0 as *mut *mut byte;
    static mut cached_width: libc::c_int = 0;
    static mut cached_height: libc::c_int = 0;
    static mut rowptr: [*mut byte; 1206] = [0 as *const byte as *mut byte; 1206];
    static mut column: [libc::c_int; 1606] = [0; 1606];
    w = r_newrefdef.width;
    h = r_newrefdef.height;
    if w != cached_width || h != cached_height {
        cached_width = w;
        cached_height = h;
        v = 0 as libc::c_int;
        while v < h + 3 as libc::c_int * 2 as libc::c_int {
            v2 = (v as libc::c_float
                / (h + 3 as libc::c_int * 2 as libc::c_int) as libc::c_float
                * r_refdef.vrect.height as libc::c_float) as libc::c_int;
            rowptr[v
                as usize] = r_warpbuffer
                .as_mut_ptr()
                .offset((320 as libc::c_int * v2) as isize);
            v += 1;
        }
        u = 0 as libc::c_int;
        while u < w + 3 as libc::c_int * 2 as libc::c_int {
            u2 = (u as libc::c_float
                / (w + 3 as libc::c_int * 2 as libc::c_int) as libc::c_float
                * r_refdef.vrect.width as libc::c_float) as libc::c_int;
            column[u as usize] = u2;
            u += 1;
        }
    }
    turb = intsintable
        .as_mut_ptr()
        .offset(
            ((r_newrefdef.time * 20 as libc::c_int as libc::c_float) as libc::c_int
                & 128 as libc::c_int - 1 as libc::c_int) as isize,
        );
    dest = (vid.buffer)
        .offset((r_newrefdef.y * vid.rowbytes) as isize)
        .offset(r_newrefdef.x as isize);
    v = 0 as libc::c_int;
    while v < h {
        col = &mut *column.as_mut_ptr().offset(*turb.offset(v as isize) as isize)
            as *mut libc::c_int;
        row = &mut *rowptr.as_mut_ptr().offset(v as isize) as *mut *mut byte;
        u = 0 as libc::c_int;
        while u < w {
            *dest
                .offset(
                    (u + 0 as libc::c_int) as isize,
                ) = *(*row
                .offset(*turb.offset((u + 0 as libc::c_int) as isize) as isize))
                .offset(*col.offset((u + 0 as libc::c_int) as isize) as isize);
            *dest
                .offset(
                    (u + 1 as libc::c_int) as isize,
                ) = *(*row
                .offset(*turb.offset((u + 1 as libc::c_int) as isize) as isize))
                .offset(*col.offset((u + 1 as libc::c_int) as isize) as isize);
            *dest
                .offset(
                    (u + 2 as libc::c_int) as isize,
                ) = *(*row
                .offset(*turb.offset((u + 2 as libc::c_int) as isize) as isize))
                .offset(*col.offset((u + 2 as libc::c_int) as isize) as isize);
            *dest
                .offset(
                    (u + 3 as libc::c_int) as isize,
                ) = *(*row
                .offset(*turb.offset((u + 3 as libc::c_int) as isize) as isize))
                .offset(*col.offset((u + 3 as libc::c_int) as isize) as isize);
            u += 4 as libc::c_int;
        }
        v += 1;
        dest = dest.offset(vid.rowbytes as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn D_DrawTurbulent8Span() {
    let mut sturb: libc::c_int = 0;
    let mut tturb: libc::c_int = 0;
    loop {
        sturb = r_turb_s
            + *r_turb_turb
                .offset(
                    (r_turb_t >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        tturb = r_turb_t
            + *r_turb_turb
                .offset(
                    (r_turb_s >> 16 as libc::c_int
                        & 128 as libc::c_int - 1 as libc::c_int) as isize,
                ) >> 16 as libc::c_int & 63 as libc::c_int;
        let fresh0 = r_turb_pdest;
        r_turb_pdest = r_turb_pdest.offset(1);
        *fresh0 = *r_turb_pbase
            .offset((tturb << 6 as libc::c_int) as isize)
            .offset(sturb as isize);
        r_turb_s += r_turb_sstep;
        r_turb_t += r_turb_tstep;
        r_turb_spancount -= 1;
        if !(r_turb_spancount > 0 as libc::c_int) {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Turbulent8(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz16stepu: libc::c_float = 0.;
    let mut tdivz16stepu: libc::c_float = 0.;
    let mut zi16stepu: libc::c_float = 0.;
    r_turb_turb = sintable
        .as_mut_ptr()
        .offset(
            ((r_newrefdef.time * 20 as libc::c_int as libc::c_float) as libc::c_int
                & 128 as libc::c_int - 1 as libc::c_int) as isize,
        );
    r_turb_sstep = 0 as libc::c_int;
    r_turb_tstep = 0 as libc::c_int;
    r_turb_pbase = cacheblock as *mut libc::c_uchar;
    sdivz16stepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivz16stepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zi16stepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    loop {
        r_turb_pdest = (d_viewbuffer as *mut byte)
            .offset((r_screenwidth * (*pspan).v) as isize)
            .offset((*pspan).u as isize) as *mut libc::c_uchar;
        count = (*pspan).count;
        du = (*pspan).u as libc::c_float;
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        r_turb_s = (sdivz * z) as libc::c_int + sadjust;
        if r_turb_s > bbextents {
            r_turb_s = bbextents;
        } else if r_turb_s < 0 as libc::c_int {
            r_turb_s = 0 as libc::c_int;
        }
        r_turb_t = (tdivz * z) as libc::c_int + tadjust;
        if r_turb_t > bbextentt {
            r_turb_t = bbextentt;
        } else if r_turb_t < 0 as libc::c_int {
            r_turb_t = 0 as libc::c_int;
        }
        loop {
            if count >= 16 as libc::c_int {
                r_turb_spancount = 16 as libc::c_int;
            } else {
                r_turb_spancount = count;
            }
            count -= r_turb_spancount;
            if count != 0 {
                sdivz += sdivz16stepu;
                tdivz += tdivz16stepu;
                zi += zi16stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int;
                }
                r_turb_sstep = snext - r_turb_s >> 4 as libc::c_int;
                r_turb_tstep = tnext - r_turb_t >> 4 as libc::c_int;
            } else {
                spancountminus1 = (r_turb_spancount - 1 as libc::c_int) as libc::c_float;
                sdivz += d_sdivzstepu * spancountminus1;
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int;
                }
                if r_turb_spancount > 1 as libc::c_int {
                    r_turb_sstep = (snext - r_turb_s)
                        / (r_turb_spancount - 1 as libc::c_int);
                    r_turb_tstep = (tnext - r_turb_t)
                        / (r_turb_spancount - 1 as libc::c_int);
                }
            }
            r_turb_s = r_turb_s
                & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
            r_turb_t = r_turb_t
                & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
            D_DrawTurbulent8Span();
            r_turb_s = snext;
            r_turb_t = tnext;
            if !(count > 0 as libc::c_int) {
                break;
            }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn NonTurbulent8(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz16stepu: libc::c_float = 0.;
    let mut tdivz16stepu: libc::c_float = 0.;
    let mut zi16stepu: libc::c_float = 0.;
    r_turb_turb = blanktable.as_mut_ptr();
    r_turb_sstep = 0 as libc::c_int;
    r_turb_tstep = 0 as libc::c_int;
    r_turb_pbase = cacheblock as *mut libc::c_uchar;
    sdivz16stepu = d_sdivzstepu * 16 as libc::c_int as libc::c_float;
    tdivz16stepu = d_tdivzstepu * 16 as libc::c_int as libc::c_float;
    zi16stepu = d_zistepu * 16 as libc::c_int as libc::c_float;
    loop {
        r_turb_pdest = (d_viewbuffer as *mut byte)
            .offset((r_screenwidth * (*pspan).v) as isize)
            .offset((*pspan).u as isize) as *mut libc::c_uchar;
        count = (*pspan).count;
        du = (*pspan).u as libc::c_float;
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        r_turb_s = (sdivz * z) as libc::c_int + sadjust;
        if r_turb_s > bbextents {
            r_turb_s = bbextents;
        } else if r_turb_s < 0 as libc::c_int {
            r_turb_s = 0 as libc::c_int;
        }
        r_turb_t = (tdivz * z) as libc::c_int + tadjust;
        if r_turb_t > bbextentt {
            r_turb_t = bbextentt;
        } else if r_turb_t < 0 as libc::c_int {
            r_turb_t = 0 as libc::c_int;
        }
        loop {
            if count >= 16 as libc::c_int {
                r_turb_spancount = 16 as libc::c_int;
            } else {
                r_turb_spancount = count;
            }
            count -= r_turb_spancount;
            if count != 0 {
                sdivz += sdivz16stepu;
                tdivz += tdivz16stepu;
                zi += zi16stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int;
                }
                r_turb_sstep = snext - r_turb_s >> 4 as libc::c_int;
                r_turb_tstep = tnext - r_turb_t >> 4 as libc::c_int;
            } else {
                spancountminus1 = (r_turb_spancount - 1 as libc::c_int) as libc::c_float;
                sdivz += d_sdivzstepu * spancountminus1;
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 16 as libc::c_int {
                    snext = 16 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 16 as libc::c_int {
                    tnext = 16 as libc::c_int;
                }
                if r_turb_spancount > 1 as libc::c_int {
                    r_turb_sstep = (snext - r_turb_s)
                        / (r_turb_spancount - 1 as libc::c_int);
                    r_turb_tstep = (tnext - r_turb_t)
                        / (r_turb_spancount - 1 as libc::c_int);
                }
            }
            r_turb_s = r_turb_s
                & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
            r_turb_t = r_turb_t
                & ((128 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
            D_DrawTurbulent8Span();
            r_turb_s = snext;
            r_turb_t = tnext;
            if !(count > 0 as libc::c_int) {
                break;
            }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn D_DrawSpans16(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut spancount: libc::c_int = 0;
    let mut pbase: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut pdest: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut s: fixed16_t = 0;
    let mut t: fixed16_t = 0;
    let mut snext: fixed16_t = 0;
    let mut tnext: fixed16_t = 0;
    let mut sstep: fixed16_t = 0;
    let mut tstep: fixed16_t = 0;
    let mut sdivz: libc::c_float = 0.;
    let mut tdivz: libc::c_float = 0.;
    let mut zi: libc::c_float = 0.;
    let mut z: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    let mut spancountminus1: libc::c_float = 0.;
    let mut sdivz8stepu: libc::c_float = 0.;
    let mut tdivz8stepu: libc::c_float = 0.;
    let mut zi8stepu: libc::c_float = 0.;
    sstep = 0 as libc::c_int;
    tstep = 0 as libc::c_int;
    pbase = cacheblock as *mut libc::c_uchar;
    sdivz8stepu = d_sdivzstepu * 8 as libc::c_int as libc::c_float;
    tdivz8stepu = d_tdivzstepu * 8 as libc::c_int as libc::c_float;
    zi8stepu = d_zistepu * 8 as libc::c_int as libc::c_float;
    loop {
        pdest = (d_viewbuffer as *mut byte)
            .offset((r_screenwidth * (*pspan).v) as isize)
            .offset((*pspan).u as isize) as *mut libc::c_uchar;
        count = (*pspan).count;
        du = (*pspan).u as libc::c_float;
        dv = (*pspan).v as libc::c_float;
        sdivz = d_sdivzorigin + dv * d_sdivzstepv + du * d_sdivzstepu;
        tdivz = d_tdivzorigin + dv * d_tdivzstepv + du * d_tdivzstepu;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        z = 0x10000 as libc::c_int as libc::c_float / zi;
        s = (sdivz * z) as libc::c_int + sadjust;
        if s > bbextents {
            s = bbextents;
        } else if s < 0 as libc::c_int {
            s = 0 as libc::c_int;
        }
        t = (tdivz * z) as libc::c_int + tadjust;
        if t > bbextentt {
            t = bbextentt;
        } else if t < 0 as libc::c_int {
            t = 0 as libc::c_int;
        }
        loop {
            if count >= 8 as libc::c_int {
                spancount = 8 as libc::c_int;
            } else {
                spancount = count;
            }
            count -= spancount;
            if count != 0 {
                sdivz += sdivz8stepu;
                tdivz += tdivz8stepu;
                zi += zi8stepu;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int;
                }
                sstep = snext - s >> 3 as libc::c_int;
                tstep = tnext - t >> 3 as libc::c_int;
            } else {
                spancountminus1 = (spancount - 1 as libc::c_int) as libc::c_float;
                sdivz += d_sdivzstepu * spancountminus1;
                tdivz += d_tdivzstepu * spancountminus1;
                zi += d_zistepu * spancountminus1;
                z = 0x10000 as libc::c_int as libc::c_float / zi;
                snext = (sdivz * z) as libc::c_int + sadjust;
                if snext > bbextents {
                    snext = bbextents;
                } else if snext < 8 as libc::c_int {
                    snext = 8 as libc::c_int;
                }
                tnext = (tdivz * z) as libc::c_int + tadjust;
                if tnext > bbextentt {
                    tnext = bbextentt;
                } else if tnext < 8 as libc::c_int {
                    tnext = 8 as libc::c_int;
                }
                if spancount > 1 as libc::c_int {
                    sstep = (snext - s) / (spancount - 1 as libc::c_int);
                    tstep = (tnext - t) / (spancount - 1 as libc::c_int);
                }
            }
            loop {
                let fresh1 = pdest;
                pdest = pdest.offset(1);
                *fresh1 = *pbase
                    .offset((s >> 16 as libc::c_int) as isize)
                    .offset(((t >> 16 as libc::c_int) * cachewidth) as isize);
                s += sstep;
                t += tstep;
                spancount -= 1;
                if !(spancount > 0 as libc::c_int) {
                    break;
                }
            }
            s = snext;
            t = tnext;
            if !(count > 0 as libc::c_int) {
                break;
            }
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn D_DrawZSpans(mut pspan: *mut espan_t) {
    let mut count: libc::c_int = 0;
    let mut doublecount: libc::c_int = 0;
    let mut izistep: libc::c_int = 0;
    let mut izi: libc::c_int = 0;
    let mut pdest: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut ltemp: libc::c_uint = 0;
    let mut zi: libc::c_float = 0.;
    let mut du: libc::c_float = 0.;
    let mut dv: libc::c_float = 0.;
    izistep = (d_zistepu * 0x8000 as libc::c_int as libc::c_float
        * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
    loop {
        pdest = d_pzbuffer
            .offset(d_zwidth.wrapping_mul((*pspan).v as libc::c_uint) as isize)
            .offset((*pspan).u as isize);
        count = (*pspan).count;
        du = (*pspan).u as libc::c_float;
        dv = (*pspan).v as libc::c_float;
        zi = d_ziorigin + dv * d_zistepv + du * d_zistepu;
        izi = (zi * 0x8000 as libc::c_int as libc::c_float
            * 0x10000 as libc::c_int as libc::c_float) as libc::c_int;
        if pdest as libc::c_long & 0x2 as libc::c_int as libc::c_long != 0 {
            let fresh2 = pdest;
            pdest = pdest.offset(1);
            *fresh2 = (izi >> 16 as libc::c_int) as libc::c_short;
            izi += izistep;
            count -= 1;
        }
        doublecount = count >> 1 as libc::c_int;
        if doublecount > 0 as libc::c_int {
            loop {
                ltemp = (izi >> 16 as libc::c_int) as libc::c_uint;
                izi += izistep;
                ltemp |= izi as libc::c_uint & 0xffff0000 as libc::c_uint;
                izi += izistep;
                *(pdest as *mut libc::c_int) = ltemp as libc::c_int;
                pdest = pdest.offset(2 as libc::c_int as isize);
                doublecount -= 1;
                if !(doublecount > 0 as libc::c_int) {
                    break;
                }
            }
        }
        if count & 1 as libc::c_int != 0 {
            *pdest = (izi >> 16 as libc::c_int) as libc::c_short;
        }
        pspan = (*pspan).pnext;
        if pspan.is_null() {
            break;
        }
    };
}
