extern crate kdri;
#[macro_use] extern crate enum_primitive;

use std::cmp::min;
use std::panic;
use std::os::raw::*;
use std::borrow::BorrowMut;
use enum_primitive::FromPrimitive;

fn carray_from_string(name: String, dst_array: *mut c_uchar, max_length: usize) {
    if max_length < 1 { return }
    let cname = unsafe{std::slice::from_raw_parts_mut(dst_array, max_length)};

    let new_length = min(name.len(), max_length - 1);
    for i in 0..new_length {
        cname[i] = name.as_bytes()[i];
    }
    cname[new_length] = 0;
}

macro_rules! kdri_kettler_enum_conversion {
    ($kdri:ident, $kettler:ident) => {
        #[allow(dead_code)]
        impl $kdri {
            fn to_kettler_enum(self) -> kdri::$kettler { FromPrimitive::from_u32(self as u32).unwrap()  }
            fn from_kettler_enum(m: kdri::$kettler) -> $kdri { FromPrimitive::from_u32(m as u32).unwrap()  }
        }
    }
}

pub type KdriAddr = kdri::BtAddr;

enum_from_primitive! {
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KdriDeviceType {
	Bike = 1,
	Crosstrainer = 2,
	Racer = 3,
	Rowing = 4,
	Treadmill = 5,
}
}


enum_from_primitive! {
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KdriDeviceState {
	Up = 0,
	Down = 1,
}
}

enum_from_primitive! {
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KdriBrakeMode {
	ConstantPower = 0,
	ConstantBrake = 1,
}
}

enum_from_primitive! {
#[repr(C)]
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum KdriPowerRange {
	Below = 0,
	In = 1,
	Above = 2,
}
}

kdri_kettler_enum_conversion!(KdriDeviceType, KettlerDeviceType);
kdri_kettler_enum_conversion!(KdriDeviceState, KettlerDeviceState);
kdri_kettler_enum_conversion!(KdriBrakeMode, KettlerBrakeMode);
kdri_kettler_enum_conversion!(KdriPowerRange, KettlerPowerRange);

pub struct KdriHandle {
}

#[allow(dead_code)]
pub struct KdriConnection {
    connection: kdri::KettlerConnection,
    handle: *mut KdriHandle,
}

impl KdriConnection {
    fn new(handle: *mut KdriHandle, connection: kdri::KettlerConnection) -> KdriConnection {
        KdriConnection {
            connection: connection,
            handle: handle,
        }
    }
}

#[repr(C)]
pub enum KdriReturn {
    Ok = 0,
    Failed = 1,
    NotInitialized = 2,
}

#[repr(C)]
pub struct KdriDevice {
    name: [c_uchar; 256],
    addr: KdriAddr,
}

impl KdriDevice {
    fn from_kettler_device(device: &kdri::KettlerDevice) -> KdriDevice {
        let mut new_dev = KdriDevice {
            name: [0 as c_uchar; 256],
            addr: device.get_addr(),
        };

        carray_from_string(device.get_name(), &mut new_dev.name[0] as *mut u8, 256);
        new_dev
    }

    // fn to_kettler_device(&self) -> kdri::KettlerDevice {
    //     kdri::KettlerDevice::new(unsafe{std::ffi::CStr::from_ptr(&self.name as *const u8 as *const i8)}.to_string_lossy().into_owned(), self.addr)
    // }
}

#[no_mangle]
pub extern fn kdri_create_handle() -> *mut KdriHandle {
    0 as *mut KdriHandle
}

#[no_mangle]
pub extern fn kdri_scan_devices(_: *mut KdriHandle, dst_device_array: *mut KdriDevice, max_array_length: u32) -> i32 {
    match panic::catch_unwind(|| {
        match kdri::scan_devices() {
            Ok(devices) => {
                let num_devices = std::cmp::min(max_array_length, devices.len() as u32);
                let device_slice = unsafe { std::slice::from_raw_parts_mut(dst_device_array, max_array_length as usize) };
                for i in 0..num_devices as usize {
                    device_slice[i] = KdriDevice::from_kettler_device(&devices[i]);
                }
                return num_devices as i32;
            }
            Err(error) => {
                println!("{}", error);
                return -1;
            }
        }
    }) {
        Ok(i) => return i,
        Err(_) => return -1,
    }
}

#[no_mangle]
pub extern fn kdri_connect(handle: *mut KdriHandle, addr: *const KdriAddr) -> *mut KdriConnection {
    match panic::catch_unwind(|| {
        let connection = kdri::KettlerConnection::connect(unsafe{*addr}).expect("connecting failed");
        let kdri_connection = KdriConnection::new(handle, connection);
        unsafe { std::mem::transmute(Box::new(kdri_connection)) }
    }) {
        Ok(r) => r,
        Err(_) => 0 as *mut KdriConnection,
    }
}

#[no_mangle]
pub extern fn kdri_addr_to_str(_: *mut KdriHandle, addr: *const KdriAddr, name: *mut c_char) {
    let name_string: String = unsafe{&*addr}.to_string();
    assert!(name_string.len() >= 17);
    let name_slice = unsafe { std::slice::from_raw_parts_mut(name as *mut u8, 17) };
    for i in 0..17 { name_slice[i] = name_string.as_bytes()[i]; }
}

