#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    static mut curtime: libc::c_int;
    fn Sys_Milliseconds() -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn SZ_Init(buf: *mut sizebuf_t, data: *mut byte, length: libc::c_int);
    fn SZ_Write(buf: *mut sizebuf_t, data: *mut libc::c_void, length: libc::c_int);
    fn MSG_WriteShort(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_WriteLong(sb: *mut sizebuf_t, c: libc::c_int);
    fn MSG_BeginReading(sb: *mut sizebuf_t);
    fn MSG_ReadShort(sb: *mut sizebuf_t) -> libc::c_int;
    fn MSG_ReadLong(sb: *mut sizebuf_t) -> libc::c_int;
    fn NET_SendPacket(
        sock: netsrc_t,
        length: libc::c_int,
        data: *mut libc::c_void,
        to: netadr_t,
    );
    fn NET_AdrToString(a: netadr_t) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
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
pub struct netchan_t {
    pub fatal_error: qboolean,
    pub sock: netsrc_t,
    pub dropped: libc::c_int,
    pub last_received: libc::c_int,
    pub last_sent: libc::c_int,
    pub remote_address: netadr_t,
    pub qport: libc::c_int,
    pub incoming_sequence: libc::c_int,
    pub incoming_acknowledged: libc::c_int,
    pub incoming_reliable_acknowledged: libc::c_int,
    pub incoming_reliable_sequence: libc::c_int,
    pub outgoing_sequence: libc::c_int,
    pub reliable_sequence: libc::c_int,
    pub last_reliable_sequence: libc::c_int,
    pub message: sizebuf_t,
    pub message_buf: [byte; 1384],
    pub reliable_length: libc::c_int,
    pub reliable_buf: [byte; 1384],
}
#[no_mangle]
pub static mut showpackets: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut showdrop: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut qport: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut net_from: netadr_t = netadr_t {
    type_0: NA_LOOPBACK,
    ip: [0; 4],
    ipx: [0; 10],
    port: 0,
};
#[no_mangle]
pub static mut net_message: sizebuf_t = sizebuf_t {
    allowoverflow: false_0,
    overflowed: false_0,
    data: 0 as *const byte as *mut byte,
    maxsize: 0,
    cursize: 0,
    readcount: 0,
};
#[no_mangle]
pub static mut net_message_buffer: [byte; 1400] = [0; 1400];
#[no_mangle]
pub unsafe extern "C" fn Netchan_Init() {
    let mut port: libc::c_int = 0;
    port = Sys_Milliseconds() & 0xffff as libc::c_int;
    showpackets = Cvar_Get(
        b"showpackets\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    showdrop = Cvar_Get(
        b"showdrop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    qport = Cvar_Get(
        b"qport\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        va(b"%i\0" as *const u8 as *const libc::c_char as *mut libc::c_char, port),
        8 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_OutOfBand(
    mut net_socket: libc::c_int,
    mut adr: netadr_t,
    mut length: libc::c_int,
    mut data: *mut byte,
) {
    let mut send: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut send_buf: [byte; 1400] = [0; 1400];
    SZ_Init(
        &mut send,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    MSG_WriteLong(&mut send, -(1 as libc::c_int));
    SZ_Write(&mut send, data as *mut libc::c_void, length);
    NET_SendPacket(
        net_socket as netsrc_t,
        send.cursize,
        send.data as *mut libc::c_void,
        adr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_OutOfBandPrint(
    mut net_socket: libc::c_int,
    mut adr: netadr_t,
    mut format: *mut libc::c_char,
    mut args: ...
) {
    let mut argptr: ::std::ffi::VaListImpl;
    static mut string: [libc::c_char; 1396] = [0; 1396];
    argptr = args.clone();
    vsprintf(string.as_mut_ptr(), format, argptr.as_va_list());
    Netchan_OutOfBand(
        net_socket,
        adr,
        strlen(string.as_mut_ptr()) as libc::c_int,
        string.as_mut_ptr() as *mut byte,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Setup(
    mut sock: netsrc_t,
    mut chan: *mut netchan_t,
    mut adr: netadr_t,
    mut qport_0: libc::c_int,
) {
    memset(
        chan as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<netchan_t>() as libc::c_ulong,
    );
    (*chan).sock = sock;
    (*chan).remote_address = adr;
    (*chan).qport = qport_0;
    (*chan).last_received = curtime;
    (*chan).incoming_sequence = 0 as libc::c_int;
    (*chan).outgoing_sequence = 1 as libc::c_int;
    SZ_Init(
        &mut (*chan).message,
        ((*chan).message_buf).as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1384]>() as libc::c_ulong as libc::c_int,
    );
    (*chan).message.allowoverflow = true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_CanReliable(mut chan: *mut netchan_t) -> qboolean {
    if (*chan).reliable_length != 0 {
        return false_0;
    }
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_NeedReliable(mut chan: *mut netchan_t) -> qboolean {
    let mut send_reliable: qboolean = false_0;
    send_reliable = false_0;
    if (*chan).incoming_acknowledged > (*chan).last_reliable_sequence
        && (*chan).incoming_reliable_acknowledged != (*chan).reliable_sequence
    {
        send_reliable = true_0;
    }
    if (*chan).reliable_length == 0 && (*chan).message.cursize != 0 {
        send_reliable = true_0;
    }
    return send_reliable;
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Transmit(
    mut chan: *mut netchan_t,
    mut length: libc::c_int,
    mut data: *mut byte,
) {
    let mut send: sizebuf_t = sizebuf_t {
        allowoverflow: false_0,
        overflowed: false_0,
        data: 0 as *const byte as *mut byte,
        maxsize: 0,
        cursize: 0,
        readcount: 0,
    };
    let mut send_buf: [byte; 1400] = [0; 1400];
    let mut send_reliable: qboolean = false_0;
    let mut w1: libc::c_uint = 0;
    let mut w2: libc::c_uint = 0;
    if (*chan).message.overflowed as u64 != 0 {
        (*chan).fatal_error = true_0;
        Com_Printf(
            b"%s:Outgoing message overflow\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            NET_AdrToString((*chan).remote_address),
        );
        return;
    }
    send_reliable = Netchan_NeedReliable(chan);
    if (*chan).reliable_length == 0 && (*chan).message.cursize != 0 {
        memcpy(
            ((*chan).reliable_buf).as_mut_ptr() as *mut libc::c_void,
            ((*chan).message_buf).as_mut_ptr() as *const libc::c_void,
            (*chan).message.cursize as libc::c_ulong,
        );
        (*chan).reliable_length = (*chan).message.cursize;
        (*chan).message.cursize = 0 as libc::c_int;
        (*chan).reliable_sequence ^= 1 as libc::c_int;
    }
    SZ_Init(
        &mut send,
        send_buf.as_mut_ptr(),
        ::std::mem::size_of::<[byte; 1400]>() as libc::c_ulong as libc::c_int,
    );
    w1 = ((*chan).outgoing_sequence & !((1 as libc::c_int) << 31 as libc::c_int))
        as libc::c_uint | (send_reliable as libc::c_uint) << 31 as libc::c_int;
    w2 = ((*chan).incoming_sequence & !((1 as libc::c_int) << 31 as libc::c_int)
        | (*chan).incoming_reliable_sequence << 31 as libc::c_int) as libc::c_uint;
    let ref mut fresh0 = (*chan).outgoing_sequence;
    *fresh0 += 1;
    (*chan).last_sent = curtime;
    MSG_WriteLong(&mut send, w1 as libc::c_int);
    MSG_WriteLong(&mut send, w2 as libc::c_int);
    if (*chan).sock as libc::c_uint == NS_CLIENT as libc::c_int as libc::c_uint {
        MSG_WriteShort(&mut send, (*qport).value as libc::c_int);
    }
    if send_reliable as u64 != 0 {
        SZ_Write(
            &mut send,
            ((*chan).reliable_buf).as_mut_ptr() as *mut libc::c_void,
            (*chan).reliable_length,
        );
        (*chan).last_reliable_sequence = (*chan).outgoing_sequence;
    }
    if send.maxsize - send.cursize >= length {
        SZ_Write(&mut send, data as *mut libc::c_void, length);
    } else {
        Com_Printf(
            b"Netchan_Transmit: dumped unreliable\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    NET_SendPacket(
        (*chan).sock,
        send.cursize,
        send.data as *mut libc::c_void,
        (*chan).remote_address,
    );
    if (*showpackets).value != 0. {
        if send_reliable as u64 != 0 {
            Com_Printf(
                b"send %4i : s=%i reliable=%i ack=%i rack=%i\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                send.cursize,
                (*chan).outgoing_sequence - 1 as libc::c_int,
                (*chan).reliable_sequence,
                (*chan).incoming_sequence,
                (*chan).incoming_reliable_sequence,
            );
        } else {
            Com_Printf(
                b"send %4i : s=%i ack=%i rack=%i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                send.cursize,
                (*chan).outgoing_sequence - 1 as libc::c_int,
                (*chan).incoming_sequence,
                (*chan).incoming_reliable_sequence,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Netchan_Process(
    mut chan: *mut netchan_t,
    mut msg: *mut sizebuf_t,
) -> qboolean {
    let mut sequence: libc::c_uint = 0;
    let mut sequence_ack: libc::c_uint = 0;
    let mut reliable_ack: libc::c_uint = 0;
    let mut reliable_message: libc::c_uint = 0;
    let mut qport_0: libc::c_int = 0;
    MSG_BeginReading(msg);
    sequence = MSG_ReadLong(msg) as libc::c_uint;
    sequence_ack = MSG_ReadLong(msg) as libc::c_uint;
    if (*chan).sock as libc::c_uint == NS_SERVER as libc::c_int as libc::c_uint {
        qport_0 = MSG_ReadShort(msg);
    }
    reliable_message = sequence >> 31 as libc::c_int;
    reliable_ack = sequence_ack >> 31 as libc::c_int;
    sequence &= !((1 as libc::c_int) << 31 as libc::c_int) as libc::c_uint;
    sequence_ack &= !((1 as libc::c_int) << 31 as libc::c_int) as libc::c_uint;
    if (*showpackets).value != 0. {
        if reliable_message != 0 {
            Com_Printf(
                b"recv %4i : s=%i reliable=%i ack=%i rack=%i\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                (*msg).cursize,
                sequence,
                (*chan).incoming_reliable_sequence ^ 1 as libc::c_int,
                sequence_ack,
                reliable_ack,
            );
        } else {
            Com_Printf(
                b"recv %4i : s=%i ack=%i rack=%i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*msg).cursize,
                sequence,
                sequence_ack,
                reliable_ack,
            );
        }
    }
    if sequence <= (*chan).incoming_sequence as libc::c_uint {
        if (*showdrop).value != 0. {
            Com_Printf(
                b"%s:Out of order packet %i at %i\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                NET_AdrToString((*chan).remote_address),
                sequence,
                (*chan).incoming_sequence,
            );
        }
        return false_0;
    }
    (*chan)
        .dropped = sequence
        .wrapping_sub(((*chan).incoming_sequence + 1 as libc::c_int) as libc::c_uint)
        as libc::c_int;
    if (*chan).dropped > 0 as libc::c_int {
        if (*showdrop).value != 0. {
            Com_Printf(
                b"%s:Dropped %i packets at %i\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                NET_AdrToString((*chan).remote_address),
                (*chan).dropped,
                sequence,
            );
        }
    }
    if reliable_ack == (*chan).reliable_sequence as libc::c_uint {
        (*chan).reliable_length = 0 as libc::c_int;
    }
    (*chan).incoming_sequence = sequence as libc::c_int;
    (*chan).incoming_acknowledged = sequence_ack as libc::c_int;
    (*chan).incoming_reliable_acknowledged = reliable_ack as libc::c_int;
    if reliable_message != 0 {
        (*chan).incoming_reliable_sequence ^= 1 as libc::c_int;
    }
    (*chan).last_received = curtime;
    return true_0;
}
