#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    static mut vid: viddef_t;
    static mut r_screenwidth: libc::c_int;
    static mut d_zwidth: libc::c_uint;
    static mut d_pix_max: libc::c_int;
    static mut d_pix_min: libc::c_int;
    static mut d_pix_shift: libc::c_int;
    static mut d_scantable: [libc::c_int; 1200];
    static mut d_viewbuffer: *mut pixel_t;
    static mut d_pzbuffer: *mut libc::c_short;
    static mut d_vrectx: libc::c_int;
    static mut d_vrecty: libc::c_int;
    static mut d_vrectright_particle: libc::c_int;
    static mut d_vrectbottom_particle: libc::c_int;
    static mut ycenter: libc::c_float;
    static mut xcenter: libc::c_float;
    static mut r_origin: vec3_t;
    static mut vup: vec3_t;
    static mut vpn: vec3_t;
    static mut vright: vec3_t;
    static mut xscaleshrink: libc::c_float;
    static mut yscaleshrink: libc::c_float;
    static mut r_newrefdef: refdef_t;
}
pub type byte = libc::c_uchar;
pub type qboolean = libc::c_uint;
pub const true_0: qboolean = 1;
pub const false_0: qboolean = 0;
pub type vec_t = libc::c_float;
pub type vec3_t = [vec_t; 3];
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
pub struct partparms_t {
    pub particle: *mut particle_t,
    pub level: libc::c_int,
    pub color: libc::c_int,
}
#[no_mangle]
pub static mut r_pright: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_pup: vec3_t = [0.; 3];
#[no_mangle]
pub static mut r_ppn: vec3_t = [0.; 3];
static mut partparms: partparms_t = partparms_t {
    particle: 0 as *const particle_t as *mut particle_t,
    level: 0,
    color: 0,
};
unsafe extern "C" fn BlendParticle33(
    mut pcolor: libc::c_int,
    mut dstcolor: libc::c_int,
) -> byte {
    return *(vid.alphamap).offset((pcolor + dstcolor * 256 as libc::c_int) as isize);
}
unsafe extern "C" fn BlendParticle66(
    mut pcolor: libc::c_int,
    mut dstcolor: libc::c_int,
) -> byte {
    return *(vid.alphamap).offset((pcolor * 256 as libc::c_int + dstcolor) as isize);
}
unsafe extern "C" fn BlendParticle100(
    mut pcolor: libc::c_int,
    mut dstcolor: libc::c_int,
) -> byte {
    dstcolor = dstcolor;
    return pcolor as byte;
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawParticle() {
    let mut pparticle: *mut particle_t = partparms.particle;
    let mut level: libc::c_int = partparms.level;
    let mut local: vec3_t = [0.; 3];
    let mut transformed: vec3_t = [0.; 3];
    let mut zi: libc::c_float = 0.;
    let mut pdest: *mut byte = 0 as *mut byte;
    let mut pz: *mut libc::c_short = 0 as *mut libc::c_short;
    let mut color: libc::c_int = (*pparticle).color;
    let mut i: libc::c_int = 0;
    let mut izi: libc::c_int = 0;
    let mut pix: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut u: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut blendparticle: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> byte,
    > = None;
    local[0 as libc::c_int
        as usize] = (*pparticle).origin[0 as libc::c_int as usize]
        - r_origin[0 as libc::c_int as usize];
    local[1 as libc::c_int
        as usize] = (*pparticle).origin[1 as libc::c_int as usize]
        - r_origin[1 as libc::c_int as usize];
    local[2 as libc::c_int
        as usize] = (*pparticle).origin[2 as libc::c_int as usize]
        - r_origin[2 as libc::c_int as usize];
    transformed[0 as libc::c_int
        as usize] = local[0 as libc::c_int as usize]
        * r_pright[0 as libc::c_int as usize]
        + local[1 as libc::c_int as usize] * r_pright[1 as libc::c_int as usize]
        + local[2 as libc::c_int as usize] * r_pright[2 as libc::c_int as usize];
    transformed[1 as libc::c_int
        as usize] = local[0 as libc::c_int as usize] * r_pup[0 as libc::c_int as usize]
        + local[1 as libc::c_int as usize] * r_pup[1 as libc::c_int as usize]
        + local[2 as libc::c_int as usize] * r_pup[2 as libc::c_int as usize];
    transformed[2 as libc::c_int
        as usize] = local[0 as libc::c_int as usize] * r_ppn[0 as libc::c_int as usize]
        + local[1 as libc::c_int as usize] * r_ppn[1 as libc::c_int as usize]
        + local[2 as libc::c_int as usize] * r_ppn[2 as libc::c_int as usize];
    if (transformed[2 as libc::c_int as usize] as libc::c_double) < 8.0f64 {
        return;
    }
    if level == 0 as libc::c_int {
        blendparticle = Some(
            BlendParticle33 as unsafe extern "C" fn(libc::c_int, libc::c_int) -> byte,
        );
    } else if level == 1 as libc::c_int {
        blendparticle = Some(
            BlendParticle66 as unsafe extern "C" fn(libc::c_int, libc::c_int) -> byte,
        );
    } else {
        blendparticle = Some(
            BlendParticle100 as unsafe extern "C" fn(libc::c_int, libc::c_int) -> byte,
        );
    }
    zi = (1.0f64 / transformed[2 as libc::c_int as usize] as libc::c_double)
        as libc::c_float;
    u = ((xcenter + zi * transformed[0 as libc::c_int as usize]) as libc::c_double
        + 0.5f64) as libc::c_int;
    v = ((ycenter - zi * transformed[1 as libc::c_int as usize]) as libc::c_double
        + 0.5f64) as libc::c_int;
    if v > d_vrectbottom_particle || u > d_vrectright_particle || v < d_vrecty
        || u < d_vrectx
    {
        return;
    }
    pz = d_pzbuffer
        .offset(d_zwidth.wrapping_mul(v as libc::c_uint) as isize)
        .offset(u as isize);
    pdest = d_viewbuffer.offset(d_scantable[v as usize] as isize).offset(u as isize);
    izi = (zi * 0x8000 as libc::c_int as libc::c_float) as libc::c_int;
    pix = izi >> d_pix_shift;
    if pix < d_pix_min {
        pix = d_pix_min;
    } else if pix > d_pix_max {
        pix = d_pix_max;
    }
    count = pix;
    match level {
        0 => {
            while count != 0 {
                i = 0 as libc::c_int;
                while i < pix {
                    if *pz.offset(i as isize) as libc::c_int <= izi {
                        *pz.offset(i as isize) = izi as libc::c_short;
                        *pdest
                            .offset(
                                i as isize,
                            ) = *(vid.alphamap)
                            .offset(
                                (color
                                    + ((*pdest.offset(i as isize) as libc::c_int)
                                        << 8 as libc::c_int)) as isize,
                            );
                    }
                    i += 1;
                }
                count -= 1;
                pz = pz.offset(d_zwidth as isize);
                pdest = pdest.offset(r_screenwidth as isize);
            }
        }
        1 => {
            while count != 0 {
                i = 0 as libc::c_int;
                while i < pix {
                    if *pz.offset(i as isize) as libc::c_int <= izi {
                        *pz.offset(i as isize) = izi as libc::c_short;
                        *pdest
                            .offset(
                                i as isize,
                            ) = *(vid.alphamap)
                            .offset(
                                ((color << 8 as libc::c_int)
                                    + *pdest.offset(i as isize) as libc::c_int) as isize,
                            );
                    }
                    i += 1;
                }
                count -= 1;
                pz = pz.offset(d_zwidth as isize);
                pdest = pdest.offset(r_screenwidth as isize);
            }
        }
        _ => {
            while count != 0 {
                i = 0 as libc::c_int;
                while i < pix {
                    if *pz.offset(i as isize) as libc::c_int <= izi {
                        *pz.offset(i as isize) = izi as libc::c_short;
                        *pdest.offset(i as isize) = color as byte;
                    }
                    i += 1;
                }
                count -= 1;
                pz = pz.offset(d_zwidth as isize);
                pdest = pdest.offset(r_screenwidth as isize);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn R_DrawParticles() {
    let mut p: *mut particle_t = 0 as *mut particle_t;
    let mut i: libc::c_int = 0;
    extern "C" {
        static mut fpu_sp24_cw: libc::c_ulong;
    }
    extern "C" {
        static mut fpu_chop_cw: libc::c_ulong;
    }
    VectorScale(vright.as_mut_ptr(), xscaleshrink, r_pright.as_mut_ptr());
    VectorScale(vup.as_mut_ptr(), yscaleshrink, r_pup.as_mut_ptr());
    r_ppn[0 as libc::c_int as usize] = vpn[0 as libc::c_int as usize];
    r_ppn[1 as libc::c_int as usize] = vpn[1 as libc::c_int as usize];
    r_ppn[2 as libc::c_int as usize] = vpn[2 as libc::c_int as usize];
    p = r_newrefdef.particles;
    i = 0 as libc::c_int;
    while i < r_newrefdef.num_particles {
        if (*p).alpha as libc::c_double > 0.66f64 {
            partparms.level = 2 as libc::c_int;
        } else if (*p).alpha as libc::c_double > 0.33f64 {
            partparms.level = 1 as libc::c_int;
        } else {
            partparms.level = 0 as libc::c_int;
        }
        partparms.particle = p;
        partparms.color = (*p).color;
        R_DrawParticle();
        i += 1;
        p = p.offset(1);
    }
}
