#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn Com_PageInMemory(buffer: *mut byte, size: libc::c_int);
    fn LittleShort(l: libc::c_short) -> libc::c_short;
    static mut ri: refimport_t;
    static mut registration_sequence: libc::c_int;
    static mut r_notexture_mip: *mut image_t;
    fn LittleLong(l: libc::c_int) -> libc::c_int;
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
pub struct pcx_t {
    pub manufacturer: libc::c_char,
    pub version: libc::c_char,
    pub encoding: libc::c_char,
    pub bits_per_pixel: libc::c_char,
    pub xmin: libc::c_ushort,
    pub ymin: libc::c_ushort,
    pub xmax: libc::c_ushort,
    pub ymax: libc::c_ushort,
    pub hres: libc::c_ushort,
    pub vres: libc::c_ushort,
    pub palette: [libc::c_uchar; 48],
    pub reserved: libc::c_char,
    pub color_planes: libc::c_char,
    pub bytes_per_line: libc::c_ushort,
    pub palette_type: libc::c_ushort,
    pub filler: [libc::c_char; 58],
    pub data: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct miptex_s {
    pub name: [libc::c_char; 32],
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub offsets: [libc::c_uint; 4],
    pub animname: [libc::c_char; 32],
    pub flags: libc::c_int,
    pub contents: libc::c_int,
    pub value: libc::c_int,
}
pub type miptex_t = miptex_s;
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
pub type image_t = image_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct refimport_t {
    pub Sys_Error: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub Cmd_AddCommand: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            Option::<unsafe extern "C" fn() -> ()>,
        ) -> (),
    >,
    pub Cmd_RemoveCommand: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub Cmd_Argc: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub Cmd_Argv: Option::<unsafe extern "C" fn(libc::c_int) -> *mut libc::c_char>,
    pub Cmd_ExecuteText: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
    >,
    pub Con_Printf: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub FS_LoadFile: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut *mut libc::c_void) -> libc::c_int,
    >,
    pub FS_FreeFile: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub FS_Gamedir: Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
    pub Cvar_Get: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> *mut cvar_t,
    >,
    pub Cvar_Set: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub Cvar_SetValue: Option::<
        unsafe extern "C" fn(*mut libc::c_char, libc::c_float) -> (),
    >,
    pub Vid_GetModeInfo: Option::<
        unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int, libc::c_int) -> qboolean,
    >,
    pub Vid_MenuInit: Option::<unsafe extern "C" fn() -> ()>,
    pub Vid_NewWindow: Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _TargaHeader {
    pub id_length: libc::c_uchar,
    pub colormap_type: libc::c_uchar,
    pub image_type: libc::c_uchar,
    pub colormap_index: libc::c_ushort,
    pub colormap_length: libc::c_ushort,
    pub colormap_size: libc::c_uchar,
    pub x_origin: libc::c_ushort,
    pub y_origin: libc::c_ushort,
    pub width: libc::c_ushort,
    pub height: libc::c_ushort,
    pub pixel_size: libc::c_uchar,
    pub attributes: libc::c_uchar,
}
pub type TargaHeader = _TargaHeader;
#[no_mangle]
pub static mut r_images: [image_t; 1024] = [image_t {
    name: [0; 64],
    type_0: it_skin,
    width: 0,
    height: 0,
    transparent: false_0,
    registration_sequence: 0,
    pixels: [0 as *const byte as *mut byte; 4],
}; 1024];
#[no_mangle]
pub static mut numr_images: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn R_ImageList_f() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut texels: libc::c_int = 0;
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"------------------\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    texels = 0 as libc::c_int;
    i = 0 as libc::c_int;
    image = r_images.as_mut_ptr();
    while i < numr_images {
        if !((*image).registration_sequence <= 0 as libc::c_int) {
            texels += (*image).width * (*image).height;
            match (*image).type_0 as libc::c_uint {
                0 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"M\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                1 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"S\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                2 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"W\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                3 => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b"P\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                _ => {
                    (ri.Con_Printf)
                        .expect(
                            "non-null function pointer",
                        )(
                        0 as libc::c_int,
                        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
            }
            (ri.Con_Printf)
                .expect(
                    "non-null function pointer",
                )(
                0 as libc::c_int,
                b" %3i %3i : %s\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*image).width,
                (*image).height,
                ((*image).name).as_mut_ptr(),
            );
        }
        i += 1;
        image = image.offset(1);
    }
    (ri.Con_Printf)
        .expect(
            "non-null function pointer",
        )(
        0 as libc::c_int,
        b"Total texel count: %i\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        texels,
    );
}
#[no_mangle]
pub unsafe extern "C" fn LoadPCX(
    mut filename: *mut libc::c_char,
    mut pic: *mut *mut byte,
    mut palette: *mut *mut byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut raw: *mut byte = 0 as *mut byte;
    let mut pcx: *mut pcx_t = 0 as *mut pcx_t;
    let mut x: libc::c_int = 0;
    let mut y: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut dataByte: libc::c_int = 0;
    let mut runLength: libc::c_int = 0;
    let mut out: *mut byte = 0 as *mut byte;
    let mut pix: *mut byte = 0 as *mut byte;
    *pic = 0 as *mut byte;
    len = (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(filename, &mut raw as *mut *mut byte as *mut *mut libc::c_void);
    if raw.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Bad pcx file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    pcx = raw as *mut pcx_t;
    (*pcx).xmin = LittleShort((*pcx).xmin as libc::c_short) as libc::c_ushort;
    (*pcx).ymin = LittleShort((*pcx).ymin as libc::c_short) as libc::c_ushort;
    (*pcx).xmax = LittleShort((*pcx).xmax as libc::c_short) as libc::c_ushort;
    (*pcx).ymax = LittleShort((*pcx).ymax as libc::c_short) as libc::c_ushort;
    (*pcx).hres = LittleShort((*pcx).hres as libc::c_short) as libc::c_ushort;
    (*pcx).vres = LittleShort((*pcx).vres as libc::c_short) as libc::c_ushort;
    (*pcx)
        .bytes_per_line = LittleShort((*pcx).bytes_per_line as libc::c_short)
        as libc::c_ushort;
    (*pcx)
        .palette_type = LittleShort((*pcx).palette_type as libc::c_short)
        as libc::c_ushort;
    raw = &mut (*pcx).data;
    if (*pcx).manufacturer as libc::c_int != 0xa as libc::c_int
        || (*pcx).version as libc::c_int != 5 as libc::c_int
        || (*pcx).encoding as libc::c_int != 1 as libc::c_int
        || (*pcx).bits_per_pixel as libc::c_int != 8 as libc::c_int
        || (*pcx).xmax as libc::c_int >= 640 as libc::c_int
        || (*pcx).ymax as libc::c_int >= 480 as libc::c_int
    {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"Bad pcx file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        return;
    }
    out = malloc(
        (((*pcx).ymax as libc::c_int + 1 as libc::c_int)
            * ((*pcx).xmax as libc::c_int + 1 as libc::c_int)) as libc::c_ulong,
    ) as *mut byte;
    *pic = out;
    pix = out;
    if !palette.is_null() {
        *palette = malloc(768 as libc::c_int as libc::c_ulong) as *mut byte;
        memcpy(
            *palette as *mut libc::c_void,
            (pcx as *mut byte)
                .offset(len as isize)
                .offset(-(768 as libc::c_int as isize)) as *const libc::c_void,
            768 as libc::c_int as libc::c_ulong,
        );
    }
    if !width.is_null() {
        *width = (*pcx).xmax as libc::c_int + 1 as libc::c_int;
    }
    if !height.is_null() {
        *height = (*pcx).ymax as libc::c_int + 1 as libc::c_int;
    }
    y = 0 as libc::c_int;
    while y <= (*pcx).ymax as libc::c_int {
        x = 0 as libc::c_int;
        while x <= (*pcx).xmax as libc::c_int {
            let fresh0 = raw;
            raw = raw.offset(1);
            dataByte = *fresh0 as libc::c_int;
            if dataByte & 0xc0 as libc::c_int == 0xc0 as libc::c_int {
                runLength = dataByte & 0x3f as libc::c_int;
                let fresh1 = raw;
                raw = raw.offset(1);
                dataByte = *fresh1 as libc::c_int;
            } else {
                runLength = 1 as libc::c_int;
            }
            loop {
                let fresh2 = runLength;
                runLength = runLength - 1;
                if !(fresh2 > 0 as libc::c_int) {
                    break;
                }
                let fresh3 = x;
                x = x + 1;
                *pix.offset(fresh3 as isize) = dataByte as byte;
            }
        }
        y += 1;
        pix = pix.offset(((*pcx).xmax as libc::c_int + 1 as libc::c_int) as isize);
    }
    if raw.offset_from(pcx as *mut byte) as libc::c_long > len as libc::c_long {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"PCX file %s was malformed\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            filename,
        );
        free(*pic as *mut libc::c_void);
        *pic = 0 as *mut byte;
    }
    (ri.FS_FreeFile).expect("non-null function pointer")(pcx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn LoadTGA(
    mut name: *mut libc::c_char,
    mut pic: *mut *mut byte,
    mut width: *mut libc::c_int,
    mut height: *mut libc::c_int,
) {
    let mut columns: libc::c_int = 0;
    let mut rows: libc::c_int = 0;
    let mut numPixels: libc::c_int = 0;
    let mut pixbuf: *mut byte = 0 as *mut byte;
    let mut row: libc::c_int = 0;
    let mut column: libc::c_int = 0;
    let mut buf_p: *mut byte = 0 as *mut byte;
    let mut buffer: *mut byte = 0 as *mut byte;
    let mut length: libc::c_int = 0;
    let mut targa_header: TargaHeader = TargaHeader {
        id_length: 0,
        colormap_type: 0,
        image_type: 0,
        colormap_index: 0,
        colormap_length: 0,
        colormap_size: 0,
        x_origin: 0,
        y_origin: 0,
        width: 0,
        height: 0,
        pixel_size: 0,
        attributes: 0,
    };
    let mut targa_rgba: *mut byte = 0 as *mut byte;
    *pic = 0 as *mut byte;
    length = (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(name, &mut buffer as *mut *mut byte as *mut *mut libc::c_void);
    if buffer.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Bad tga file %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return;
    }
    buf_p = buffer;
    let fresh4 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.id_length = *fresh4;
    let fresh5 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.colormap_type = *fresh5;
    let fresh6 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.image_type = *fresh6;
    targa_header
        .colormap_index = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header
        .colormap_length = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    let fresh7 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.colormap_size = *fresh7;
    targa_header
        .x_origin = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header
        .y_origin = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.width = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    targa_header.height = LittleShort(*(buf_p as *mut libc::c_short)) as libc::c_ushort;
    buf_p = buf_p.offset(2 as libc::c_int as isize);
    let fresh8 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.pixel_size = *fresh8;
    let fresh9 = buf_p;
    buf_p = buf_p.offset(1);
    targa_header.attributes = *fresh9;
    if targa_header.image_type as libc::c_int != 2 as libc::c_int
        && targa_header.image_type as libc::c_int != 10 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"LoadTGA: Only type 2 and 10 targa RGB images supported\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if targa_header.colormap_type as libc::c_int != 0 as libc::c_int
        || targa_header.pixel_size as libc::c_int != 32 as libc::c_int
            && targa_header.pixel_size as libc::c_int != 24 as libc::c_int
    {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"LoadTGA: Only 32 or 24 bit images supported (no colormaps)\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    columns = targa_header.width as libc::c_int;
    rows = targa_header.height as libc::c_int;
    numPixels = columns * rows;
    if !width.is_null() {
        *width = columns;
    }
    if !height.is_null() {
        *height = rows;
    }
    targa_rgba = malloc((numPixels * 4 as libc::c_int) as libc::c_ulong) as *mut byte;
    *pic = targa_rgba;
    if targa_header.id_length as libc::c_int != 0 as libc::c_int {
        buf_p = buf_p.offset(targa_header.id_length as libc::c_int as isize);
    }
    if targa_header.image_type as libc::c_int == 2 as libc::c_int {
        row = rows - 1 as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset((row * columns * 4 as libc::c_int) as isize);
            column = 0 as libc::c_int;
            while column < columns {
                let mut red: libc::c_uchar = 0;
                let mut green: libc::c_uchar = 0;
                let mut blue: libc::c_uchar = 0;
                let mut alphabyte: libc::c_uchar = 0;
                match targa_header.pixel_size as libc::c_int {
                    24 => {
                        let fresh10 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh10;
                        let fresh11 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh11;
                        let fresh12 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh12;
                        let fresh13 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh13 = red;
                        let fresh14 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh14 = green;
                        let fresh15 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh15 = blue;
                        let fresh16 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh16 = 255 as libc::c_int as byte;
                    }
                    32 => {
                        let fresh17 = buf_p;
                        buf_p = buf_p.offset(1);
                        blue = *fresh17;
                        let fresh18 = buf_p;
                        buf_p = buf_p.offset(1);
                        green = *fresh18;
                        let fresh19 = buf_p;
                        buf_p = buf_p.offset(1);
                        red = *fresh19;
                        let fresh20 = buf_p;
                        buf_p = buf_p.offset(1);
                        alphabyte = *fresh20;
                        let fresh21 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh21 = red;
                        let fresh22 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh22 = green;
                        let fresh23 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh23 = blue;
                        let fresh24 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh24 = alphabyte;
                    }
                    _ => {}
                }
                column += 1;
            }
            row -= 1;
        }
    } else if targa_header.image_type as libc::c_int == 10 as libc::c_int {
        let mut red_0: libc::c_uchar = 0;
        let mut green_0: libc::c_uchar = 0;
        let mut blue_0: libc::c_uchar = 0;
        let mut alphabyte_0: libc::c_uchar = 0;
        let mut packetHeader: libc::c_uchar = 0;
        let mut packetSize: libc::c_uchar = 0;
        let mut j: libc::c_uchar = 0;
        row = rows - 1 as libc::c_int;
        while row >= 0 as libc::c_int {
            pixbuf = targa_rgba.offset((row * columns * 4 as libc::c_int) as isize);
            column = 0 as libc::c_int;
            's_335: while column < columns {
                let fresh25 = buf_p;
                buf_p = buf_p.offset(1);
                packetHeader = *fresh25;
                packetSize = (1 as libc::c_int
                    + (packetHeader as libc::c_int & 0x7f as libc::c_int))
                    as libc::c_uchar;
                if packetHeader as libc::c_int & 0x80 as libc::c_int != 0 {
                    match targa_header.pixel_size as libc::c_int {
                        24 => {
                            let fresh26 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh26;
                            let fresh27 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh27;
                            let fresh28 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh28;
                            alphabyte_0 = 255 as libc::c_int as libc::c_uchar;
                        }
                        32 => {
                            let fresh29 = buf_p;
                            buf_p = buf_p.offset(1);
                            blue_0 = *fresh29;
                            let fresh30 = buf_p;
                            buf_p = buf_p.offset(1);
                            green_0 = *fresh30;
                            let fresh31 = buf_p;
                            buf_p = buf_p.offset(1);
                            red_0 = *fresh31;
                            let fresh32 = buf_p;
                            buf_p = buf_p.offset(1);
                            alphabyte_0 = *fresh32;
                        }
                        _ => {}
                    }
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        let fresh33 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh33 = red_0;
                        let fresh34 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh34 = green_0;
                        let fresh35 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh35 = blue_0;
                        let fresh36 = pixbuf;
                        pixbuf = pixbuf.offset(1);
                        *fresh36 = alphabyte_0;
                        column += 1;
                        if column == columns {
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_335;
                            }
                            row -= 1;
                            pixbuf = targa_rgba
                                .offset((row * columns * 4 as libc::c_int) as isize);
                        }
                        j = j.wrapping_add(1);
                    }
                } else {
                    j = 0 as libc::c_int as libc::c_uchar;
                    while (j as libc::c_int) < packetSize as libc::c_int {
                        match targa_header.pixel_size as libc::c_int {
                            24 => {
                                let fresh37 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh37;
                                let fresh38 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh38;
                                let fresh39 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh39;
                                let fresh40 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh40 = red_0;
                                let fresh41 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh41 = green_0;
                                let fresh42 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh42 = blue_0;
                                let fresh43 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh43 = 255 as libc::c_int as byte;
                            }
                            32 => {
                                let fresh44 = buf_p;
                                buf_p = buf_p.offset(1);
                                blue_0 = *fresh44;
                                let fresh45 = buf_p;
                                buf_p = buf_p.offset(1);
                                green_0 = *fresh45;
                                let fresh46 = buf_p;
                                buf_p = buf_p.offset(1);
                                red_0 = *fresh46;
                                let fresh47 = buf_p;
                                buf_p = buf_p.offset(1);
                                alphabyte_0 = *fresh47;
                                let fresh48 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh48 = red_0;
                                let fresh49 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh49 = green_0;
                                let fresh50 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh50 = blue_0;
                                let fresh51 = pixbuf;
                                pixbuf = pixbuf.offset(1);
                                *fresh51 = alphabyte_0;
                            }
                            _ => {}
                        }
                        column += 1;
                        if column == columns {
                            column = 0 as libc::c_int;
                            if !(row > 0 as libc::c_int) {
                                break 's_335;
                            }
                            row -= 1;
                            pixbuf = targa_rgba
                                .offset((row * columns * 4 as libc::c_int) as isize);
                        }
                        j = j.wrapping_add(1);
                    }
                }
            }
            row -= 1;
        }
    }
    (ri.FS_FreeFile).expect("non-null function pointer")(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn R_FindFreeImage() -> *mut image_t {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    image = r_images.as_mut_ptr();
    while i < numr_images {
        if (*image).registration_sequence == 0 {
            break;
        }
        i += 1;
        image = image.offset(1);
    }
    if i == numr_images {
        if numr_images == 1024 as libc::c_int {
            (ri.Sys_Error)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"MAX_RIMAGES\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        numr_images += 1;
    }
    image = &mut *r_images.as_mut_ptr().offset(i as isize) as *mut image_t;
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn GL_LoadPic(
    mut name: *mut libc::c_char,
    mut pic: *mut byte,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut type_0: imagetype_t,
) -> *mut image_t {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    image = R_FindFreeImage();
    if strlen(name) >= ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong {
        (ri.Sys_Error)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"Draw_LoadPic: \"%s\" is too long\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
    }
    strcpy(((*image).name).as_mut_ptr(), name);
    (*image).registration_sequence = registration_sequence;
    (*image).width = width;
    (*image).height = height;
    (*image).type_0 = type_0;
    c = width * height;
    let ref mut fresh52 = (*image).pixels[0 as libc::c_int as usize];
    *fresh52 = malloc(c as libc::c_ulong) as *mut byte;
    (*image).transparent = false_0;
    i = 0 as libc::c_int;
    while i < c {
        b = *pic.offset(i as isize) as libc::c_int;
        if b == 255 as libc::c_int {
            (*image).transparent = true_0;
        }
        *((*image).pixels[0 as libc::c_int as usize]).offset(i as isize) = b as byte;
        i += 1;
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn R_LoadWal(mut name: *mut libc::c_char) -> *mut image_t {
    let mut mt: *mut miptex_t = 0 as *mut miptex_t;
    let mut ofs: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut size: libc::c_int = 0;
    (ri.FS_LoadFile)
        .expect(
            "non-null function pointer",
        )(name, &mut mt as *mut *mut miptex_t as *mut *mut libc::c_void);
    if mt.is_null() {
        (ri.Con_Printf)
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int,
            b"R_LoadWal: can't load %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            name,
        );
        return r_notexture_mip;
    }
    image = R_FindFreeImage();
    strcpy(((*image).name).as_mut_ptr(), name);
    (*image).width = LittleLong((*mt).width as libc::c_int);
    (*image).height = LittleLong((*mt).height as libc::c_int);
    (*image).type_0 = it_wall;
    (*image).registration_sequence = registration_sequence;
    size = (*image).width * (*image).height
        * (256 as libc::c_int + 64 as libc::c_int + 16 as libc::c_int + 4 as libc::c_int)
        / 256 as libc::c_int;
    let ref mut fresh53 = (*image).pixels[0 as libc::c_int as usize];
    *fresh53 = malloc(size as libc::c_ulong) as *mut byte;
    let ref mut fresh54 = (*image).pixels[1 as libc::c_int as usize];
    *fresh54 = ((*image).pixels[0 as libc::c_int as usize])
        .offset(((*image).width * (*image).height) as isize);
    let ref mut fresh55 = (*image).pixels[2 as libc::c_int as usize];
    *fresh55 = ((*image).pixels[1 as libc::c_int as usize])
        .offset(((*image).width * (*image).height / 4 as libc::c_int) as isize);
    let ref mut fresh56 = (*image).pixels[3 as libc::c_int as usize];
    *fresh56 = ((*image).pixels[2 as libc::c_int as usize])
        .offset(((*image).width * (*image).height / 16 as libc::c_int) as isize);
    ofs = LittleLong((*mt).offsets[0 as libc::c_int as usize] as libc::c_int);
    memcpy(
        (*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void,
        (mt as *mut byte).offset(ofs as isize) as *const libc::c_void,
        size as libc::c_ulong,
    );
    (ri.FS_FreeFile).expect("non-null function pointer")(mt as *mut libc::c_void);
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn R_FindImage(
    mut name: *mut libc::c_char,
    mut type_0: imagetype_t,
) -> *mut image_t {
    let mut image: *mut image_t = 0 as *mut image_t;
    let mut i: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut pic: *mut byte = 0 as *mut byte;
    let mut palette: *mut byte = 0 as *mut byte;
    let mut width: libc::c_int = 0;
    let mut height: libc::c_int = 0;
    if name.is_null() {
        return 0 as *mut image_t;
    }
    len = strlen(name) as libc::c_int;
    if len < 5 as libc::c_int {
        return 0 as *mut image_t;
    }
    i = 0 as libc::c_int;
    image = r_images.as_mut_ptr();
    while i < numr_images {
        if strcmp(name, ((*image).name).as_mut_ptr()) == 0 {
            (*image).registration_sequence = registration_sequence;
            return image;
        }
        i += 1;
        image = image.offset(1);
    }
    pic = 0 as *mut byte;
    palette = 0 as *mut byte;
    if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".pcx\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        LoadPCX(name, &mut pic, &mut palette, &mut width, &mut height);
        if pic.is_null() {
            return 0 as *mut image_t;
        }
        image = GL_LoadPic(name, pic, width, height, type_0);
    } else if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".wal\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        image = R_LoadWal(name);
    } else if strcmp(
        name.offset(len as isize).offset(-(4 as libc::c_int as isize)),
        b".tga\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return 0 as *mut image_t
    } else {
        return 0 as *mut image_t
    }
    if !pic.is_null() {
        free(pic as *mut libc::c_void);
    }
    if !palette.is_null() {
        free(palette as *mut libc::c_void);
    }
    return image;
}
#[no_mangle]
pub unsafe extern "C" fn R_RegisterSkin(mut name: *mut libc::c_char) -> *mut image_s {
    return R_FindImage(name, it_skin);
}
#[no_mangle]
pub unsafe extern "C" fn R_FreeUnusedImages() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    i = 0 as libc::c_int;
    image = r_images.as_mut_ptr();
    while i < numr_images {
        if (*image).registration_sequence == registration_sequence {
            Com_PageInMemory(
                (*image).pixels[0 as libc::c_int as usize],
                (*image).width * (*image).height,
            );
        } else if !((*image).registration_sequence == 0) {
            if !((*image).type_0 as libc::c_uint
                == it_pic as libc::c_int as libc::c_uint)
            {
                free((*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void);
                memset(
                    image as *mut libc::c_void,
                    0 as libc::c_int,
                    ::std::mem::size_of::<image_t>() as libc::c_ulong,
                );
            }
        }
        i += 1;
        image = image.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn R_InitImages() {
    registration_sequence = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn R_ShutdownImages() {
    let mut i: libc::c_int = 0;
    let mut image: *mut image_t = 0 as *mut image_t;
    i = 0 as libc::c_int;
    image = r_images.as_mut_ptr();
    while i < numr_images {
        if !((*image).registration_sequence == 0) {
            free((*image).pixels[0 as libc::c_int as usize] as *mut libc::c_void);
            memset(
                image as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<image_t>() as libc::c_ulong,
            );
        }
        i += 1;
        image = image.offset(1);
    }
}
