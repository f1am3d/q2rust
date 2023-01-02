#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]
extern "C" {
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn CTFCheckRules() -> qboolean;
    fn CTFInMatch() -> qboolean;
    fn CTFNextMap() -> qboolean;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn free(__ptr: *mut libc::c_void);
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_Spawn() -> *mut edict_t;
    fn M_CheckGround(ent: *mut edict_t);
    fn AI_SetSightClient();
    fn BeginIntermission(targ: *mut edict_t);
    fn ClientBeginServerFrame(ent: *mut edict_t);
    fn ServerCommand();
    fn ClientEndServerFrame(ent: *mut edict_t);
    fn G_RunEntity(ent: *mut edict_t);
    static mut ctf: *mut cvar_t;
    fn SpawnEntities(
        mapname: *mut libc::c_char,
        entities: *mut libc::c_char,
        spawnpoint: *mut libc::c_char,
    );
    fn ClientThink(ent: *mut edict_t, cmd: *mut usercmd_t);
    fn ClientConnect(ent: *mut edict_t, userinfo: *mut libc::c_char) -> qboolean;
    fn ClientUserinfoChanged(ent: *mut edict_t, userinfo: *mut libc::c_char);
    fn ClientDisconnect(ent: *mut edict_t);
    fn ClientBegin(ent: *mut edict_t);
    fn ClientCommand(ent: *mut edict_t);
    fn WriteGame(filename: *mut libc::c_char, autosave: qboolean);
    fn ReadGame(filename: *mut libc::c_char);
    fn WriteLevel(filename: *mut libc::c_char);
    fn ReadLevel(filename: *mut libc::c_char);
    fn InitGame();
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
pub type multicast_t = libc::c_uint;
pub const MULTICAST_PVS_R: multicast_t = 5;
pub const MULTICAST_PHS_R: multicast_t = 4;
pub const MULTICAST_ALL_R: multicast_t = 3;
pub const MULTICAST_PVS: multicast_t = 2;
pub const MULTICAST_PHS: multicast_t = 1;
pub const MULTICAST_ALL: multicast_t = 0;
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
pub struct game_import_t {
    pub bprintf: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub dprintf: Option::<unsafe extern "C" fn(*mut libc::c_char, ...) -> ()>,
    pub cprintf: Option::<
        unsafe extern "C" fn(*mut edict_t, libc::c_int, *mut libc::c_char, ...) -> (),
    >,
    pub centerprintf: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut libc::c_char, ...) -> (),
    >,
    pub sound: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub positioned_sound: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut edict_t,
            libc::c_int,
            libc::c_int,
            libc::c_float,
            libc::c_float,
            libc::c_float,
        ) -> (),
    >,
    pub configstring: Option::<
        unsafe extern "C" fn(libc::c_int, *mut libc::c_char) -> (),
    >,
    pub error: Option::<unsafe extern "C" fn(*mut libc::c_char, ...) -> ()>,
    pub modelindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub soundindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub imageindex: Option::<unsafe extern "C" fn(*mut libc::c_char) -> libc::c_int>,
    pub setmodel: Option::<unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> ()>,
    pub trace: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
            *mut vec_t,
            *mut edict_t,
            libc::c_int,
        ) -> trace_t,
    >,
    pub pointcontents: Option::<unsafe extern "C" fn(*mut vec_t) -> libc::c_int>,
    pub inPVS: Option::<unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean>,
    pub inPHS: Option::<unsafe extern "C" fn(*mut vec_t, *mut vec_t) -> qboolean>,
    pub SetAreaPortalState: Option::<unsafe extern "C" fn(libc::c_int, qboolean) -> ()>,
    pub AreasConnected: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> qboolean,
    >,
    pub linkentity: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub unlinkentity: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub BoxEdicts: Option::<
        unsafe extern "C" fn(
            *mut vec_t,
            *mut vec_t,
            *mut *mut edict_t,
            libc::c_int,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub Pmove: Option::<unsafe extern "C" fn(*mut pmove_t) -> ()>,
    pub multicast: Option::<unsafe extern "C" fn(*mut vec_t, multicast_t) -> ()>,
    pub unicast: Option::<unsafe extern "C" fn(*mut edict_t, qboolean) -> ()>,
    pub WriteChar: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteByte: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteShort: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteLong: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub WriteFloat: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub WriteString: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub WritePosition: Option::<unsafe extern "C" fn(*mut vec_t) -> ()>,
    pub WriteDir: Option::<unsafe extern "C" fn(*mut vec_t) -> ()>,
    pub WriteAngle: Option::<unsafe extern "C" fn(libc::c_float) -> ()>,
    pub TagMalloc: Option::<
        unsafe extern "C" fn(libc::c_int, libc::c_int) -> *mut libc::c_void,
    >,
    pub TagFree: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub FreeTags: Option::<unsafe extern "C" fn(libc::c_int) -> ()>,
    pub cvar: Option::<
        unsafe extern "C" fn(
            *mut libc::c_char,
            *mut libc::c_char,
            libc::c_int,
        ) -> *mut cvar_t,
    >,
    pub cvar_set: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub cvar_forceset: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut libc::c_char) -> *mut cvar_t,
    >,
    pub argc: Option::<unsafe extern "C" fn() -> libc::c_int>,
    pub argv: Option::<unsafe extern "C" fn(libc::c_int) -> *mut libc::c_char>,
    pub args: Option::<unsafe extern "C" fn() -> *mut libc::c_char>,
    pub AddCommandString: Option::<unsafe extern "C" fn(*mut libc::c_char) -> ()>,
    pub DebugGraph: Option::<unsafe extern "C" fn(libc::c_float, libc::c_int) -> ()>,
}
pub type cvar_t = cvar_s;
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
pub type edict_t = edict_s;
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
    pub movetype: libc::c_int,
    pub flags: libc::c_int,
    pub model: *mut libc::c_char,
    pub freetime: libc::c_float,
    pub message: *mut libc::c_char,
    pub classname: *mut libc::c_char,
    pub spawnflags: libc::c_int,
    pub timestamp: libc::c_float,
    pub angle: libc::c_float,
    pub target: *mut libc::c_char,
    pub targetname: *mut libc::c_char,
    pub killtarget: *mut libc::c_char,
    pub team: *mut libc::c_char,
    pub pathtarget: *mut libc::c_char,
    pub deathtarget: *mut libc::c_char,
    pub combattarget: *mut libc::c_char,
    pub target_ent: *mut edict_t,
    pub speed: libc::c_float,
    pub accel: libc::c_float,
    pub decel: libc::c_float,
    pub movedir: vec3_t,
    pub pos1: vec3_t,
    pub pos2: vec3_t,
    pub velocity: vec3_t,
    pub avelocity: vec3_t,
    pub mass: libc::c_int,
    pub air_finished: libc::c_float,
    pub gravity: libc::c_float,
    pub goalentity: *mut edict_t,
    pub movetarget: *mut edict_t,
    pub yaw_speed: libc::c_float,
    pub ideal_yaw: libc::c_float,
    pub nextthink: libc::c_float,
    pub prethink: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub think: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub blocked: Option::<unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> ()>,
    pub touch: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            *mut cplane_t,
            *mut csurface_t,
        ) -> (),
    >,
    pub use_0: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut edict_t, *mut edict_t) -> (),
    >,
    pub pain: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            libc::c_float,
            libc::c_int,
        ) -> (),
    >,
    pub die: Option::<
        unsafe extern "C" fn(
            *mut edict_t,
            *mut edict_t,
            *mut edict_t,
            libc::c_int,
            *mut vec_t,
        ) -> (),
    >,
    pub touch_debounce_time: libc::c_float,
    pub pain_debounce_time: libc::c_float,
    pub damage_debounce_time: libc::c_float,
    pub fly_sound_debounce_time: libc::c_float,
    pub last_move_time: libc::c_float,
    pub health: libc::c_int,
    pub max_health: libc::c_int,
    pub gib_health: libc::c_int,
    pub deadflag: libc::c_int,
    pub show_hostile: qboolean,
    pub powerarmor_time: libc::c_float,
    pub map: *mut libc::c_char,
    pub viewheight: libc::c_int,
    pub takedamage: libc::c_int,
    pub dmg: libc::c_int,
    pub radius_dmg: libc::c_int,
    pub dmg_radius: libc::c_float,
    pub sounds: libc::c_int,
    pub count: libc::c_int,
    pub chain: *mut edict_t,
    pub enemy: *mut edict_t,
    pub oldenemy: *mut edict_t,
    pub activator: *mut edict_t,
    pub groundentity: *mut edict_t,
    pub groundentity_linkcount: libc::c_int,
    pub teamchain: *mut edict_t,
    pub teammaster: *mut edict_t,
    pub mynoise: *mut edict_t,
    pub mynoise2: *mut edict_t,
    pub noise_index: libc::c_int,
    pub noise_index2: libc::c_int,
    pub volume: libc::c_float,
    pub attenuation: libc::c_float,
    pub wait: libc::c_float,
    pub delay: libc::c_float,
    pub random: libc::c_float,
    pub teleport_time: libc::c_float,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub move_origin: vec3_t,
    pub move_angles: vec3_t,
    pub light_level: libc::c_int,
    pub style: libc::c_int,
    pub item: *mut gitem_t,
    pub moveinfo: moveinfo_t,
    pub monsterinfo: monsterinfo_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct monsterinfo_t {
    pub currentmove: *mut mmove_t,
    pub aiflags: libc::c_int,
    pub nextframe: libc::c_int,
    pub scale: libc::c_float,
    pub stand: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub idle: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub search: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub walk: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub run: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub dodge: Option::<
        unsafe extern "C" fn(*mut edict_t, *mut edict_t, libc::c_float) -> (),
    >,
    pub attack: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub melee: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    pub sight: Option::<unsafe extern "C" fn(*mut edict_t, *mut edict_t) -> ()>,
    pub checkattack: Option::<unsafe extern "C" fn(*mut edict_t) -> qboolean>,
    pub pausetime: libc::c_float,
    pub attack_finished: libc::c_float,
    pub saved_goal: vec3_t,
    pub search_time: libc::c_float,
    pub trail_time: libc::c_float,
    pub last_sighting: vec3_t,
    pub attack_state: libc::c_int,
    pub lefty: libc::c_int,
    pub idle_time: libc::c_float,
    pub linkcount: libc::c_int,
    pub power_armor_type: libc::c_int,
    pub power_armor_power: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mmove_t {
    pub firstframe: libc::c_int,
    pub lastframe: libc::c_int,
    pub frame: *mut mframe_t,
    pub endfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mframe_t {
    pub aifunc: Option::<unsafe extern "C" fn(*mut edict_t, libc::c_float) -> ()>,
    pub dist: libc::c_float,
    pub thinkfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moveinfo_t {
    pub start_origin: vec3_t,
    pub start_angles: vec3_t,
    pub end_origin: vec3_t,
    pub end_angles: vec3_t,
    pub sound_start: libc::c_int,
    pub sound_middle: libc::c_int,
    pub sound_end: libc::c_int,
    pub accel: libc::c_float,
    pub speed: libc::c_float,
    pub decel: libc::c_float,
    pub distance: libc::c_float,
    pub wait: libc::c_float,
    pub state: libc::c_int,
    pub dir: vec3_t,
    pub current_speed: libc::c_float,
    pub move_speed: libc::c_float,
    pub next_speed: libc::c_float,
    pub remaining_distance: libc::c_float,
    pub decel_distance: libc::c_float,
    pub endfunc: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
}
pub type gitem_t = gitem_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gitem_s {
    pub classname: *mut libc::c_char,
    pub pickup: Option::<unsafe extern "C" fn(*mut edict_s, *mut edict_s) -> qboolean>,
    pub use_0: Option::<unsafe extern "C" fn(*mut edict_s, *mut gitem_s) -> ()>,
    pub drop: Option::<unsafe extern "C" fn(*mut edict_s, *mut gitem_s) -> ()>,
    pub weaponthink: Option::<unsafe extern "C" fn(*mut edict_s) -> ()>,
    pub pickup_sound: *mut libc::c_char,
    pub world_model: *mut libc::c_char,
    pub world_model_flags: libc::c_int,
    pub view_model: *mut libc::c_char,
    pub icon: *mut libc::c_char,
    pub pickup_name: *mut libc::c_char,
    pub count_width: libc::c_int,
    pub quantity: libc::c_int,
    pub ammo: *mut libc::c_char,
    pub flags: libc::c_int,
    pub weapmodel: libc::c_int,
    pub info: *mut libc::c_void,
    pub tag: libc::c_int,
    pub precaches: *mut libc::c_char,
}
pub type csurface_t = csurface_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csurface_s {
    pub name: [libc::c_char; 16],
    pub flags: libc::c_int,
    pub value: libc::c_int,
}
pub type cplane_t = cplane_s;
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
    pub pers: client_persistant_t,
    pub resp: client_respawn_t,
    pub old_pmove: pmove_state_t,
    pub showscores: qboolean,
    pub inmenu: qboolean,
    pub menu: *mut pmenuhnd_t,
    pub showinventory: qboolean,
    pub showhelp: qboolean,
    pub showhelpicon: qboolean,
    pub ammo_index: libc::c_int,
    pub buttons: libc::c_int,
    pub oldbuttons: libc::c_int,
    pub latched_buttons: libc::c_int,
    pub weapon_thunk: qboolean,
    pub newweapon: *mut gitem_t,
    pub damage_armor: libc::c_int,
    pub damage_parmor: libc::c_int,
    pub damage_blood: libc::c_int,
    pub damage_knockback: libc::c_int,
    pub damage_from: vec3_t,
    pub killer_yaw: libc::c_float,
    pub weaponstate: weaponstate_t,
    pub kick_angles: vec3_t,
    pub kick_origin: vec3_t,
    pub v_dmg_roll: libc::c_float,
    pub v_dmg_pitch: libc::c_float,
    pub v_dmg_time: libc::c_float,
    pub fall_time: libc::c_float,
    pub fall_value: libc::c_float,
    pub damage_alpha: libc::c_float,
    pub bonus_alpha: libc::c_float,
    pub damage_blend: vec3_t,
    pub v_angle: vec3_t,
    pub bobtime: libc::c_float,
    pub oldviewangles: vec3_t,
    pub oldvelocity: vec3_t,
    pub next_drown_time: libc::c_float,
    pub old_waterlevel: libc::c_int,
    pub breather_sound: libc::c_int,
    pub machinegun_shots: libc::c_int,
    pub anim_end: libc::c_int,
    pub anim_priority: libc::c_int,
    pub anim_duck: qboolean,
    pub anim_run: qboolean,
    pub quad_framenum: libc::c_float,
    pub invincible_framenum: libc::c_float,
    pub breather_framenum: libc::c_float,
    pub enviro_framenum: libc::c_float,
    pub grenade_blew_up: qboolean,
    pub grenade_time: libc::c_float,
    pub silencer_shots: libc::c_int,
    pub weapon_sound: libc::c_int,
    pub pickup_msg_time: libc::c_float,
    pub flood_locktill: libc::c_float,
    pub flood_when: [libc::c_float; 10],
    pub flood_whenhead: libc::c_int,
    pub respawn_time: libc::c_float,
    pub ctf_grapple: *mut libc::c_void,
    pub ctf_grapplestate: libc::c_int,
    pub ctf_grapplereleasetime: libc::c_float,
    pub ctf_regentime: libc::c_float,
    pub ctf_techsndtime: libc::c_float,
    pub ctf_lasttechmsg: libc::c_float,
    pub chase_target: *mut edict_t,
    pub update_chase: qboolean,
    pub menutime: libc::c_float,
    pub menudirty: qboolean,
}
pub type weaponstate_t = libc::c_uint;
pub const WEAPON_FIRING: weaponstate_t = 3;
pub const WEAPON_DROPPING: weaponstate_t = 2;
pub const WEAPON_ACTIVATING: weaponstate_t = 1;
pub const WEAPON_READY: weaponstate_t = 0;
pub type pmenuhnd_t = pmenuhnd_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmenuhnd_s {
    pub entries: *mut pmenu_s,
    pub cur: libc::c_int,
    pub num: libc::c_int,
    pub arg: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pmenu_s {
    pub text: *mut libc::c_char,
    pub align: libc::c_int,
    pub SelectFunc: SelectFunc_t,
}
pub type SelectFunc_t = Option::<
    unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_respawn_t {
    pub coop_respawn: client_persistant_t,
    pub enterframe: libc::c_int,
    pub score: libc::c_int,
    pub ctf_team: libc::c_int,
    pub ctf_state: libc::c_int,
    pub ctf_lasthurtcarrier: libc::c_float,
    pub ctf_lastreturnedflag: libc::c_float,
    pub ctf_flagsince: libc::c_float,
    pub ctf_lastfraggedcarrier: libc::c_float,
    pub id_state: qboolean,
    pub voted: qboolean,
    pub ready: qboolean,
    pub admin: qboolean,
    pub ghost: *mut ghost_s,
    pub cmd_angles: vec3_t,
    pub game_helpchanged: libc::c_int,
    pub helpchanged: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ghost_s {
    pub netname: [libc::c_char; 16],
    pub number: libc::c_int,
    pub deaths: libc::c_int,
    pub kills: libc::c_int,
    pub caps: libc::c_int,
    pub basedef: libc::c_int,
    pub carrierdef: libc::c_int,
    pub code: libc::c_int,
    pub team: libc::c_int,
    pub score: libc::c_int,
    pub ent: *mut edict_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client_persistant_t {
    pub userinfo: [libc::c_char; 512],
    pub netname: [libc::c_char; 16],
    pub hand: libc::c_int,
    pub connected: qboolean,
    pub health: libc::c_int,
    pub max_health: libc::c_int,
    pub powerArmorActive: qboolean,
    pub selected_item: libc::c_int,
    pub inventory: [libc::c_int; 256],
    pub max_bullets: libc::c_int,
    pub max_shells: libc::c_int,
    pub max_rockets: libc::c_int,
    pub max_grenades: libc::c_int,
    pub max_cells: libc::c_int,
    pub max_slugs: libc::c_int,
    pub weapon: *mut gitem_t,
    pub lastweapon: *mut gitem_t,
    pub power_cubes: libc::c_int,
    pub score: libc::c_int,
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
pub struct pmove_t {
    pub s: pmove_state_t,
    pub cmd: usercmd_t,
    pub snapinitial: qboolean,
    pub numtouch: libc::c_int,
    pub touchents: [*mut edict_s; 32],
    pub viewangles: vec3_t,
    pub viewheight: libc::c_float,
    pub mins: vec3_t,
    pub maxs: vec3_t,
    pub groundentity: *mut edict_s,
    pub watertype: libc::c_int,
    pub waterlevel: libc::c_int,
    pub trace: Option::<
        unsafe extern "C" fn(*mut vec_t, *mut vec_t, *mut vec_t, *mut vec_t) -> trace_t,
    >,
    pub pointcontents: Option::<unsafe extern "C" fn(*mut vec_t) -> libc::c_int>,
}
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
pub type usercmd_t = usercmd_s;
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
pub type gclient_t = gclient_s;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct game_locals_t {
    pub helpmessage1: [libc::c_char; 512],
    pub helpmessage2: [libc::c_char; 512],
    pub helpchanged: libc::c_int,
    pub clients: *mut gclient_t,
    pub spawnpoint: [libc::c_char; 512],
    pub maxclients: libc::c_int,
    pub maxentities: libc::c_int,
    pub serverflags: libc::c_int,
    pub num_items: libc::c_int,
    pub autosaved: qboolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct level_locals_t {
    pub framenum: libc::c_int,
    pub time: libc::c_float,
    pub level_name: [libc::c_char; 64],
    pub mapname: [libc::c_char; 64],
    pub nextmap: [libc::c_char; 64],
    pub forcemap: [libc::c_char; 64],
    pub intermissiontime: libc::c_float,
    pub changemap: *mut libc::c_char,
    pub exitintermission: libc::c_int,
    pub intermission_origin: vec3_t,
    pub intermission_angle: vec3_t,
    pub sight_client: *mut edict_t,
    pub sight_entity: *mut edict_t,
    pub sight_entity_framenum: libc::c_int,
    pub sound_entity: *mut edict_t,
    pub sound_entity_framenum: libc::c_int,
    pub sound2_entity: *mut edict_t,
    pub sound2_entity_framenum: libc::c_int,
    pub pic_health: libc::c_int,
    pub total_secrets: libc::c_int,
    pub found_secrets: libc::c_int,
    pub total_goals: libc::c_int,
    pub found_goals: libc::c_int,
    pub total_monsters: libc::c_int,
    pub killed_monsters: libc::c_int,
    pub current_entity: *mut edict_t,
    pub body_que: libc::c_int,
    pub power_cubes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spawn_temp_t {
    pub sky: *mut libc::c_char,
    pub skyrotate: libc::c_float,
    pub skyaxis: vec3_t,
    pub nextmap: *mut libc::c_char,
    pub lip: libc::c_int,
    pub distance: libc::c_int,
    pub height: libc::c_int,
    pub noise: *mut libc::c_char,
    pub pausetime: libc::c_float,
    pub item: *mut libc::c_char,
    pub gravity: *mut libc::c_char,
    pub minyaw: libc::c_float,
    pub maxyaw: libc::c_float,
    pub minpitch: libc::c_float,
    pub maxpitch: libc::c_float,
}
#[no_mangle]
pub static mut game: game_locals_t = game_locals_t {
    helpmessage1: [0; 512],
    helpmessage2: [0; 512],
    helpchanged: 0,
    clients: 0 as *const gclient_t as *mut gclient_t,
    spawnpoint: [0; 512],
    maxclients: 0,
    maxentities: 0,
    serverflags: 0,
    num_items: 0,
    autosaved: false_0,
};
#[no_mangle]
pub static mut level: level_locals_t = level_locals_t {
    framenum: 0,
    time: 0.,
    level_name: [0; 64],
    mapname: [0; 64],
    nextmap: [0; 64],
    forcemap: [0; 64],
    intermissiontime: 0.,
    changemap: 0 as *const libc::c_char as *mut libc::c_char,
    exitintermission: 0,
    intermission_origin: [0.; 3],
    intermission_angle: [0.; 3],
    sight_client: 0 as *const edict_t as *mut edict_t,
    sight_entity: 0 as *const edict_t as *mut edict_t,
    sight_entity_framenum: 0,
    sound_entity: 0 as *const edict_t as *mut edict_t,
    sound_entity_framenum: 0,
    sound2_entity: 0 as *const edict_t as *mut edict_t,
    sound2_entity_framenum: 0,
    pic_health: 0,
    total_secrets: 0,
    found_secrets: 0,
    total_goals: 0,
    found_goals: 0,
    total_monsters: 0,
    killed_monsters: 0,
    current_entity: 0 as *const edict_t as *mut edict_t,
    body_que: 0,
    power_cubes: 0,
};
#[no_mangle]
pub static mut gi: game_import_t = game_import_t {
    bprintf: None,
    dprintf: None,
    cprintf: None,
    centerprintf: None,
    sound: None,
    positioned_sound: None,
    configstring: None,
    error: None,
    modelindex: None,
    soundindex: None,
    imageindex: None,
    setmodel: None,
    trace: None,
    pointcontents: None,
    inPVS: None,
    inPHS: None,
    SetAreaPortalState: None,
    AreasConnected: None,
    linkentity: None,
    unlinkentity: None,
    BoxEdicts: None,
    Pmove: None,
    multicast: None,
    unicast: None,
    WriteChar: None,
    WriteByte: None,
    WriteShort: None,
    WriteLong: None,
    WriteFloat: None,
    WriteString: None,
    WritePosition: None,
    WriteDir: None,
    WriteAngle: None,
    TagMalloc: None,
    TagFree: None,
    FreeTags: None,
    cvar: None,
    cvar_set: None,
    cvar_forceset: None,
    argc: None,
    argv: None,
    args: None,
    AddCommandString: None,
    DebugGraph: None,
};
#[no_mangle]
pub static mut globals: game_export_t = game_export_t {
    apiversion: 0,
    Init: None,
    Shutdown: None,
    SpawnEntities: None,
    WriteGame: None,
    ReadGame: None,
    WriteLevel: None,
    ReadLevel: None,
    ClientConnect: None,
    ClientBegin: None,
    ClientUserinfoChanged: None,
    ClientDisconnect: None,
    ClientCommand: None,
    ClientThink: None,
    RunFrame: None,
    ServerCommand: None,
    edicts: 0 as *const edict_s as *mut edict_s,
    edict_size: 0,
    num_edicts: 0,
    max_edicts: 0,
};
#[no_mangle]
pub static mut st: spawn_temp_t = spawn_temp_t {
    sky: 0 as *const libc::c_char as *mut libc::c_char,
    skyrotate: 0.,
    skyaxis: [0.; 3],
    nextmap: 0 as *const libc::c_char as *mut libc::c_char,
    lip: 0,
    distance: 0,
    height: 0,
    noise: 0 as *const libc::c_char as *mut libc::c_char,
    pausetime: 0.,
    item: 0 as *const libc::c_char as *mut libc::c_char,
    gravity: 0 as *const libc::c_char as *mut libc::c_char,
    minyaw: 0.,
    maxyaw: 0.,
    minpitch: 0.,
    maxpitch: 0.,
};
#[no_mangle]
pub static mut sm_meat_index: libc::c_int = 0;
#[no_mangle]
pub static mut snd_fry: libc::c_int = 0;
#[no_mangle]
pub static mut meansOfDeath: libc::c_int = 0;
#[no_mangle]
pub static mut g_edicts: *mut edict_t = 0 as *const edict_t as *mut edict_t;
#[no_mangle]
pub static mut deathmatch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut coop: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut dmflags: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut skill: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut fraglimit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut timelimit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut capturelimit: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut instantweap: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut password: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut maxclients: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut maxentities: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut g_select_empty: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut dedicated: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_maxvelocity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_gravity: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_rollspeed: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_rollangle: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gun_x: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gun_y: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut gun_z: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut run_pitch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut run_roll: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut bob_up: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut bob_pitch: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut bob_roll: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_cheats: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut flood_msgs: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut flood_persecond: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut flood_waitdelay: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut sv_maplist: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub unsafe extern "C" fn ShutdownGame() {
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"==== ShutdownGame ====\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    (gi.FreeTags).expect("non-null function pointer")(766 as libc::c_int);
    (gi.FreeTags).expect("non-null function pointer")(765 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn GetGameAPI(
    mut import: *mut game_import_t,
) -> *mut game_export_t {
    gi = *import;
    globals.apiversion = 3 as libc::c_int;
    globals.Init = Some(InitGame as unsafe extern "C" fn() -> ());
    globals.Shutdown = Some(ShutdownGame as unsafe extern "C" fn() -> ());
    globals
        .SpawnEntities = Some(
        SpawnEntities
            as unsafe extern "C" fn(
                *mut libc::c_char,
                *mut libc::c_char,
                *mut libc::c_char,
            ) -> (),
    );
    globals
        .WriteGame = Some(
        WriteGame as unsafe extern "C" fn(*mut libc::c_char, qboolean) -> (),
    );
    globals.ReadGame = Some(ReadGame as unsafe extern "C" fn(*mut libc::c_char) -> ());
    globals
        .WriteLevel = Some(WriteLevel as unsafe extern "C" fn(*mut libc::c_char) -> ());
    globals.ReadLevel = Some(ReadLevel as unsafe extern "C" fn(*mut libc::c_char) -> ());
    globals
        .ClientThink = Some(
        ClientThink as unsafe extern "C" fn(*mut edict_t, *mut usercmd_t) -> (),
    );
    globals
        .ClientConnect = Some(
        ClientConnect
            as unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> qboolean,
    );
    globals
        .ClientUserinfoChanged = Some(
        ClientUserinfoChanged
            as unsafe extern "C" fn(*mut edict_t, *mut libc::c_char) -> (),
    );
    globals
        .ClientDisconnect = Some(
        ClientDisconnect as unsafe extern "C" fn(*mut edict_t) -> (),
    );
    globals.ClientBegin = Some(ClientBegin as unsafe extern "C" fn(*mut edict_t) -> ());
    globals
        .ClientCommand = Some(ClientCommand as unsafe extern "C" fn(*mut edict_t) -> ());
    globals.RunFrame = Some(G_RunFrame as unsafe extern "C" fn() -> ());
    globals.ServerCommand = Some(ServerCommand as unsafe extern "C" fn() -> ());
    globals
        .edict_size = ::std::mem::size_of::<edict_t>() as libc::c_ulong as libc::c_int;
    return &mut globals;
}
#[no_mangle]
pub unsafe extern "C" fn Sys_Error(mut error: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    vsprintf(text.as_mut_ptr(), error, argptr.as_va_list());
    (gi.error)
        .expect(
            "non-null function pointer",
        )(
        0 as *mut libc::c_char,
        b"%s\0" as *const u8 as *const libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn Com_Printf(mut msg: *mut libc::c_char, mut args: ...) {
    let mut argptr: ::std::ffi::VaListImpl;
    let mut text: [libc::c_char; 1024] = [0; 1024];
    argptr = args.clone();
    vsprintf(text.as_mut_ptr(), msg, argptr.as_va_list());
    (gi.dprintf)
        .expect(
            "non-null function pointer",
        )(
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ClientEndServerFrames() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*ent).inuse as u64 == 0 || ((*ent).client).is_null()) {
            ClientEndServerFrame(ent);
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CreateTargetChangeLevel(
    mut map: *mut libc::c_char,
) -> *mut edict_t {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    ent = G_Spawn();
    let ref mut fresh0 = (*ent).classname;
    *fresh0 = b"target_changelevel\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    Com_sprintf(
        (level.nextmap).as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        map,
    );
    let ref mut fresh1 = (*ent).map;
    *fresh1 = (level.nextmap).as_mut_ptr();
    return ent;
}
#[no_mangle]
pub unsafe extern "C" fn EndDMLevel() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut seps: *const libc::c_char = b" ,\n\r\0" as *const u8
        as *const libc::c_char;
    if (*dmflags).value as libc::c_int & 0x20 as libc::c_int != 0 {
        BeginIntermission(CreateTargetChangeLevel((level.mapname).as_mut_ptr()));
        return;
    }
    if *(level.forcemap).as_mut_ptr() != 0 {
        BeginIntermission(CreateTargetChangeLevel((level.forcemap).as_mut_ptr()));
        return;
    }
    if *(*sv_maplist).string != 0 {
        s = strdup((*sv_maplist).string);
        f = 0 as *mut libc::c_char;
        t = strtok(s, seps);
        while !t.is_null() {
            if Q_stricmp(t, (level.mapname).as_mut_ptr()) == 0 as libc::c_int {
                t = strtok(0 as *mut libc::c_char, seps);
                if t.is_null() {
                    if f.is_null() {
                        BeginIntermission(
                            CreateTargetChangeLevel((level.mapname).as_mut_ptr()),
                        );
                    } else {
                        BeginIntermission(CreateTargetChangeLevel(f));
                    }
                } else {
                    BeginIntermission(CreateTargetChangeLevel(t));
                }
                free(s as *mut libc::c_void);
                return;
            }
            if f.is_null() {
                f = t;
            }
            t = strtok(0 as *mut libc::c_char, seps);
        }
        free(s as *mut libc::c_void);
    }
    if level.nextmap[0 as libc::c_int as usize] != 0 {
        BeginIntermission(CreateTargetChangeLevel((level.nextmap).as_mut_ptr()));
    } else {
        ent = G_Find(
            0 as *mut edict_t,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"target_changelevel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        if ent.is_null() {
            BeginIntermission(CreateTargetChangeLevel((level.mapname).as_mut_ptr()));
            return;
        }
        BeginIntermission(ent);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CheckDMRules() {
    let mut i: libc::c_int = 0;
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    if level.intermissiontime != 0. {
        return;
    }
    if (*deathmatch).value == 0. {
        return;
    }
    if (*ctf).value != 0. && CTFCheckRules() as libc::c_uint != 0 {
        EndDMLevel();
        return;
    }
    if CTFInMatch() as u64 != 0 {
        return;
    }
    if (*timelimit).value != 0. {
        if level.time >= (*timelimit).value * 60 as libc::c_int as libc::c_float {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                2 as libc::c_int,
                b"Timelimit hit.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            EndDMLevel();
            return;
        }
    }
    if (*fraglimit).value != 0. {
        i = 0 as libc::c_int;
        while (i as libc::c_float) < (*maxclients).value {
            cl = (game.clients).offset(i as isize);
            if !((*g_edicts.offset((i + 1 as libc::c_int) as isize)).inuse as u64 == 0) {
                if (*cl).resp.score as libc::c_float >= (*fraglimit).value {
                    (gi.bprintf)
                        .expect(
                            "non-null function pointer",
                        )(
                        2 as libc::c_int,
                        b"Fraglimit hit.\n\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    EndDMLevel();
                    return;
                }
            }
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ExitLevel() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut command: [libc::c_char; 256] = [0; 256];
    level.exitintermission = 0 as libc::c_int;
    level.intermissiontime = 0 as libc::c_int as libc::c_float;
    if CTFNextMap() as u64 != 0 {
        return;
    }
    Com_sprintf(
        command.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        b"gamemap \"%s\"\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        level.changemap,
    );
    (gi.AddCommandString).expect("non-null function pointer")(command.as_mut_ptr());
    ClientEndServerFrames();
    level.changemap = 0 as *mut libc::c_char;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*ent).inuse as u64 == 0) {
            if (*ent).health > (*(*ent).client).pers.max_health {
                (*ent).health = (*(*ent).client).pers.max_health;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn G_RunFrame() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    level.framenum += 1;
    level.time = (level.framenum as libc::c_double * 0.1f64) as libc::c_float;
    AI_SetSightClient();
    if level.exitintermission != 0 {
        ExitLevel();
        return;
    }
    ent = &mut *g_edicts.offset(0 as libc::c_int as isize) as *mut edict_t;
    i = 0 as libc::c_int;
    while i < globals.num_edicts {
        if !((*ent).inuse as u64 == 0) {
            level.current_entity = ent;
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
            if !((*ent).groundentity).is_null()
                && (*(*ent).groundentity).linkcount != (*ent).groundentity_linkcount
            {
                let ref mut fresh2 = (*ent).groundentity;
                *fresh2 = 0 as *mut edict_t;
                if (*ent).flags & (0x2 as libc::c_int | 0x1 as libc::c_int) == 0
                    && (*ent).svflags & 0x4 as libc::c_int != 0
                {
                    M_CheckGround(ent);
                }
            }
            if i > 0 as libc::c_int && i as libc::c_float <= (*maxclients).value {
                ClientBeginServerFrame(ent);
            } else {
                G_RunEntity(ent);
            }
        }
        i += 1;
        ent = ent.offset(1);
    }
    CheckDMRules();
    ClientEndServerFrames();
}
