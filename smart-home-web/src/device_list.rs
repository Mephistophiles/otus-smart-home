use actix_web::{delete, get, http::StatusCode, post, put, web, web::Json, HttpResponse};
use smart_home_lib::{Device, SmartSocket, SmartThermometer};

use crate::{
    error::{Error, WebResult},
    home_list::GlobalContext,
    types::{WebSocket, WebSocketResult, WebThermometer, WebThermometerResult},
    WebDevice,
};

#[get("/{home}/{room}/")]
async fn read_device_list(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
) -> WebResult<Json<Vec<WebDevice>>> {
    let home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room(&room).ok_or(Error::RoomNotFound)?;

    Ok(Json(room.device_iter().map(|d| d.into()).collect()))
}

#[get("/{home}/{room}/{device}")]
async fn read_device(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<Json<WebDevice>> {
    let home_list = ctx.home_list.lock().await;
    let (home, room, device) = path.into_inner();
    let home = home_list.get_home(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room(&room).ok_or(Error::RoomNotFound)?;
    let device = room.device(&device).ok_or(Error::DeviceNotFound)?;

    Ok(Json(device.into()))
}

#[put("/{home}/{room}/thermometer")]
async fn create_thermometer(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
    thermometer: web::Json<WebThermometer>,
) -> WebResult<Json<WebDevice>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let thermometer = thermometer.into_inner();
    let device = room
        .add_device(
            SmartThermometer::new(
                thermometer.name,
                thermometer.description,
                thermometer.server_addr,
            )
            .await,
        )
        .map_err(|_| Error::DeviceAlreadyExists)?;

    Ok(Json(device.into()))
}

#[get("/{home}/{room}/{thermometer}/current_temperature")]
async fn get_current_temperature(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<Json<WebThermometerResult>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room, thermometer) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let thermometer = room.device(&thermometer).ok_or(Error::DeviceNotFound)?;

    if let Device::Thermometer(thermometer) = thermometer {
        Ok(Json(WebThermometerResult {
            current_temperature: thermometer
                .current_temperature()
                .await
                .map_err(|_| Error::Internal)?,
        }))
    } else {
        Err(Error::DeviceNotCompatible)
    }
}

#[get("/{home}/{room}/{socket}/current_power")]
async fn get_current_power(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<Json<WebSocketResult>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room, socket) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let socket = room.device(&socket).ok_or(Error::DeviceNotFound)?;

    if let Device::Socket(socket) = socket {
        Ok(Json(WebSocketResult {
            current_power: socket.current_power().await.map_err(|_| Error::Internal)?,
        }))
    } else {
        Err(Error::DeviceNotCompatible)
    }
}

#[post("/{home}/{room}/{socket}/on")]
async fn socket_on(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<HttpResponse> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room, socket) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let socket = room.device(&socket).ok_or(Error::DeviceNotFound)?;

    if let Device::Socket(socket) = socket {
        socket.on().await.map_err(|_| Error::Internal)?;

        Ok(HttpResponse::new(StatusCode::OK))
    } else {
        Err(Error::DeviceNotCompatible)
    }
}

#[post("/{home}/{room}/{socket}/off")]
async fn socket_off(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<HttpResponse> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room, socket) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let socket = room.device(&socket).ok_or(Error::DeviceNotFound)?;

    if let Device::Socket(socket) = socket {
        socket.off().await.map_err(|_| Error::Internal)?;

        Ok(HttpResponse::new(StatusCode::OK))
    } else {
        Err(Error::DeviceNotCompatible)
    }
}

#[put("/{home}/{room}/socket")]
async fn create_socket(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
    socket: web::Json<WebSocket>,
) -> WebResult<Json<WebDevice>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let socket = socket.into_inner();
    let device = room
        .add_device(SmartSocket::new(socket.name, socket.description, socket.server_addr).await)
        .map_err(|_| Error::DeviceAlreadyExists)?;

    Ok(Json(device.into()))
}

#[delete("/{home}/{room}/{device}")]
async fn delete_device(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String, String)>,
) -> WebResult<Json<WebDevice>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room, device) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room_mut(&room).ok_or(Error::RoomNotFound)?;
    let device = room.del_device(&device).ok_or(Error::DeviceNotFound)?;

    Ok(Json(device.into()))
}
