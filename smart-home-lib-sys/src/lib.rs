use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use smart_home_lib::{Device, Home, Room, SmartDevice, SmartHub, SmartSocket, SmartThermometer};

#[repr(C)]
pub struct Handle {
    hub: SmartHub,
    rt: tokio::runtime::Runtime,
}

#[repr(C)]
pub enum ReturnCode {
    Success = 0,
    Fail = -1,
}

#[no_mangle]
/// get new smart hub
pub extern "C" fn smart_home_new() -> *mut Handle {
    let handle = Handle {
        hub: SmartHub::new(),
        rt: tokio::runtime::Runtime::new().unwrap(),
    };

    let handle = Box::new(handle);
    Box::into_raw(handle)
}

#[no_mangle]
/// free handle
/// # Safety
///
/// It's ok, because memory allocated from smart_home_new()
pub unsafe extern "C" fn smart_home_free(hub: *mut Handle) {
    Box::from_raw(hub);
}

#[no_mangle]
/// add a new home to SmartHub
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// * `handle`: smart hub handle
/// * `name`: home name to add
pub unsafe extern "C" fn smart_home_add_home(
    handle: *mut Handle,
    name: *const c_char,
) -> *mut Home {
    let handle = &mut *handle;
    let home_name = CStr::from_ptr(name).to_str().unwrap();

    match handle.hub.add_home(Home::new(home_name)) {
        Ok(home) => home as *mut Home,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// remove home from SmartHub
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// * `handle`: smart hub handle
/// * `name`: home name to del
pub unsafe extern "C" fn smart_home_del_home(
    handle: *mut Handle,
    name: *const c_char,
) -> ReturnCode {
    let handle = &mut *handle;
    let home_name = CStr::from_ptr(name).to_str().unwrap();

    match handle.hub.del_home(home_name) {
        Some(_) => ReturnCode::Success,
        None => ReturnCode::Fail,
    }
}

#[no_mangle]
/// Get Home handle from SmartHub by name
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// * `handle`: smart hub handle
/// * `name`: home name to get
pub unsafe extern "C" fn smart_home_get_home(
    handle: *mut Handle,
    name: *const c_char,
) -> *mut Home {
    let handle = &mut *handle;
    let home_name = CStr::from_ptr(name).to_str().unwrap();

    match handle.hub.get_home_mut(home_name) {
        Some(home) => home as *mut Home,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Get list of all home names in hub
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// * `handle`: smart hub handle
pub unsafe extern "C" fn smart_home_get_home_list(handle: *mut Handle) -> *const *mut c_char {
    let handle = &mut *handle;

    let mut home_list: Vec<*mut c_char> = handle
        .hub
        .iter()
        .map(|home| CString::new(home.name()).unwrap().into_raw())
        .collect();

    home_list.push(std::ptr::null_mut());

    let ptr = home_list.as_ptr();

    std::mem::forget(home_list);

    ptr
}

#[no_mangle]
/// Adds a new room to home
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
/// * `name`: room name to add
pub unsafe extern "C" fn smart_home_add_room(handle: *mut Home, name: *const c_char) -> *mut Room {
    let home = &mut *handle;
    let room_name = CStr::from_ptr(name).to_str().unwrap();

    match home.add_room(Room::new(room_name)) {
        Ok(room) => room as *mut Room,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Dels a room from home
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
/// * `name`: room name to del
pub unsafe extern "C" fn smart_home_del_room(handle: *mut Home, name: *const c_char) -> ReturnCode {
    let home = &mut *handle;
    let room_name = CStr::from_ptr(name).to_str().unwrap();

    match home.del_room(room_name) {
        Some(_) => ReturnCode::Success,
        None => ReturnCode::Fail,
    }
}

#[no_mangle]
/// Gets a room from home by name
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
/// * `name`: room name to get
pub unsafe extern "C" fn smart_home_get_room(handle: *mut Home, name: *const c_char) -> *mut Room {
    let home = &mut *handle;
    let room_name = CStr::from_ptr(name).to_str().unwrap();

    match home.room_mut(room_name) {
        Some(room) => room as *mut Room,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Gets a room list from home by name
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
pub unsafe extern "C" fn smart_home_get_room_list(handle: *mut Home) -> *const *mut c_char {
    let home = &mut *handle;

    let mut room_list: Vec<*mut c_char> = home
        .room_iter_mut()
        .map(|room| CString::new(room.name()).unwrap().into_raw())
        .collect();

    room_list.push(std::ptr::null_mut());

    let ptr = room_list.as_ptr();

    std::mem::forget(room_list);

    ptr
}

#[no_mangle]
/// Adds a new thermometer to room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
/// * `name`: thermometer name to add
/// * `description`: thermometer description to add
/// * `server`: thermometer server to add
pub unsafe extern "C" fn smart_home_add_thermometer(
    handle: *mut Handle,
    room: *mut Room,
    name: *const c_char,
    description: *const c_char,
    server: *const c_char,
) -> *mut SmartThermometer {
    let handle = &mut *handle;
    let room = &mut *room;
    let device_name = CStr::from_ptr(name).to_str().unwrap();
    let device_description = CStr::from_ptr(description).to_str().unwrap();
    let device_server = CStr::from_ptr(server).to_str().unwrap();

    let thermometer = handle.rt.block_on(async {
        SmartThermometer::new(device_name, device_description, device_server).await
    });

    match room.add_device(thermometer) {
        Ok(thermometer) => match thermometer {
            Device::Thermometer(t) => t as *mut SmartThermometer,
            _ => unreachable!(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Adds a new socket to room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
/// * `name`: socket name to add
/// * `description`: socket description to add
/// * `server`: socket server to add
pub unsafe extern "C" fn smart_home_add_socket(
    handle: *mut Handle,
    room: *mut Room,
    name: *const c_char,
    description: *const c_char,
    server: *const c_char,
) -> *mut SmartSocket {
    let handle = &mut *handle;
    let room = &mut *room;
    let device_name = CStr::from_ptr(name).to_str().unwrap();
    let device_description = CStr::from_ptr(description).to_str().unwrap();
    let device_server = CStr::from_ptr(server).to_str().unwrap();

    let thermometer = handle
        .rt
        .block_on(async { SmartSocket::new(device_name, device_description, device_server).await });

    match room.add_device(thermometer) {
        Ok(device) => match device {
            Device::Socket(s) => s as *mut SmartSocket,
            _ => unreachable!(),
        },
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Removes a device from room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
/// * `name`: smart device name to del
pub unsafe extern "C" fn smart_home_del_device(
    handle: *mut Room,
    name: *const c_char,
) -> ReturnCode {
    let room = &mut *handle;
    let device_name = CStr::from_ptr(name).to_str().unwrap();

    match room.del_device(device_name) {
        Some(_) => ReturnCode::Success,
        None => ReturnCode::Fail,
    }
}

#[no_mangle]
/// Get a device by name
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
/// * `name`: smart device name to del
pub unsafe extern "C" fn smart_home_get_device(
    handle: *mut Room,
    name: *const c_char,
) -> *mut Device {
    let room = &mut *handle;
    let device_name = CStr::from_ptr(name).to_str().unwrap();

    match room.device_mut(device_name) {
        Some(device) => device as *mut Device,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Gets all thermometers in room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_list(
    handle: *mut Room,
) -> *mut *mut SmartThermometer {
    let room = &mut *handle;
    let mut list: Vec<*mut SmartThermometer> = room
        .device_iter_mut()
        .filter_map(|t| match t {
            Device::Thermometer(x) => Some(x),
            _ => None,
        })
        .map(|t| t as *mut SmartThermometer)
        .collect();

    list.push(std::ptr::null_mut());

    let ptr = list.as_mut_ptr();

    std::mem::forget(list);

    ptr
}

#[no_mangle]
/// Gets all sockets in room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_list(handle: *mut Room) -> *mut *mut SmartSocket {
    let room = &mut *handle;
    let mut list: Vec<*mut SmartSocket> = room
        .device_iter_mut()
        .filter_map(|t| match t {
            Device::Socket(x) => Some(x),
            _ => None,
        })
        .map(|t| t as *mut SmartSocket)
        .collect();

    list.push(std::ptr::null_mut());

    let ptr = list.as_mut_ptr();

    std::mem::forget(list);

    ptr
}

#[no_mangle]
/// Get thermometer name
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_name(
    handle: *mut SmartThermometer,
) -> *const c_char {
    let device = &mut *handle;

    let str = CString::new(device.name()).unwrap();

    let ptr = str.as_ptr();

    std::mem::forget(str);

    ptr
}

#[no_mangle]
/// Get thermometer description
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_description(
    handle: *mut SmartThermometer,
) -> *const c_char {
    let device = &mut *handle;

    let str = CString::new(device.description()).unwrap();

    let ptr = str.as_ptr();

    std::mem::forget(str);

    ptr
}

#[no_mangle]
/// Get thermometer temperature
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_temperature(
    handle: *mut Handle,
    device: *mut SmartThermometer,
) -> f64 {
    let handle = &mut *handle;
    let device = &mut *device;

    handle
        .rt
        .block_on(async { device.current_temperature().await.unwrap_or_default() })
}

#[no_mangle]

/// Get socket name
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_name(handle: *mut SmartSocket) -> *const c_char {
    let device = &mut *handle;

    let str = CString::new(device.name()).unwrap();

    let ptr = str.as_ptr();

    std::mem::forget(str);

    ptr
}

#[no_mangle]
/// Get socket description
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_description(
    handle: *mut SmartSocket,
) -> *const c_char {
    let device = &mut *handle;

    let str = CString::new(device.description()).unwrap();

    let ptr = str.as_ptr();

    std::mem::forget(str);

    ptr
}

#[no_mangle]
/// Get socket power
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_power(
    handle: *mut Handle,
    device: *mut SmartSocket,
) -> f64 {
    let handle = &mut *handle;
    let device = &mut *device;

    handle
        .rt
        .block_on(async { device.current_power().await.unwrap_or_default() })
}

#[no_mangle]
/// Turn on socket
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_socket_on(
    handle: *mut Handle,
    device: *mut SmartSocket,
) -> ReturnCode {
    let handle = &mut *handle;
    let device = &mut *device;

    handle.rt.block_on(async {
        match device.on().await {
            Ok(()) => ReturnCode::Success,
            Err(_) => ReturnCode::Fail,
        }
    })
}

#[no_mangle]
/// Turn off socket
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_socket_off(
    handle: *mut Handle,
    device: *mut SmartSocket,
) -> ReturnCode {
    let handle = &mut *handle;
    let device = &mut *device;

    handle.rt.block_on(async {
        match device.off().await {
            Ok(()) => ReturnCode::Success,
            Err(_) => ReturnCode::Fail,
        }
    })
}