#[no_mangle]
pub extern fn kdri_connection_close(connection: *mut KdriConnection) -> i32 {
    let mut kdri_connection_box: Box<KdriConnection> = unsafe { std::mem::transmute(connection) };
    let kdri_connection: &mut KdriConnection = kdri_connection_box.borrow_mut();
    match kdri_connection.connection.close() {
        Ok(()) => 0,
        Err(s) => { println!("{}", s); 1 },
    }
}

#[no_mangle]
pub extern fn kdri_destroy_handle(_: *mut KdriHandle) -> i32 {
    0
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// SETTERS
////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! kdri_set {
    ($e:ident, $f:ident, $t:ty) => {
        #[no_mangle]
        pub extern fn $e(connection: *mut KdriConnection, v: $t) -> KdriReturn {
            unsafe{&mut *connection}.connection.$f(v);
            KdriReturn::Ok
        }
    }
}

kdri_set!(kdri_set_speed, set_speed, u16);
kdri_set!(kdri_set_power, set_power, u16);
kdri_set!(kdri_set_incline, set_incline, u16);
kdri_set!(kdri_set_brake_level, set_brake_level, u8);
kdri_set!(kdri_set_update_interval, set_update_interval, u32);

#[no_mangle]
pub extern fn kdri_set_brake_mode(connection: *mut KdriConnection, v: KdriBrakeMode) -> KdriReturn {
    unsafe{&mut *connection}.connection.set_brake_mode(v.to_kettler_enum());
    KdriReturn::Ok
}

#[no_mangle]
pub extern fn kdri_set_online(connection: *mut KdriConnection, v: u8) -> KdriReturn {
    unsafe{&mut *connection}.connection.set_online(v != 0);
    KdriReturn::Ok
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// GETTERS
////////////////////////////////////////////////////////////////////////////////////////////////////
macro_rules! kdri_get {
    ($e:ident, $f:ident, $t:ty) => {
        #[no_mangle]
        pub extern fn $e(connection: *mut KdriConnection, v: *mut $t) -> KdriReturn {
            match unsafe{&mut *connection}.connection.$f() {
                Some(x) => { unsafe{*v = x as $t;}; KdriReturn::Ok },
                None => { KdriReturn::NotInitialized }
            }
        }
    }
}


macro_rules! kdri_get_string {
    ($e:ident, $f:ident) => {
        #[no_mangle]
        pub extern fn $e(connection: *mut KdriConnection, v: *mut c_uchar, max_length: usize) -> KdriReturn {
            match unsafe{&mut *connection}.connection.$f() {
                Some(x) => { carray_from_string(x, v, max_length); KdriReturn::Ok },
                None => { KdriReturn::NotInitialized }
            }
        }
    }
}

macro_rules! kdri_get_enum {
    ($e:ident, $f:ident, $t:ident) => {
        #[no_mangle]
        pub extern fn $e(connection: *mut KdriConnection, v: *mut $t) -> KdriReturn {
            match unsafe{&mut *connection}.connection.$f() {
                Some(x) => { unsafe{*v = $t::from_kettler_enum(x)}; KdriReturn::Ok },
                None => { KdriReturn::NotInitialized }
            }
        }
    }
}

kdri_get!(kdri_get_power_target, get_power_target, u16);
kdri_get!(kdri_get_power, get_power, u16);
kdri_get!(kdri_get_power_min, get_power_min, u16);
kdri_get!(kdri_get_power_max, get_power_max, u16);
kdri_get!(kdri_get_speed_target, get_speed_target, u16);
kdri_get!(kdri_get_speed, get_speed, u16);
kdri_get!(kdri_get_speed_min, get_speed_min, u16);
kdri_get!(kdri_get_speed_max, get_speed_max, u16);
kdri_get!(kdri_get_incline_target, get_incline_target, u16);
kdri_get!(kdri_get_incline, get_incline, u16);
kdri_get!(kdri_get_incline_min, get_incline_min, u16);
kdri_get!(kdri_get_incline_max, get_incline_max, u16);
kdri_get!(kdri_get_brake_level, get_brake_level, u8);
kdri_get!(kdri_get_brake_level_min, get_brake_level_min, u8);
kdri_get!(kdri_get_brake_level_max, get_brake_level_max, u8);
kdri_get!(kdri_get_online, get_online, u8);
kdri_get!(kdri_get_pulse, get_pulse, u16);
kdri_get!(kdri_get_rpm, get_rpm, u16);
kdri_get!(kdri_get_distance, get_distance, u16);
kdri_get!(kdri_get_energy, get_energy, u16);
kdri_get!(kdri_get_time, get_time, u16);
kdri_get!(kdri_get_time_mode, get_time_mode, u16);
kdri_get_string!(kdri_get_device_name, get_device_name);
kdri_get_string!(kdri_get_device_id, get_device_id);
kdri_get_enum!(kdri_get_power_range, get_power_range, KdriPowerRange);
kdri_get_enum!(kdri_get_device_type, get_device_type, KdriDeviceType);
kdri_get_enum!(kdri_get_device_state, get_device_state, KdriDeviceState);
kdri_get_enum!(kdri_get_brake_mode, get_brake_mode, KdriBrakeMode);
