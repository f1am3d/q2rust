#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(register_tool)]
extern "C" {
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut vec3_origin: vec3_t;
    fn VectorMA(
        veca: *mut vec_t,
        scale: libc::c_float,
        vecb: *mut vec_t,
        vecc: *mut vec_t,
    );
    fn VectorLength(v: *mut vec_t) -> vec_t;
    fn VectorNormalize(v: *mut vec_t) -> vec_t;
    fn VectorScale(in_0: *mut vec_t, scale: vec_t, out: *mut vec_t);
    fn AngleVectors(
        angles: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        up: *mut vec_t,
    );
    fn Com_sprintf(
        dest: *mut libc::c_char,
        size: libc::c_int,
        fmt: *mut libc::c_char,
        _: ...
    );
    fn Q_stricmp(s1: *mut libc::c_char, s2: *mut libc::c_char) -> libc::c_int;
    fn va(format: *mut libc::c_char, _: ...) -> *mut libc::c_char;
    fn Info_ValueForKey(
        s: *mut libc::c_char,
        key: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn PMenu_Open(
        ent: *mut edict_t,
        entries: *mut pmenu_t,
        cur: libc::c_int,
        num: libc::c_int,
        arg: *mut libc::c_void,
    ) -> *mut pmenuhnd_t;
    fn PMenu_Close(ent: *mut edict_t);
    fn PMenu_UpdateEntry(
        entry: *mut pmenu_t,
        text: *const libc::c_char,
        align: libc::c_int,
        SelectFunc: SelectFunc_t,
    );
    fn PMenu_Update(ent: *mut edict_t);
    static mut game: game_locals_t;
    static mut level: level_locals_t;
    static mut gi: game_import_t;
    static mut globals: game_export_t;
    static mut g_edicts: *mut edict_t;
    static mut dmflags: *mut cvar_t;
    static mut capturelimit: *mut cvar_t;
    static mut instantweap: *mut cvar_t;
    static mut maxclients: *mut cvar_t;
    static mut itemlist: [gitem_t; 0];
    fn CheckFlood(ent: *mut edict_t) -> qboolean;
    fn FindItem(pickup_name: *mut libc::c_char) -> *mut gitem_t;
    fn FindItemByClassname(classname: *mut libc::c_char) -> *mut gitem_t;
    fn Drop_Item(ent: *mut edict_t, item: *mut gitem_t) -> *mut edict_t;
    fn ArmorIndex(ent: *mut edict_t) -> libc::c_int;
    fn PowerArmorType(ent: *mut edict_t) -> libc::c_int;
    fn GetItemByIndex(index: libc::c_int) -> *mut gitem_t;
    fn Touch_Item(
        ent: *mut edict_t,
        other: *mut edict_t,
        plane: *mut cplane_t,
        surf: *mut csurface_t,
    );
    fn KillBox(ent: *mut edict_t) -> qboolean;
    fn G_Find(
        from: *mut edict_t,
        fieldofs: libc::c_int,
        match_0: *mut libc::c_char,
    ) -> *mut edict_t;
    fn G_Spawn() -> *mut edict_t;
    fn G_FreeEdict(e: *mut edict_t);
    fn tv(x: libc::c_float, y: libc::c_float, z: libc::c_float) -> *mut libc::c_float;
    fn vtos(v: *mut vec_t) -> *mut libc::c_char;
    fn vectoangles(vec: *mut vec_t, angles: *mut vec_t);
    fn CheckTeamDamage(targ: *mut edict_t, attacker: *mut edict_t) -> qboolean;
    fn T_Damage(
        targ: *mut edict_t,
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        dir: *mut vec_t,
        point: *mut vec_t,
        normal: *mut vec_t,
        damage: libc::c_int,
        knockback: libc::c_int,
        dflags: libc::c_int,
        mod_0: libc::c_int,
    );
    fn respawn(ent: *mut edict_t);
    fn PutClientInServer(ent: *mut edict_t);
    fn player_die(
        self_0: *mut edict_t,
        inflictor: *mut edict_t,
        attacker: *mut edict_t,
        damage: libc::c_int,
        point: *mut vec_t,
    );
    fn PlayerNoise(who: *mut edict_t, where_0: *mut vec_t, type_0: libc::c_int);
    fn P_ProjectSource(
        client: *mut gclient_t,
        point: *mut vec_t,
        distance: *mut vec_t,
        forward: *mut vec_t,
        right: *mut vec_t,
        result: *mut vec_t,
    );
    fn Weapon_Generic(
        ent: *mut edict_t,
        FRAME_ACTIVATE_LAST: libc::c_int,
        FRAME_FIRE_LAST: libc::c_int,
        FRAME_IDLE_LAST: libc::c_int,
        FRAME_DEACTIVATE_LAST: libc::c_int,
        pause_frames: *mut libc::c_int,
        fire_frames: *mut libc::c_int,
        fire: Option::<unsafe extern "C" fn(*mut edict_t) -> ()>,
    );
    fn EndDMLevel();
    fn SelectRandomDeathmatchSpawnPoint() -> *mut edict_t;
    fn SelectFarthestDeathmatchSpawnPoint() -> *mut edict_t;
    fn PlayersRangeFromSpot(spot: *mut edict_t) -> libc::c_float;
    fn SV_AddGravity(ent: *mut edict_t);
    fn DoRespawn(ent: *mut edict_t);
    fn DeathmatchScoreboard(ent: *mut edict_t);
}
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
pub type cplane_t = cplane_s;
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
pub type edict_t = edict_s;
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
pub type C2RustUnnamed = libc::c_uint;
pub const TE_FLECHETTE: C2RustUnnamed = 55;
pub const TE_EXPLOSION1_NP: C2RustUnnamed = 54;
pub const TE_EXPLOSION1_BIG: C2RustUnnamed = 53;
pub const TE_WIDOWSPLASH: C2RustUnnamed = 52;
pub const TE_NUKEBLAST: C2RustUnnamed = 51;
pub const TE_WIDOWBEAMOUT: C2RustUnnamed = 50;
pub const TE_DBALL_GOAL: C2RustUnnamed = 49;
pub const TE_TELEPORT_EFFECT: C2RustUnnamed = 48;
pub const TE_TRACKER_EXPLOSION: C2RustUnnamed = 47;
pub const TE_ELECTRIC_SPARKS: C2RustUnnamed = 46;
pub const TE_CHAINFIST_SMOKE: C2RustUnnamed = 45;
pub const TE_HEATBEAM_STEAM: C2RustUnnamed = 44;
pub const TE_HEATBEAM_SPARKS: C2RustUnnamed = 43;
pub const TE_MOREBLOOD: C2RustUnnamed = 42;
pub const TE_BUBBLETRAIL2: C2RustUnnamed = 41;
pub const TE_STEAM: C2RustUnnamed = 40;
pub const TE_MONSTER_HEATBEAM: C2RustUnnamed = 39;
pub const TE_HEATBEAM: C2RustUnnamed = 38;
pub const TE_FORCEWALL: C2RustUnnamed = 37;
pub const TE_FLASHLIGHT: C2RustUnnamed = 36;
pub const TE_PLAIN_EXPLOSION: C2RustUnnamed = 35;
pub const TE_DEBUGTRAIL: C2RustUnnamed = 34;
pub const TE_LIGHTNING: C2RustUnnamed = 33;
pub const TE_FLAME: C2RustUnnamed = 32;
pub const TE_RAILTRAIL2: C2RustUnnamed = 31;
pub const TE_BLASTER2: C2RustUnnamed = 30;
pub const TE_TUNNEL_SPARKS: C2RustUnnamed = 29;
pub const TE_PLASMA_EXPLOSION: C2RustUnnamed = 28;
pub const TE_BLUEHYPERBLASTER: C2RustUnnamed = 27;
pub const TE_GREENBLOOD: C2RustUnnamed = 26;
pub const TE_WELDING_SPARKS: C2RustUnnamed = 25;
pub const TE_GRAPPLE_CABLE: C2RustUnnamed = 24;
pub const TE_BFG_LASER: C2RustUnnamed = 23;
pub const TE_BOSSTPORT: C2RustUnnamed = 22;
pub const TE_BFG_BIGEXPLOSION: C2RustUnnamed = 21;
pub const TE_BFG_EXPLOSION: C2RustUnnamed = 20;
pub const TE_MEDIC_CABLE_ATTACK: C2RustUnnamed = 19;
pub const TE_GRENADE_EXPLOSION_WATER: C2RustUnnamed = 18;
pub const TE_ROCKET_EXPLOSION_WATER: C2RustUnnamed = 17;
pub const TE_PARASITE_ATTACK: C2RustUnnamed = 16;
pub const TE_LASER_SPARKS: C2RustUnnamed = 15;
pub const TE_BULLET_SPARKS: C2RustUnnamed = 14;
pub const TE_SHIELD_SPARKS: C2RustUnnamed = 13;
pub const TE_SCREEN_SPARKS: C2RustUnnamed = 12;
pub const TE_BUBBLETRAIL: C2RustUnnamed = 11;
pub const TE_SPLASH: C2RustUnnamed = 10;
pub const TE_SPARKS: C2RustUnnamed = 9;
pub const TE_GRENADE_EXPLOSION: C2RustUnnamed = 8;
pub const TE_ROCKET_EXPLOSION: C2RustUnnamed = 7;
pub const TE_EXPLOSION2: C2RustUnnamed = 6;
pub const TE_EXPLOSION1: C2RustUnnamed = 5;
pub const TE_SHOTGUN: C2RustUnnamed = 4;
pub const TE_RAILTRAIL: C2RustUnnamed = 3;
pub const TE_BLASTER: C2RustUnnamed = 2;
pub const TE_BLOOD: C2RustUnnamed = 1;
pub const TE_GUNSHOT: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const EV_OTHER_TELEPORT: C2RustUnnamed_0 = 7;
pub const EV_PLAYER_TELEPORT: C2RustUnnamed_0 = 6;
pub const EV_FALLFAR: C2RustUnnamed_0 = 5;
pub const EV_FALL: C2RustUnnamed_0 = 4;
pub const EV_FALLSHORT: C2RustUnnamed_0 = 3;
pub const EV_FOOTSTEP: C2RustUnnamed_0 = 2;
pub const EV_ITEM_RESPAWN: C2RustUnnamed_0 = 1;
pub const EV_NONE: C2RustUnnamed_0 = 0;
pub type gclient_t = gclient_s;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const PMENU_ALIGN_RIGHT: C2RustUnnamed_1 = 2;
pub const PMENU_ALIGN_CENTER: C2RustUnnamed_1 = 1;
pub const PMENU_ALIGN_LEFT: C2RustUnnamed_1 = 0;
pub type pmenu_t = pmenu_s;
pub type C2RustUnnamed_2 = libc::c_uint;
pub const MOVETYPE_BOUNCE: C2RustUnnamed_2 = 9;
pub const MOVETYPE_FLYMISSILE: C2RustUnnamed_2 = 8;
pub const MOVETYPE_TOSS: C2RustUnnamed_2 = 7;
pub const MOVETYPE_FLY: C2RustUnnamed_2 = 6;
pub const MOVETYPE_STEP: C2RustUnnamed_2 = 5;
pub const MOVETYPE_WALK: C2RustUnnamed_2 = 4;
pub const MOVETYPE_STOP: C2RustUnnamed_2 = 3;
pub const MOVETYPE_PUSH: C2RustUnnamed_2 = 2;
pub const MOVETYPE_NOCLIP: C2RustUnnamed_2 = 1;
pub const MOVETYPE_NONE: C2RustUnnamed_2 = 0;
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
pub type C2RustUnnamed_3 = libc::c_uint;
pub const CTF_TEAM2: C2RustUnnamed_3 = 2;
pub const CTF_TEAM1: C2RustUnnamed_3 = 1;
pub const CTF_NOTEAM: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_4 = libc::c_uint;
pub const CTF_GRAPPLE_STATE_HANG: C2RustUnnamed_4 = 2;
pub const CTF_GRAPPLE_STATE_PULL: C2RustUnnamed_4 = 1;
pub const CTF_GRAPPLE_STATE_FLY: C2RustUnnamed_4 = 0;
pub type ghost_t = ghost_s;
pub type ctfgame_t = ctfgame_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctfgame_s {
    pub team1: libc::c_int,
    pub team2: libc::c_int,
    pub total1: libc::c_int,
    pub total2: libc::c_int,
    pub last_flag_capture: libc::c_float,
    pub last_capture_team: libc::c_int,
    pub match_0: match_t,
    pub matchtime: libc::c_float,
    pub lasttime: libc::c_int,
    pub election: elect_t,
    pub etarget: *mut edict_t,
    pub elevel: [libc::c_char; 32],
    pub evotes: libc::c_int,
    pub needvotes: libc::c_int,
    pub electtime: libc::c_float,
    pub emsg: [libc::c_char; 256],
    pub ghosts: [ghost_t; 256],
}
pub type elect_t = libc::c_uint;
pub const ELECT_MAP: elect_t = 3;
pub const ELECT_ADMIN: elect_t = 2;
pub const ELECT_MATCH: elect_t = 1;
pub const ELECT_NONE: elect_t = 0;
pub type match_t = match_s;
pub type match_s = libc::c_uint;
pub const MATCH_POST: match_s = 4;
pub const MATCH_GAME: match_s = 3;
pub const MATCH_PREGAME: match_s = 2;
pub const MATCH_SETUP: match_s = 1;
pub const MATCH_NONE: match_s = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub classname: *mut libc::c_char,
    pub priority: libc::c_int,
}
pub type admin_settings_t = admin_settings_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct admin_settings_s {
    pub matchlen: libc::c_int,
    pub matchsetuplen: libc::c_int,
    pub matchstartlen: libc::c_int,
    pub weaponsstay: qboolean,
    pub instantitems: qboolean,
    pub quaddrop: qboolean,
    pub instantweap: qboolean,
    pub matchlock: qboolean,
}
#[no_mangle]
pub static mut ctfgame: ctfgame_t = ctfgame_t {
    team1: 0,
    team2: 0,
    total1: 0,
    total2: 0,
    last_flag_capture: 0.,
    last_capture_team: 0,
    match_0: MATCH_NONE,
    matchtime: 0.,
    lasttime: 0,
    election: ELECT_NONE,
    etarget: 0 as *const edict_t as *mut edict_t,
    elevel: [0; 32],
    evotes: 0,
    needvotes: 0,
    electtime: 0.,
    emsg: [0; 256],
    ghosts: [ghost_t {
        netname: [0; 16],
        number: 0,
        deaths: 0,
        kills: 0,
        caps: 0,
        basedef: 0,
        carrierdef: 0,
        code: 0,
        team: 0,
        score: 0,
        ent: 0 as *const edict_t as *mut edict_t,
    }; 256],
};
#[no_mangle]
pub static mut ctf: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut ctf_forcejoin: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut competition: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut matchlock: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut electpercentage: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut matchtime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut matchsetuptime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut matchstarttime: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut admin_password: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut warp_list: *mut cvar_t = 0 as *const cvar_t as *mut cvar_t;
#[no_mangle]
pub static mut ctf_statusbar: *mut libc::c_char = b"yb\t-24 xv\t0 hnum xv\t50 pic 0 if 2 \txv\t100 \tanum \txv\t150 \tpic 2 endif if 4 \txv\t200 \trnum \txv\t250 \tpic 4 endif if 6 \txv\t296 \tpic 6 endif yb\t-50 if 7 \txv\t0 \tpic 7 \txv\t26 \tyb\t-42 \tstat_string 8 \tyb\t-50 endif if 9 xv 246 num 2 10 xv 296 pic 9 endif if 11 xv 148 pic 11 endif xr\t-50 yt 2 num 3 14 yb -129 if 26 xr -26 pic 26 endif yb -102 if 17 xr -26 pic 17 endif xr -62 num 2 18 if 22 yb -104 xr -28 pic 22 endif yb -75 if 19 xr -26 pic 19 endif xr -62 num 2 20 if 23 yb -77 xr -28 pic 23 endif if 21 yt 26 xr -24 pic 21 endif if 27 xv 0 yb -58 string \"Viewing\" xv 64 stat_string 27 endif if 28 xl 0 yb -78 stat_string 28 endif \0"
    as *const u8 as *const libc::c_char as *mut libc::c_char;
