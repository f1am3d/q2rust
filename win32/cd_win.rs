#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn Q_strcasecmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn Com_Printf(msg: *mut libc::c_char, _: ...);
    fn Cmd_AddCommand(cmd_name: *mut libc::c_char, function: xcommand_t);
    fn Cmd_Argc() -> libc::c_int;
    fn Cmd_Argv(arg: libc::c_int) -> *mut libc::c_char;
    fn Cvar_Get(
        var_name: *mut libc::c_char,
        value: *mut libc::c_char,
        flags: libc::c_int,
    ) -> *mut cvar_t;
    fn Cvar_VariableValue(var_name: *mut libc::c_char) -> libc::c_float;
    fn Com_DPrintf(fmt: *mut libc::c_char, _: ...);
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
pub type xcommand_t = Option::<unsafe extern "C" fn() -> ()>;
static mut cdValid: qboolean = false_0;
static mut playing: qboolean = false_0;
static mut wasPlaying: qboolean = false_0;
static mut initialized: qboolean = false_0;
static mut enabled: qboolean = false_0;
static mut playLooping: qboolean = false_0;
static mut remap: [byte; 100] = [0; 100];
static mut playTrack: byte = 0;
static mut maxTrack: byte = 0;
#[no_mangle]
pub static mut cd_nocd: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cd_loopcount: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut cd_looptrack: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut wDeviceID: libc::c_int = 0;
#[no_mangle]
pub static mut loopcounter: libc::c_int = 0;
unsafe extern "C" fn CDAudio_Eject() {}
unsafe extern "C" fn CDAudio_CloseDoor() {}
#[no_mangle]
pub unsafe extern "C" fn CDAudio_Play(mut track: libc::c_int, mut looping: qboolean) {
    loopcounter = 0 as libc::c_int;
    CDAudio_Play2(track, looping);
}
#[no_mangle]
pub unsafe extern "C" fn CDAudio_Stop() {
    if enabled as u64 == 0 {
        return;
    }
    if playing as u64 == 0 {
        return;
    }
    wasPlaying = false_0;
    playing = false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CDAudio_Pause() {
    if enabled as u64 == 0 {
        return;
    }
    if playing as u64 == 0 {
        return;
    }
    wasPlaying = playing;
    playing = false_0;
}
unsafe extern "C" fn CD_f() {
    let mut command: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if Cmd_Argc() < 2 as libc::c_int {
        return;
    }
    command = Cmd_Argv(1 as libc::c_int);
    if Q_strcasecmp(
        command,
        b"on\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        enabled = true_0;
        return;
    }
    if Q_strcasecmp(
        command,
        b"off\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        if playing as u64 != 0 {
            CDAudio_Stop();
        }
        enabled = false_0;
        return;
    }
    if Q_strcasecmp(
        command,
        b"reset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        enabled = true_0;
        if playing as u64 != 0 {
            CDAudio_Stop();
        }
        n = 0 as libc::c_int;
        while n < 100 as libc::c_int {
            remap[n as usize] = n as byte;
            n += 1;
        }
        CDAudio_GetAudioDiskInfo();
        return;
    }
    if Q_strcasecmp(
        command,
        b"remap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        ret = Cmd_Argc() - 2 as libc::c_int;
        if ret <= 0 as libc::c_int {
            n = 1 as libc::c_int;
            while n < 100 as libc::c_int {
                if remap[n as usize] as libc::c_int != n {
                    Com_Printf(
                        b"  %u -> %u\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        n,
                        remap[n as usize] as libc::c_int,
                    );
                }
                n += 1;
            }
            return;
        }
        n = 1 as libc::c_int;
        while n <= ret {
            remap[n as usize] = atoi(Cmd_Argv(n + 1 as libc::c_int)) as byte;
            n += 1;
        }
        return;
    }
    if Q_strcasecmp(
        command,
        b"close\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_CloseDoor();
        return;
    }
    if cdValid as u64 == 0 {
        CDAudio_GetAudioDiskInfo();
        if cdValid as u64 == 0 {
            Com_Printf(
                b"No CD in player.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            return;
        }
    }
    if Q_strcasecmp(
        command,
        b"play\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_Play(atoi(Cmd_Argv(2 as libc::c_int)), false_0);
        return;
    }
    if Q_strcasecmp(
        command,
        b"loop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_Play(atoi(Cmd_Argv(2 as libc::c_int)), true_0);
        return;
    }
    if Q_strcasecmp(
        command,
        b"stop\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_Stop();
        return;
    }
    if Q_strcasecmp(
        command,
        b"pause\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_Pause();
        return;
    }
    if Q_strcasecmp(
        command,
        b"resume\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        CDAudio_Resume();
        return;
    }
    if Q_strcasecmp(
        command,
        b"eject\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        if playing as u64 != 0 {
            CDAudio_Stop();
        }
        CDAudio_Eject();
        cdValid = false_0;
        return;
    }
    if Q_strcasecmp(
        command,
        b"info\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        Com_Printf(
            b"%u tracks\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            maxTrack as libc::c_int,
        );
        if playing as u64 != 0 {
            Com_Printf(
                b"Currently %s track %u\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                if playLooping as libc::c_uint != 0 {
                    b"looping\0" as *const u8 as *const libc::c_char
                } else {
                    b"playing\0" as *const u8 as *const libc::c_char
                },
                playTrack as libc::c_int,
            );
        } else if wasPlaying as u64 != 0 {
            Com_Printf(
                b"Paused %s track %u\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                if playLooping as libc::c_uint != 0 {
                    b"looping\0" as *const u8 as *const libc::c_char
                } else {
                    b"playing\0" as *const u8 as *const libc::c_char
                },
                playTrack as libc::c_int,
            );
        }
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CDAudio_Update() {
    if (*cd_nocd).value != (enabled as u64 == 0) as libc::c_int as libc::c_float {
        if (*cd_nocd).value != 0. {
            CDAudio_Stop();
            enabled = false_0;
        } else {
            enabled = true_0;
            CDAudio_Resume();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CDAudio_Activate(mut active: qboolean) {
    if active as u64 != 0 {
        CDAudio_Resume();
    } else {
        CDAudio_Pause();
    };
}
