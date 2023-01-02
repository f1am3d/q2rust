#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
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
}
pub type POINTER = *mut libc::c_uchar;
pub type UINT4 = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MD4_CTX {
    pub state: [UINT4; 4],
    pub count: [UINT4; 2],
    pub buffer: [libc::c_uchar; 64],
}
static mut PADDING: [libc::c_uchar; 64] = [
    0x80 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn MD4Init(mut context: *mut MD4_CTX) {
    let ref mut fresh0 = (*context).count[1 as libc::c_int as usize];
    *fresh0 = 0 as libc::c_int as UINT4;
    (*context).count[0 as libc::c_int as usize] = *fresh0;
    (*context).state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as UINT4;
    (*context).state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint as UINT4;
    (*context).state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint as UINT4;
    (*context).state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as UINT4;
}
#[no_mangle]
pub unsafe extern "C" fn MD4Update(
    mut context: *mut MD4_CTX,
    mut input: *mut libc::c_uchar,
    mut inputLen: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut index: libc::c_uint = 0;
    let mut partLen: libc::c_uint = 0;
    index = ((*context).count[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_ulong) as libc::c_uint;
    let ref mut fresh1 = (*context).count[0 as libc::c_int as usize];
    *fresh1 = (*fresh1 as libc::c_ulong)
        .wrapping_add((inputLen as UINT4) << 3 as libc::c_int) as UINT4 as UINT4;
    if *fresh1 < (inputLen as UINT4) << 3 as libc::c_int {
        let ref mut fresh2 = (*context).count[1 as libc::c_int as usize];
        *fresh2 = (*fresh2).wrapping_add(1);
    }
    let ref mut fresh3 = (*context).count[1 as libc::c_int as usize];
    *fresh3 = (*fresh3 as libc::c_ulong)
        .wrapping_add(inputLen as UINT4 >> 29 as libc::c_int) as UINT4 as UINT4;
    partLen = (64 as libc::c_int as libc::c_uint).wrapping_sub(index);
    if inputLen >= partLen {
        memcpy(
            &mut *((*context).buffer).as_mut_ptr().offset(index as isize)
                as *mut libc::c_uchar as *mut libc::c_void,
            input as *const libc::c_void,
            partLen as libc::c_ulong,
        );
        MD4Transform(((*context).state).as_mut_ptr(), ((*context).buffer).as_mut_ptr());
        i = partLen;
        while i.wrapping_add(63 as libc::c_int as libc::c_uint) < inputLen {
            MD4Transform(
                ((*context).state).as_mut_ptr(),
                &mut *input.offset(i as isize),
            );
            i = i.wrapping_add(64 as libc::c_int as libc::c_uint);
        }
        index = 0 as libc::c_int as libc::c_uint;
    } else {
        i = 0 as libc::c_int as libc::c_uint;
    }
    memcpy(
        &mut *((*context).buffer).as_mut_ptr().offset(index as isize)
            as *mut libc::c_uchar as *mut libc::c_void,
        &mut *input.offset(i as isize) as *mut libc::c_uchar as *const libc::c_void,
        inputLen.wrapping_sub(i) as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn MD4Final(
    mut digest: *mut libc::c_uchar,
    mut context: *mut MD4_CTX,
) {
    let mut bits: [libc::c_uchar; 8] = [0; 8];
    let mut index: libc::c_uint = 0;
    let mut padLen: libc::c_uint = 0;
    Encode(
        bits.as_mut_ptr(),
        ((*context).count).as_mut_ptr(),
        8 as libc::c_int as libc::c_uint,
    );
    index = ((*context).count[0 as libc::c_int as usize] >> 3 as libc::c_int
        & 0x3f as libc::c_int as libc::c_ulong) as libc::c_uint;
    padLen = if index < 56 as libc::c_int as libc::c_uint {
        (56 as libc::c_int as libc::c_uint).wrapping_sub(index)
    } else {
        (120 as libc::c_int as libc::c_uint).wrapping_sub(index)
    };
    MD4Update(context, PADDING.as_mut_ptr(), padLen);
    MD4Update(context, bits.as_mut_ptr(), 8 as libc::c_int as libc::c_uint);
    Encode(digest, ((*context).state).as_mut_ptr(), 16 as libc::c_int as libc::c_uint);
    memset(
        context as POINTER as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<MD4_CTX>() as libc::c_ulong,
    );
}
unsafe extern "C" fn MD4Transform(mut state: *mut UINT4, mut block: *mut libc::c_uchar) {
    let mut a: UINT4 = *state.offset(0 as libc::c_int as isize);
    let mut b: UINT4 = *state.offset(1 as libc::c_int as isize);
    let mut c: UINT4 = *state.offset(2 as libc::c_int as isize);
    let mut d: UINT4 = *state.offset(3 as libc::c_int as isize);
    let mut x: [UINT4; 16] = [0; 16];
    Decode(x.as_mut_ptr(), block, 64 as libc::c_int as libc::c_uint);
    a = (a as libc::c_ulong)
        .wrapping_add((b & c | !b & d).wrapping_add(x[0 as libc::c_int as usize]))
        as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add((a & b | !a & c).wrapping_add(x[1 as libc::c_int as usize]))
        as UINT4 as UINT4;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add((d & a | !d & b).wrapping_add(x[2 as libc::c_int as usize]))
        as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add((c & d | !c & a).wrapping_add(x[3 as libc::c_int as usize]))
        as UINT4 as UINT4;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add((b & c | !b & d).wrapping_add(x[4 as libc::c_int as usize]))
        as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add((a & b | !a & c).wrapping_add(x[5 as libc::c_int as usize]))
        as UINT4 as UINT4;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add((d & a | !d & b).wrapping_add(x[6 as libc::c_int as usize]))
        as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add((c & d | !c & a).wrapping_add(x[7 as libc::c_int as usize]))
        as UINT4 as UINT4;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add((b & c | !b & d).wrapping_add(x[8 as libc::c_int as usize]))
        as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add((a & b | !a & c).wrapping_add(x[9 as libc::c_int as usize]))
        as UINT4 as UINT4;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add((d & a | !d & b).wrapping_add(x[10 as libc::c_int as usize]))
        as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add((c & d | !c & a).wrapping_add(x[11 as libc::c_int as usize]))
        as UINT4 as UINT4;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add((b & c | !b & d).wrapping_add(x[12 as libc::c_int as usize]))
        as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add((a & b | !a & c).wrapping_add(x[13 as libc::c_int as usize]))
        as UINT4 as UINT4;
    d = d << 7 as libc::c_int | d >> 32 as libc::c_int - 7 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add((d & a | !d & b).wrapping_add(x[14 as libc::c_int as usize]))
        as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add((c & d | !c & a).wrapping_add(x[15 as libc::c_int as usize]))
        as UINT4 as UINT4;
    b = b << 19 as libc::c_int | b >> 32 as libc::c_int - 19 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b & c | b & d | c & d)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a & b | a & c | b & c)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 5 as libc::c_int | d >> 32 as libc::c_int - 5 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d & a | d & b | a & b)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 9 as libc::c_int | c >> 32 as libc::c_int - 9 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c & d | c & a | d & a)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x5a827999 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 13 as libc::c_int | b >> 32 as libc::c_int - 13 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[0 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[8 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[4 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[12 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[2 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[10 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[6 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[14 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[1 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[9 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[5 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[13 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    a = (a as libc::c_ulong)
        .wrapping_add(
            (b ^ c ^ d)
                .wrapping_add(x[3 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    a = a << 3 as libc::c_int | a >> 32 as libc::c_int - 3 as libc::c_int;
    d = (d as libc::c_ulong)
        .wrapping_add(
            (a ^ b ^ c)
                .wrapping_add(x[11 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    d = d << 9 as libc::c_int | d >> 32 as libc::c_int - 9 as libc::c_int;
    c = (c as libc::c_ulong)
        .wrapping_add(
            (d ^ a ^ b)
                .wrapping_add(x[7 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    c = c << 11 as libc::c_int | c >> 32 as libc::c_int - 11 as libc::c_int;
    b = (b as libc::c_ulong)
        .wrapping_add(
            (c ^ d ^ a)
                .wrapping_add(x[15 as libc::c_int as usize])
                .wrapping_add(0x6ed9eba1 as libc::c_int as UINT4),
        ) as UINT4 as UINT4;
    b = b << 15 as libc::c_int | b >> 32 as libc::c_int - 15 as libc::c_int;
    let ref mut fresh4 = *state.offset(0 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_ulong).wrapping_add(a) as UINT4 as UINT4;
    let ref mut fresh5 = *state.offset(1 as libc::c_int as isize);
    *fresh5 = (*fresh5 as libc::c_ulong).wrapping_add(b) as UINT4 as UINT4;
    let ref mut fresh6 = *state.offset(2 as libc::c_int as isize);
    *fresh6 = (*fresh6 as libc::c_ulong).wrapping_add(c) as UINT4 as UINT4;
    let ref mut fresh7 = *state.offset(3 as libc::c_int as isize);
    *fresh7 = (*fresh7 as libc::c_ulong).wrapping_add(d) as UINT4 as UINT4;
    memset(
        x.as_mut_ptr() as POINTER as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[UINT4; 16]>() as libc::c_ulong,
    );
}
unsafe extern "C" fn Encode(
    mut output: *mut libc::c_uchar,
    mut input: *mut UINT4,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < len {
        *output
            .offset(
                j as isize,
            ) = (*input.offset(i as isize) & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *output
            .offset(
                j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
            ) = (*input.offset(i as isize) >> 24 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
unsafe extern "C" fn Decode(
    mut output: *mut UINT4,
    mut input: *mut libc::c_uchar,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    j = 0 as libc::c_int as libc::c_uint;
    while j < len {
        *output
            .offset(
                i as isize,
            ) = *input.offset(j as isize) as UINT4
            | (*input.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 8 as libc::c_int
            | (*input.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 16 as libc::c_int
            | (*input.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                as UINT4) << 24 as libc::c_int;
        i = i.wrapping_add(1);
        j = j.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn Com_BlockChecksum(
    mut buffer: *mut libc::c_void,
    mut length: libc::c_int,
) -> libc::c_uint {
    let mut digest: [libc::c_int; 4] = [0; 4];
    let mut val: libc::c_uint = 0;
    let mut ctx: MD4_CTX = MD4_CTX {
        state: [0; 4],
        count: [0; 2],
        buffer: [0; 64],
    };
    MD4Init(&mut ctx);
    MD4Update(&mut ctx, buffer as *mut libc::c_uchar, length as libc::c_uint);
    MD4Final(digest.as_mut_ptr() as *mut libc::c_uchar, &mut ctx);
    val = (digest[0 as libc::c_int as usize] ^ digest[1 as libc::c_int as usize]
        ^ digest[2 as libc::c_int as usize] ^ digest[3 as libc::c_int as usize])
        as libc::c_uint;
    return val;
}
