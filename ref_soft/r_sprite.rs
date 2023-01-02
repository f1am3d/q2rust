#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    static mut vup: vec3_t;
    static mut vpn: vec3_t;
    static mut vright: vec3_t;
    static mut currentmodel: *mut model_t;
    static mut currententity: *mut entity_t;
    static mut modelorg: vec3_t;
    static mut r_entorigin: vec3_t;
    fn VectorInverse(v: *mut vec_t);
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    static mut r_polydesc: polydesc_t;
    static mut r_clip_verts: [[vec5_t; 70]; 2];
    fn R_ClipAndDrawPoly(
        alpha: libc::c_float,
        isturbulent: qboolean,
        textured: qboolean,
    );
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
pub type vec5_t = [vec_t; 5];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprframe_t {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub origin_x: libc::c_int,
    pub origin_y: libc::c_int,
    pub name: [libc::c_char; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dsprite_t {
    pub ident: libc::c_int,
    pub version: libc::c_int,
    pub numframes: libc::c_int,
    pub frames: [dsprframe_t; 1],
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
pub type model_t = model_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct emitpoint_t {
    pub u: libc::c_float,
    pub v: libc::c_float,
    pub s: libc::c_float,
    pub t: libc::c_float,
    pub zi: libc::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct polydesc_t {
    pub nump: libc::c_int,
    pub pverts: *mut emitpoint_t,
    pub pixels: *mut byte,
    pub pixel_width: libc::c_int,
    pub pixel_height: libc::c_int,
    pub vup: vec3_t,
    pub vright: vec3_t,
    pub vpn: vec3_t,
    pub dist: libc::c_float,
    pub s_offset: libc::c_float,
    pub t_offset: libc::c_float,
    pub viewer_position: [libc::c_float; 3],
    pub drawspanlet: Option::<unsafe extern "C" fn() -> ()>,
    pub stipple_parity: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawSprite() {
    let mut pverts: *mut vec5_t = 0 as *mut vec5_t;
    let mut left: vec3_t = [0.; 3];
    let mut up: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut down: vec3_t = [0.; 3];
    let mut s_psprite: *mut dsprite_t = 0 as *mut dsprite_t;
    let mut s_psprframe: *mut dsprframe_t = 0 as *mut dsprframe_t;
    s_psprite = (*currentmodel).extradata as *mut dsprite_t;
    (*currententity).frame %= (*s_psprite).numframes;
    s_psprframe = &mut *((*s_psprite).frames)
        .as_mut_ptr()
        .offset((*currententity).frame as isize) as *mut dsprframe_t;
    r_polydesc
        .pixels = (*(*currentmodel).skins[(*currententity).frame as usize])
        .pixels[0 as libc::c_int as usize];
    r_polydesc.pixel_width = (*s_psprframe).width;
    r_polydesc.pixel_height = (*s_psprframe).height;
    r_polydesc.dist = 0 as libc::c_int as libc::c_float;
    r_polydesc.vup[0 as libc::c_int as usize] = vup[0 as libc::c_int as usize];
    r_polydesc.vup[1 as libc::c_int as usize] = vup[1 as libc::c_int as usize];
    r_polydesc.vup[2 as libc::c_int as usize] = vup[2 as libc::c_int as usize];
    r_polydesc.vright[0 as libc::c_int as usize] = vright[0 as libc::c_int as usize];
    r_polydesc.vright[1 as libc::c_int as usize] = vright[1 as libc::c_int as usize];
    r_polydesc.vright[2 as libc::c_int as usize] = vright[2 as libc::c_int as usize];
    r_polydesc.vpn[0 as libc::c_int as usize] = vpn[0 as libc::c_int as usize];
    r_polydesc.vpn[1 as libc::c_int as usize] = vpn[1 as libc::c_int as usize];
    r_polydesc.vpn[2 as libc::c_int as usize] = vpn[2 as libc::c_int as usize];
    VectorScale(
        (r_polydesc.vright).as_mut_ptr(),
        ((*s_psprframe).width - (*s_psprframe).origin_x) as vec_t,
        right.as_mut_ptr(),
    );
    VectorScale(
        (r_polydesc.vup).as_mut_ptr(),
        ((*s_psprframe).height - (*s_psprframe).origin_y) as vec_t,
        up.as_mut_ptr(),
    );
    VectorScale(
        (r_polydesc.vright).as_mut_ptr(),
        -(*s_psprframe).origin_x as vec_t,
        left.as_mut_ptr(),
    );
    VectorScale(
        (r_polydesc.vup).as_mut_ptr(),
        -(*s_psprframe).origin_y as vec_t,
        down.as_mut_ptr(),
    );
    VectorInverse((r_polydesc.vup).as_mut_ptr());
    pverts = (r_clip_verts[0 as libc::c_int as usize]).as_mut_ptr();
    (*pverts
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = r_entorigin[0 as libc::c_int as usize]
        + up[0 as libc::c_int as usize] + left[0 as libc::c_int as usize];
    (*pverts
        .offset(
            0 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = r_entorigin[1 as libc::c_int as usize]
        + up[1 as libc::c_int as usize] + left[1 as libc::c_int as usize];
    (*pverts
        .offset(
            0 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = r_entorigin[2 as libc::c_int as usize]
        + up[2 as libc::c_int as usize] + left[2 as libc::c_int as usize];
    (*pverts
        .offset(
            0 as libc::c_int as isize,
        ))[3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pverts
        .offset(
            0 as libc::c_int as isize,
        ))[4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pverts
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = r_entorigin[0 as libc::c_int as usize]
        + up[0 as libc::c_int as usize] + right[0 as libc::c_int as usize];
    (*pverts
        .offset(
            1 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = r_entorigin[1 as libc::c_int as usize]
        + up[1 as libc::c_int as usize] + right[1 as libc::c_int as usize];
    (*pverts
        .offset(
            1 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = r_entorigin[2 as libc::c_int as usize]
        + up[2 as libc::c_int as usize] + right[2 as libc::c_int as usize];
    (*pverts
        .offset(
            1 as libc::c_int as isize,
        ))[3 as libc::c_int as usize] = (*s_psprframe).width as vec_t;
    (*pverts
        .offset(
            1 as libc::c_int as isize,
        ))[4 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pverts
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = r_entorigin[0 as libc::c_int as usize]
        + down[0 as libc::c_int as usize] + right[0 as libc::c_int as usize];
    (*pverts
        .offset(
            2 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = r_entorigin[1 as libc::c_int as usize]
        + down[1 as libc::c_int as usize] + right[1 as libc::c_int as usize];
    (*pverts
        .offset(
            2 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = r_entorigin[2 as libc::c_int as usize]
        + down[2 as libc::c_int as usize] + right[2 as libc::c_int as usize];
    (*pverts
        .offset(
            2 as libc::c_int as isize,
        ))[3 as libc::c_int as usize] = (*s_psprframe).width as vec_t;
    (*pverts
        .offset(
            2 as libc::c_int as isize,
        ))[4 as libc::c_int as usize] = (*s_psprframe).height as vec_t;
    (*pverts
        .offset(
            3 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = r_entorigin[0 as libc::c_int as usize]
        + down[0 as libc::c_int as usize] + left[0 as libc::c_int as usize];
    (*pverts
        .offset(
            3 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = r_entorigin[1 as libc::c_int as usize]
        + down[1 as libc::c_int as usize] + left[1 as libc::c_int as usize];
    (*pverts
        .offset(
            3 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = r_entorigin[2 as libc::c_int as usize]
        + down[2 as libc::c_int as usize] + left[2 as libc::c_int as usize];
    (*pverts
        .offset(
            3 as libc::c_int as isize,
        ))[3 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*pverts
        .offset(
            3 as libc::c_int as isize,
        ))[4 as libc::c_int as usize] = (*s_psprframe).height as vec_t;
    r_polydesc.nump = 4 as libc::c_int;
    r_polydesc.s_offset = (r_polydesc.pixel_width >> 1 as libc::c_int) as libc::c_float;
    r_polydesc.t_offset = (r_polydesc.pixel_height >> 1 as libc::c_int) as libc::c_float;
    r_polydesc
        .viewer_position[0 as libc::c_int
        as usize] = modelorg[0 as libc::c_int as usize];
    r_polydesc
        .viewer_position[1 as libc::c_int
        as usize] = modelorg[1 as libc::c_int as usize];
    r_polydesc
        .viewer_position[2 as libc::c_int
        as usize] = modelorg[2 as libc::c_int as usize];
    r_polydesc.stipple_parity = 1 as libc::c_int;
    if (*currententity).flags & 32 as libc::c_int != 0 {
        R_ClipAndDrawPoly((*currententity).alpha, false_0, true_0);
    } else {
        R_ClipAndDrawPoly(1.0f32, false_0, true_0);
    }
    r_polydesc.stipple_parity = 0 as libc::c_int;
}