static mut tnames: [*mut libc::c_char; 5] = [
    b"item_tech1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"item_tech2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"item_tech3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"item_tech4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    0 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn stuffcmd(mut ent: *mut edict_t, mut s: *mut libc::c_char) {
    (gi.WriteByte).expect("non-null function pointer")(11 as libc::c_int);
    (gi.WriteString).expect("non-null function pointer")(s);
    (gi.unicast).expect("non-null function pointer")(ent, true_0);
}
unsafe extern "C" fn loc_findradius(
    mut from: *mut edict_t,
    mut org: *mut vec_t,
    mut rad: libc::c_float,
) -> *mut edict_t {
    let mut eorg: vec3_t = [0.; 3];
    let mut j: libc::c_int = 0;
    if from.is_null() {
        from = g_edicts;
    } else {
        from = from.offset(1);
    }
    while from < &mut *g_edicts.offset(globals.num_edicts as isize) as *mut edict_t {
        if !((*from).inuse as u64 == 0) {
            j = 0 as libc::c_int;
            while j < 3 as libc::c_int {
                eorg[j
                    as usize] = (*org.offset(j as isize) as libc::c_double
                    - ((*from).s.origin[j as usize] as libc::c_double
                        + ((*from).mins[j as usize] + (*from).maxs[j as usize])
                            as libc::c_double * 0.5f64)) as vec_t;
                j += 1;
            }
            if !(VectorLength(eorg.as_mut_ptr()) > rad) {
                return from;
            }
        }
        from = from.offset(1);
    }
    return 0 as *mut edict_t;
}
unsafe extern "C" fn loc_buildboxpoints(
    mut p: *mut vec3_t,
    mut org: *mut vec_t,
    mut mins: *mut vec_t,
    mut maxs: *mut vec_t,
) {
    (*p
        .offset(
            0 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = *org.offset(0 as libc::c_int as isize)
        + *mins.offset(0 as libc::c_int as isize);
    (*p
        .offset(
            0 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = *org.offset(1 as libc::c_int as isize)
        + *mins.offset(1 as libc::c_int as isize);
    (*p
        .offset(
            0 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = *org.offset(2 as libc::c_int as isize)
        + *mins.offset(2 as libc::c_int as isize);
    (*p
        .offset(
            1 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            1 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            1 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh0 = (*p
        .offset(1 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh0 -= *mins.offset(0 as libc::c_int as isize);
    (*p
        .offset(
            2 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            2 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            2 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh1 = (*p
        .offset(2 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh1 -= *mins.offset(1 as libc::c_int as isize);
    (*p
        .offset(
            3 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            3 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            3 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh2 = (*p
        .offset(3 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh2 -= *mins.offset(0 as libc::c_int as isize);
    let ref mut fresh3 = (*p
        .offset(3 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh3 -= *mins.offset(1 as libc::c_int as isize);
    (*p
        .offset(
            4 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = *org.offset(0 as libc::c_int as isize)
        + *maxs.offset(0 as libc::c_int as isize);
    (*p
        .offset(
            4 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = *org.offset(1 as libc::c_int as isize)
        + *maxs.offset(1 as libc::c_int as isize);
    (*p
        .offset(
            4 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = *org.offset(2 as libc::c_int as isize)
        + *maxs.offset(2 as libc::c_int as isize);
    (*p
        .offset(
            5 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(4 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            5 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(4 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            5 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(4 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh4 = (*p
        .offset(5 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh4 -= *maxs.offset(0 as libc::c_int as isize);
    (*p
        .offset(
            6 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            6 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            6 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh5 = (*p
        .offset(6 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh5 -= *maxs.offset(1 as libc::c_int as isize);
    (*p
        .offset(
            7 as libc::c_int as isize,
        ))[0 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[0 as libc::c_int as usize];
    (*p
        .offset(
            7 as libc::c_int as isize,
        ))[1 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[1 as libc::c_int as usize];
    (*p
        .offset(
            7 as libc::c_int as isize,
        ))[2 as libc::c_int
        as usize] = (*p.offset(0 as libc::c_int as isize))[2 as libc::c_int as usize];
    let ref mut fresh6 = (*p
        .offset(7 as libc::c_int as isize))[0 as libc::c_int as usize];
    *fresh6 -= *maxs.offset(0 as libc::c_int as isize);
    let ref mut fresh7 = (*p
        .offset(7 as libc::c_int as isize))[1 as libc::c_int as usize];
    *fresh7 -= *maxs.offset(1 as libc::c_int as isize);
}
unsafe extern "C" fn loc_CanSee(
    mut targ: *mut edict_t,
    mut inflictor: *mut edict_t,
) -> qboolean {
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
    let mut targpoints: [vec3_t; 8] = [[0.; 3]; 8];
    let mut i: libc::c_int = 0;
    let mut viewpoint: vec3_t = [0.; 3];
    if (*targ).movetype == MOVETYPE_PUSH as libc::c_int {
        return false_0;
    }
    loc_buildboxpoints(
        targpoints.as_mut_ptr(),
        ((*targ).s.origin).as_mut_ptr(),
        ((*targ).mins).as_mut_ptr(),
        ((*targ).maxs).as_mut_ptr(),
    );
    viewpoint[0 as libc::c_int
        as usize] = (*inflictor).s.origin[0 as libc::c_int as usize];
    viewpoint[1 as libc::c_int
        as usize] = (*inflictor).s.origin[1 as libc::c_int as usize];
    viewpoint[2 as libc::c_int
        as usize] = (*inflictor).s.origin[2 as libc::c_int as usize];
    viewpoint[2 as libc::c_int as usize] += (*inflictor).viewheight as libc::c_float;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        trace = (gi.trace)
            .expect(
                "non-null function pointer",
            )(
            viewpoint.as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            vec3_origin.as_mut_ptr(),
            (targpoints[i as usize]).as_mut_ptr(),
            inflictor,
            1 as libc::c_int | 2 as libc::c_int,
        );
        if trace.fraction as libc::c_double == 1.0f64 {
            return true_0;
        }
        i += 1;
    }
    return false_0;
}
static mut flag1_item: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
static mut flag2_item: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
#[no_mangle]
pub unsafe extern "C" fn CTFSpawn() {
    if flag1_item.is_null() {
        flag1_item = FindItemByClassname(
            b"item_flag_team1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if flag2_item.is_null() {
        flag2_item = FindItemByClassname(
            b"item_flag_team2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    memset(
        &mut ctfgame as *mut ctfgame_t as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<ctfgame_t>() as libc::c_ulong,
    );
    CTFSetupTechSpawn();
    if (*competition).value > 1 as libc::c_int as libc::c_float {
        ctfgame.match_0 = MATCH_SETUP;
        ctfgame
            .matchtime = level.time
            + (*matchsetuptime).value * 60 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFInit() {
    ctf = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"ctf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    ctf_forcejoin = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"ctf_forcejoin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    competition = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"competition\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    matchlock = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"matchlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    electpercentage = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"electpercentage\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"66\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    matchtime = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"matchtime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        4 as libc::c_int,
    );
    matchsetuptime = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"matchsetuptime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"10\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    matchstarttime = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"matchstarttime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"20\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    admin_password = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"admin_password\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    warp_list = (gi.cvar)
        .expect(
            "non-null function pointer",
        )(
        b"warp_list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"q2ctf1 q2ctf2 q2ctf3 q2ctf4 q2ctf5\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFTeamName(mut team: libc::c_int) -> *mut libc::c_char {
    match team {
        1 => return b"RED\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"BLUE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {}
    }
    return b"UKNOWN\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn CTFOtherTeamName(mut team: libc::c_int) -> *mut libc::c_char {
    match team {
        1 => return b"BLUE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"RED\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => {}
    }
    return b"UKNOWN\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn CTFOtherTeam(mut team: libc::c_int) -> libc::c_int {
    match team {
        1 => return CTF_TEAM2 as libc::c_int,
        2 => return CTF_TEAM1 as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAssignSkin(mut ent: *mut edict_t, mut s: *mut libc::c_char) {
    let mut playernum: libc::c_int = (ent.offset_from(g_edicts) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: [libc::c_char; 64] = [0; 64];
    Com_sprintf(
        t.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        s,
    );
    p = strrchr(t.as_mut_ptr(), '/' as i32);
    if !p.is_null() {
        *p.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    } else {
        strcpy(t.as_mut_ptr(), b"male/\0" as *const u8 as *const libc::c_char);
    }
    match (*(*ent).client).resp.ctf_team {
        1 => {
            (gi.configstring)
                .expect(
                    "non-null function pointer",
                )(
                32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + playernum,
                va(
                    b"%s\\%s%s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*(*ent).client).pers.netname).as_mut_ptr(),
                    t.as_mut_ptr(),
                    b"ctf_r\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        2 => {
            (gi.configstring)
                .expect(
                    "non-null function pointer",
                )(
                32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + playernum,
                va(
                    b"%s\\%s%s\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*(*ent).client).pers.netname).as_mut_ptr(),
                    t.as_mut_ptr(),
                    b"ctf_b\0" as *const u8 as *const libc::c_char,
                ),
            );
        }
        _ => {
            (gi.configstring)
                .expect(
                    "non-null function pointer",
                )(
                32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
                    + playernum,
                va(
                    b"%s\\%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ((*(*ent).client).pers.netname).as_mut_ptr(),
                    s,
                ),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFAssignTeam(mut who: *mut gclient_t) {
    let mut player: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    let mut team1count: libc::c_int = 0 as libc::c_int;
    let mut team2count: libc::c_int = 0 as libc::c_int;
    (*who).resp.ctf_state = 0 as libc::c_int;
    if (*dmflags).value as libc::c_int & 131072 as libc::c_int == 0 {
        (*who).resp.ctf_team = CTF_NOTEAM as libc::c_int;
        return;
    }
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        player = &mut *g_edicts.offset(i as isize) as *mut edict_t;
        if !((*player).inuse as u64 == 0 || (*player).client == who) {
            match (*(*player).client).resp.ctf_team {
                1 => {
                    team1count += 1;
                }
                2 => {
                    team2count += 1;
                }
                _ => {}
            }
        }
        i += 1;
    }
    if team1count < team2count {
        (*who).resp.ctf_team = CTF_TEAM1 as libc::c_int;
    } else if team2count < team1count {
        (*who).resp.ctf_team = CTF_TEAM2 as libc::c_int;
    } else if rand() & 1 as libc::c_int != 0 {
        (*who).resp.ctf_team = CTF_TEAM1 as libc::c_int;
    } else {
        (*who).resp.ctf_team = CTF_TEAM2 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn SelectCTFSpawnPoint(mut ent: *mut edict_t) -> *mut edict_t {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut spot1: *mut edict_t = 0 as *mut edict_t;
    let mut spot2: *mut edict_t = 0 as *mut edict_t;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut selection: libc::c_int = 0;
    let mut range: libc::c_float = 0.;
    let mut range1: libc::c_float = 0.;
    let mut range2: libc::c_float = 0.;
    let mut cname: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*(*ent).client).resp.ctf_state != 0 {
        if (*dmflags).value as libc::c_int & 0x200 as libc::c_int != 0 {
            return SelectFarthestDeathmatchSpawnPoint()
        } else {
            return SelectRandomDeathmatchSpawnPoint()
        }
    }
    let ref mut fresh8 = (*(*ent).client).resp.ctf_state;
    *fresh8 += 1;
    match (*(*ent).client).resp.ctf_team {
        1 => {
            cname = b"info_player_team1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            cname = b"info_player_team2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => return SelectRandomDeathmatchSpawnPoint(),
    }
    spot = 0 as *mut edict_t;
    range2 = 99999 as libc::c_int as libc::c_float;
    range1 = range2;
    spot2 = 0 as *mut edict_t;
    spot1 = spot2;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            cname,
        );
        if spot.is_null() {
            break;
        }
        count += 1;
        range = PlayersRangeFromSpot(spot);
        if range < range1 {
            range1 = range;
            spot1 = spot;
        } else if range < range2 {
            range2 = range;
            spot2 = spot;
        }
    }
    if count == 0 {
        return SelectRandomDeathmatchSpawnPoint();
    }
    if count <= 2 as libc::c_int {
        spot2 = 0 as *mut edict_t;
        spot1 = spot2;
    } else {
        count -= 2 as libc::c_int;
    }
    selection = rand() % count;
    spot = 0 as *mut edict_t;
    loop {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            cname,
        );
        if spot == spot1 || spot == spot2 {
            selection += 1;
        }
        let fresh9 = selection;
        selection = selection - 1;
        if !(fresh9 != 0) {
            break;
        }
    }
    return spot;
}
#[no_mangle]
pub unsafe extern "C" fn CTFFragBonuses(
    mut targ: *mut edict_t,
    mut inflictor: *mut edict_t,
    mut attacker: *mut edict_t,
) {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut flag_item: *mut gitem_t = 0 as *mut gitem_t;
    let mut enemy_flag_item: *mut gitem_t = 0 as *mut gitem_t;
    let mut otherteam: libc::c_int = 0;
    let mut flag: *mut edict_t = 0 as *mut edict_t;
    let mut carrier: *mut edict_t = 0 as *mut edict_t;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v1: vec3_t = [0.; 3];
    let mut v2: vec3_t = [0.; 3];
    if !((*targ).client).is_null() && !((*attacker).client).is_null() {
        if !((*(*attacker).client).resp.ghost).is_null() {
            if attacker != targ {
                let ref mut fresh10 = (*(*(*attacker).client).resp.ghost).kills;
                *fresh10 += 1;
            }
        }
        if !((*(*targ).client).resp.ghost).is_null() {
            let ref mut fresh11 = (*(*(*targ).client).resp.ghost).deaths;
            *fresh11 += 1;
        }
    }
    if ((*targ).client).is_null() || ((*attacker).client).is_null() || targ == attacker {
        return;
    }
    otherteam = CTFOtherTeam((*(*targ).client).resp.ctf_team);
    if otherteam < 0 as libc::c_int {
        return;
    }
    if (*(*targ).client).resp.ctf_team == CTF_TEAM1 as libc::c_int {
        flag_item = flag1_item;
        enemy_flag_item = flag2_item;
    } else {
        flag_item = flag2_item;
        enemy_flag_item = flag1_item;
    }
    if (*(*targ).client)
        .pers
        .inventory[enemy_flag_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] != 0
    {
        (*(*attacker).client).resp.ctf_lastfraggedcarrier = level.time;
        (*(*attacker).client).resp.score += 2 as libc::c_int;
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            attacker,
            1 as libc::c_int,
            b"BONUS: %d points for fragging enemy flag carrier.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            2 as libc::c_int,
        );
        i = 1 as libc::c_int;
        while i as libc::c_float <= (*maxclients).value {
            ent = g_edicts.offset(i as isize);
            if (*ent).inuse as libc::c_uint != 0
                && (*(*ent).client).resp.ctf_team == otherteam
            {
                (*(*ent).client)
                    .resp
                    .ctf_lasthurtcarrier = 0 as libc::c_int as libc::c_float;
            }
            i += 1;
        }
        return;
    }
    if (*(*targ).client).resp.ctf_lasthurtcarrier != 0.
        && level.time - (*(*targ).client).resp.ctf_lasthurtcarrier
            < 8 as libc::c_int as libc::c_float
        && (*(*attacker).client)
            .pers
            .inventory[flag_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] == 0
    {
        (*(*attacker).client).resp.score += 2 as libc::c_int;
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            1 as libc::c_int,
            b"%s defends %s's flag carrier against an agressive enemy\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*(*attacker).client).pers.netname).as_mut_ptr(),
            CTFTeamName((*(*attacker).client).resp.ctf_team),
        );
        if !((*(*attacker).client).resp.ghost).is_null() {
            let ref mut fresh12 = (*(*(*attacker).client).resp.ghost).carrierdef;
            *fresh12 += 1;
        }
        return;
    }
    match (*(*attacker).client).resp.ctf_team {
        1 => {
            c = b"item_flag_team1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            c = b"item_flag_team2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => return,
    }
    flag = 0 as *mut edict_t;
    loop {
        flag = G_Find(
            flag,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            c,
        );
        if flag.is_null() {
            break;
        }
        if (*flag).spawnflags & 0x10000 as libc::c_int == 0 {
            break;
        }
    }
    if flag.is_null() {
        return;
    }
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        carrier = g_edicts.offset(i as isize);
        if (*carrier).inuse as libc::c_uint != 0
            && (*(*carrier).client)
                .pers
                .inventory[flag_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            break;
        }
        carrier = 0 as *mut edict_t;
        i += 1;
    }
    v1[0 as libc::c_int
        as usize] = (*targ).s.origin[0 as libc::c_int as usize]
        - (*flag).s.origin[0 as libc::c_int as usize];
    v1[1 as libc::c_int
        as usize] = (*targ).s.origin[1 as libc::c_int as usize]
        - (*flag).s.origin[1 as libc::c_int as usize];
    v1[2 as libc::c_int
        as usize] = (*targ).s.origin[2 as libc::c_int as usize]
        - (*flag).s.origin[2 as libc::c_int as usize];
    v2[0 as libc::c_int
        as usize] = (*attacker).s.origin[0 as libc::c_int as usize]
        - (*flag).s.origin[0 as libc::c_int as usize];
    v2[1 as libc::c_int
        as usize] = (*attacker).s.origin[1 as libc::c_int as usize]
        - (*flag).s.origin[1 as libc::c_int as usize];
    v2[2 as libc::c_int
        as usize] = (*attacker).s.origin[2 as libc::c_int as usize]
        - (*flag).s.origin[2 as libc::c_int as usize];
    if (VectorLength(v1.as_mut_ptr()) < 400 as libc::c_int as libc::c_float
        || VectorLength(v2.as_mut_ptr()) < 400 as libc::c_int as libc::c_float
        || loc_CanSee(flag, targ) as libc::c_uint != 0
        || loc_CanSee(flag, attacker) as libc::c_uint != 0)
        && (*(*attacker).client).resp.ctf_team != (*(*targ).client).resp.ctf_team
    {
        (*(*attacker).client).resp.score += 1 as libc::c_int;
        if (*flag).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"%s defends the %s base.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*attacker).client).pers.netname).as_mut_ptr(),
                CTFTeamName((*(*attacker).client).resp.ctf_team),
            );
        } else {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"%s defends the %s flag.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*attacker).client).pers.netname).as_mut_ptr(),
                CTFTeamName((*(*attacker).client).resp.ctf_team),
            );
        }
        if !((*(*attacker).client).resp.ghost).is_null() {
            let ref mut fresh13 = (*(*(*attacker).client).resp.ghost).basedef;
            *fresh13 += 1;
        }
        return;
    }
    if !carrier.is_null() && carrier != attacker {
        v1[0 as libc::c_int
            as usize] = (*targ).s.origin[0 as libc::c_int as usize]
            - (*carrier).s.origin[0 as libc::c_int as usize];
        v1[1 as libc::c_int
            as usize] = (*targ).s.origin[1 as libc::c_int as usize]
            - (*carrier).s.origin[1 as libc::c_int as usize];
        v1[2 as libc::c_int
            as usize] = (*targ).s.origin[2 as libc::c_int as usize]
            - (*carrier).s.origin[2 as libc::c_int as usize];
        v1[0 as libc::c_int
            as usize] = (*attacker).s.origin[0 as libc::c_int as usize]
            - (*carrier).s.origin[0 as libc::c_int as usize];
        v1[1 as libc::c_int
            as usize] = (*attacker).s.origin[1 as libc::c_int as usize]
            - (*carrier).s.origin[1 as libc::c_int as usize];
        v1[2 as libc::c_int
            as usize] = (*attacker).s.origin[2 as libc::c_int as usize]
            - (*carrier).s.origin[2 as libc::c_int as usize];
        if VectorLength(v1.as_mut_ptr()) < 400 as libc::c_int as libc::c_float
            || VectorLength(v2.as_mut_ptr()) < 400 as libc::c_int as libc::c_float
            || loc_CanSee(carrier, targ) as libc::c_uint != 0
            || loc_CanSee(carrier, attacker) as libc::c_uint != 0
        {
            (*(*attacker).client).resp.score += 1 as libc::c_int;
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                1 as libc::c_int,
                b"%s defends the %s's flag carrier.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ((*(*attacker).client).pers.netname).as_mut_ptr(),
                CTFTeamName((*(*attacker).client).resp.ctf_team),
            );
            if !((*(*attacker).client).resp.ghost).is_null() {
                let ref mut fresh14 = (*(*(*attacker).client).resp.ghost).carrierdef;
                *fresh14 += 1;
            }
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFCheckHurtCarrier(
    mut targ: *mut edict_t,
    mut attacker: *mut edict_t,
) {
    let mut flag_item: *mut gitem_t = 0 as *mut gitem_t;
    if ((*targ).client).is_null() || ((*attacker).client).is_null() {
        return;
    }
    if (*(*targ).client).resp.ctf_team == CTF_TEAM1 as libc::c_int {
        flag_item = flag2_item;
    } else {
        flag_item = flag1_item;
    }
    if (*(*targ).client)
        .pers
        .inventory[flag_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
        != 0 && (*(*targ).client).resp.ctf_team != (*(*attacker).client).resp.ctf_team
    {
        (*(*attacker).client).resp.ctf_lasthurtcarrier = level.time;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFResetFlag(mut ctf_team: libc::c_int) {
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    match ctf_team {
        1 => {
            c = b"item_flag_team1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            c = b"item_flag_team2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => return,
    }
    ent = 0 as *mut edict_t;
    loop {
        ent = G_Find(
            ent,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            c,
        );
        if ent.is_null() {
            break;
        }
        if (*ent).spawnflags & 0x10000 as libc::c_int != 0 {
            G_FreeEdict(ent);
        } else {
            (*ent).svflags &= !(0x1 as libc::c_int);
            (*ent).solid = SOLID_TRIGGER;
            (gi.linkentity).expect("non-null function pointer")(ent);
            (*ent).s.event = EV_ITEM_RESPAWN as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFResetFlags() {
    CTFResetFlag(CTF_TEAM1 as libc::c_int);
    CTFResetFlag(CTF_TEAM2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CTFPickup_Flag(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut ctf_team: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut player: *mut edict_t = 0 as *mut edict_t;
    let mut flag_item: *mut gitem_t = 0 as *mut gitem_t;
    let mut enemy_flag_item: *mut gitem_t = 0 as *mut gitem_t;
    if strcmp((*ent).classname, b"item_flag_team1\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        ctf_team = CTF_TEAM1 as libc::c_int;
    } else if strcmp(
        (*ent).classname,
        b"item_flag_team2\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        ctf_team = CTF_TEAM2 as libc::c_int;
    } else {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Don't know what team the flag is on.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return false_0;
    }
    if ctf_team == CTF_TEAM1 as libc::c_int {
        flag_item = flag1_item;
        enemy_flag_item = flag2_item;
    } else {
        flag_item = flag2_item;
        enemy_flag_item = flag1_item;
    }
    if ctf_team == (*(*other).client).resp.ctf_team {
        if (*ent).spawnflags & 0x10000 as libc::c_int == 0 {
            if (*(*other).client)
                .pers
                .inventory[enemy_flag_item.offset_from(itemlist.as_mut_ptr())
                as libc::c_long as usize] != 0
            {
                (gi.bprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    2 as libc::c_int,
                    b"%s captured the %s flag!\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*(*other).client).pers.netname).as_mut_ptr(),
                    CTFOtherTeamName(ctf_team),
                );
                (*(*other).client)
                    .pers
                    .inventory[enemy_flag_item.offset_from(itemlist.as_mut_ptr())
                    as libc::c_long as usize] = 0 as libc::c_int;
                ctfgame.last_flag_capture = level.time;
                ctfgame.last_capture_team = ctf_team;
                if ctf_team == CTF_TEAM1 as libc::c_int {
                    ctfgame.team1 += 1;
                } else {
                    ctfgame.team2 += 1;
                }
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    16 as libc::c_int + 8 as libc::c_int + 2 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"ctf/flagcap.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
                (*(*other).client).resp.score += 15 as libc::c_int;
                if !((*(*other).client).resp.ghost).is_null() {
                    let ref mut fresh15 = (*(*(*other).client).resp.ghost).caps;
                    *fresh15 += 1;
                }
                i = 1 as libc::c_int;
                while i as libc::c_float <= (*maxclients).value {
                    player = &mut *g_edicts.offset(i as isize) as *mut edict_t;
                    if !((*player).inuse as u64 == 0) {
                        if (*(*player).client).resp.ctf_team
                            != (*(*other).client).resp.ctf_team
                        {
                            (*(*player).client)
                                .resp
                                .ctf_lasthurtcarrier = -(5 as libc::c_int) as libc::c_float;
                        } else if (*(*player).client).resp.ctf_team
                            == (*(*other).client).resp.ctf_team
                        {
                            if player != other {
                                (*(*player).client).resp.score += 10 as libc::c_int;
                            }
                            if (*(*player).client).resp.ctf_lastreturnedflag
                                + 10 as libc::c_int as libc::c_float > level.time
                            {
                                (gi.bprintf)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    2 as libc::c_int,
                                    b"%s gets an assist for returning the flag!\n\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    ((*(*player).client).pers.netname).as_mut_ptr(),
                                );
                                (*(*player).client).resp.score += 1 as libc::c_int;
                            }
                            if (*(*player).client).resp.ctf_lastfraggedcarrier
                                + 10 as libc::c_int as libc::c_float > level.time
                            {
                                (gi.bprintf)
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    2 as libc::c_int,
                                    b"%s gets an assist for fragging the flag carrier!\n\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                    ((*(*player).client).pers.netname).as_mut_ptr(),
                                );
                                (*(*player).client).resp.score += 2 as libc::c_int;
                            }
                        }
                    }
                    i += 1;
                }
                CTFResetFlags();
                return false_0;
            }
            return false_0;
        }
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s returned the %s flag!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*other).client).pers.netname).as_mut_ptr(),
            CTFTeamName(ctf_team),
        );
        (*(*other).client).resp.score += 1 as libc::c_int;
        (*(*other).client).resp.ctf_lastreturnedflag = level.time;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            16 as libc::c_int + 8 as libc::c_int + 2 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"ctf/flagret.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        CTFResetFlag(ctf_team);
        return false_0;
    }
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s got the %s flag!\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*(*other).client).pers.netname).as_mut_ptr(),
        CTFTeamName(ctf_team),
    );
    (*(*other).client).resp.score += 0 as libc::c_int;
    (*(*other).client)
        .pers
        .inventory[flag_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] = 1 as libc::c_int;
    (*(*other).client).resp.ctf_flagsince = level.time;
    if (*ent).spawnflags & 0x10000 as libc::c_int == 0 {
        let ref mut fresh16 = (*ent).flags;
        *fresh16 = (*fresh16 as libc::c_uint | 0x80000000 as libc::c_uint)
            as libc::c_int;
        (*ent).svflags |= 0x1 as libc::c_int;
        (*ent).solid = SOLID_NOT;
    }
    return true_0;
}
unsafe extern "C" fn CTFDropFlagTouch(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    if other == (*ent).owner
        && (*ent).nextthink - level.time
            > (30 as libc::c_int - 2 as libc::c_int) as libc::c_float
    {
        return;
    }
    Touch_Item(ent, other, plane, surf);
}
unsafe extern "C" fn CTFDropFlagThink(mut ent: *mut edict_t) {
    if strcmp((*ent).classname, b"item_flag_team1\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        CTFResetFlag(CTF_TEAM1 as libc::c_int);
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"The %s flag has returned!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName(CTF_TEAM1 as libc::c_int),
        );
    } else if strcmp(
        (*ent).classname,
        b"item_flag_team2\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        CTFResetFlag(CTF_TEAM2 as libc::c_int);
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"The %s flag has returned!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName(CTF_TEAM2 as libc::c_int),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFDeadDropFlag(mut self_0: *mut edict_t) {
    let mut dropped: *mut edict_t = 0 as *mut edict_t;
    if (*(*self_0).client)
        .pers
        .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] != 0
    {
        dropped = Drop_Item(self_0, flag1_item);
        (*(*self_0).client)
            .pers
            .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = 0 as libc::c_int;
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s lost the %s flag!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*self_0).client).pers.netname).as_mut_ptr(),
            CTFTeamName(CTF_TEAM1 as libc::c_int),
        );
    } else if (*(*self_0).client)
        .pers
        .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] != 0
    {
        dropped = Drop_Item(self_0, flag2_item);
        (*(*self_0).client)
            .pers
            .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] = 0 as libc::c_int;
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s lost the %s flag!\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*self_0).client).pers.netname).as_mut_ptr(),
            CTFTeamName(CTF_TEAM2 as libc::c_int),
        );
    }
    if !dropped.is_null() {
        let ref mut fresh17 = (*dropped).think;
        *fresh17 = Some(CTFDropFlagThink as unsafe extern "C" fn(*mut edict_t) -> ());
        (*dropped).nextthink = level.time + 30 as libc::c_int as libc::c_float;
        let ref mut fresh18 = (*dropped).touch;
        *fresh18 = Some(
            CTFDropFlagTouch
                as unsafe extern "C" fn(
                    *mut edict_t,
                    *mut edict_t,
                    *mut cplane_t,
                    *mut csurface_t,
                ) -> (),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFDrop_Flag(
    mut ent: *mut edict_t,
    mut item: *mut gitem_t,
) -> qboolean {
    if rand() & 1 as libc::c_int != 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Only lusers drop flags.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Winners don't drop flags.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return false_0;
}
unsafe extern "C" fn CTFFlagThink(mut ent: *mut edict_t) {
    if (*ent).solid as libc::c_uint != SOLID_NOT as libc::c_int as libc::c_uint {
        (*ent)
            .s
            .frame = 173 as libc::c_int
            + ((*ent).s.frame - 173 as libc::c_int + 1 as libc::c_int)
                % 16 as libc::c_int;
    }
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CTFFlagSetup(mut ent: *mut edict_t) {
    let mut tr: trace_t = trace_t {
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
    let mut dest: vec3_t = [0.; 3];
    let mut v: *mut libc::c_float = 0 as *mut libc::c_float;
    v = tv(
        -(15 as libc::c_int) as libc::c_float,
        -(15 as libc::c_int) as libc::c_float,
        -(15 as libc::c_int) as libc::c_float,
    );
    (*ent).mins[0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
    (*ent).mins[1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
    (*ent).mins[2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
    v = tv(
        15 as libc::c_int as libc::c_float,
        15 as libc::c_int as libc::c_float,
        15 as libc::c_int as libc::c_float,
    );
    (*ent).maxs[0 as libc::c_int as usize] = *v.offset(0 as libc::c_int as isize);
    (*ent).maxs[1 as libc::c_int as usize] = *v.offset(1 as libc::c_int as isize);
    (*ent).maxs[2 as libc::c_int as usize] = *v.offset(2 as libc::c_int as isize);
    if !((*ent).model).is_null() {
        (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    } else {
        (gi.setmodel)
            .expect("non-null function pointer")(ent, (*(*ent).item).world_model);
    }
    (*ent).solid = SOLID_TRIGGER;
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh19 = (*ent).touch;
    *fresh19 = Some(
        Touch_Item
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    v = tv(
        0 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
        -(128 as libc::c_int) as libc::c_float,
    );
    dest[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + *v.offset(0 as libc::c_int as isize);
    dest[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + *v.offset(1 as libc::c_int as isize);
    dest[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + *v.offset(2 as libc::c_int as isize);
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        ((*ent).mins).as_mut_ptr(),
        ((*ent).maxs).as_mut_ptr(),
        dest.as_mut_ptr(),
        ent,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if tr.startsolid as u64 != 0 {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"CTFFlagSetup: %s startsolid at %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*ent).classname,
            vtos(((*ent).s.origin).as_mut_ptr()),
        );
        G_FreeEdict(ent);
        return;
    }
    (*ent).s.origin[0 as libc::c_int as usize] = tr.endpos[0 as libc::c_int as usize];
    (*ent).s.origin[1 as libc::c_int as usize] = tr.endpos[1 as libc::c_int as usize];
    (*ent).s.origin[2 as libc::c_int as usize] = tr.endpos[2 as libc::c_int as usize];
    (gi.linkentity).expect("non-null function pointer")(ent);
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
    let ref mut fresh20 = (*ent).think;
    *fresh20 = Some(CTFFlagThink as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn CTFEffects(mut player: *mut edict_t) {
    (*player).s.effects
        &= !(0x40000 as libc::c_int | 0x80000 as libc::c_int) as libc::c_uint;
    if (*player).health > 0 as libc::c_int {
        if (*(*player).client)
            .pers
            .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] != 0
        {
            (*player).s.effects |= 0x40000 as libc::c_int as libc::c_uint;
        }
        if (*(*player).client)
            .pers
            .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] != 0
        {
            (*player).s.effects |= 0x80000 as libc::c_int as libc::c_uint;
        }
    }
    if (*(*player).client)
        .pers
        .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] != 0
    {
        (*player)
            .s
            .modelindex3 = (gi.modelindex)
            .expect(
                "non-null function pointer",
            )(
            b"players/male/flag1.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if (*(*player).client)
        .pers
        .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] != 0
    {
        (*player)
            .s
            .modelindex3 = (gi.modelindex)
            .expect(
                "non-null function pointer",
            )(
            b"players/male/flag2.md2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else {
        (*player).s.modelindex3 = 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFCalcScores() {
    let mut i: libc::c_int = 0;
    ctfgame.total2 = 0 as libc::c_int;
    ctfgame.total1 = ctfgame.total2;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        if !((*g_edicts.offset((i + 1 as libc::c_int) as isize)).inuse as u64 == 0) {
            if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM1 as libc::c_int
            {
                ctfgame.total1 += (*(game.clients).offset(i as isize)).resp.score;
            } else if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM2 as libc::c_int
            {
                ctfgame.total2 += (*(game.clients).offset(i as isize)).resp.score;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFID_f(mut ent: *mut edict_t) {
    if (*(*ent).client).resp.id_state as u64 != 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Disabling player identication display.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        (*(*ent).client).resp.id_state = false_0;
    } else {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Activating player identication display.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        (*(*ent).client).resp.id_state = true_0;
    };
}
unsafe extern "C" fn CTFSetIDView(mut ent: *mut edict_t) {
    let mut forward: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut tr: trace_t = trace_t {
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
    let mut who: *mut edict_t = 0 as *mut edict_t;
    let mut best: *mut edict_t = 0 as *mut edict_t;
    let mut bd: libc::c_float = 0 as libc::c_int as libc::c_float;
    let mut d: libc::c_float = 0.;
    let mut i: libc::c_int = 0;
    (*(*ent).client)
        .ps
        .stats[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        1024 as libc::c_int as vec_t,
        forward.as_mut_ptr(),
    );
    forward[0 as libc::c_int
        as usize] = (*ent).s.origin[0 as libc::c_int as usize]
        + forward[0 as libc::c_int as usize];
    forward[1 as libc::c_int
        as usize] = (*ent).s.origin[1 as libc::c_int as usize]
        + forward[1 as libc::c_int as usize];
    forward[2 as libc::c_int
        as usize] = (*ent).s.origin[2 as libc::c_int as usize]
        + forward[2 as libc::c_int as usize];
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*ent).s.origin).as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        forward.as_mut_ptr(),
        ent,
        1 as libc::c_int | 2 as libc::c_int,
    );
    if tr.fraction < 1 as libc::c_int as libc::c_float && !(tr.ent).is_null()
        && !((*tr.ent).client).is_null()
    {
        (*(*ent).client)
            .ps
            .stats[27 as libc::c_int
            as usize] = ((32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int)
            as libc::c_long
            + (ent.offset_from(g_edicts) as libc::c_long
                - 1 as libc::c_int as libc::c_long)) as libc::c_short;
        return;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    best = 0 as *mut edict_t;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        who = g_edicts.offset(i as isize);
        if !((*who).inuse as u64 == 0
            || (*who).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint)
        {
            dir[0 as libc::c_int
                as usize] = (*who).s.origin[0 as libc::c_int as usize]
                - (*ent).s.origin[0 as libc::c_int as usize];
            dir[1 as libc::c_int
                as usize] = (*who).s.origin[1 as libc::c_int as usize]
                - (*ent).s.origin[1 as libc::c_int as usize];
            dir[2 as libc::c_int
                as usize] = (*who).s.origin[2 as libc::c_int as usize]
                - (*ent).s.origin[2 as libc::c_int as usize];
            VectorNormalize(dir.as_mut_ptr());
            d = forward[0 as libc::c_int as usize] * dir[0 as libc::c_int as usize]
                + forward[1 as libc::c_int as usize] * dir[1 as libc::c_int as usize]
                + forward[2 as libc::c_int as usize] * dir[2 as libc::c_int as usize];
            if d > bd && loc_CanSee(ent, who) as libc::c_uint != 0 {
                bd = d;
                best = who;
            }
        }
        i += 1;
    }
    if bd as libc::c_double > 0.90f64 {
        (*(*ent).client)
            .ps
            .stats[27 as libc::c_int
            as usize] = ((32 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int
            + 256 as libc::c_int + 256 as libc::c_int + 256 as libc::c_int)
            as libc::c_long
            + (best.offset_from(g_edicts) as libc::c_long
                - 1 as libc::c_int as libc::c_long)) as libc::c_short;
    }
}
#[no_mangle]
pub unsafe extern "C" fn SetCTFStats(mut ent: *mut edict_t) {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    let mut p1: libc::c_int = 0;
    let mut p2: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    if ctfgame.match_0 as libc::c_uint > MATCH_NONE as libc::c_int as libc::c_uint {
        (*(*ent).client)
            .ps
            .stats[28 as libc::c_int
            as usize] = (30 as libc::c_int - 1 as libc::c_int) as libc::c_short;
    } else {
        (*(*ent).client)
            .ps
            .stats[28 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    }
    if !((*(*ent).client).resp.ghost).is_null() {
        (*(*(*ent).client).resp.ghost).score = (*(*ent).client).resp.score;
        strcpy(
            ((*(*(*ent).client).resp.ghost).netname).as_mut_ptr(),
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
        (*(*(*ent).client).resp.ghost).number = (*ent).s.number;
    }
    (*(*ent).client)
        .ps
        .stats[24 as libc::c_int
        as usize] = (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"ctfsb1\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[25 as libc::c_int
        as usize] = (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"ctfsb2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        as libc::c_short;
    if level.intermissiontime != 0. && level.framenum & 8 as libc::c_int != 0 {
        if ctfgame.team1 > ctfgame.team2 {
            (*(*ent).client)
                .ps
                .stats[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        } else if ctfgame.team2 > ctfgame.team1 {
            (*(*ent).client)
                .ps
                .stats[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        } else if ctfgame.total1 > ctfgame.total2 {
            (*(*ent).client)
                .ps
                .stats[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        } else if ctfgame.total2 > ctfgame.total1 {
            (*(*ent).client)
                .ps
                .stats[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        } else {
            (*(*ent).client)
                .ps
                .stats[24 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
            (*(*ent).client)
                .ps
                .stats[25 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        }
    }
    i = 0 as libc::c_int;
    (*(*ent).client)
        .ps
        .stats[26 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && (*(*ent).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            (*(*ent).client)
                .ps
                .stats[26 as libc::c_int
                as usize] = (gi.imageindex)
                .expect("non-null function pointer")((*tech).icon) as libc::c_short;
            break;
        } else {
            i += 1;
        }
    }
    p1 = (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"i_ctf1\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    e = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char as libc::c_int,
        b"item_flag_team1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !e.is_null() {
        if (*e).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
            let mut i_0: libc::c_int = 0;
            p1 = (gi.imageindex)
                .expect(
                    "non-null function pointer",
                )(b"i_ctf1d\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            i_0 = 1 as libc::c_int;
            while i_0 as libc::c_float <= (*maxclients).value {
                if (*g_edicts.offset(i_0 as isize)).inuse as libc::c_uint != 0
                    && (*(*g_edicts.offset(i_0 as isize)).client)
                        .pers
                        .inventory[flag1_item.offset_from(itemlist.as_mut_ptr())
                        as libc::c_long as usize] != 0
                {
                    p1 = (gi.imageindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"i_ctf1t\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    break;
                } else {
                    i_0 += 1;
                }
            }
        } else if (*e).spawnflags & 0x10000 as libc::c_int != 0 {
            p1 = (gi.imageindex)
                .expect(
                    "non-null function pointer",
                )(b"i_ctf1d\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    p2 = (gi.imageindex)
        .expect(
            "non-null function pointer",
        )(b"i_ctf2\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    e = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char as libc::c_int,
        b"item_flag_team2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !e.is_null() {
        if (*e).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
            let mut i_1: libc::c_int = 0;
            p2 = (gi.imageindex)
                .expect(
                    "non-null function pointer",
                )(b"i_ctf2d\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
            i_1 = 1 as libc::c_int;
            while i_1 as libc::c_float <= (*maxclients).value {
                if (*g_edicts.offset(i_1 as isize)).inuse as libc::c_uint != 0
                    && (*(*g_edicts.offset(i_1 as isize)).client)
                        .pers
                        .inventory[flag2_item.offset_from(itemlist.as_mut_ptr())
                        as libc::c_long as usize] != 0
                {
                    p2 = (gi.imageindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"i_ctf2t\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    break;
                } else {
                    i_1 += 1;
                }
            }
        } else if (*e).spawnflags & 0x10000 as libc::c_int != 0 {
            p2 = (gi.imageindex)
                .expect(
                    "non-null function pointer",
                )(b"i_ctf2d\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        }
    }
    (*(*ent).client).ps.stats[17 as libc::c_int as usize] = p1 as libc::c_short;
    (*(*ent).client).ps.stats[19 as libc::c_int as usize] = p2 as libc::c_short;
    if ctfgame.last_flag_capture != 0.
        && level.time - ctfgame.last_flag_capture < 5 as libc::c_int as libc::c_float
    {
        if ctfgame.last_capture_team == CTF_TEAM1 as libc::c_int {
            if level.framenum & 8 as libc::c_int != 0 {
                (*(*ent).client)
                    .ps
                    .stats[17 as libc::c_int as usize] = p1 as libc::c_short;
            } else {
                (*(*ent).client)
                    .ps
                    .stats[17 as libc::c_int
                    as usize] = 0 as libc::c_int as libc::c_short;
            }
        } else if level.framenum & 8 as libc::c_int != 0 {
            (*(*ent).client).ps.stats[19 as libc::c_int as usize] = p2 as libc::c_short;
        } else {
            (*(*ent).client)
                .ps
                .stats[19 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        }
    }
    (*(*ent).client)
        .ps
        .stats[18 as libc::c_int as usize] = ctfgame.team1 as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[20 as libc::c_int as usize] = ctfgame.team2 as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[21 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*(*ent).client).resp.ctf_team == CTF_TEAM1 as libc::c_int
        && (*(*ent).client)
            .pers
            .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] != 0 && level.framenum & 8 as libc::c_int != 0
    {
        (*(*ent).client)
            .ps
            .stats[21 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_ctf2\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
    } else if (*(*ent).client).resp.ctf_team == CTF_TEAM2 as libc::c_int
        && (*(*ent).client)
            .pers
            .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
            as usize] != 0 && level.framenum & 8 as libc::c_int != 0
    {
        (*(*ent).client)
            .ps
            .stats[21 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_ctf1\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
    }
    (*(*ent).client)
        .ps
        .stats[22 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    (*(*ent).client)
        .ps
        .stats[23 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*(*ent).client).resp.ctf_team == CTF_TEAM1 as libc::c_int {
        (*(*ent).client)
            .ps
            .stats[22 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_ctfj\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
    } else if (*(*ent).client).resp.ctf_team == CTF_TEAM2 as libc::c_int {
        (*(*ent).client)
            .ps
            .stats[23 as libc::c_int
            as usize] = (gi.imageindex)
            .expect(
                "non-null function pointer",
            )(b"i_ctfj\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
            as libc::c_short;
    }
    (*(*ent).client)
        .ps
        .stats[27 as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
    if (*(*ent).client).resp.id_state as u64 != 0 {
        CTFSetIDView(ent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_team1(mut self_0: *mut edict_t) {}
#[no_mangle]
pub unsafe extern "C" fn SP_info_player_team2(mut self_0: *mut edict_t) {}
#[no_mangle]
pub unsafe extern "C" fn CTFPlayerResetGrapple(mut ent: *mut edict_t) {
    if !((*ent).client).is_null() && !((*(*ent).client).ctf_grapple).is_null() {
        CTFResetGrapple((*(*ent).client).ctf_grapple as *mut edict_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFResetGrapple(mut self_0: *mut edict_t) {
    if !((*(*(*self_0).owner).client).ctf_grapple).is_null() {
        let mut volume: libc::c_float = 1.0f64 as libc::c_float;
        let mut cl: *mut gclient_t = 0 as *mut gclient_t;
        if (*(*(*self_0).owner).client).silencer_shots != 0 {
            volume = 0.2f64 as libc::c_float;
        }
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            (*self_0).owner,
            16 as libc::c_int + 1 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"weapons/grapple/grreset.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            volume,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        cl = (*(*self_0).owner).client;
        let ref mut fresh21 = (*cl).ctf_grapple;
        *fresh21 = 0 as *mut libc::c_void;
        (*cl).ctf_grapplereleasetime = level.time;
        (*cl).ctf_grapplestate = CTF_GRAPPLE_STATE_FLY as libc::c_int;
        let ref mut fresh22 = (*cl).ps.pmove.pm_flags;
        *fresh22 = (*fresh22 as libc::c_int & !(64 as libc::c_int)) as byte;
        G_FreeEdict(self_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFGrappleTouch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    if other == (*self_0).owner {
        return;
    }
    if (*(*(*self_0).owner).client).ctf_grapplestate
        != CTF_GRAPPLE_STATE_FLY as libc::c_int
    {
        return;
    }
    if !surf.is_null() && (*surf).flags & 0x4 as libc::c_int != 0 {
        CTFResetGrapple(self_0);
        return;
    }
    (*self_0)
        .velocity[0 as libc::c_int as usize] = vec3_origin[0 as libc::c_int as usize];
    (*self_0)
        .velocity[1 as libc::c_int as usize] = vec3_origin[1 as libc::c_int as usize];
    (*self_0)
        .velocity[2 as libc::c_int as usize] = vec3_origin[2 as libc::c_int as usize];
    PlayerNoise((*self_0).owner, ((*self_0).s.origin).as_mut_ptr(), 2 as libc::c_int);
    if (*other).takedamage != 0 {
        T_Damage(
            other,
            self_0,
            (*self_0).owner,
            ((*self_0).velocity).as_mut_ptr(),
            ((*self_0).s.origin).as_mut_ptr(),
            ((*plane).normal).as_mut_ptr(),
            (*self_0).dmg,
            1 as libc::c_int,
            0 as libc::c_int,
            34 as libc::c_int,
        );
        CTFResetGrapple(self_0);
        return;
    }
    (*(*(*self_0).owner).client)
        .ctf_grapplestate = CTF_GRAPPLE_STATE_PULL as libc::c_int;
    let ref mut fresh23 = (*self_0).enemy;
    *fresh23 = other;
    (*self_0).solid = SOLID_NOT;
    if (*(*(*self_0).owner).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        (*self_0).owner,
        16 as libc::c_int + 1 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/grapple/grpull.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        volume,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        self_0,
        1 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/grapple/grhit.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        volume,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_SPARKS as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*self_0).s.origin).as_mut_ptr());
    if plane.is_null() {
        (gi.WriteDir).expect("non-null function pointer")(vec3_origin.as_mut_ptr());
    } else {
        (gi.WriteDir)
            .expect("non-null function pointer")(((*plane).normal).as_mut_ptr());
    }
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn CTFGrappleDrawCable(mut self_0: *mut edict_t) {
    let mut offset: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut end: vec3_t = [0.; 3];
    let mut f: vec3_t = [0.; 3];
    let mut r: vec3_t = [0.; 3];
    let mut dir: vec3_t = [0.; 3];
    let mut distance: libc::c_float = 0.;
    AngleVectors(
        ((*(*(*self_0).owner).client).v_angle).as_mut_ptr(),
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        0 as *mut vec_t,
    );
    offset[0 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 16 as libc::c_int as vec_t;
    offset[2 as libc::c_int
        as usize] = ((*(*self_0).owner).viewheight - 8 as libc::c_int) as vec_t;
    P_ProjectSource(
        (*(*self_0).owner).client,
        ((*(*self_0).owner).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        f.as_mut_ptr(),
        r.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    offset[0 as libc::c_int
        as usize] = start[0 as libc::c_int as usize]
        - (*(*self_0).owner).s.origin[0 as libc::c_int as usize];
    offset[1 as libc::c_int
        as usize] = start[1 as libc::c_int as usize]
        - (*(*self_0).owner).s.origin[1 as libc::c_int as usize];
    offset[2 as libc::c_int
        as usize] = start[2 as libc::c_int as usize]
        - (*(*self_0).owner).s.origin[2 as libc::c_int as usize];
    dir[0 as libc::c_int
        as usize] = start[0 as libc::c_int as usize]
        - (*self_0).s.origin[0 as libc::c_int as usize];
    dir[1 as libc::c_int
        as usize] = start[1 as libc::c_int as usize]
        - (*self_0).s.origin[1 as libc::c_int as usize];
    dir[2 as libc::c_int
        as usize] = start[2 as libc::c_int as usize]
        - (*self_0).s.origin[2 as libc::c_int as usize];
    distance = VectorLength(dir.as_mut_ptr());
    if distance < 64 as libc::c_int as libc::c_float {
        return;
    }
    end[0 as libc::c_int as usize] = (*self_0).s.origin[0 as libc::c_int as usize];
    end[1 as libc::c_int as usize] = (*self_0).s.origin[1 as libc::c_int as usize];
    end[2 as libc::c_int as usize] = (*self_0).s.origin[2 as libc::c_int as usize];
    (gi.WriteByte).expect("non-null function pointer")(3 as libc::c_int);
    (gi.WriteByte).expect("non-null function pointer")(TE_GRAPPLE_CABLE as libc::c_int);
    (gi.WriteShort)
        .expect(
            "non-null function pointer",
        )(((*self_0).owner).offset_from(g_edicts) as libc::c_long as libc::c_int);
    (gi.WritePosition)
        .expect("non-null function pointer")(((*(*self_0).owner).s.origin).as_mut_ptr());
    (gi.WritePosition).expect("non-null function pointer")(end.as_mut_ptr());
    (gi.WritePosition).expect("non-null function pointer")(offset.as_mut_ptr());
    (gi.multicast)
        .expect(
            "non-null function pointer",
        )(((*self_0).s.origin).as_mut_ptr(), MULTICAST_PVS);
}
#[no_mangle]
pub unsafe extern "C" fn CTFGrapplePull(mut self_0: *mut edict_t) {
    let mut hookdir: vec3_t = [0.; 3];
    let mut v: vec3_t = [0.; 3];
    let mut vlen: libc::c_float = 0.;
    if strcmp(
        (*(*(*(*self_0).owner).client).pers.weapon).classname,
        b"weapon_grapple\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int && ((*(*(*self_0).owner).client).newweapon).is_null()
        && (*(*(*self_0).owner).client).weaponstate as libc::c_uint
            != WEAPON_FIRING as libc::c_int as libc::c_uint
        && (*(*(*self_0).owner).client).weaponstate as libc::c_uint
            != WEAPON_ACTIVATING as libc::c_int as libc::c_uint
    {
        CTFResetGrapple(self_0);
        return;
    }
    if !((*self_0).enemy).is_null() {
        if (*(*self_0).enemy).solid as libc::c_uint
            == SOLID_NOT as libc::c_int as libc::c_uint
        {
            CTFResetGrapple(self_0);
            return;
        }
        if (*(*self_0).enemy).solid as libc::c_uint
            == SOLID_BBOX as libc::c_int as libc::c_uint
        {
            VectorScale(
                ((*(*self_0).enemy).size).as_mut_ptr(),
                0.5f64 as vec_t,
                v.as_mut_ptr(),
            );
            v[0 as libc::c_int
                as usize] = v[0 as libc::c_int as usize]
                + (*(*self_0).enemy).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = v[1 as libc::c_int as usize]
                + (*(*self_0).enemy).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = v[2 as libc::c_int as usize]
                + (*(*self_0).enemy).s.origin[2 as libc::c_int as usize];
            (*self_0)
                .s
                .origin[0 as libc::c_int
                as usize] = v[0 as libc::c_int as usize]
                + (*(*self_0).enemy).mins[0 as libc::c_int as usize];
            (*self_0)
                .s
                .origin[1 as libc::c_int
                as usize] = v[1 as libc::c_int as usize]
                + (*(*self_0).enemy).mins[1 as libc::c_int as usize];
            (*self_0)
                .s
                .origin[2 as libc::c_int
                as usize] = v[2 as libc::c_int as usize]
                + (*(*self_0).enemy).mins[2 as libc::c_int as usize];
            (gi.linkentity).expect("non-null function pointer")(self_0);
        } else {
            (*self_0)
                .velocity[0 as libc::c_int
                as usize] = (*(*self_0).enemy).velocity[0 as libc::c_int as usize];
            (*self_0)
                .velocity[1 as libc::c_int
                as usize] = (*(*self_0).enemy).velocity[1 as libc::c_int as usize];
            (*self_0)
                .velocity[2 as libc::c_int
                as usize] = (*(*self_0).enemy).velocity[2 as libc::c_int as usize];
        }
        if (*(*self_0).enemy).takedamage != 0
            && CheckTeamDamage((*self_0).enemy, (*self_0).owner) as u64 == 0
        {
            let mut volume: libc::c_float = 1.0f64 as libc::c_float;
            if (*(*(*self_0).owner).client).silencer_shots != 0 {
                volume = 0.2f64 as libc::c_float;
            }
            T_Damage(
                (*self_0).enemy,
                self_0,
                (*self_0).owner,
                ((*self_0).velocity).as_mut_ptr(),
                ((*self_0).s.origin).as_mut_ptr(),
                vec3_origin.as_mut_ptr(),
                1 as libc::c_int,
                1 as libc::c_int,
                0 as libc::c_int,
                34 as libc::c_int,
            );
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                self_0,
                1 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"weapons/grapple/grhurt.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
        if (*(*self_0).enemy).deadflag != 0 {
            CTFResetGrapple(self_0);
            return;
        }
    }
    CTFGrappleDrawCable(self_0);
    if (*(*(*self_0).owner).client).ctf_grapplestate
        > CTF_GRAPPLE_STATE_FLY as libc::c_int
    {
        let mut forward: vec3_t = [0.; 3];
        let mut up: vec3_t = [0.; 3];
        AngleVectors(
            ((*(*(*self_0).owner).client).v_angle).as_mut_ptr(),
            forward.as_mut_ptr(),
            0 as *mut vec_t,
            up.as_mut_ptr(),
        );
        v[0 as libc::c_int
            as usize] = (*(*self_0).owner).s.origin[0 as libc::c_int as usize];
        v[1 as libc::c_int
            as usize] = (*(*self_0).owner).s.origin[1 as libc::c_int as usize];
        v[2 as libc::c_int
            as usize] = (*(*self_0).owner).s.origin[2 as libc::c_int as usize];
        v[2 as libc::c_int as usize] += (*(*self_0).owner).viewheight as libc::c_float;
        hookdir[0 as libc::c_int
            as usize] = (*self_0).s.origin[0 as libc::c_int as usize]
            - v[0 as libc::c_int as usize];
        hookdir[1 as libc::c_int
            as usize] = (*self_0).s.origin[1 as libc::c_int as usize]
            - v[1 as libc::c_int as usize];
        hookdir[2 as libc::c_int
            as usize] = (*self_0).s.origin[2 as libc::c_int as usize]
            - v[2 as libc::c_int as usize];
        vlen = VectorLength(hookdir.as_mut_ptr());
        if (*(*(*self_0).owner).client).ctf_grapplestate
            == CTF_GRAPPLE_STATE_PULL as libc::c_int
            && vlen < 64 as libc::c_int as libc::c_float
        {
            let mut volume_0: libc::c_float = 1.0f64 as libc::c_float;
            if (*(*(*self_0).owner).client).silencer_shots != 0 {
                volume_0 = 0.2f64 as libc::c_float;
            }
            let ref mut fresh24 = (*(*(*self_0).owner).client).ps.pmove.pm_flags;
            *fresh24 = (*fresh24 as libc::c_int | 64 as libc::c_int) as byte;
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                (*self_0).owner,
                16 as libc::c_int + 1 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"weapons/grapple/grhang.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume_0,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
            (*(*(*self_0).owner).client)
                .ctf_grapplestate = CTF_GRAPPLE_STATE_HANG as libc::c_int;
        }
        VectorNormalize(hookdir.as_mut_ptr());
        VectorScale(
            hookdir.as_mut_ptr(),
            650 as libc::c_int as vec_t,
            hookdir.as_mut_ptr(),
        );
        (*(*self_0).owner)
            .velocity[0 as libc::c_int as usize] = hookdir[0 as libc::c_int as usize];
        (*(*self_0).owner)
            .velocity[1 as libc::c_int as usize] = hookdir[1 as libc::c_int as usize];
        (*(*self_0).owner)
            .velocity[2 as libc::c_int as usize] = hookdir[2 as libc::c_int as usize];
        SV_AddGravity((*self_0).owner);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFFireGrapple(
    mut self_0: *mut edict_t,
    mut start: *mut vec_t,
    mut dir: *mut vec_t,
    mut damage: libc::c_int,
    mut speed: libc::c_int,
    mut effect: libc::c_int,
) {
    let mut grapple: *mut edict_t = 0 as *mut edict_t;
    let mut tr: trace_t = trace_t {
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
    VectorNormalize(dir);
    grapple = G_Spawn();
    (*grapple)
        .s
        .origin[0 as libc::c_int as usize] = *start.offset(0 as libc::c_int as isize);
    (*grapple)
        .s
        .origin[1 as libc::c_int as usize] = *start.offset(1 as libc::c_int as isize);
    (*grapple)
        .s
        .origin[2 as libc::c_int as usize] = *start.offset(2 as libc::c_int as isize);
    (*grapple)
        .s
        .old_origin[0 as libc::c_int
        as usize] = *start.offset(0 as libc::c_int as isize);
    (*grapple)
        .s
        .old_origin[1 as libc::c_int
        as usize] = *start.offset(1 as libc::c_int as isize);
    (*grapple)
        .s
        .old_origin[2 as libc::c_int
        as usize] = *start.offset(2 as libc::c_int as isize);
    vectoangles(dir, ((*grapple).s.angles).as_mut_ptr());
    VectorScale(dir, speed as vec_t, ((*grapple).velocity).as_mut_ptr());
    (*grapple).movetype = MOVETYPE_FLYMISSILE as libc::c_int;
    (*grapple)
        .clipmask = 1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
        | 0x4000000 as libc::c_int;
    (*grapple).solid = SOLID_BBOX;
    (*grapple).s.effects |= effect as libc::c_uint;
    let ref mut fresh25 = (*grapple).mins[2 as libc::c_int as usize];
    *fresh25 = 0 as libc::c_int as vec_t;
    let ref mut fresh26 = (*grapple).mins[1 as libc::c_int as usize];
    *fresh26 = *fresh25;
    (*grapple).mins[0 as libc::c_int as usize] = *fresh26;
    let ref mut fresh27 = (*grapple).maxs[2 as libc::c_int as usize];
    *fresh27 = 0 as libc::c_int as vec_t;
    let ref mut fresh28 = (*grapple).maxs[1 as libc::c_int as usize];
    *fresh28 = *fresh27;
    (*grapple).maxs[0 as libc::c_int as usize] = *fresh28;
    (*grapple)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/weapons/grapple/hook/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    let ref mut fresh29 = (*grapple).owner;
    *fresh29 = self_0;
    let ref mut fresh30 = (*grapple).touch;
    *fresh30 = Some(
        CTFGrappleTouch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (*grapple).dmg = damage;
    let ref mut fresh31 = (*(*self_0).client).ctf_grapple;
    *fresh31 = grapple as *mut libc::c_void;
    (*(*self_0).client).ctf_grapplestate = CTF_GRAPPLE_STATE_FLY as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(grapple);
    tr = (gi.trace)
        .expect(
            "non-null function pointer",
        )(
        ((*self_0).s.origin).as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
        ((*grapple).s.origin).as_mut_ptr(),
        grapple,
        1 as libc::c_int | 0x2000000 as libc::c_int | 2 as libc::c_int
            | 0x4000000 as libc::c_int,
    );
    if (tr.fraction as libc::c_double) < 1.0f64 {
        VectorMA(
            ((*grapple).s.origin).as_mut_ptr(),
            -(10 as libc::c_int) as libc::c_float,
            dir,
            ((*grapple).s.origin).as_mut_ptr(),
        );
        ((*grapple).touch)
            .expect(
                "non-null function pointer",
            )(grapple, tr.ent, 0 as *mut cplane_t, 0 as *mut csurface_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFGrappleFire(
    mut ent: *mut edict_t,
    mut g_offset: *mut vec_t,
    mut damage: libc::c_int,
    mut effect: libc::c_int,
) {
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut start: vec3_t = [0.; 3];
    let mut offset: vec3_t = [0.; 3];
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    if (*(*ent).client).ctf_grapplestate > CTF_GRAPPLE_STATE_FLY as libc::c_int {
        return;
    }
    AngleVectors(
        ((*(*ent).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    offset[0 as libc::c_int as usize] = 24 as libc::c_int as vec_t;
    offset[1 as libc::c_int as usize] = 8 as libc::c_int as vec_t;
    offset[2 as libc::c_int
        as usize] = ((*ent).viewheight - 8 as libc::c_int + 2 as libc::c_int) as vec_t;
    offset[0 as libc::c_int
        as usize] = offset[0 as libc::c_int as usize]
        + *g_offset.offset(0 as libc::c_int as isize);
    offset[1 as libc::c_int
        as usize] = offset[1 as libc::c_int as usize]
        + *g_offset.offset(1 as libc::c_int as isize);
    offset[2 as libc::c_int
        as usize] = offset[2 as libc::c_int as usize]
        + *g_offset.offset(2 as libc::c_int as isize);
    P_ProjectSource(
        (*ent).client,
        ((*ent).s.origin).as_mut_ptr(),
        offset.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        start.as_mut_ptr(),
    );
    VectorScale(
        forward.as_mut_ptr(),
        -(2 as libc::c_int) as vec_t,
        ((*(*ent).client).kick_origin).as_mut_ptr(),
    );
    (*(*ent).client)
        .kick_angles[0 as libc::c_int as usize] = -(1 as libc::c_int) as vec_t;
    if (*(*ent).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    (gi.sound)
        .expect(
            "non-null function pointer",
        )(
        ent,
        16 as libc::c_int + 1 as libc::c_int,
        (gi.soundindex)
            .expect(
                "non-null function pointer",
            )(
            b"weapons/grapple/grfire.wav\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ),
        volume,
        1 as libc::c_int as libc::c_float,
        0 as libc::c_int as libc::c_float,
    );
    CTFFireGrapple(
        ent,
        start.as_mut_ptr(),
        forward.as_mut_ptr(),
        damage,
        650 as libc::c_int,
        effect,
    );
    PlayerNoise(ent, start.as_mut_ptr(), 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CTFWeapon_Grapple_Fire(mut ent: *mut edict_t) {
    let mut damage: libc::c_int = 0;
    damage = 10 as libc::c_int;
    CTFGrappleFire(ent, vec3_origin.as_mut_ptr(), damage, 0 as libc::c_int);
    let ref mut fresh32 = (*(*ent).client).ps.gunframe;
    *fresh32 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn CTFWeapon_Grapple(mut ent: *mut edict_t) {
    static mut pause_frames: [libc::c_int; 4] = [
        10 as libc::c_int,
        18 as libc::c_int,
        27 as libc::c_int,
        0 as libc::c_int,
    ];
    static mut fire_frames: [libc::c_int; 2] = [6 as libc::c_int, 0 as libc::c_int];
    let mut prevstate: libc::c_int = 0;
    if (*(*ent).client).buttons & 1 as libc::c_int != 0
        && (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_FIRING as libc::c_int as libc::c_uint
        && !((*(*ent).client).ctf_grapple).is_null()
    {
        (*(*ent).client).ps.gunframe = 9 as libc::c_int;
    }
    if (*(*ent).client).buttons & 1 as libc::c_int == 0
        && !((*(*ent).client).ctf_grapple).is_null()
    {
        CTFResetGrapple((*(*ent).client).ctf_grapple as *mut edict_t);
        if (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_FIRING as libc::c_int as libc::c_uint
        {
            (*(*ent).client).weaponstate = WEAPON_READY;
        }
    }
    if !((*(*ent).client).newweapon).is_null()
        && (*(*ent).client).ctf_grapplestate > CTF_GRAPPLE_STATE_FLY as libc::c_int
        && (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_FIRING as libc::c_int as libc::c_uint
    {
        (*(*ent).client).weaponstate = WEAPON_DROPPING;
        (*(*ent).client).ps.gunframe = 32 as libc::c_int;
    }
    prevstate = (*(*ent).client).weaponstate as libc::c_int;
    Weapon_Generic(
        ent,
        5 as libc::c_int,
        9 as libc::c_int,
        31 as libc::c_int,
        36 as libc::c_int,
        pause_frames.as_mut_ptr(),
        fire_frames.as_mut_ptr(),
        Some(CTFWeapon_Grapple_Fire as unsafe extern "C" fn(*mut edict_t) -> ()),
    );
    if prevstate == WEAPON_ACTIVATING as libc::c_int
        && (*(*ent).client).weaponstate as libc::c_uint
            == WEAPON_READY as libc::c_int as libc::c_uint
        && (*(*ent).client).ctf_grapplestate > CTF_GRAPPLE_STATE_FLY as libc::c_int
    {
        if (*(*ent).client).buttons & 1 as libc::c_int == 0 {
            (*(*ent).client).ps.gunframe = 9 as libc::c_int;
        } else {
            (*(*ent).client).ps.gunframe = 5 as libc::c_int;
        }
        (*(*ent).client).weaponstate = WEAPON_FIRING;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFTeam_f(mut ent: *mut edict_t) {
    let mut t: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut desired_team: libc::c_int = 0;
    t = (gi.args).expect("non-null function pointer")();
    if *t == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You are on the %s team.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName((*(*ent).client).resp.ctf_team),
        );
        return;
    }
    if ctfgame.match_0 as libc::c_uint > MATCH_SETUP as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Can't change teams in a match.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if Q_stricmp(t, b"red\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        == 0 as libc::c_int
    {
        desired_team = CTF_TEAM1 as libc::c_int;
    } else if Q_stricmp(
        t,
        b"blue\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == 0 as libc::c_int
    {
        desired_team = CTF_TEAM2 as libc::c_int;
    } else {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Unknown team %s.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            t,
        );
        return;
    }
    if (*(*ent).client).resp.ctf_team == desired_team {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You are already on the %s team.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            CTFTeamName((*(*ent).client).resp.ctf_team),
        );
        return;
    }
    (*ent).svflags = 0 as libc::c_int;
    (*ent).flags &= !(0x10 as libc::c_int);
    (*(*ent).client).resp.ctf_team = desired_team;
    (*(*ent).client).resp.ctf_state = 0 as libc::c_int;
    s = Info_ValueForKey(
        ((*(*ent).client).pers.userinfo).as_mut_ptr(),
        b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CTFAssignSkin(ent, s);
    if (*ent).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint {
        PutClientInServer(ent);
        (*ent).s.event = EV_PLAYER_TELEPORT as libc::c_int;
        (*(*ent).client).ps.pmove.pm_flags = 32 as libc::c_int as byte;
        (*(*ent).client).ps.pmove.pm_time = 14 as libc::c_int as byte;
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s joined the %s team.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            CTFTeamName(desired_team),
        );
        return;
    }
    (*ent).health = 0 as libc::c_int;
    player_die(ent, ent, ent, 100000 as libc::c_int, vec3_origin.as_mut_ptr());
    (*ent).deadflag = 2 as libc::c_int;
    respawn(ent);
    (*(*ent).client).resp.score = 0 as libc::c_int;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s changed to the %s team.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
        CTFTeamName(desired_team),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFScoreboardMessage(
    mut ent: *mut edict_t,
    mut killer: *mut edict_t,
) {
    let mut entry: [libc::c_char; 1024] = [0; 1024];
    let mut string: [libc::c_char; 1400] = [0; 1400];
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut sorted: [[libc::c_int; 256]; 2] = [[0; 256]; 2];
    let mut sortedscores: [[libc::c_int; 256]; 2] = [[0; 256]; 2];
    let mut score: libc::c_int = 0;
    let mut total: [libc::c_int; 2] = [0; 2];
    let mut totalscore: [libc::c_int; 2] = [0; 2];
    let mut last: [libc::c_int; 2] = [0; 2];
    let mut cl: *mut gclient_t = 0 as *mut gclient_t;
    let mut cl_ent: *mut edict_t = 0 as *mut edict_t;
    let mut team: libc::c_int = 0;
    let mut maxsize: libc::c_int = 1000 as libc::c_int;
    total[1 as libc::c_int as usize] = 0 as libc::c_int;
    total[0 as libc::c_int as usize] = total[1 as libc::c_int as usize];
    last[1 as libc::c_int as usize] = 0 as libc::c_int;
    last[0 as libc::c_int as usize] = last[1 as libc::c_int as usize];
    totalscore[1 as libc::c_int as usize] = 0 as libc::c_int;
    totalscore[0 as libc::c_int as usize] = totalscore[1 as libc::c_int as usize];
    let mut current_block_18: u64;
    i = 0 as libc::c_int;
    while i < game.maxclients {
        cl_ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*cl_ent).inuse as u64 == 0) {
            if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM1 as libc::c_int
            {
                team = 0 as libc::c_int;
                current_block_18 = 15976848397966268834;
            } else if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM2 as libc::c_int
            {
                team = 1 as libc::c_int;
                current_block_18 = 15976848397966268834;
            } else {
                current_block_18 = 13109137661213826276;
            }
            match current_block_18 {
                13109137661213826276 => {}
                _ => {
                    score = (*(game.clients).offset(i as isize)).resp.score;
                    j = 0 as libc::c_int;
                    while j < total[team as usize] {
                        if score > sortedscores[team as usize][j as usize] {
                            break;
                        }
                        j += 1;
                    }
                    k = total[team as usize];
                    while k > j {
                        sorted[team
                            as usize][k
                            as usize] = sorted[team
                            as usize][(k - 1 as libc::c_int) as usize];
                        sortedscores[team
                            as usize][k
                            as usize] = sortedscores[team
                            as usize][(k - 1 as libc::c_int) as usize];
                        k -= 1;
                    }
                    sorted[team as usize][j as usize] = i;
                    sortedscores[team as usize][j as usize] = score;
                    totalscore[team as usize] += score;
                    total[team as usize] += 1;
                }
            }
        }
        i += 1;
    }
    *string.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    len = 0 as libc::c_int;
    sprintf(
        string.as_mut_ptr(),
        b"if 24 xv 8 yv 8 pic 24 endif xv 40 yv 28 string \"%4d/%-3d\" xv 98 yv 12 num 2 18 if 25 xv 168 yv 8 pic 25 endif xv 200 yv 28 string \"%4d/%-3d\" xv 256 yv 12 num 2 20 \0"
            as *const u8 as *const libc::c_char,
        totalscore[0 as libc::c_int as usize],
        total[0 as libc::c_int as usize],
        totalscore[1 as libc::c_int as usize],
        total[1 as libc::c_int as usize],
    );
    len = strlen(string.as_mut_ptr()) as libc::c_int;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if i >= total[0 as libc::c_int as usize] && i >= total[1 as libc::c_int as usize]
        {
            break;
        }
        *entry.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
        if i < total[0 as libc::c_int as usize] {
            cl = &mut *(game.clients)
                .offset(
                    *(*sorted.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(i as isize) as isize,
                ) as *mut gclient_t;
            cl_ent = g_edicts
                .offset(1 as libc::c_int as isize)
                .offset(sorted[0 as libc::c_int as usize][i as usize] as isize);
            sprintf(
                entry.as_mut_ptr().offset(strlen(entry.as_mut_ptr()) as isize),
                b"ctf 0 %d %d %d %d \0" as *const u8 as *const libc::c_char,
                42 as libc::c_int + i * 8 as libc::c_int,
                sorted[0 as libc::c_int as usize][i as usize],
                (*cl).resp.score,
                if (*cl).ping > 999 as libc::c_int {
                    999 as libc::c_int
                } else {
                    (*cl).ping
                },
            );
            if (*(*cl_ent).client)
                .pers
                .inventory[flag2_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
            {
                sprintf(
                    entry.as_mut_ptr().offset(strlen(entry.as_mut_ptr()) as isize),
                    b"xv 56 yv %d picn sbfctf2 \0" as *const u8 as *const libc::c_char,
                    42 as libc::c_int + i * 8 as libc::c_int,
                );
            }
            if (maxsize - len) as libc::c_ulong > strlen(entry.as_mut_ptr()) {
                strcat(string.as_mut_ptr(), entry.as_mut_ptr());
                len = strlen(string.as_mut_ptr()) as libc::c_int;
                last[0 as libc::c_int as usize] = i;
            }
        }
        if i < total[1 as libc::c_int as usize] {
            cl = &mut *(game.clients)
                .offset(
                    *(*sorted.as_mut_ptr().offset(1 as libc::c_int as isize))
                        .as_mut_ptr()
                        .offset(i as isize) as isize,
                ) as *mut gclient_t;
            cl_ent = g_edicts
                .offset(1 as libc::c_int as isize)
                .offset(sorted[1 as libc::c_int as usize][i as usize] as isize);
            sprintf(
                entry.as_mut_ptr().offset(strlen(entry.as_mut_ptr()) as isize),
                b"ctf 160 %d %d %d %d \0" as *const u8 as *const libc::c_char,
                42 as libc::c_int + i * 8 as libc::c_int,
                sorted[1 as libc::c_int as usize][i as usize],
                (*cl).resp.score,
                if (*cl).ping > 999 as libc::c_int {
                    999 as libc::c_int
                } else {
                    (*cl).ping
                },
            );
            if (*(*cl_ent).client)
                .pers
                .inventory[flag1_item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
            {
                sprintf(
                    entry.as_mut_ptr().offset(strlen(entry.as_mut_ptr()) as isize),
                    b"xv 216 yv %d picn sbfctf1 \0" as *const u8 as *const libc::c_char,
                    42 as libc::c_int + i * 8 as libc::c_int,
                );
            }
            if (maxsize - len) as libc::c_ulong > strlen(entry.as_mut_ptr()) {
                strcat(string.as_mut_ptr(), entry.as_mut_ptr());
                len = strlen(string.as_mut_ptr()) as libc::c_int;
                last[1 as libc::c_int as usize] = i;
            }
        }
        i += 1;
    }
    if last[0 as libc::c_int as usize] > last[1 as libc::c_int as usize] {
        j = last[0 as libc::c_int as usize];
    } else {
        j = last[1 as libc::c_int as usize];
    }
    j = (j + 2 as libc::c_int) * 8 as libc::c_int + 42 as libc::c_int;
    n = 0 as libc::c_int;
    k = n;
    if maxsize - len > 50 as libc::c_int {
        i = 0 as libc::c_int;
        while (i as libc::c_float) < (*maxclients).value {
            cl_ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
            cl = &mut *(game.clients).offset(i as isize) as *mut gclient_t;
            if !((*cl_ent).inuse as u64 == 0
                || (*cl_ent).solid as libc::c_uint
                    != SOLID_NOT as libc::c_int as libc::c_uint
                || (*(*cl_ent).client).resp.ctf_team != CTF_NOTEAM as libc::c_int)
            {
                if k == 0 {
                    k = 1 as libc::c_int;
                    sprintf(
                        entry.as_mut_ptr(),
                        b"xv 0 yv %d string2 \"Spectators\" \0" as *const u8
                            as *const libc::c_char,
                        j,
                    );
                    strcat(string.as_mut_ptr(), entry.as_mut_ptr());
                    len = strlen(string.as_mut_ptr()) as libc::c_int;
                    j += 8 as libc::c_int;
                }
                sprintf(
                    entry.as_mut_ptr().offset(strlen(entry.as_mut_ptr()) as isize),
                    b"ctf %d %d %d %d %d \0" as *const u8 as *const libc::c_char,
                    if n & 1 as libc::c_int != 0 {
                        160 as libc::c_int
                    } else {
                        0 as libc::c_int
                    },
                    j,
                    i,
                    (*cl).resp.score,
                    if (*cl).ping > 999 as libc::c_int {
                        999 as libc::c_int
                    } else {
                        (*cl).ping
                    },
                );
                if (maxsize - len) as libc::c_ulong > strlen(entry.as_mut_ptr()) {
                    strcat(string.as_mut_ptr(), entry.as_mut_ptr());
                    len = strlen(string.as_mut_ptr()) as libc::c_int;
                }
                if n & 1 as libc::c_int != 0 {
                    j += 8 as libc::c_int;
                }
                n += 1;
            }
            i += 1;
        }
    }
    if total[0 as libc::c_int as usize] - last[0 as libc::c_int as usize]
        > 1 as libc::c_int
    {
        sprintf(
            string.as_mut_ptr().offset(strlen(string.as_mut_ptr()) as isize),
            b"xv 8 yv %d string \"..and %d more\" \0" as *const u8
                as *const libc::c_char,
            42 as libc::c_int
                + (last[0 as libc::c_int as usize] + 1 as libc::c_int)
                    * 8 as libc::c_int,
            total[0 as libc::c_int as usize] - last[0 as libc::c_int as usize]
                - 1 as libc::c_int,
        );
    }
    if total[1 as libc::c_int as usize] - last[1 as libc::c_int as usize]
        > 1 as libc::c_int
    {
        sprintf(
            string.as_mut_ptr().offset(strlen(string.as_mut_ptr()) as isize),
            b"xv 168 yv %d string \"..and %d more\" \0" as *const u8
                as *const libc::c_char,
            42 as libc::c_int
                + (last[1 as libc::c_int as usize] + 1 as libc::c_int)
                    * 8 as libc::c_int,
            total[1 as libc::c_int as usize] - last[1 as libc::c_int as usize]
                - 1 as libc::c_int,
        );
    }
    (gi.WriteByte).expect("non-null function pointer")(4 as libc::c_int);
    (gi.WriteString).expect("non-null function pointer")(string.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CTFHasTech(mut who: *mut edict_t) {
    if level.time - (*(*who).client).ctf_lasttechmsg > 2 as libc::c_int as libc::c_float
    {
        (gi.centerprintf)
            .expect(
                "non-null function pointer",
            )(
            who,
            b"You already have a TECH powerup.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (*(*who).client).ctf_lasttechmsg = level.time;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFWhat_Tech(mut ent: *mut edict_t) -> *mut gitem_t {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && (*(*ent).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            return tech;
        }
        i += 1;
    }
    return 0 as *mut gitem_t;
}
#[no_mangle]
pub unsafe extern "C" fn CTFPickup_Tech(
    mut ent: *mut edict_t,
    mut other: *mut edict_t,
) -> qboolean {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && (*(*other).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            CTFHasTech(other);
            return false_0;
        }
        i += 1;
    }
    let ref mut fresh33 = (*(*other).client)
        .pers
        .inventory[((*ent).item).offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize];
    *fresh33 += 1;
    (*(*other).client).ctf_regentime = level.time;
    return true_0;
}
unsafe extern "C" fn FindTechSpawn() -> *mut edict_t {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = rand() % 16 as libc::c_int;
    loop {
        let fresh34 = i;
        i = i - 1;
        if !(fresh34 != 0) {
            break;
        }
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if spot.is_null() {
        spot = G_Find(
            spot,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"info_player_deathmatch\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    return spot;
}
unsafe extern "C" fn TechThink(mut tech: *mut edict_t) {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    spot = FindTechSpawn();
    if !spot.is_null() {
        SpawnTech((*tech).item, spot);
        G_FreeEdict(tech);
    } else {
        (*tech).nextthink = level.time + 60 as libc::c_int as libc::c_float;
        let ref mut fresh35 = (*tech).think;
        *fresh35 = Some(TechThink as unsafe extern "C" fn(*mut edict_t) -> ());
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFDrop_Tech(mut ent: *mut edict_t, mut item: *mut gitem_t) {
    let mut tech: *mut edict_t = 0 as *mut edict_t;
    tech = Drop_Item(ent, item);
    (*tech).nextthink = level.time + 60 as libc::c_int as libc::c_float;
    let ref mut fresh36 = (*tech).think;
    *fresh36 = Some(TechThink as unsafe extern "C" fn(*mut edict_t) -> ());
    (*(*ent).client)
        .pers
        .inventory[item.offset_from(itemlist.as_mut_ptr()) as libc::c_long
        as usize] = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn CTFDeadDropTech(mut ent: *mut edict_t) {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut dropped: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && (*(*ent).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            dropped = Drop_Item(ent, tech);
            (*dropped)
                .velocity[0 as libc::c_int
                as usize] = (rand() % 600 as libc::c_int - 300 as libc::c_int) as vec_t;
            (*dropped)
                .velocity[1 as libc::c_int
                as usize] = (rand() % 600 as libc::c_int - 300 as libc::c_int) as vec_t;
            (*dropped).nextthink = level.time + 60 as libc::c_int as libc::c_float;
            let ref mut fresh37 = (*dropped).think;
            *fresh37 = Some(TechThink as unsafe extern "C" fn(*mut edict_t) -> ());
            let ref mut fresh38 = (*dropped).owner;
            *fresh38 = 0 as *mut edict_t;
            (*(*ent).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] = 0 as libc::c_int;
        }
        i += 1;
    }
}
unsafe extern "C" fn SpawnTech(mut item: *mut gitem_t, mut spot: *mut edict_t) {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut forward: vec3_t = [0.; 3];
    let mut right: vec3_t = [0.; 3];
    let mut angles: vec3_t = [0.; 3];
    ent = G_Spawn();
    let ref mut fresh39 = (*ent).classname;
    *fresh39 = (*item).classname;
    let ref mut fresh40 = (*ent).item;
    *fresh40 = item;
    (*ent).spawnflags = 0x10000 as libc::c_int;
    (*ent).s.effects = (*item).world_model_flags as libc::c_uint;
    (*ent).s.renderfx = 512 as libc::c_int;
    (*ent).mins[0 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*ent).mins[1 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*ent).mins[2 as libc::c_int as usize] = -(15 as libc::c_int) as vec_t;
    (*ent).maxs[0 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (*ent).maxs[1 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (*ent).maxs[2 as libc::c_int as usize] = 15 as libc::c_int as vec_t;
    (gi.setmodel).expect("non-null function pointer")(ent, (*(*ent).item).world_model);
    (*ent).solid = SOLID_TRIGGER;
    (*ent).movetype = MOVETYPE_TOSS as libc::c_int;
    let ref mut fresh41 = (*ent).touch;
    *fresh41 = Some(
        Touch_Item
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    let ref mut fresh42 = (*ent).owner;
    *fresh42 = ent;
    angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    angles[1 as libc::c_int as usize] = (rand() % 360 as libc::c_int) as vec_t;
    angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    AngleVectors(
        angles.as_mut_ptr(),
        forward.as_mut_ptr(),
        right.as_mut_ptr(),
        0 as *mut vec_t,
    );
    (*ent)
        .s
        .origin[0 as libc::c_int as usize] = (*spot).s.origin[0 as libc::c_int as usize];
    (*ent)
        .s
        .origin[1 as libc::c_int as usize] = (*spot).s.origin[1 as libc::c_int as usize];
    (*ent)
        .s
        .origin[2 as libc::c_int as usize] = (*spot).s.origin[2 as libc::c_int as usize];
    let ref mut fresh43 = (*ent).s.origin[2 as libc::c_int as usize];
    *fresh43 += 16 as libc::c_int as libc::c_float;
    VectorScale(
        forward.as_mut_ptr(),
        100 as libc::c_int as vec_t,
        ((*ent).velocity).as_mut_ptr(),
    );
    (*ent).velocity[2 as libc::c_int as usize] = 300 as libc::c_int as vec_t;
    (*ent).nextthink = level.time + 60 as libc::c_int as libc::c_float;
    let ref mut fresh44 = (*ent).think;
    *fresh44 = Some(TechThink as unsafe extern "C" fn(*mut edict_t) -> ());
    (gi.linkentity).expect("non-null function pointer")(ent);
}
unsafe extern "C" fn SpawnTechs(mut ent: *mut edict_t) {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && {
                spot = FindTechSpawn();
                !spot.is_null()
            }
        {
            SpawnTech(tech, spot);
        }
        i += 1;
    }
    if !ent.is_null() {
        G_FreeEdict(ent);
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFRespawnTech(mut ent: *mut edict_t) {
    let mut spot: *mut edict_t = 0 as *mut edict_t;
    spot = FindTechSpawn();
    if !spot.is_null() {
        SpawnTech((*ent).item, spot);
    }
    G_FreeEdict(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFSetupTechSpawn() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    if (*dmflags).value as libc::c_int & 524288 as libc::c_int != 0 {
        return;
    }
    ent = G_Spawn();
    (*ent).nextthink = level.time + 2 as libc::c_int as libc::c_float;
    let ref mut fresh45 = (*ent).think;
    *fresh45 = Some(SpawnTechs as unsafe extern "C" fn(*mut edict_t) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn CTFResetTech() {
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    ent = g_edicts.offset(1 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < globals.num_edicts {
        if (*ent).inuse as u64 != 0 {
            if !((*ent).item).is_null() && (*(*ent).item).flags & 64 as libc::c_int != 0
            {
                G_FreeEdict(ent);
            }
        }
        i += 1;
        ent = ent.offset(1);
    }
    SpawnTechs(0 as *mut edict_t);
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyResistance(
    mut ent: *mut edict_t,
    mut dmg: libc::c_int,
) -> libc::c_int {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    if !((*ent).client).is_null() && (*(*ent).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if dmg != 0 && !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"ctf/tech1.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            volume,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
        return dmg / 2 as libc::c_int;
    }
    return dmg;
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyStrength(
    mut ent: *mut edict_t,
    mut dmg: libc::c_int,
) -> libc::c_int {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if dmg != 0 && !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        return dmg * 2 as libc::c_int;
    }
    return dmg;
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyStrengthSound(mut ent: *mut edict_t) -> qboolean {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    if !((*ent).client).is_null() && (*(*ent).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        if (*(*ent).client).ctf_techsndtime < level.time {
            (*(*ent).client)
                .ctf_techsndtime = level.time + 1 as libc::c_int as libc::c_float;
            if (*(*ent).client).quad_framenum > level.framenum as libc::c_float {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    2 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"ctf/tech2x.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    volume,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            } else {
                (gi.sound)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    2 as libc::c_int,
                    (gi.soundindex)
                        .expect(
                            "non-null function pointer",
                        )(
                        b"ctf/tech2.wav\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    ),
                    volume,
                    1 as libc::c_int as libc::c_float,
                    0 as libc::c_int as libc::c_float,
                );
            }
        }
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyHaste(mut ent: *mut edict_t) -> qboolean {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyHasteSound(mut ent: *mut edict_t) {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    if !((*ent).client).is_null() && (*(*ent).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0 && (*(*ent).client).ctf_techsndtime < level.time
    {
        (*(*ent).client)
            .ctf_techsndtime = level.time + 1 as libc::c_int as libc::c_float;
        (gi.sound)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            (gi.soundindex)
                .expect(
                    "non-null function pointer",
                )(
                b"ctf/tech3.wav\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            volume,
            1 as libc::c_int as libc::c_float,
            0 as libc::c_int as libc::c_float,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFApplyRegeneration(mut ent: *mut edict_t) {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    let mut noise: qboolean = false_0;
    let mut client: *mut gclient_t = 0 as *mut gclient_t;
    let mut index: libc::c_int = 0;
    let mut volume: libc::c_float = 1.0f64 as libc::c_float;
    client = (*ent).client;
    if client.is_null() {
        return;
    }
    if (*(*ent).client).silencer_shots != 0 {
        volume = 0.2f64 as libc::c_float;
    }
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !tech.is_null()
        && (*client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        if (*client).ctf_regentime < level.time {
            (*client).ctf_regentime = level.time;
            if (*ent).health < 150 as libc::c_int {
                (*ent).health += 5 as libc::c_int;
                if (*ent).health > 150 as libc::c_int {
                    (*ent).health = 150 as libc::c_int;
                }
                let ref mut fresh46 = (*client).ctf_regentime;
                *fresh46 = (*fresh46 as libc::c_double + 0.5f64) as libc::c_float;
                noise = true_0;
            }
            index = ArmorIndex(ent);
            if index != 0
                && (*client).pers.inventory[index as usize] < 150 as libc::c_int
            {
                (*client).pers.inventory[index as usize] += 5 as libc::c_int;
                if (*client).pers.inventory[index as usize] > 150 as libc::c_int {
                    (*client).pers.inventory[index as usize] = 150 as libc::c_int;
                }
                let ref mut fresh47 = (*client).ctf_regentime;
                *fresh47 = (*fresh47 as libc::c_double + 0.5f64) as libc::c_float;
                noise = true_0;
            }
        }
        if noise as libc::c_uint != 0 && (*(*ent).client).ctf_techsndtime < level.time {
            (*(*ent).client)
                .ctf_techsndtime = level.time + 1 as libc::c_int as libc::c_float;
            (gi.sound)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                (gi.soundindex)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"ctf/tech4.wav\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                volume,
                1 as libc::c_int as libc::c_float,
                0 as libc::c_int as libc::c_float,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFHasRegeneration(mut ent: *mut edict_t) -> qboolean {
    static mut tech: *mut gitem_t = 0 as *const gitem_t as *mut gitem_t;
    if tech.is_null() {
        tech = FindItemByClassname(
            b"item_tech4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !tech.is_null() && !((*ent).client).is_null()
        && (*(*ent).client)
            .pers
            .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize]
            != 0
    {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub static mut loc_names: [C2RustUnnamed_5; 25] = [
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_flag_team1\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_flag_team2\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_quad\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_invulnerability\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_bfg\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_railgun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_rocketlauncher\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_hyperblaster\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_chaingun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_grenadelauncher\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_machinegun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_supershotgun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"weapon_shotgun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_power_screen\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_power_shield\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 5 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_armor_body\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_armor_combat\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_armor_jacket\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 6 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_silencer\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_breather\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_enviro\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_adrenaline\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 7 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_bandolier\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: b"item_pack\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            priority: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = C2RustUnnamed_5 {
            classname: 0 as *const libc::c_char as *mut libc::c_char,
            priority: 0 as libc::c_int,
        };
        init
    },
];
unsafe extern "C" fn CTFSay_Team_Location(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    let mut what: *mut edict_t = 0 as *mut edict_t;
    let mut hot: *mut edict_t = 0 as *mut edict_t;
    let mut hotdist: libc::c_float = 999999 as libc::c_int as libc::c_float;
    let mut newdist: libc::c_float = 0.;
    let mut v: vec3_t = [0.; 3];
    let mut hotindex: libc::c_int = 999 as libc::c_int;
    let mut i: libc::c_int = 0;
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut nearteam: libc::c_int = -(1 as libc::c_int);
    let mut flag1: *mut edict_t = 0 as *mut edict_t;
    let mut flag2: *mut edict_t = 0 as *mut edict_t;
    let mut hotsee: qboolean = false_0;
    let mut cansee: qboolean = false_0;
    loop {
        what = loc_findradius(
            what,
            ((*who).s.origin).as_mut_ptr(),
            1024 as libc::c_int as libc::c_float,
        );
        if what.is_null() {
            break;
        }
        i = 0 as libc::c_int;
        while !(loc_names[i as usize].classname).is_null() {
            if strcmp((*what).classname, loc_names[i as usize].classname)
                == 0 as libc::c_int
            {
                break;
            }
            i += 1;
        }
        if (loc_names[i as usize].classname).is_null() {
            continue;
        }
        cansee = loc_CanSee(what, who);
        if cansee as libc::c_uint != 0 && hotsee as u64 == 0 {
            hotsee = true_0;
            hotindex = loc_names[i as usize].priority;
            hot = what;
            v[0 as libc::c_int
                as usize] = (*what).s.origin[0 as libc::c_int as usize]
                - (*who).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*what).s.origin[1 as libc::c_int as usize]
                - (*who).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*what).s.origin[2 as libc::c_int as usize]
                - (*who).s.origin[2 as libc::c_int as usize];
            hotdist = VectorLength(v.as_mut_ptr());
        } else {
            if hotsee as libc::c_uint != 0 && cansee as u64 == 0 {
                continue;
            }
            if hotsee as libc::c_uint != 0 && hotindex < loc_names[i as usize].priority {
                continue;
            }
            v[0 as libc::c_int
                as usize] = (*what).s.origin[0 as libc::c_int as usize]
                - (*who).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*what).s.origin[1 as libc::c_int as usize]
                - (*who).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*what).s.origin[2 as libc::c_int as usize]
                - (*who).s.origin[2 as libc::c_int as usize];
            newdist = VectorLength(v.as_mut_ptr());
            if newdist < hotdist
                || cansee as libc::c_uint != 0
                    && loc_names[i as usize].priority < hotindex
            {
                hot = what;
                hotdist = newdist;
                hotindex = i;
                hotsee = loc_CanSee(hot, who);
            }
        }
    }
    if hot.is_null() {
        strcpy(buf, b"nowhere\0" as *const u8 as *const libc::c_char);
        return;
    }
    what = 0 as *mut edict_t;
    loop {
        what = G_Find(
            what,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            (*hot).classname,
        );
        if what.is_null() {
            break;
        }
        if what == hot {
            continue;
        }
        flag1 = G_Find(
            0 as *mut edict_t,
            &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                as libc::c_int,
            b"item_flag_team1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !flag1.is_null()
            && {
                flag2 = G_Find(
                    0 as *mut edict_t,
                    &mut (*(0 as *mut edict_t)).classname as *mut *mut libc::c_char
                        as libc::c_int,
                    b"item_flag_team2\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                !flag2.is_null()
            }
        {
            v[0 as libc::c_int
                as usize] = (*hot).s.origin[0 as libc::c_int as usize]
                - (*flag1).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*hot).s.origin[1 as libc::c_int as usize]
                - (*flag1).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*hot).s.origin[2 as libc::c_int as usize]
                - (*flag1).s.origin[2 as libc::c_int as usize];
            hotdist = VectorLength(v.as_mut_ptr());
            v[0 as libc::c_int
                as usize] = (*hot).s.origin[0 as libc::c_int as usize]
                - (*flag2).s.origin[0 as libc::c_int as usize];
            v[1 as libc::c_int
                as usize] = (*hot).s.origin[1 as libc::c_int as usize]
                - (*flag2).s.origin[1 as libc::c_int as usize];
            v[2 as libc::c_int
                as usize] = (*hot).s.origin[2 as libc::c_int as usize]
                - (*flag2).s.origin[2 as libc::c_int as usize];
            newdist = VectorLength(v.as_mut_ptr());
            if hotdist < newdist {
                nearteam = CTF_TEAM1 as libc::c_int;
            } else if hotdist > newdist {
                nearteam = CTF_TEAM2 as libc::c_int;
            }
        }
        break;
    }
    item = FindItemByClassname((*hot).classname);
    if item.is_null() {
        strcpy(buf, b"nowhere\0" as *const u8 as *const libc::c_char);
        return;
    }
    if (*who).waterlevel != 0 {
        strcpy(buf, b"in the water \0" as *const u8 as *const libc::c_char);
    } else {
        *buf = 0 as libc::c_int as libc::c_char;
    }
    v[0 as libc::c_int
        as usize] = (*who).s.origin[0 as libc::c_int as usize]
        - (*hot).s.origin[0 as libc::c_int as usize];
    v[1 as libc::c_int
        as usize] = (*who).s.origin[1 as libc::c_int as usize]
        - (*hot).s.origin[1 as libc::c_int as usize];
    v[2 as libc::c_int
        as usize] = (*who).s.origin[2 as libc::c_int as usize]
        - (*hot).s.origin[2 as libc::c_int as usize];
    if fabs(v[2 as libc::c_int as usize] as libc::c_double)
        > fabs(v[0 as libc::c_int as usize] as libc::c_double)
        && fabs(v[2 as libc::c_int as usize] as libc::c_double)
            > fabs(v[1 as libc::c_int as usize] as libc::c_double)
    {
        if v[2 as libc::c_int as usize] > 0 as libc::c_int as libc::c_float {
            strcat(buf, b"above \0" as *const u8 as *const libc::c_char);
        } else {
            strcat(buf, b"below \0" as *const u8 as *const libc::c_char);
        }
    } else {
        strcat(buf, b"near \0" as *const u8 as *const libc::c_char);
    }
    if nearteam == CTF_TEAM1 as libc::c_int {
        strcat(buf, b"the red \0" as *const u8 as *const libc::c_char);
    } else if nearteam == CTF_TEAM2 as libc::c_int {
        strcat(buf, b"the blue \0" as *const u8 as *const libc::c_char);
    } else {
        strcat(buf, b"the \0" as *const u8 as *const libc::c_char);
    }
    strcat(buf, (*item).pickup_name);
}
unsafe extern "C" fn CTFSay_Team_Armor(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    let mut item: *mut gitem_t = 0 as *mut gitem_t;
    let mut index: libc::c_int = 0;
    let mut cells: libc::c_int = 0;
    let mut power_armor_type: libc::c_int = 0;
    *buf = 0 as libc::c_int as libc::c_char;
    power_armor_type = PowerArmorType(who);
    if power_armor_type != 0 {
        cells = (*(*who).client)
            .pers
            .inventory[(FindItem(
            b"cells\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ))
            .offset_from(itemlist.as_mut_ptr()) as libc::c_long as usize];
        if cells != 0 {
            sprintf(
                buf.offset(strlen(buf) as isize),
                b"%s with %i cells \0" as *const u8 as *const libc::c_char,
                if power_armor_type == 1 as libc::c_int {
                    b"Power Screen\0" as *const u8 as *const libc::c_char
                } else {
                    b"Power Shield\0" as *const u8 as *const libc::c_char
                },
                cells,
            );
        }
    }
    index = ArmorIndex(who);
    if index != 0 {
        item = GetItemByIndex(index);
        if !item.is_null() {
            if *buf != 0 {
                strcat(buf, b"and \0" as *const u8 as *const libc::c_char);
            }
            sprintf(
                buf.offset(strlen(buf) as isize),
                b"%i units of %s\0" as *const u8 as *const libc::c_char,
                (*(*who).client).pers.inventory[index as usize],
                (*item).pickup_name,
            );
        }
    }
    if *buf == 0 {
        strcpy(buf, b"no armor\0" as *const u8 as *const libc::c_char);
    }
}
unsafe extern "C" fn CTFSay_Team_Health(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    if (*who).health <= 0 as libc::c_int {
        strcpy(buf, b"dead\0" as *const u8 as *const libc::c_char);
    } else {
        sprintf(buf, b"%i health\0" as *const u8 as *const libc::c_char, (*who).health);
    };
}
unsafe extern "C" fn CTFSay_Team_Tech(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    let mut tech: *mut gitem_t = 0 as *mut gitem_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while !(tnames[i as usize]).is_null() {
        tech = FindItemByClassname(tnames[i as usize]);
        if !tech.is_null()
            && (*(*who).client)
                .pers
                .inventory[tech.offset_from(itemlist.as_mut_ptr()) as libc::c_long
                as usize] != 0
        {
            sprintf(
                buf,
                b"the %s\0" as *const u8 as *const libc::c_char,
                (*tech).pickup_name,
            );
            return;
        }
        i += 1;
    }
    strcpy(buf, b"no powerup\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn CTFSay_Team_Weapon(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    if !((*(*who).client).pers.weapon).is_null() {
        strcpy(buf, (*(*(*who).client).pers.weapon).pickup_name);
    } else {
        strcpy(buf, b"none\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn CTFSay_Team_Sight(
    mut who: *mut edict_t,
    mut buf: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    let mut targ: *mut edict_t = 0 as *mut edict_t;
    let mut n: libc::c_int = 0 as libc::c_int;
    let mut s: [libc::c_char; 1024] = [0; 1024];
    let mut s2: [libc::c_char; 1024] = [0; 1024];
    let ref mut fresh48 = *s2.as_mut_ptr();
    *fresh48 = 0 as libc::c_int as libc::c_char;
    *s.as_mut_ptr() = *fresh48;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        targ = g_edicts.offset(i as isize);
        if !((*targ).inuse as u64 == 0 || targ == who
            || loc_CanSee(targ, who) as u64 == 0)
        {
            if *s2.as_mut_ptr() != 0 {
                if (strlen(s.as_mut_ptr()))
                    .wrapping_add(strlen(s2.as_mut_ptr()))
                    .wrapping_add(3 as libc::c_int as libc::c_ulong)
                    < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                {
                    if n != 0 {
                        strcat(
                            s.as_mut_ptr(),
                            b", \0" as *const u8 as *const libc::c_char,
                        );
                    }
                    strcat(s.as_mut_ptr(), s2.as_mut_ptr());
                    *s2.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
                }
                n += 1;
            }
            strcpy(s2.as_mut_ptr(), ((*(*targ).client).pers.netname).as_mut_ptr());
        }
        i += 1;
    }
    if *s2.as_mut_ptr() != 0 {
        if (strlen(s.as_mut_ptr()))
            .wrapping_add(strlen(s2.as_mut_ptr()))
            .wrapping_add(6 as libc::c_int as libc::c_ulong)
            < ::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
        {
            if n != 0 {
                strcat(s.as_mut_ptr(), b" and \0" as *const u8 as *const libc::c_char);
            }
            strcat(s.as_mut_ptr(), s2.as_mut_ptr());
        }
        strcpy(buf, s.as_mut_ptr());
    } else {
        strcpy(buf, b"no one\0" as *const u8 as *const libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFSay_Team(mut who: *mut edict_t, mut msg: *mut libc::c_char) {
    let mut outmsg: [libc::c_char; 1024] = [0; 1024];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut i: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cl_ent: *mut edict_t = 0 as *mut edict_t;
    if CheckFlood(who) as u64 != 0 {
        return;
    }
    outmsg[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if *msg as libc::c_int == '"' as i32 {
        *msg
            .offset(
                (strlen(msg)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = 0 as libc::c_int as libc::c_char;
        msg = msg.offset(1);
    }
    p = outmsg.as_mut_ptr();
    while *msg as libc::c_int != 0
        && (p.offset_from(outmsg.as_mut_ptr()) as libc::c_long as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        if *msg as libc::c_int == '%' as i32 {
            msg = msg.offset(1);
            match *msg as libc::c_int {
                108 | 76 => {
                    CTFSay_Team_Location(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                97 | 65 => {
                    CTFSay_Team_Armor(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                104 | 72 => {
                    CTFSay_Team_Health(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                116 | 84 => {
                    CTFSay_Team_Tech(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                119 | 87 => {
                    CTFSay_Team_Weapon(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                110 | 78 => {
                    CTFSay_Team_Sight(who, buf.as_mut_ptr());
                    strcpy(p, buf.as_mut_ptr());
                    p = p.offset(strlen(buf.as_mut_ptr()) as isize);
                }
                _ => {
                    let fresh49 = p;
                    p = p.offset(1);
                    *fresh49 = *msg;
                }
            }
        } else {
            let fresh50 = p;
            p = p.offset(1);
            *fresh50 = *msg;
        }
        msg = msg.offset(1);
    }
    *p = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        cl_ent = g_edicts.offset(1 as libc::c_int as isize).offset(i as isize);
        if !((*cl_ent).inuse as u64 == 0) {
            if (*(*cl_ent).client).resp.ctf_team == (*(*who).client).resp.ctf_team {
                (gi.cprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    cl_ent,
                    3 as libc::c_int,
                    b"(%s): %s\n\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ((*(*who).client).pers.netname).as_mut_ptr(),
                    outmsg.as_mut_ptr(),
                );
            }
        }
        i += 1;
    }
}
unsafe extern "C" fn misc_ctf_banner_think(mut ent: *mut edict_t) {
    (*ent).s.frame = ((*ent).s.frame + 1 as libc::c_int) % 16 as libc::c_int;
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_ctf_banner(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/ctf/banner/tris.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent).s.skinnum = 1 as libc::c_int;
    }
    (*ent).s.frame = rand() % 16 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(ent);
    let ref mut fresh51 = (*ent).think;
    *fresh51 = Some(misc_ctf_banner_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn SP_misc_ctf_small_banner(mut ent: *mut edict_t) {
    (*ent).movetype = MOVETYPE_NONE as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent)
        .s
        .modelindex = (gi.modelindex)
        .expect(
            "non-null function pointer",
        )(
        b"models/ctf/banner/small.md2\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    if (*ent).spawnflags & 1 as libc::c_int != 0 {
        (*ent).s.skinnum = 1 as libc::c_int;
    }
    (*ent).s.frame = rand() % 16 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(ent);
    let ref mut fresh52 = (*ent).think;
    *fresh52 = Some(misc_ctf_banner_think as unsafe extern "C" fn(*mut edict_t) -> ());
    (*ent).nextthink = (level.time as libc::c_double + 0.1f64) as libc::c_float;
}
unsafe extern "C" fn SetLevelName(mut p: *mut pmenu_t) {
    static mut levelname: [libc::c_char; 33] = [0; 33];
    levelname[0 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    if !((*g_edicts.offset(0 as libc::c_int as isize)).message).is_null() {
        strncpy(
            levelname.as_mut_ptr().offset(1 as libc::c_int as isize),
            (*g_edicts.offset(0 as libc::c_int as isize)).message,
            (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
    } else {
        strncpy(
            levelname.as_mut_ptr().offset(1 as libc::c_int as isize),
            (level.mapname).as_mut_ptr(),
            (::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
    }
    levelname[(::std::mem::size_of::<[libc::c_char; 33]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        as usize] = 0 as libc::c_int as libc::c_char;
    let ref mut fresh53 = (*p).text;
    *fresh53 = levelname.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn CTFBeginElection(
    mut ent: *mut edict_t,
    mut type_0: elect_t,
    mut msg: *mut libc::c_char,
) -> qboolean {
    let mut i: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    if (*electpercentage).value == 0 as libc::c_int as libc::c_float {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Elections are disabled, only an admin can process this action.\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        return false_0;
    }
    if ctfgame.election as libc::c_uint != ELECT_NONE as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Election already in progress.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return false_0;
    }
    count = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        e = g_edicts.offset(i as isize);
        (*(*e).client).resp.voted = false_0;
        if (*e).inuse as u64 != 0 {
            count += 1;
        }
        i += 1;
    }
    if count < 2 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Not enough players for election.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return false_0;
    }
    ctfgame.etarget = ent;
    ctfgame.election = type_0;
    ctfgame.evotes = 0 as libc::c_int;
    ctfgame
        .needvotes = (count as libc::c_float * (*electpercentage).value
        / 100 as libc::c_int as libc::c_float) as libc::c_int;
    ctfgame.electtime = level.time + 20 as libc::c_int as libc::c_float;
    strncpy(
        (ctfgame.emsg).as_mut_ptr(),
        msg,
        (::std::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        3 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (ctfgame.emsg).as_mut_ptr(),
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"Type YES or NO to vote on this request.\n\0" as *const u8
            as *const libc::c_char as *mut libc::c_char,
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"Votes: %d  Needed: %d  Time left: %ds\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.evotes,
        ctfgame.needvotes,
        (ctfgame.electtime - level.time) as libc::c_int,
    );
    return true_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFResetAllPlayers() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        ent = g_edicts.offset(i as isize);
        if !((*ent).inuse as u64 == 0) {
            if !((*(*ent).client).menu).is_null() {
                PMenu_Close(ent);
            }
            CTFPlayerResetGrapple(ent);
            CTFDeadDropFlag(ent);
            CTFDeadDropTech(ent);
            (*(*ent).client).resp.ctf_team = CTF_NOTEAM as libc::c_int;
            (*(*ent).client).resp.ready = false_0;
            (*ent).svflags = 0 as libc::c_int;
            (*ent).flags &= !(0x10 as libc::c_int);
            PutClientInServer(ent);
        }
        i += 1;
    }
    CTFResetTech();
    CTFResetFlags();
    ent = g_edicts.offset(1 as libc::c_int as isize);
    i = 1 as libc::c_int;
    while i < globals.num_edicts {
        if (*ent).inuse as libc::c_uint != 0 && ((*ent).client).is_null() {
            if (*ent).solid as libc::c_uint == SOLID_NOT as libc::c_int as libc::c_uint
                && (*ent).think
                    == Some(DoRespawn as unsafe extern "C" fn(*mut edict_t) -> ())
                && (*ent).nextthink >= level.time
            {
                (*ent).nextthink = 0 as libc::c_int as libc::c_float;
                DoRespawn(ent);
            }
        }
        i += 1;
        ent = ent.offset(1);
    }
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        ctfgame
            .matchtime = level.time
            + (*matchsetuptime).value * 60 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFAssignGhost(mut ent: *mut edict_t) {
    let mut ghost: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    ghost = 0 as libc::c_int;
    while ghost < 256 as libc::c_int {
        if ctfgame.ghosts[ghost as usize].code == 0 {
            break;
        }
        ghost += 1;
    }
    if ghost == 256 as libc::c_int {
        return;
    }
    ctfgame.ghosts[ghost as usize].team = (*(*ent).client).resp.ctf_team;
    ctfgame.ghosts[ghost as usize].score = 0 as libc::c_int;
    loop {
        ctfgame
            .ghosts[ghost as usize]
            .code = 10000 as libc::c_int + rand() % 90000 as libc::c_int;
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            if i != ghost
                && ctfgame.ghosts[i as usize].code == ctfgame.ghosts[ghost as usize].code
            {
                break;
            }
            i += 1;
        }
        if i == 256 as libc::c_int {
            break;
        }
    }
    ctfgame.ghosts[ghost as usize].ent = ent;
    strcpy(
        (ctfgame.ghosts[ghost as usize].netname).as_mut_ptr(),
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    let ref mut fresh54 = (*(*ent).client).resp.ghost;
    *fresh54 = (ctfgame.ghosts).as_mut_ptr().offset(ghost as isize);
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        3 as libc::c_int,
        b"Your ghost code is **** %d ****\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.ghosts[ghost as usize].code,
    );
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"If you lose connection, you can rejoin with your score intact by typing \"ghost %d\".\n\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
        ctfgame.ghosts[ghost as usize].code,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFStartMatch() {
    let mut i: libc::c_int = 0;
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    let mut ghost: libc::c_int = 0 as libc::c_int;
    ctfgame.match_0 = MATCH_GAME;
    ctfgame
        .matchtime = level.time
        + (*matchtime).value * 60 as libc::c_int as libc::c_float;
    ctfgame.team2 = 0 as libc::c_int;
    ctfgame.team1 = ctfgame.team2;
    memset(
        (ctfgame.ghosts).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<[ghost_t; 256]>() as libc::c_ulong,
    );
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        ent = g_edicts.offset(i as isize);
        if !((*ent).inuse as u64 == 0) {
            (*(*ent).client).resp.score = 0 as libc::c_int;
            (*(*ent).client).resp.ctf_state = 0 as libc::c_int;
            let ref mut fresh55 = (*(*ent).client).resp.ghost;
            *fresh55 = 0 as *mut ghost_s;
            (gi.centerprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                b"******************\n\nMATCH HAS STARTED!\n\n******************\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if (*(*ent).client).resp.ctf_team != CTF_NOTEAM as libc::c_int {
                CTFAssignGhost(ent);
                CTFPlayerResetGrapple(ent);
                (*ent).svflags = 0x1 as libc::c_int;
                (*ent).flags &= !(0x10 as libc::c_int);
                (*(*ent).client)
                    .respawn_time = (level.time as libc::c_double + 1.0f64
                    + (rand() % 30 as libc::c_int) as libc::c_double / 10.0f64)
                    as libc::c_float;
                (*(*ent).client).ps.pmove.pm_type = PM_DEAD;
                (*(*ent).client).anim_priority = 5 as libc::c_int;
                (*ent).s.frame = 197 as libc::c_int - 1 as libc::c_int;
                (*(*ent).client).anim_end = 197 as libc::c_int;
                (*ent).deadflag = 2 as libc::c_int;
                (*ent).movetype = MOVETYPE_NOCLIP as libc::c_int;
                (*(*ent).client).ps.gunindex = 0 as libc::c_int;
                (gi.linkentity).expect("non-null function pointer")(ent);
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFEndMatch() {
    ctfgame.match_0 = MATCH_POST;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        3 as libc::c_int,
        b"MATCH COMPLETED!\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CTFCalcScores();
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"RED TEAM:  %d captures, %d points\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.team1,
        ctfgame.total1,
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"BLUE TEAM:  %d captures, %d points\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.team2,
        ctfgame.total2,
    );
    if ctfgame.team1 > ctfgame.team2 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"RED team won over the BLUE team by %d CAPTURES!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ctfgame.team1 - ctfgame.team2,
        );
    } else if ctfgame.team2 > ctfgame.team1 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"BLUE team won over the RED team by %d CAPTURES!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ctfgame.team2 - ctfgame.team1,
        );
    } else if ctfgame.total1 > ctfgame.total2 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"RED team won over the BLUE team by %d POINTS!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ctfgame.total1 - ctfgame.total2,
        );
    } else if ctfgame.total2 > ctfgame.total1 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"BLUE team won over the RED team by %d POINTS!\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ctfgame.total2 - ctfgame.total1,
        );
    } else {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"TIE GAME!\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    EndDMLevel();
}
#[no_mangle]
pub unsafe extern "C" fn CTFNextMap() -> qboolean {
    if ctfgame.match_0 as libc::c_uint == MATCH_POST as libc::c_int as libc::c_uint {
        ctfgame.match_0 = MATCH_SETUP;
        CTFResetAllPlayers();
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFWinElection() {
    match ctfgame.election as libc::c_uint {
        1 => {
            if (*competition).value < 3 as libc::c_int as libc::c_float {
                (gi.cvar_set)
                    .expect(
                        "non-null function pointer",
                    )(
                    b"competition\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            ctfgame.match_0 = MATCH_SETUP;
            CTFResetAllPlayers();
        }
        2 => {
            (*(*ctfgame.etarget).client).resp.admin = true_0;
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                2 as libc::c_int,
                b"%s has become an admin.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*ctfgame.etarget).client).pers.netname).as_mut_ptr(),
            );
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ctfgame.etarget,
                2 as libc::c_int,
                b"Type 'admin' to access the adminstration menu.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        3 => {
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                2 as libc::c_int,
                b"%s is warping to level %s.\n\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ((*(*ctfgame.etarget).client).pers.netname).as_mut_ptr(),
                (ctfgame.elevel).as_mut_ptr(),
            );
            strncpy(
                (level.forcemap).as_mut_ptr(),
                (ctfgame.elevel).as_mut_ptr(),
                (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
            EndDMLevel();
        }
        _ => {}
    }
    ctfgame.election = ELECT_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn CTFVoteYes(mut ent: *mut edict_t) {
    if ctfgame.election as libc::c_uint == ELECT_NONE as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No election is in progress.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client).resp.voted as u64 != 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You already voted.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if ctfgame.etarget == ent {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You can't vote for yourself.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    (*(*ent).client).resp.voted = true_0;
    ctfgame.evotes += 1;
    if ctfgame.evotes == ctfgame.needvotes {
        CTFWinElection();
        return;
    }
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (ctfgame.emsg).as_mut_ptr(),
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        3 as libc::c_int,
        b"Votes: %d  Needed: %d  Time left: %ds\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.evotes,
        ctfgame.needvotes,
        (ctfgame.electtime - level.time) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFVoteNo(mut ent: *mut edict_t) {
    if ctfgame.election as libc::c_uint == ELECT_NONE as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No election is in progress.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client).resp.voted as u64 != 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You already voted.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if ctfgame.etarget == ent {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You can't vote for yourself.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    (*(*ent).client).resp.voted = true_0;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        (ctfgame.emsg).as_mut_ptr(),
    );
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        3 as libc::c_int,
        b"Votes: %d  Needed: %d  Time left: %ds\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ctfgame.evotes,
        ctfgame.needvotes,
        (ctfgame.electtime - level.time) as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFReady(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    let mut t1: libc::c_int = 0;
    let mut t2: libc::c_int = 0;
    if (*(*ent).client).resp.ctf_team == CTF_NOTEAM as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Pick a team first (hit <TAB> for menu)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if ctfgame.match_0 as libc::c_uint != MATCH_SETUP as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"A match is not being setup.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client).resp.ready as u64 != 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You have already commited.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    (*(*ent).client).resp.ready = true_0;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s is ready.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    t2 = 0 as libc::c_int;
    t1 = t2;
    j = 0 as libc::c_int;
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        e = g_edicts.offset(i as isize);
        if !((*e).inuse as u64 == 0) {
            if (*(*e).client).resp.ctf_team != CTF_NOTEAM as libc::c_int
                && (*(*e).client).resp.ready as u64 == 0
            {
                j += 1;
            }
            if (*(*e).client).resp.ctf_team == CTF_TEAM1 as libc::c_int {
                t1 += 1;
            } else if (*(*e).client).resp.ctf_team == CTF_TEAM2 as libc::c_int {
                t2 += 1;
            }
        }
        i += 1;
    }
    if j == 0 && t1 != 0 && t2 != 0 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"All players have commited.  Match starting\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        ctfgame.match_0 = MATCH_PREGAME;
        ctfgame.matchtime = level.time + (*matchstarttime).value;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFNotReady(mut ent: *mut edict_t) {
    if (*(*ent).client).resp.ctf_team == CTF_NOTEAM as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Pick a team first (hit <TAB> for menu)\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    if ctfgame.match_0 as libc::c_uint != MATCH_SETUP as libc::c_int as libc::c_uint
        && ctfgame.match_0 as libc::c_uint
            != MATCH_PREGAME as libc::c_int as libc::c_uint
    {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"A match is not being setup.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client).resp.ready as u64 == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You haven't commited.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    (*(*ent).client).resp.ready = false_0;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s is no longer ready.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    if ctfgame.match_0 as libc::c_uint == MATCH_PREGAME as libc::c_int as libc::c_uint {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"Match halted.\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        ctfgame.match_0 = MATCH_SETUP;
        ctfgame
            .matchtime = level.time
            + (*matchsetuptime).value * 60 as libc::c_int as libc::c_float;
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFGhost(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if (gi.argc).expect("non-null function pointer")() < 2 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Usage:  ghost <code>\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(*ent).client).resp.ctf_team != CTF_NOTEAM as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You are already in the game.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if ctfgame.match_0 as libc::c_uint != MATCH_GAME as libc::c_int as libc::c_uint {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No match is in progress.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    n = atoi((gi.argv).expect("non-null function pointer")(1 as libc::c_int));
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        if ctfgame.ghosts[i as usize].code != 0 && ctfgame.ghosts[i as usize].code == n {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"Ghost code accepted, your position has been reinstated.\n\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            let ref mut fresh56 = (*(*ctfgame.ghosts[i as usize].ent).client).resp.ghost;
            *fresh56 = 0 as *mut ghost_s;
            (*(*ent).client).resp.ctf_team = ctfgame.ghosts[i as usize].team;
            let ref mut fresh57 = (*(*ent).client).resp.ghost;
            *fresh57 = (ctfgame.ghosts).as_mut_ptr().offset(i as isize);
            (*(*ent).client).resp.score = ctfgame.ghosts[i as usize].score;
            (*(*ent).client).resp.ctf_state = 0 as libc::c_int;
            ctfgame.ghosts[i as usize].ent = ent;
            (*ent).svflags = 0 as libc::c_int;
            (*ent).flags &= !(0x10 as libc::c_int);
            PutClientInServer(ent);
            (gi.bprintf)
                .expect(
                    "non-null function pointer",
                )(
                2 as libc::c_int,
                b"%s has been reinstated to %s team.\n\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                ((*(*ent).client).pers.netname).as_mut_ptr(),
                CTFTeamName((*(*ent).client).resp.ctf_team),
            );
            return;
        }
        i += 1;
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"Invalid ghost code.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFMatchSetup() -> qboolean {
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint
        || ctfgame.match_0 as libc::c_uint
            == MATCH_PREGAME as libc::c_int as libc::c_uint
    {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFMatchOn() -> qboolean {
    if ctfgame.match_0 as libc::c_uint == MATCH_GAME as libc::c_int as libc::c_uint {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub static mut creditsmenu: [pmenu_t; 18] = unsafe {
    [
        {
            let mut init = pmenu_s {
                text: b"*Quake II\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*ThreeWave Capture the Flag\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*Programming\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Dave 'Zoid' Kirsch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*Level Design\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Christian Antkow\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Tim Willits\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Dave 'Zoid' Kirsch\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*Art\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Adrian Carmack Paul Steed\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Kevin Cloud\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*Sound\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Tom 'Bjorn' Klok\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*Original CTF Art Design\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Brian 'Whaleboy' Cozzens\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Return to Main Menu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFReturnToMain
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
    ]
};
static mut jmenu_level: libc::c_int = 2 as libc::c_int;
static mut jmenu_match: libc::c_int = 3 as libc::c_int;
static mut jmenu_red: libc::c_int = 5 as libc::c_int;
static mut jmenu_blue: libc::c_int = 7 as libc::c_int;
static mut jmenu_chase: libc::c_int = 9 as libc::c_int;
static mut jmenu_reqmatch: libc::c_int = 11 as libc::c_int;
#[no_mangle]
pub static mut joinmenu: [pmenu_t; 18] = unsafe {
    [
        {
            let mut init = pmenu_s {
                text: b"*Quake II\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*ThreeWave Capture the Flag\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Join Red Team\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFJoinTeam1
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Join Blue Team\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFJoinTeam2
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Chase Camera\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFChaseCam
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Credits\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFCredits
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Use [ and ] to move cursor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"ENTER to select\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"ESC to Exit Menu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"(TAB to Return)\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"v1.09b\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_RIGHT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut nochasemenu: [pmenu_t; 7] = unsafe {
    [
        {
            let mut init = pmenu_s {
                text: b"*Quake II\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"*ThreeWave Capture the Flag\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"No one to chase\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Return to Main Menu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFReturnToMain
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CTFJoinTeam(
    mut ent: *mut edict_t,
    mut desired_team: libc::c_int,
) {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    PMenu_Close(ent);
    (*ent).svflags &= !(0x1 as libc::c_int);
    (*(*ent).client).resp.ctf_team = desired_team;
    (*(*ent).client).resp.ctf_state = 0 as libc::c_int;
    s = Info_ValueForKey(
        ((*(*ent).client).pers.userinfo).as_mut_ptr(),
        b"skin\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    CTFAssignSkin(ent, s);
    if ctfgame.match_0 as libc::c_uint == MATCH_GAME as libc::c_int as libc::c_uint {
        if !((*(*ent).client).resp.ghost).is_null() {
            (*(*(*ent).client).resp.ghost).code = 0 as libc::c_int;
        }
        let ref mut fresh58 = (*(*ent).client).resp.ghost;
        *fresh58 = 0 as *mut ghost_s;
        CTFAssignGhost(ent);
    }
    PutClientInServer(ent);
    (*ent).s.event = EV_PLAYER_TELEPORT as libc::c_int;
    (*(*ent).client).ps.pmove.pm_flags = 32 as libc::c_int as byte;
    (*(*ent).client).ps.pmove.pm_time = 14 as libc::c_int as byte;
    (gi.bprintf)
        .expect(
            "non-null function pointer",
        )(
        2 as libc::c_int,
        b"%s joined the %s team.\n\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
        CTFTeamName(desired_team),
    );
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        (gi.centerprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            b"***********************\nType \"ready\" in console\nto ready up.\n***********************\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFJoinTeam1(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    CTFJoinTeam(ent, CTF_TEAM1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CTFJoinTeam2(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    CTFJoinTeam(ent, CTF_TEAM2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn CTFChaseCam(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    let mut i: libc::c_int = 0;
    let mut e: *mut edict_t = 0 as *mut edict_t;
    if !((*(*ent).client).chase_target).is_null() {
        let ref mut fresh59 = (*(*ent).client).chase_target;
        *fresh59 = 0 as *mut edict_t;
        PMenu_Close(ent);
        return;
    }
    i = 1 as libc::c_int;
    while i as libc::c_float <= (*maxclients).value {
        e = g_edicts.offset(i as isize);
        if (*e).inuse as libc::c_uint != 0
            && (*e).solid as libc::c_uint != SOLID_NOT as libc::c_int as libc::c_uint
        {
            let ref mut fresh60 = (*(*ent).client).chase_target;
            *fresh60 = e;
            PMenu_Close(ent);
            (*(*ent).client).update_chase = true_0;
            return;
        }
        i += 1;
    }
    SetLevelName(nochasemenu.as_mut_ptr().offset(jmenu_level as isize));
    PMenu_Close(ent);
    PMenu_Open(
        ent,
        nochasemenu.as_mut_ptr(),
        -(1 as libc::c_int),
        (::std::mem::size_of::<[pmenu_t; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pmenu_t>() as libc::c_ulong)
            as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFReturnToMain(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    PMenu_Close(ent);
    CTFOpenJoinMenu(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFRequestMatch(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    PMenu_Close(ent);
    sprintf(
        text.as_mut_ptr(),
        b"%s has requested to switch to competition mode.\0" as *const u8
            as *const libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
    );
    CTFBeginElection(ent, ELECT_MATCH, text.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn CTFShowScores(mut ent: *mut edict_t, mut p: *mut pmenu_t) {
    PMenu_Close(ent);
    (*(*ent).client).showscores = true_0;
    (*(*ent).client).showinventory = false_0;
    DeathmatchScoreboard(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFUpdateJoinMenu(mut ent: *mut edict_t) -> libc::c_int {
    static mut team1players: [libc::c_char; 32] = [0; 32];
    static mut team2players: [libc::c_char; 32] = [0; 32];
    let mut num1: libc::c_int = 0;
    let mut num2: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if ctfgame.match_0 as libc::c_uint >= MATCH_PREGAME as libc::c_int as libc::c_uint
        && (*matchlock).value != 0.
    {
        joinmenu[jmenu_red as usize]
            .text = b"MATCH IS LOCKED\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        joinmenu[jmenu_red as usize].SelectFunc = None;
        joinmenu[jmenu_blue as usize]
            .text = b"  (entry is not permitted)\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        joinmenu[jmenu_blue as usize].SelectFunc = None;
    } else {
        if ctfgame.match_0 as libc::c_uint
            >= MATCH_PREGAME as libc::c_int as libc::c_uint
        {
            joinmenu[jmenu_red as usize]
                .text = b"Join Red MATCH Team\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            joinmenu[jmenu_blue as usize]
                .text = b"Join Blue MATCH Team\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            joinmenu[jmenu_red as usize]
                .text = b"Join Red Team\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            joinmenu[jmenu_blue as usize]
                .text = b"Join Blue Team\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        joinmenu[jmenu_red as usize]
            .SelectFunc = Some(
            CTFJoinTeam1 as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
        joinmenu[jmenu_blue as usize]
            .SelectFunc = Some(
            CTFJoinTeam2 as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
    }
    if !((*ctf_forcejoin).string).is_null()
        && *(*ctf_forcejoin).string as libc::c_int != 0
    {
        if stricmp((*ctf_forcejoin).string, b"red\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            joinmenu[jmenu_blue as usize].text = 0 as *mut libc::c_char;
            joinmenu[jmenu_blue as usize].SelectFunc = None;
        } else if stricmp(
            (*ctf_forcejoin).string,
            b"blue\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            joinmenu[jmenu_red as usize].text = 0 as *mut libc::c_char;
            joinmenu[jmenu_red as usize].SelectFunc = None;
        }
    }
    if !((*(*ent).client).chase_target).is_null() {
        joinmenu[jmenu_chase as usize]
            .text = b"Leave Chase Camera\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else {
        joinmenu[jmenu_chase as usize]
            .text = b"Chase Camera\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    }
    SetLevelName(joinmenu.as_mut_ptr().offset(jmenu_level as isize));
    num2 = 0 as libc::c_int;
    num1 = num2;
    i = 0 as libc::c_int;
    while (i as libc::c_float) < (*maxclients).value {
        if !((*g_edicts.offset((i + 1 as libc::c_int) as isize)).inuse as u64 == 0) {
            if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM1 as libc::c_int
            {
                num1 += 1;
            } else if (*(game.clients).offset(i as isize)).resp.ctf_team
                == CTF_TEAM2 as libc::c_int
            {
                num2 += 1;
            }
        }
        i += 1;
    }
    sprintf(
        team1players.as_mut_ptr(),
        b"  (%d players)\0" as *const u8 as *const libc::c_char,
        num1,
    );
    sprintf(
        team2players.as_mut_ptr(),
        b"  (%d players)\0" as *const u8 as *const libc::c_char,
        num2,
    );
    match ctfgame.match_0 as libc::c_uint {
        0 => {
            joinmenu[jmenu_match as usize].text = 0 as *mut libc::c_char;
        }
        1 => {
            joinmenu[jmenu_match as usize]
                .text = b"*MATCH SETUP IN PROGRESS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        2 => {
            joinmenu[jmenu_match as usize]
                .text = b"*MATCH STARTING\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        3 => {
            joinmenu[jmenu_match as usize]
                .text = b"*MATCH IN PROGRESS\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        _ => {}
    }
    if !(joinmenu[jmenu_red as usize].text).is_null() {
        joinmenu[(jmenu_red + 1 as libc::c_int) as usize]
            .text = team1players.as_mut_ptr();
    } else {
        joinmenu[(jmenu_red + 1 as libc::c_int) as usize].text = 0 as *mut libc::c_char;
    }
    if !(joinmenu[jmenu_blue as usize].text).is_null() {
        joinmenu[(jmenu_blue + 1 as libc::c_int) as usize]
            .text = team2players.as_mut_ptr();
    } else {
        joinmenu[(jmenu_blue + 1 as libc::c_int) as usize].text = 0 as *mut libc::c_char;
    }
    joinmenu[jmenu_reqmatch as usize].text = 0 as *mut libc::c_char;
    joinmenu[jmenu_reqmatch as usize].SelectFunc = None;
    if (*competition).value != 0.
        && (ctfgame.match_0 as libc::c_uint) < MATCH_SETUP as libc::c_int as libc::c_uint
    {
        joinmenu[jmenu_reqmatch as usize]
            .text = b"Request Match\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        joinmenu[jmenu_reqmatch as usize]
            .SelectFunc = Some(
            CTFRequestMatch as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
    }
    if num1 > num2 {
        return CTF_TEAM1 as libc::c_int
    } else {
        if num2 > num1 {
            return CTF_TEAM2 as libc::c_int;
        }
    }
    return if rand() & 1 as libc::c_int != 0 {
        CTF_TEAM1 as libc::c_int
    } else {
        CTF_TEAM2 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn CTFOpenJoinMenu(mut ent: *mut edict_t) {
    let mut team: libc::c_int = 0;
    team = CTFUpdateJoinMenu(ent);
    if !((*(*ent).client).chase_target).is_null() {
        team = 8 as libc::c_int;
    } else if team == CTF_TEAM1 as libc::c_int {
        team = 4 as libc::c_int;
    } else {
        team = 6 as libc::c_int;
    }
    PMenu_Open(
        ent,
        joinmenu.as_mut_ptr(),
        team,
        (::std::mem::size_of::<[pmenu_t; 18]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pmenu_t>() as libc::c_ulong)
            as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFCredits(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    PMenu_Close(ent);
    PMenu_Open(
        ent,
        creditsmenu.as_mut_ptr(),
        -(1 as libc::c_int),
        (::std::mem::size_of::<[pmenu_t; 18]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pmenu_t>() as libc::c_ulong)
            as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFStartClient(mut ent: *mut edict_t) -> qboolean {
    if (*(*ent).client).resp.ctf_team != CTF_NOTEAM as libc::c_int {
        return false_0;
    }
    if (*dmflags).value as libc::c_int & 131072 as libc::c_int == 0
        || ctfgame.match_0 as libc::c_uint >= MATCH_SETUP as libc::c_int as libc::c_uint
    {
        (*ent).movetype = MOVETYPE_NOCLIP as libc::c_int;
        (*ent).solid = SOLID_NOT;
        (*ent).svflags |= 0x1 as libc::c_int;
        (*(*ent).client).resp.ctf_team = CTF_NOTEAM as libc::c_int;
        (*(*ent).client).ps.gunindex = 0 as libc::c_int;
        (gi.linkentity).expect("non-null function pointer")(ent);
        CTFOpenJoinMenu(ent);
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFObserver(mut ent: *mut edict_t) {
    if (*ent).movetype == MOVETYPE_NOCLIP as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You are already an observer.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    CTFPlayerResetGrapple(ent);
    CTFDeadDropFlag(ent);
    CTFDeadDropTech(ent);
    (*ent).movetype = MOVETYPE_NOCLIP as libc::c_int;
    (*ent).solid = SOLID_NOT;
    (*ent).svflags |= 0x1 as libc::c_int;
    (*(*ent).client).resp.ctf_team = CTF_NOTEAM as libc::c_int;
    (*(*ent).client).ps.gunindex = 0 as libc::c_int;
    (*(*ent).client).resp.score = 0 as libc::c_int;
    (gi.linkentity).expect("non-null function pointer")(ent);
    CTFOpenJoinMenu(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFInMatch() -> qboolean {
    if ctfgame.match_0 as libc::c_uint > MATCH_NONE as libc::c_int as libc::c_uint {
        return true_0;
    }
    return false_0;
}
#[no_mangle]
pub unsafe extern "C" fn CTFCheckRules() -> qboolean {
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut text: [libc::c_char; 64] = [0; 64];
    let mut ent: *mut edict_t = 0 as *mut edict_t;
    if ctfgame.election as libc::c_uint != ELECT_NONE as libc::c_int as libc::c_uint
        && ctfgame.electtime <= level.time
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"Election timed out and has been cancelled.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        ctfgame.election = ELECT_NONE;
    }
    if ctfgame.match_0 as libc::c_uint != MATCH_NONE as libc::c_int as libc::c_uint {
        t = (ctfgame.matchtime - level.time) as libc::c_int;
        if t <= 0 as libc::c_int {
            match ctfgame.match_0 as libc::c_uint {
                1 => {
                    if (*competition).value < 3 as libc::c_int as libc::c_float {
                        ctfgame.match_0 = MATCH_NONE;
                        (gi.cvar_set)
                            .expect(
                                "non-null function pointer",
                            )(
                            b"competition\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            b"1\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                        );
                        CTFResetAllPlayers();
                    } else {
                        ctfgame
                            .matchtime = level.time
                            + (*matchsetuptime).value
                                * 60 as libc::c_int as libc::c_float;
                    }
                    return false_0;
                }
                2 => {
                    CTFStartMatch();
                    return false_0;
                }
                3 => {
                    CTFEndMatch();
                    return false_0;
                }
                _ => {}
            }
        }
        if t == ctfgame.lasttime {
            return false_0;
        }
        ctfgame.lasttime = t;
        match ctfgame.match_0 as libc::c_uint {
            1 => {
                j = 0 as libc::c_int;
                i = 1 as libc::c_int;
                while i as libc::c_float <= (*maxclients).value {
                    ent = g_edicts.offset(i as isize);
                    if !((*ent).inuse as u64 == 0) {
                        if (*(*ent).client).resp.ctf_team != CTF_NOTEAM as libc::c_int
                            && (*(*ent).client).resp.ready as u64 == 0
                        {
                            j += 1;
                        }
                    }
                    i += 1;
                }
                if (*competition).value < 3 as libc::c_int as libc::c_float {
                    sprintf(
                        text.as_mut_ptr(),
                        b"%02d:%02d SETUP: %d not ready\0" as *const u8
                            as *const libc::c_char,
                        t / 60 as libc::c_int,
                        t % 60 as libc::c_int,
                        j,
                    );
                } else {
                    sprintf(
                        text.as_mut_ptr(),
                        b"SETUP: %d not ready\0" as *const u8 as *const libc::c_char,
                        j,
                    );
                }
                (gi.configstring)
                    .expect(
                        "non-null function pointer",
                    )(30 as libc::c_int - 1 as libc::c_int, text.as_mut_ptr());
            }
            2 => {
                sprintf(
                    text.as_mut_ptr(),
                    b"%02d:%02d UNTIL START\0" as *const u8 as *const libc::c_char,
                    t / 60 as libc::c_int,
                    t % 60 as libc::c_int,
                );
                (gi.configstring)
                    .expect(
                        "non-null function pointer",
                    )(30 as libc::c_int - 1 as libc::c_int, text.as_mut_ptr());
            }
            3 => {
                sprintf(
                    text.as_mut_ptr(),
                    b"%02d:%02d MATCH\0" as *const u8 as *const libc::c_char,
                    t / 60 as libc::c_int,
                    t % 60 as libc::c_int,
                );
                (gi.configstring)
                    .expect(
                        "non-null function pointer",
                    )(30 as libc::c_int - 1 as libc::c_int, text.as_mut_ptr());
            }
            _ => {}
        }
        return false_0;
    }
    if (*capturelimit).value != 0.
        && (ctfgame.team1 as libc::c_float >= (*capturelimit).value
            || ctfgame.team2 as libc::c_float >= (*capturelimit).value)
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"Capturelimit hit.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return true_0;
    }
    return false_0;
}
unsafe extern "C" fn old_teleporter_touch(
    mut self_0: *mut edict_t,
    mut other: *mut edict_t,
    mut plane: *mut cplane_t,
    mut surf: *mut csurface_t,
) {
    let mut dest: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    let mut forward: vec3_t = [0.; 3];
    if ((*other).client).is_null() {
        return;
    }
    dest = G_Find(
        0 as *mut edict_t,
        &mut (*(0 as *mut edict_t)).targetname as *mut *mut libc::c_char as libc::c_int,
        (*self_0).target,
    );
    if dest.is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"Couldn't find destination\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    CTFPlayerResetGrapple(other);
    (gi.unlinkentity).expect("non-null function pointer")(other);
    (*other)
        .s
        .origin[0 as libc::c_int as usize] = (*dest).s.origin[0 as libc::c_int as usize];
    (*other)
        .s
        .origin[1 as libc::c_int as usize] = (*dest).s.origin[1 as libc::c_int as usize];
    (*other)
        .s
        .origin[2 as libc::c_int as usize] = (*dest).s.origin[2 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[0 as libc::c_int
        as usize] = (*dest).s.origin[0 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[1 as libc::c_int
        as usize] = (*dest).s.origin[1 as libc::c_int as usize];
    (*other)
        .s
        .old_origin[2 as libc::c_int
        as usize] = (*dest).s.origin[2 as libc::c_int as usize];
    let ref mut fresh61 = (*other).velocity[2 as libc::c_int as usize];
    *fresh61 = 0 as libc::c_int as vec_t;
    let ref mut fresh62 = (*other).velocity[1 as libc::c_int as usize];
    *fresh62 = *fresh61;
    (*other).velocity[0 as libc::c_int as usize] = *fresh62;
    (*(*other).client)
        .ps
        .pmove
        .pm_time = (160 as libc::c_int >> 3 as libc::c_int) as byte;
    let ref mut fresh63 = (*(*other).client).ps.pmove.pm_flags;
    *fresh63 = (*fresh63 as libc::c_int | 32 as libc::c_int) as byte;
    (*(*self_0).enemy).s.event = EV_PLAYER_TELEPORT as libc::c_int;
    (*other).s.event = EV_PLAYER_TELEPORT as libc::c_int;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*(*other).client)
            .ps
            .pmove
            .delta_angles[i
            as usize] = ((((*dest).s.angles[i as usize]
            - (*(*other).client).resp.cmd_angles[i as usize])
            * 65536 as libc::c_int as libc::c_float
            / 360 as libc::c_int as libc::c_float) as libc::c_int & 65535 as libc::c_int)
            as libc::c_short;
        i += 1;
    }
    (*other).s.angles[0 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*other)
        .s
        .angles[1 as libc::c_int as usize] = (*dest).s.angles[1 as libc::c_int as usize];
    (*other).s.angles[2 as libc::c_int as usize] = 0 as libc::c_int as vec_t;
    (*(*other).client)
        .ps
        .viewangles[0 as libc::c_int
        as usize] = (*dest).s.angles[0 as libc::c_int as usize];
    (*(*other).client)
        .ps
        .viewangles[1 as libc::c_int
        as usize] = (*dest).s.angles[1 as libc::c_int as usize];
    (*(*other).client)
        .ps
        .viewangles[2 as libc::c_int
        as usize] = (*dest).s.angles[2 as libc::c_int as usize];
    (*(*other).client)
        .v_angle[0 as libc::c_int
        as usize] = (*dest).s.angles[0 as libc::c_int as usize];
    (*(*other).client)
        .v_angle[1 as libc::c_int
        as usize] = (*dest).s.angles[1 as libc::c_int as usize];
    (*(*other).client)
        .v_angle[2 as libc::c_int
        as usize] = (*dest).s.angles[2 as libc::c_int as usize];
    AngleVectors(
        ((*(*other).client).v_angle).as_mut_ptr(),
        forward.as_mut_ptr(),
        0 as *mut vec_t,
        0 as *mut vec_t,
    );
    VectorScale(
        forward.as_mut_ptr(),
        200 as libc::c_int as vec_t,
        ((*other).velocity).as_mut_ptr(),
    );
    KillBox(other) as u64 == 0;
    (gi.linkentity).expect("non-null function pointer")(other);
}
#[no_mangle]
pub unsafe extern "C" fn SP_trigger_teleport(mut ent: *mut edict_t) {
    let mut s: *mut edict_t = 0 as *mut edict_t;
    let mut i: libc::c_int = 0;
    if ((*ent).target).is_null() {
        (gi.dprintf)
            .expect(
                "non-null function pointer",
            )(
            b"teleporter without a target.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        G_FreeEdict(ent);
        return;
    }
    (*ent).svflags |= 0x1 as libc::c_int;
    (*ent).solid = SOLID_TRIGGER;
    let ref mut fresh64 = (*ent).touch;
    *fresh64 = Some(
        old_teleporter_touch
            as unsafe extern "C" fn(
                *mut edict_t,
                *mut edict_t,
                *mut cplane_t,
                *mut csurface_t,
            ) -> (),
    );
    (gi.setmodel).expect("non-null function pointer")(ent, (*ent).model);
    (gi.linkentity).expect("non-null function pointer")(ent);
    s = G_Spawn();
    let ref mut fresh65 = (*ent).enemy;
    *fresh65 = s;
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        (*s)
            .s
            .origin[i
            as usize] = (*ent).mins[i as usize]
            + ((*ent).maxs[i as usize] - (*ent).mins[i as usize])
                / 2 as libc::c_int as libc::c_float;
        i += 1;
    }
    (*s)
        .s
        .sound = (gi.soundindex)
        .expect(
            "non-null function pointer",
        )(b"world/hum1.wav\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    (gi.linkentity).expect("non-null function pointer")(s);
}
#[no_mangle]
pub unsafe extern "C" fn SP_info_teleport_destination(mut ent: *mut edict_t) {
    let ref mut fresh66 = (*ent).s.origin[2 as libc::c_int as usize];
    *fresh66 += 16 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_SettingsApply(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    let mut st: [libc::c_char; 80] = [0; 80];
    let mut i: libc::c_int = 0;
    if (*settings).matchlen as libc::c_float != (*matchtime).value {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s changed the match length to %d minutes.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            (*settings).matchlen,
        );
        if ctfgame.match_0 as libc::c_uint == MATCH_GAME as libc::c_int as libc::c_uint {
            ctfgame
                .matchtime = ctfgame.matchtime
                - (*matchtime).value * 60 as libc::c_int as libc::c_float
                + ((*settings).matchlen * 60 as libc::c_int) as libc::c_float;
        }
        sprintf(
            st.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*settings).matchlen,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"matchtime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).matchsetuplen as libc::c_float != (*matchsetuptime).value {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s changed the match setup time to %d minutes.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            (*settings).matchsetuplen,
        );
        if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint
        {
            ctfgame
                .matchtime = ctfgame.matchtime
                - (*matchsetuptime).value * 60 as libc::c_int as libc::c_float
                + ((*settings).matchsetuplen * 60 as libc::c_int) as libc::c_float;
        }
        sprintf(
            st.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*settings).matchsetuplen,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"matchsetuptime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).matchstartlen as libc::c_float != (*matchstarttime).value {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s changed the match start time to %d seconds.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            (*settings).matchstartlen,
        );
        if ctfgame.match_0 as libc::c_uint
            == MATCH_PREGAME as libc::c_int as libc::c_uint
        {
            ctfgame
                .matchtime = ctfgame.matchtime - (*matchstarttime).value
                + (*settings).matchstartlen as libc::c_float;
        }
        sprintf(
            st.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*settings).matchstartlen,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"matchstarttime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).weaponsstay as libc::c_uint
        != ((*dmflags).value as libc::c_int & 0x4 as libc::c_int != 0) as libc::c_int
            as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s turned %s weapons stay.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            if (*settings).weaponsstay as libc::c_uint != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        i = (*dmflags).value as libc::c_int;
        if (*settings).weaponsstay as u64 != 0 {
            i |= 0x4 as libc::c_int;
        } else {
            i &= !(0x4 as libc::c_int);
        }
        sprintf(st.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, i);
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).instantitems as libc::c_uint
        != ((*dmflags).value as libc::c_int & 0x10 as libc::c_int != 0) as libc::c_int
            as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s turned %s instant items.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            if (*settings).instantitems as libc::c_uint != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        i = (*dmflags).value as libc::c_int;
        if (*settings).instantitems as u64 != 0 {
            i |= 0x10 as libc::c_int;
        } else {
            i &= !(0x10 as libc::c_int);
        }
        sprintf(st.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, i);
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).quaddrop as libc::c_uint
        != ((*dmflags).value as libc::c_int & 0x4000 as libc::c_int != 0) as libc::c_int
            as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s turned %s quad drop.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            if (*settings).quaddrop as libc::c_uint != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        i = (*dmflags).value as libc::c_int;
        if (*settings).quaddrop as u64 != 0 {
            i |= 0x4000 as libc::c_int;
        } else {
            i &= !(0x4000 as libc::c_int);
        }
        sprintf(st.as_mut_ptr(), b"%d\0" as *const u8 as *const libc::c_char, i);
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"dmflags\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).instantweap as libc::c_uint
        != ((*instantweap).value as libc::c_int != 0) as libc::c_int as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s turned %s instant weapons.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            if (*settings).instantweap as libc::c_uint != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        sprintf(
            st.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*settings).instantweap as libc::c_int,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"instantweap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    if (*settings).matchlock as libc::c_uint
        != ((*matchlock).value as libc::c_int != 0) as libc::c_int as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s turned %s match lock.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            if (*settings).matchlock as libc::c_uint != 0 {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        sprintf(
            st.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*settings).matchlock as libc::c_int,
        );
        (gi.cvar_set)
            .expect(
                "non-null function pointer",
            )(
            b"matchlock\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            st.as_mut_ptr(),
        );
    }
    PMenu_Close(ent);
    CTFOpenAdminMenu(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_SettingsCancel(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    PMenu_Close(ent);
    CTFOpenAdminMenu(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeMatchLen(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings).matchlen = (*settings).matchlen % 60 as libc::c_int + 5 as libc::c_int;
    if (*settings).matchlen < 5 as libc::c_int {
        (*settings).matchlen = 5 as libc::c_int;
    }
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeMatchSetupLen(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .matchsetuplen = (*settings).matchsetuplen % 60 as libc::c_int
        + 5 as libc::c_int;
    if (*settings).matchsetuplen < 5 as libc::c_int {
        (*settings).matchsetuplen = 5 as libc::c_int;
    }
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeMatchStartLen(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .matchstartlen = (*settings).matchstartlen % 600 as libc::c_int
        + 10 as libc::c_int;
    if (*settings).matchstartlen < 20 as libc::c_int {
        (*settings).matchstartlen = 20 as libc::c_int;
    }
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeWeapStay(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .weaponsstay = ((*settings).weaponsstay as u64 == 0) as libc::c_int as qboolean;
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeInstantItems(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .instantitems = ((*settings).instantitems as u64 == 0) as libc::c_int
        as qboolean;
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeQuadDrop(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings).quaddrop = ((*settings).quaddrop as u64 == 0) as libc::c_int as qboolean;
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeInstantWeap(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .instantweap = ((*settings).instantweap as u64 == 0) as libc::c_int as qboolean;
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_ChangeMatchLock(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = (*p).arg as *mut admin_settings_t;
    (*settings)
        .matchlock = ((*settings).matchlock as u64 == 0) as libc::c_int as qboolean;
    CTFAdmin_UpdateSettings(ent, p);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_UpdateSettings(
    mut ent: *mut edict_t,
    mut setmenu: *mut pmenuhnd_t,
) {
    let mut i: libc::c_int = 2 as libc::c_int;
    let mut text: [libc::c_char; 64] = [0; 64];
    let mut settings: *mut admin_settings_t = (*setmenu).arg as *mut admin_settings_t;
    sprintf(
        text.as_mut_ptr(),
        b"Match Len:       %2d mins\0" as *const u8 as *const libc::c_char,
        (*settings).matchlen,
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeMatchLen
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Match Setup Len: %2d mins\0" as *const u8 as *const libc::c_char,
        (*settings).matchsetuplen,
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeMatchSetupLen
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Match Start Len: %2d secs\0" as *const u8 as *const libc::c_char,
        (*settings).matchstartlen,
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeMatchStartLen
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Weapons Stay:    %s\0" as *const u8 as *const libc::c_char,
        if (*settings).weaponsstay as libc::c_uint != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeWeapStay
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Instant Items:   %s\0" as *const u8 as *const libc::c_char,
        if (*settings).instantitems as libc::c_uint != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeInstantItems
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Quad Drop:       %s\0" as *const u8 as *const libc::c_char,
        if (*settings).quaddrop as libc::c_uint != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeQuadDrop
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Instant Weapons: %s\0" as *const u8 as *const libc::c_char,
        if (*settings).instantweap as libc::c_uint != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeInstantWeap
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    sprintf(
        text.as_mut_ptr(),
        b"Match Lock:      %s\0" as *const u8 as *const libc::c_char,
        if (*settings).matchlock as libc::c_uint != 0 {
            b"Yes\0" as *const u8 as *const libc::c_char
        } else {
            b"No\0" as *const u8 as *const libc::c_char
        },
    );
    PMenu_UpdateEntry(
        ((*setmenu).entries).offset(i as isize),
        text.as_mut_ptr(),
        PMENU_ALIGN_LEFT as libc::c_int,
        Some(
            CTFAdmin_ChangeMatchLock
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        ),
    );
    i += 1;
    PMenu_Update(ent);
}
#[no_mangle]
pub static mut def_setmenu: [pmenu_t; 13] = unsafe {
    [
        {
            let mut init = pmenu_s {
                text: b"*Settings Menu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Apply\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFAdmin_SettingsApply
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Cancel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFAdmin_SettingsCancel
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_Settings(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    let mut settings: *mut admin_settings_t = 0 as *mut admin_settings_t;
    let mut menu: *mut pmenuhnd_t = 0 as *mut pmenuhnd_t;
    PMenu_Close(ent);
    settings = malloc(::std::mem::size_of::<admin_settings_t>() as libc::c_ulong)
        as *mut admin_settings_t;
    (*settings).matchlen = (*matchtime).value as libc::c_int;
    (*settings).matchsetuplen = (*matchsetuptime).value as libc::c_int;
    (*settings).matchstartlen = (*matchstarttime).value as libc::c_int;
    (*settings)
        .weaponsstay = ((*dmflags).value as libc::c_int & 0x4 as libc::c_int != 0)
        as libc::c_int as qboolean;
    (*settings)
        .instantitems = ((*dmflags).value as libc::c_int & 0x10 as libc::c_int != 0)
        as libc::c_int as qboolean;
    (*settings)
        .quaddrop = ((*dmflags).value as libc::c_int & 0x4000 as libc::c_int != 0)
        as libc::c_int as qboolean;
    (*settings)
        .instantweap = ((*instantweap).value != 0 as libc::c_int as libc::c_float)
        as libc::c_int as qboolean;
    (*settings)
        .matchlock = ((*matchlock).value != 0 as libc::c_int as libc::c_float)
        as libc::c_int as qboolean;
    menu = PMenu_Open(
        ent,
        def_setmenu.as_mut_ptr(),
        -(1 as libc::c_int),
        (::std::mem::size_of::<[pmenu_t; 13]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pmenu_t>() as libc::c_ulong)
            as libc::c_int,
        settings as *mut libc::c_void,
    );
    CTFAdmin_UpdateSettings(ent, menu);
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_MatchSet(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    PMenu_Close(ent);
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"Match has been forced to start.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        ctfgame.match_0 = MATCH_PREGAME;
        ctfgame.matchtime = level.time + (*matchstarttime).value;
    } else if ctfgame.match_0 as libc::c_uint
        == MATCH_GAME as libc::c_int as libc::c_uint
    {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            3 as libc::c_int,
            b"Match has been forced to terminate.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        ctfgame.match_0 = MATCH_SETUP;
        ctfgame
            .matchtime = level.time
            + (*matchsetuptime).value * 60 as libc::c_int as libc::c_float;
        CTFResetAllPlayers();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_MatchMode(
    mut ent: *mut edict_t,
    mut p: *mut pmenuhnd_t,
) {
    PMenu_Close(ent);
    if ctfgame.match_0 as libc::c_uint != MATCH_SETUP as libc::c_int as libc::c_uint {
        if (*competition).value < 3 as libc::c_int as libc::c_float {
            (gi.cvar_set)
                .expect(
                    "non-null function pointer",
                )(
                b"competition\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                b"2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        ctfgame.match_0 = MATCH_SETUP;
        CTFResetAllPlayers();
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin_Cancel(mut ent: *mut edict_t, mut p: *mut pmenuhnd_t) {
    PMenu_Close(ent);
}
#[no_mangle]
pub static mut adminmenu: [pmenu_t; 7] = unsafe {
    [
        {
            let mut init = pmenu_s {
                text: b"*Administration Menu\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Settings\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFAdmin_Settings
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: None,
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: b"Cancel\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                align: PMENU_ALIGN_LEFT as libc::c_int,
                SelectFunc: Some(
                    CTFAdmin_Cancel
                        as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
                ),
            };
            init
        },
        {
            let mut init = pmenu_s {
                text: 0 as *const libc::c_char as *mut libc::c_char,
                align: PMENU_ALIGN_CENTER as libc::c_int,
                SelectFunc: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn CTFOpenAdminMenu(mut ent: *mut edict_t) {
    adminmenu[3 as libc::c_int as usize].text = 0 as *mut libc::c_char;
    adminmenu[3 as libc::c_int as usize].SelectFunc = None;
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        adminmenu[3 as libc::c_int as usize]
            .text = b"Force start match\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        adminmenu[3 as libc::c_int as usize]
            .SelectFunc = Some(
            CTFAdmin_MatchSet
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
    } else if ctfgame.match_0 as libc::c_uint
        == MATCH_GAME as libc::c_int as libc::c_uint
    {
        adminmenu[3 as libc::c_int as usize]
            .text = b"Cancel match\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        adminmenu[3 as libc::c_int as usize]
            .SelectFunc = Some(
            CTFAdmin_MatchSet
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
    } else if ctfgame.match_0 as libc::c_uint
        == MATCH_NONE as libc::c_int as libc::c_uint && (*competition).value != 0.
    {
        adminmenu[3 as libc::c_int as usize]
            .text = b"Switch to match mode\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        adminmenu[3 as libc::c_int as usize]
            .SelectFunc = Some(
            CTFAdmin_MatchMode
                as unsafe extern "C" fn(*mut edict_t, *mut pmenuhnd_t) -> (),
        );
    }
    PMenu_Open(
        ent,
        adminmenu.as_mut_ptr(),
        -(1 as libc::c_int),
        (::std::mem::size_of::<[pmenu_t; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<pmenu_t>() as libc::c_ulong)
            as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFAdmin(mut ent: *mut edict_t) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    if (gi.argc).expect("non-null function pointer")() > 1 as libc::c_int
        && !((*admin_password).string).is_null()
        && *(*admin_password).string as libc::c_int != 0
        && (*(*ent).client).resp.admin as u64 == 0
        && strcmp(
            (*admin_password).string,
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
        ) == 0 as libc::c_int
    {
        (*(*ent).client).resp.admin = true_0;
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s has become an admin.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Type 'admin' to access the adminstration menu.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (*(*ent).client).resp.admin as u64 == 0 {
        sprintf(
            text.as_mut_ptr(),
            b"%s has requested admin rights.\0" as *const u8 as *const libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
        );
        CTFBeginElection(ent, ELECT_ADMIN, text.as_mut_ptr());
        return;
    }
    if !((*(*ent).client).menu).is_null() {
        PMenu_Close(ent);
    }
    CTFOpenAdminMenu(ent);
}
#[no_mangle]
pub unsafe extern "C" fn CTFStats(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    let mut g: *mut ghost_t = 0 as *mut ghost_t;
    let mut st: [libc::c_char; 80] = [0; 80];
    let mut text: [libc::c_char; 1400] = [0; 1400];
    let mut e2: *mut edict_t = 0 as *mut edict_t;
    *text.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int;
        while i as libc::c_float <= (*maxclients).value {
            e2 = g_edicts.offset(i as isize);
            if !((*e2).inuse as u64 == 0) {
                if (*(*e2).client).resp.ready as u64 == 0
                    && (*(*e2).client).resp.ctf_team != CTF_NOTEAM as libc::c_int
                {
                    sprintf(
                        st.as_mut_ptr(),
                        b"%s is not ready.\n\0" as *const u8 as *const libc::c_char,
                        ((*(*e2).client).pers.netname).as_mut_ptr(),
                    );
                    if (strlen(text.as_mut_ptr())).wrapping_add(strlen(st.as_mut_ptr()))
                        < (::std::mem::size_of::<[libc::c_char; 1400]>()
                            as libc::c_ulong)
                            .wrapping_sub(50 as libc::c_int as libc::c_ulong)
                    {
                        strcat(text.as_mut_ptr(), st.as_mut_ptr());
                    }
                }
            }
            i += 1;
        }
    }
    i = 0 as libc::c_int;
    g = (ctfgame.ghosts).as_mut_ptr();
    while i < 256 as libc::c_int {
        if !((*g).ent).is_null() {
            break;
        }
        i += 1;
        g = g.offset(1);
    }
    if i == 256 as libc::c_int {
        if *text.as_mut_ptr() != 0 {
            (gi.cprintf)
                .expect(
                    "non-null function pointer",
                )(
                ent,
                2 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                text.as_mut_ptr(),
            );
        }
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"No statistics available.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    strcat(
        text.as_mut_ptr(),
        b"  #|Name            |Score|Kills|Death|BasDf|CarDf|Effcy|\n\0" as *const u8
            as *const libc::c_char,
    );
    i = 0 as libc::c_int;
    g = (ctfgame.ghosts).as_mut_ptr();
    while i < 256 as libc::c_int {
        if !(*((*g).netname).as_mut_ptr() == 0) {
            if (*g).deaths + (*g).kills == 0 as libc::c_int {
                e = 50 as libc::c_int;
            } else {
                e = (*g).kills * 100 as libc::c_int / ((*g).kills + (*g).deaths);
            }
            sprintf(
                st.as_mut_ptr(),
                b"%3d|%-16.16s|%5d|%5d|%5d|%5d|%5d|%4d%%|\n\0" as *const u8
                    as *const libc::c_char,
                (*g).number,
                ((*g).netname).as_mut_ptr(),
                (*g).score,
                (*g).kills,
                (*g).deaths,
                (*g).basedef,
                (*g).carrierdef,
                e,
            );
            if (strlen(text.as_mut_ptr())).wrapping_add(strlen(st.as_mut_ptr()))
                > (::std::mem::size_of::<[libc::c_char; 1400]>() as libc::c_ulong)
                    .wrapping_sub(50 as libc::c_int as libc::c_ulong)
            {
                sprintf(
                    text.as_mut_ptr().offset(strlen(text.as_mut_ptr()) as isize),
                    b"And more...\n\0" as *const u8 as *const libc::c_char,
                );
                (gi.cprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    2 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    text.as_mut_ptr(),
                );
                return;
            }
            strcat(text.as_mut_ptr(), st.as_mut_ptr());
        }
        i += 1;
        g = g.offset(1);
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFPlayerList(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut st: [libc::c_char; 80] = [0; 80];
    let mut text: [libc::c_char; 1400] = [0; 1400];
    let mut e2: *mut edict_t = 0 as *mut edict_t;
    *text.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    if ctfgame.match_0 as libc::c_uint == MATCH_SETUP as libc::c_int as libc::c_uint {
        i = 1 as libc::c_int;
        while i as libc::c_float <= (*maxclients).value {
            e2 = g_edicts.offset(i as isize);
            if !((*e2).inuse as u64 == 0) {
                if (*(*e2).client).resp.ready as u64 == 0
                    && (*(*e2).client).resp.ctf_team != CTF_NOTEAM as libc::c_int
                {
                    sprintf(
                        st.as_mut_ptr(),
                        b"%s is not ready.\n\0" as *const u8 as *const libc::c_char,
                        ((*(*e2).client).pers.netname).as_mut_ptr(),
                    );
                    if (strlen(text.as_mut_ptr())).wrapping_add(strlen(st.as_mut_ptr()))
                        < (::std::mem::size_of::<[libc::c_char; 1400]>()
                            as libc::c_ulong)
                            .wrapping_sub(50 as libc::c_int as libc::c_ulong)
                    {
                        strcat(text.as_mut_ptr(), st.as_mut_ptr());
                    }
                }
            }
            i += 1;
        }
    }
    *text.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    e2 = g_edicts.offset(1 as libc::c_int as isize);
    while (i as libc::c_float) < (*maxclients).value {
        if !((*e2).inuse as u64 == 0) {
            sprintf(
                st.as_mut_ptr(),
                b"%3d %-16.16s %02d:%02d %4d %3d%s%s\n\0" as *const u8
                    as *const libc::c_char,
                i + 1 as libc::c_int,
                ((*(*e2).client).pers.netname).as_mut_ptr(),
                (level.framenum - (*(*e2).client).resp.enterframe) / 600 as libc::c_int,
                (level.framenum - (*(*e2).client).resp.enterframe) % 600 as libc::c_int
                    / 10 as libc::c_int,
                (*(*e2).client).ping,
                (*(*e2).client).resp.score,
                if ctfgame.match_0 as libc::c_uint
                    == MATCH_SETUP as libc::c_int as libc::c_uint
                    || ctfgame.match_0 as libc::c_uint
                        == MATCH_PREGAME as libc::c_int as libc::c_uint
                {
                    if (*(*e2).client).resp.ready as libc::c_uint != 0 {
                        b" (ready)\0" as *const u8 as *const libc::c_char
                    } else {
                        b" (notready)\0" as *const u8 as *const libc::c_char
                    }
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                if (*(*e2).client).resp.admin as libc::c_uint != 0 {
                    b" (admin)\0" as *const u8 as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
            );
            if (strlen(text.as_mut_ptr())).wrapping_add(strlen(st.as_mut_ptr()))
                > (::std::mem::size_of::<[libc::c_char; 1400]>() as libc::c_ulong)
                    .wrapping_sub(50 as libc::c_int as libc::c_ulong)
            {
                sprintf(
                    text.as_mut_ptr().offset(strlen(text.as_mut_ptr()) as isize),
                    b"And more...\n\0" as *const u8 as *const libc::c_char,
                );
                (gi.cprintf)
                    .expect(
                        "non-null function pointer",
                    )(
                    ent,
                    2 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    text.as_mut_ptr(),
                );
                return;
            }
            strcat(text.as_mut_ptr(), st.as_mut_ptr());
        }
        i += 1;
        e2 = e2.offset(1);
    }
    (gi.cprintf)
        .expect(
            "non-null function pointer",
        )(
        ent,
        2 as libc::c_int,
        b"%s\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        text.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn CTFWarp(mut ent: *mut edict_t) {
    let mut text: [libc::c_char; 1024] = [0; 1024];
    let mut mlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut seps: *const libc::c_char = b" \t\n\r\0" as *const u8
        as *const libc::c_char;
    if (gi.argc).expect("non-null function pointer")() < 2 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Where do you want to warp to?\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Available levels are: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*warp_list).string,
        );
        return;
    }
    mlist = strdup((*warp_list).string);
    token = strtok(mlist, seps);
    while !token.is_null() {
        if Q_stricmp(
            token,
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
        ) == 0 as libc::c_int
        {
            break;
        }
        token = strtok(0 as *mut libc::c_char, seps);
    }
    if token.is_null() {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Unknown CTF level.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Available levels are: %s\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            (*warp_list).string,
        );
        free(mlist as *mut libc::c_void);
        return;
    }
    free(mlist as *mut libc::c_void);
    if (*(*ent).client).resp.admin as u64 != 0 {
        (gi.bprintf)
            .expect(
                "non-null function pointer",
            )(
            2 as libc::c_int,
            b"%s is warping to level %s.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ((*(*ent).client).pers.netname).as_mut_ptr(),
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
        );
        strncpy(
            (level.forcemap).as_mut_ptr(),
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
            (::std::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        EndDMLevel();
        return;
    }
    sprintf(
        text.as_mut_ptr(),
        b"%s has requested warping to level %s.\0" as *const u8 as *const libc::c_char,
        ((*(*ent).client).pers.netname).as_mut_ptr(),
        (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
    );
    if CTFBeginElection(ent, ELECT_MAP, text.as_mut_ptr()) as u64 != 0 {
        strncpy(
            (ctfgame.elevel).as_mut_ptr(),
            (gi.argv).expect("non-null function pointer")(1 as libc::c_int),
            (::std::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn CTFBoot(mut ent: *mut edict_t) {
    let mut i: libc::c_int = 0;
    let mut targ: *mut edict_t = 0 as *mut edict_t;
    let mut text: [libc::c_char; 80] = [0; 80];
    if (*(*ent).client).resp.admin as u64 == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"You are not an admin.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (gi.argc).expect("non-null function pointer")() < 2 as libc::c_int {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Who do you want to kick?\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    if (*(gi.argv).expect("non-null function pointer")(1 as libc::c_int) as libc::c_int)
        < '0' as i32
        && *(gi.argv).expect("non-null function pointer")(1 as libc::c_int)
            as libc::c_int > '9' as i32
    {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Specify the player number to kick.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    i = atoi((gi.argv).expect("non-null function pointer")(1 as libc::c_int));
    if i < 1 as libc::c_int || i as libc::c_float > (*maxclients).value {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"Invalid player number.\n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        return;
    }
    targ = g_edicts.offset(i as isize);
    if (*targ).inuse as u64 == 0 {
        (gi.cprintf)
            .expect(
                "non-null function pointer",
            )(
            ent,
            2 as libc::c_int,
            b"That player number is not connected.\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
        );
        return;
    }
    sprintf(
        text.as_mut_ptr(),
        b"kick %d\n\0" as *const u8 as *const libc::c_char,
        i - 1 as libc::c_int,
    );
    (gi.AddCommandString).expect("non-null function pointer")(text.as_mut_ptr());
}
