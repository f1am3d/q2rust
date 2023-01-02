#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(asm, extern_types, register_tool)]
use c2rust_asm_casts::AsmCastTrait;
use std::arch::asm;
extern "C" {
    pub type sockaddr_ipx;
    pub type sockaddr_in;
    pub type sockaddr;
    pub type sockaddr_0;
    pub type sockaddr_1;
    pub type sockaddr_2;
    pub type sockaddr_3;
    pub type sockaddr_4;
    pub type hostent;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Com_Error(code: libc::c_int, fmt: *mut libc::c_char, _: ...);
    static mut dedicated: *mut cvar_t;
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
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
pub struct sizebuf_s {
    pub allowoverflow: qboolean,
    pub overflowed: qboolean,
    pub data: *mut byte,
    pub maxsize: libc::c_int,
    pub cursize: libc::c_int,
    pub readcount: libc::c_int,
}
pub type sizebuf_t = sizebuf_s;
pub type netadrtype_t = libc::c_uint;
pub const NA_BROADCAST_IPX: netadrtype_t = 4;
pub const NA_IPX: netadrtype_t = 3;
pub const NA_IP: netadrtype_t = 2;
pub const NA_BROADCAST: netadrtype_t = 1;
pub const NA_LOOPBACK: netadrtype_t = 0;
pub type netsrc_t = libc::c_uint;
pub const NS_SERVER: netsrc_t = 1;
pub const NS_CLIENT: netsrc_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct netadr_t {
    pub type_0: netadrtype_t,
    pub ip: [byte; 4],
    pub ipx: [byte; 10],
    pub port: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loopmsg_t {
    pub data: [byte; 1400],
    pub datalen: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loopback_t {
    pub msgs: [loopmsg_t; 4],
    pub get: libc::c_int,
    pub send: libc::c_int,
}
#[no_mangle]
pub static mut net_shownet: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut noudp: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
static mut noipx: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut loopbacks: [loopback_t; 2] = [loopback_t {
    msgs: [loopmsg_t {
        data: [0; 1400],
        datalen: 0,
    }; 4],
    get: 0,
    send: 0,
}; 2];
#[no_mangle]
pub static mut ip_sockets: [libc::c_int; 2] = [0; 2];
#[no_mangle]
pub static mut ipx_sockets: [libc::c_int; 2] = [0; 2];
#[no_mangle]
pub unsafe extern "C" fn NetadrToSockadr(mut a: *mut netadr_t, mut s: *mut sockaddr_2) {
    if !((*a).type_0 as libc::c_uint == NA_BROADCAST as libc::c_int as libc::c_uint) {
        if !((*a).type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint) {
            if !((*a).type_0 as libc::c_uint == NA_IPX as libc::c_int as libc::c_uint) {
                (*a).type_0 as libc::c_uint
                    == NA_BROADCAST_IPX as libc::c_int as libc::c_uint;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn NET_CompareAdr(mut a: netadr_t, mut b: netadr_t) -> qboolean {
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint {
        return false_0;
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if a.ip[0 as libc::c_int as usize] as libc::c_int
            == b.ip[0 as libc::c_int as usize] as libc::c_int
            && a.ip[1 as libc::c_int as usize] as libc::c_int
                == b.ip[1 as libc::c_int as usize] as libc::c_int
            && a.ip[2 as libc::c_int as usize] as libc::c_int
                == b.ip[2 as libc::c_int as usize] as libc::c_int
            && a.ip[3 as libc::c_int as usize] as libc::c_int
                == b.ip[3 as libc::c_int as usize] as libc::c_int
            && a.port as libc::c_int == b.port as libc::c_int
        {
            return true_0;
        }
        return false_0;
    }
    if a.type_0 as libc::c_uint == NA_IPX as libc::c_int as libc::c_uint {
        if memcmp(
            (a.ipx).as_mut_ptr() as *const libc::c_void,
            (b.ipx).as_mut_ptr() as *const libc::c_void,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int && a.port as libc::c_int == b.port as libc::c_int
        {
            return true_0;
        }
        return false_0;
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn NET_CompareBaseAdr(
    mut a: netadr_t,
    mut b: netadr_t,
) -> qboolean {
    if a.type_0 as libc::c_uint != b.type_0 as libc::c_uint {
        return false_0;
    }
    if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        if a.ip[0 as libc::c_int as usize] as libc::c_int
            == b.ip[0 as libc::c_int as usize] as libc::c_int
            && a.ip[1 as libc::c_int as usize] as libc::c_int
                == b.ip[1 as libc::c_int as usize] as libc::c_int
            && a.ip[2 as libc::c_int as usize] as libc::c_int
                == b.ip[2 as libc::c_int as usize] as libc::c_int
            && a.ip[3 as libc::c_int as usize] as libc::c_int
                == b.ip[3 as libc::c_int as usize] as libc::c_int
        {
            return true_0;
        }
        return false_0;
    }
    if a.type_0 as libc::c_uint == NA_IPX as libc::c_int as libc::c_uint {
        if memcmp(
            (a.ipx).as_mut_ptr() as *const libc::c_void,
            (b.ipx).as_mut_ptr() as *const libc::c_void,
            10 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return true_0;
        }
        return false_0;
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn NET_AdrToString(mut a: netadr_t) -> *mut libc::c_char {
    static mut s: [libc::c_char; 64] = [0; 64];
    if a.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"loopback\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else if a.type_0 as libc::c_uint == NA_IP as libc::c_int as libc::c_uint {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%i.%i.%i.%i:%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            a.ip[0 as libc::c_int as usize] as libc::c_int,
            a.ip[1 as libc::c_int as usize] as libc::c_int,
            a.ip[2 as libc::c_int as usize] as libc::c_int,
            a.ip[3 as libc::c_int as usize] as libc::c_int,
            ntohs(a.port as libc::c_int),
        );
    } else {
        Com_sprintf(
            s.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
            b"%02x%02x%02x%02x:%02x%02x%02x%02x%02x%02x:%i\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            a.ipx[0 as libc::c_int as usize] as libc::c_int,
            a.ipx[1 as libc::c_int as usize] as libc::c_int,
            a.ipx[2 as libc::c_int as usize] as libc::c_int,
            a.ipx[3 as libc::c_int as usize] as libc::c_int,
            a.ipx[4 as libc::c_int as usize] as libc::c_int,
            a.ipx[5 as libc::c_int as usize] as libc::c_int,
            a.ipx[6 as libc::c_int as usize] as libc::c_int,
            a.ipx[7 as libc::c_int as usize] as libc::c_int,
            a.ipx[8 as libc::c_int as usize] as libc::c_int,
            a.ipx[9 as libc::c_int as usize] as libc::c_int,
            ntohs(a.port as libc::c_int),
        );
    }
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn NET_StringToSockaddr(
    mut s: *mut libc::c_char,
    mut sadr: *mut sockaddr_4,
) -> qboolean {
    let mut h: *mut hostent = 0 as *mut hostent;
    let mut colon: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut val: libc::c_int = 0;
    let mut copy: [libc::c_char; 128] = [0; 128];
    if strlen(s) >= 23 as libc::c_int as libc::c_ulong
        && *s.offset(8 as libc::c_int as isize) as libc::c_int == ':' as i32
        && *s.offset(21 as libc::c_int as isize) as libc::c_int == ':' as i32
    {
        copy[2 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
        copy[0 as libc::c_int as usize] = *s.offset(0 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((0 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(2 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((2 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(4 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((4 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(6 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((6 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(9 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((9 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(11 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((11 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(13 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((13 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(15 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((15 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(17 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((17 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        copy[0 as libc::c_int as usize] = *s.offset(19 as libc::c_int as isize);
        copy[1 as libc::c_int
            as usize] = *s.offset((19 as libc::c_int + 1 as libc::c_int) as isize);
        sscanf(
            copy.as_mut_ptr(),
            b"%x\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        sscanf(
            &mut *s.offset(22 as libc::c_int as isize) as *mut libc::c_char,
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
    } else {
        strcpy(copy.as_mut_ptr(), s);
        colon = copy.as_mut_ptr();
        while *colon != 0 {
            if *colon as libc::c_int == ':' as i32 {
                *colon = 0 as libc::c_int as libc::c_char;
            }
            colon = colon.offset(1);
        }
        if !(copy[0 as libc::c_int as usize] as libc::c_int >= '0' as i32
            && copy[0 as libc::c_int as usize] as libc::c_int <= '9' as i32)
        {
            h = gethostbyname(copy.as_mut_ptr()) as *mut hostent;
            if h.is_null() {
                return false_0;
            }
        }
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn NET_IsLocalAddress(mut adr: netadr_t) -> qboolean {
    return (adr.type_0 as libc::c_uint == NA_LOOPBACK as libc::c_int as libc::c_uint)
        as libc::c_int as qboolean;
}
#[no_mangle]
pub unsafe extern "C" fn NET_GetLoopPacket(
    mut sock: netsrc_t,
    mut net_from: *mut netadr_t,
    mut net_message: *mut sizebuf_t,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks.as_mut_ptr().offset(sock as isize) as *mut loopback_t;
    if (*loop_0).send - (*loop_0).get > 4 as libc::c_int {
        (*loop_0).get = (*loop_0).send - 4 as libc::c_int;
    }
    if (*loop_0).get >= (*loop_0).send {
        return false_0;
    }
    i = (*loop_0).get & 4 as libc::c_int - 1 as libc::c_int;
    let ref mut fresh0 = (*loop_0).get;
    *fresh0 += 1;
    memcpy(
        (*net_message).data as *mut libc::c_void,
        ((*loop_0).msgs[i as usize].data).as_mut_ptr() as *const libc::c_void,
        (*loop_0).msgs[i as usize].datalen as libc::c_ulong,
    );
    (*net_message).cursize = (*loop_0).msgs[i as usize].datalen;
    memset(
        net_from as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<netadr_t>() as libc::c_ulong,
    );
    (*net_from).type_0 = NA_LOOPBACK;
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn NET_SendLoopPacket(
    mut sock: netsrc_t,
    mut length: libc::c_int,
    mut data: *mut libc::c_void,
    mut to: netadr_t,
) {
    let mut i: libc::c_int = 0;
    let mut loop_0: *mut loopback_t = 0 as *mut loopback_t;
    loop_0 = &mut *loopbacks
        .as_mut_ptr()
        .offset((sock as libc::c_uint ^ 1 as libc::c_int as libc::c_uint) as isize)
        as *mut loopback_t;
    i = (*loop_0).send & 4 as libc::c_int - 1 as libc::c_int;
    let ref mut fresh1 = (*loop_0).send;
    *fresh1 += 1;
    memcpy(
        ((*loop_0).msgs[i as usize].data).as_mut_ptr() as *mut libc::c_void,
        data,
        length as libc::c_ulong,
    );
    (*loop_0).msgs[i as usize].datalen = length;
}
#[no_mangle]
pub unsafe extern "C" fn NET_OpenIP() {
    let mut ip: *mut cvar_t = 0 as *mut cvar_t;
    let mut port: libc::c_int = 0;
    let mut dedicated_0: libc::c_int = 0;
    ip = Cvar_Get(
        b"ip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"localhost\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    dedicated_0 = Cvar_VariableValue(
        b"dedicated\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    if ip_sockets[NS_SERVER as libc::c_int as usize] == 0 {
        port = (*Cvar_Get(
            b"ip_hostport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            8 as libc::c_int,
        ))
            .value as libc::c_int;
        if port == 0 {
            port = (*Cvar_Get(
                b"hostport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                8 as libc::c_int,
            ))
                .value as libc::c_int;
            if port == 0 {
                port = (*Cvar_Get(
                    b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    va(
                        b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        27910 as libc::c_int,
                    ),
                    8 as libc::c_int,
                ))
                    .value as libc::c_int;
            }
        }
        ip_sockets[NS_SERVER as libc::c_int as usize] = NET_IPSocket((*ip).string, port);
        if ip_sockets[NS_SERVER as libc::c_int as usize] == 0 && dedicated_0 != 0 {
            Com_Error(
                0 as libc::c_int,
                b"Couldn't allocate dedicated server IP port\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
    }
    if dedicated_0 != 0 {
        return;
    }
    if ip_sockets[NS_CLIENT as libc::c_int as usize] == 0 {
        port = (*Cvar_Get(
            b"ip_clientport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            8 as libc::c_int,
        ))
            .value as libc::c_int;
        if port == 0 {
            port = (*Cvar_Get(
                b"clientport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                va(
                    b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    27901 as libc::c_int,
                ),
                8 as libc::c_int,
            ))
                .value as libc::c_int;
            if port == 0 {
                port = -(1 as libc::c_int);
            }
        }
        ip_sockets[NS_CLIENT as libc::c_int as usize] = NET_IPSocket((*ip).string, port);
        if ip_sockets[NS_CLIENT as libc::c_int as usize] == 0 {
            ip_sockets[NS_CLIENT as libc::c_int
                as usize] = NET_IPSocket((*ip).string, -(1 as libc::c_int));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn NET_OpenIPX() {
    let mut port: libc::c_int = 0;
    let mut dedicated_0: libc::c_int = 0;
    dedicated_0 = Cvar_VariableValue(
        b"dedicated\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_int;
    if ipx_sockets[NS_SERVER as libc::c_int as usize] == 0 {
        port = (*Cvar_Get(
            b"ipx_hostport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            8 as libc::c_int,
        ))
            .value as libc::c_int;
        if port == 0 {
            port = (*Cvar_Get(
                b"hostport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                8 as libc::c_int,
            ))
                .value as libc::c_int;
            if port == 0 {
                port = (*Cvar_Get(
                    b"port\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    va(
                        b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        27910 as libc::c_int,
                    ),
                    8 as libc::c_int,
                ))
                    .value as libc::c_int;
            }
        }
        ipx_sockets[NS_SERVER as libc::c_int as usize] = NET_IPXSocket(port);
    }
    if dedicated_0 != 0 {
        return;
    }
    if ipx_sockets[NS_CLIENT as libc::c_int as usize] == 0 {
        port = (*Cvar_Get(
            b"ipx_clientport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            8 as libc::c_int,
        ))
            .value as libc::c_int;
        if port == 0 {
            port = (*Cvar_Get(
                b"clientport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                va(
                    b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    27901 as libc::c_int,
                ),
                8 as libc::c_int,
            ))
                .value as libc::c_int;
            if port == 0 {
                port = -(1 as libc::c_int);
            }
        }
        ipx_sockets[NS_CLIENT as libc::c_int as usize] = NET_IPXSocket(port);
        if ipx_sockets[NS_CLIENT as libc::c_int as usize] == 0 {
            ipx_sockets[NS_CLIENT as libc::c_int
                as usize] = NET_IPXSocket(-(1 as libc::c_int));
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn NET_Config(mut multiplayer: qboolean) {
    let mut i: libc::c_int = 0;
    static mut old_config: qboolean = false_0;
    if old_config as libc::c_uint == multiplayer as libc::c_uint {
        return;
    }
    old_config = multiplayer;
    if multiplayer as u64 == 0 {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if ip_sockets[i as usize] != 0 {
                closesocket(ip_sockets[i as usize]);
                ip_sockets[i as usize] = 0 as libc::c_int;
            }
            if ipx_sockets[i as usize] != 0 {
                closesocket(ipx_sockets[i as usize]);
                ipx_sockets[i as usize] = 0 as libc::c_int;
            }
            i += 1;
        }
    } else {
        if (*noudp).value == 0. {
            NET_OpenIP();
        }
        if (*noipx).value == 0. {
            NET_OpenIPX();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn NET_Sleep(mut msec: libc::c_int) {
    let mut timeout: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut fdset: fd_set = fd_set { __fds_bits: [0; 16] };
    extern "C" {
        #[link_name = "dedicated"]
        static mut dedicated_0: *mut cvar_t;
    }
    let mut i: libc::c_int = 0;
    if dedicated.is_null() || (*dedicated).value == 0. {
        return;
    }
    let mut __d0: libc::c_int = 0;
    let mut __d1: libc::c_int = 0;
    let fresh2 = &mut __d0;
    let fresh3;
    let fresh4 = (::std::mem::size_of::<fd_set>() as libc::c_ulong)
        .wrapping_div(::std::mem::size_of::<__fd_mask>() as libc::c_ulong);
    let fresh5 = &mut __d1;
    let fresh6;
    let fresh7 = &mut *(fdset.__fds_bits).as_mut_ptr().offset(0 as libc::c_int as isize)
        as *mut __fd_mask;
    asm!(
        "cld; rep; stosq", inlateout("cx") c2rust_asm_casts::AsmCast::cast_in(fresh2,
        fresh4) => fresh3, inlateout("di") c2rust_asm_casts::AsmCast::cast_in(fresh5,
        fresh7) => fresh6, inlateout("ax") 0 as libc::c_int => _,
        options(preserves_flags)
    );
    c2rust_asm_casts::AsmCast::cast_out(fresh2, fresh4, fresh3);
    c2rust_asm_casts::AsmCast::cast_out(fresh5, fresh7, fresh6);
    i = 0 as libc::c_int;
    if ip_sockets[NS_SERVER as libc::c_int as usize] != 0 {
        fdset
            .__fds_bits[(ip_sockets[NS_SERVER as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << ip_sockets[NS_SERVER as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        i = ip_sockets[NS_SERVER as libc::c_int as usize];
    }
    if ipx_sockets[NS_SERVER as libc::c_int as usize] != 0 {
        fdset
            .__fds_bits[(ipx_sockets[NS_SERVER as libc::c_int as usize]
            / (8 as libc::c_int
                * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong as libc::c_int))
            as usize]
            |= ((1 as libc::c_ulong)
                << ipx_sockets[NS_SERVER as libc::c_int as usize]
                    % (8 as libc::c_int
                        * ::std::mem::size_of::<__fd_mask>() as libc::c_ulong
                            as libc::c_int)) as __fd_mask;
        if ipx_sockets[NS_SERVER as libc::c_int as usize] > i {
            i = ipx_sockets[NS_SERVER as libc::c_int as usize];
        }
    }
    timeout.tv_sec = (msec / 1000 as libc::c_int) as __time_t;
    timeout
        .tv_usec = (msec % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
    select(
        i + 1 as libc::c_int,
        &mut fdset,
        0 as *mut fd_set,
        0 as *mut fd_set,
        &mut timeout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NET_Init() {
    let mut r: libc::c_int = 0;
    if r != 0 {
        Com_Error(
            0 as libc::c_int,
            b"Winsock initialization failed.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    Com_Printf(
        b"Winsock Initialized\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    noudp = Cvar_Get(
        b"noudp\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    noipx = Cvar_Get(
        b"noipx\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        8 as libc::c_int,
    );
    net_shownet = Cvar_Get(
        b"net_shownet\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn NET_Shutdown() {
    NET_Config(false_0);
    WSACleanup();
}
#[no_mangle]
pub unsafe extern "C" fn NET_ErrorString() -> *mut libc::c_char {
    let mut code: libc::c_int = 0;
    code = WSAGetLastError();
    match code {
        _ => {}
    }
    return b"NO ERROR\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
