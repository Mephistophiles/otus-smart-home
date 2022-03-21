use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use smart_home_lib::{Device, Home, Room, SmartDevice, SmartHub, SmartSocket, SmartThermometer};

#[repr(C)]
pub struct Handle {
    controller: SmartHub,
    home_name_buffer: Option<CString>,
    room_name_buffer: Option<CString>,
    device_name_buffer: Option<CString>,
    device_desc_buffer: Option<CString>,
    rt: tokio::runtime::Runtime,
}

#[repr(C)]
pub struct HandleIter {
    handle: *mut Handle,
    cursor: usize,
}

#[repr(C)]
pub struct HandleRoomIter {
    handle: *mut Handle,
    home: *mut Home,
    cursor: usize,
}

#[repr(C)]
pub struct HandleDeviceIter {
    handle: *mut Handle,
    room: *mut Room,
    cursor: usize,
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
        controller: SmartHub::new(),
        room_name_buffer: None,
        home_name_buffer: None,
        device_name_buffer: None,
        device_desc_buffer: None,
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

    match handle.controller.add_home(Home::new(home_name)) {
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

    match handle.controller.del_home(home_name) {
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

    match handle.controller.get_home_mut(home_name) {
        Some(home) => home as *mut Home,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
/// Get home count
///
/// # Safety
///
/// handle gets from smart_home_new()
pub unsafe extern "C" fn smart_home_get_home_size(handle: *mut Handle) -> usize {
    let handle = &mut *handle;

    handle.controller.iter().count()
}

#[no_mangle]
/// Get iterator over homes
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// * `handle`: smart hub handle
pub unsafe extern "C" fn smart_home_get_home_iter(handle: *mut Handle) -> HandleIter {
    HandleIter { handle, cursor: 0 }
}

#[no_mangle]
/// Get list of all home names in hub
///
/// # Safety
///
/// handle gets from smart_home_new()
///
/// # Warning
///
/// This function returns borrowed pointer, use copy name before next call
///
/// * `handle`: smart hub handle
pub unsafe extern "C" fn smart_home_get_home_next(iter: *mut HandleIter) -> *const c_char {
    let iter = &mut *iter;
    let handle = &mut *iter.handle;

    let home = match handle.controller.iter().nth(iter.cursor) {
        Some(home) => home,
        None => return std::ptr::null(),
    };

    iter.cursor += 1;

    handle.home_name_buffer = Some(CString::new(home.name()).unwrap());

    handle.home_name_buffer.as_ref().unwrap().as_ptr()
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
/// Gets a room count in room
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
pub unsafe extern "C" fn smart_home_get_room_size(handle: *mut Home) -> usize {
    let home = &mut *handle;

    home.room_iter().count()
}

#[no_mangle]
/// Gets a room iterator
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
pub unsafe extern "C" fn smart_home_get_room_iter(
    handle: *mut Handle,
    home: *mut Home,
) -> HandleRoomIter {
    HandleRoomIter {
        handle,
        home,
        cursor: 0,
    }
}

#[no_mangle]
/// Gets a room list from home
///
/// # Safety
///
/// Home gets from smart_home_get_home()
///
/// * `handle`: home handle
pub unsafe extern "C" fn smart_home_get_room_next(iter: *mut HandleRoomIter) -> *const c_char {
    let iter = &mut *iter;
    let handle = &mut *iter.handle;
    let home = &mut *iter.home;

    let room = match home.room_iter().nth(iter.cursor) {
        Some(room) => room,
        None => return std::ptr::null(),
    };

    iter.cursor += 1;

    handle.room_name_buffer = Some(CString::new(room.name()).unwrap());

    handle.room_name_buffer.as_ref().unwrap().as_ptr()
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
/// Gets all thermometers size
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_size(handle: *const Room) -> usize {
    let room = &*handle;
    room.thermometer_devices().count()
}

#[no_mangle]
/// Gets all thermometers iter
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_thermometer_iter(
    handle: *mut Handle,
    room: *mut Room,
) -> HandleDeviceIter {
    HandleDeviceIter {
        handle,
        room,
        cursor: 0,
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
pub unsafe extern "C" fn smart_home_get_thermometer_next(
    handle: *mut HandleDeviceIter,
) -> *const SmartThermometer {
    let handle = &mut *handle;
    let room = &*handle.room;

    let device = match room.thermometer_devices().nth(handle.cursor) {
        Some(device) => device,
        None => return std::ptr::null(),
    };

    handle.cursor += 1;

    device as *const SmartThermometer
}

#[no_mangle]
/// Gets all sockets size
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_size(handle: *const Room) -> usize {
    let room = &*handle;
    room.socket_devices().count()
}

#[no_mangle]
/// Gets all sockets iter
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_iter(
    handle: *mut Handle,
    room: *mut Room,
) -> HandleDeviceIter {
    HandleDeviceIter {
        handle,
        room,
        cursor: 0,
    }
}

#[no_mangle]
/// Gets all sockets in room
///
/// # Safety
///
/// Room gets from smart_home_get_room()
///
/// * `handle`: room handle
pub unsafe extern "C" fn smart_home_get_socket_next(
    handle: *mut HandleDeviceIter,
) -> *const SmartSocket {
    let handle = &mut *handle;
    let room = &*handle.room;

    let device = match room.socket_devices().nth(handle.cursor) {
        Some(device) => device,
        None => return std::ptr::null(),
    };

    handle.cursor += 1;

    device as *const SmartSocket
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
    handle: *mut Handle,
    device: *const SmartThermometer,
) -> *const c_char {
    let handle = &mut *handle;
    let device = &*device;

    handle.device_name_buffer = Some(CString::new(device.name()).unwrap());

    handle.device_name_buffer.as_ref().unwrap().as_ptr()
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
    handle: *mut Handle,
    device: *const SmartThermometer,
) -> *const c_char {
    let handle = &mut *handle;
    let device = &*device;

    handle.device_desc_buffer = Some(CString::new(device.description()).unwrap());

    handle.device_desc_buffer.as_ref().unwrap().as_ptr()
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
    device: *const SmartThermometer,
) -> f64 {
    let handle = &mut *handle;
    let device = &*device;

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
pub unsafe extern "C" fn smart_home_get_socket_name(
    handle: *mut Handle,
    device: *const SmartSocket,
) -> *const c_char {
    let handle = &mut *handle;
    let device = &*device;

    handle.device_name_buffer = Some(CString::new(device.name()).unwrap());

    handle.device_name_buffer.as_ref().unwrap().as_ptr()
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
    handle: *mut Handle,
    device: *const SmartSocket,
) -> *const c_char {
    let handle = &mut *handle;
    let device = &*device;

    handle.device_desc_buffer = Some(CString::new(device.description()).unwrap());

    handle.device_desc_buffer.as_ref().unwrap().as_ptr()
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
    device: *const SmartSocket,
) -> f64 {
    let handle = &mut *handle;
    let device = &*device;

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
    device: *const SmartSocket,
) -> ReturnCode {
    let handle = &mut *handle;
    let device = &*device;

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
    device: *const SmartSocket,
) -> ReturnCode {
    let handle = &mut *handle;
    let device = &*device;

    handle.rt.block_on(async {
        match device.off().await {
            Ok(()) => ReturnCode::Success,
            Err(_) => ReturnCode::Fail,
        }
    })
}
