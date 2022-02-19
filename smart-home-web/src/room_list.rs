use actix_web::{delete, get, put, web, web::Json};
use smart_home_lib::Room;

use crate::{
    error::{Error, WebResult},
    home_list::GlobalContext,
    WebRoom,
};

#[get("/{home}/")]
async fn read_room_list(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String,)>,
) -> WebResult<Json<Vec<WebRoom>>> {
    let home_list = ctx.home_list.lock().await;
    let (home,) = path.into_inner();
    let home = home_list.get_home(&home).ok_or(Error::HomeNotFound)?;

    Ok(Json(
        home.room_iter()
            .map(|room| WebRoom {
                name: room.name().to_string(),
            })
            .collect(),
    ))
}

#[get("/{home}/{room}")]
async fn read_room(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
) -> WebResult<Json<WebRoom>> {
    let home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home(&home).ok_or(Error::HomeNotFound)?;
    let room = home.room(&room).ok_or(Error::RoomNotFound)?;

    Ok(Json(room.into()))
}

#[put("/{home}/{room}")]
async fn create_room(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
) -> WebResult<Json<WebRoom>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;

    let room = home
        .add_room(Room::new(&room))
        .map_err(|_| Error::RoomAlreadyExists)?;

    Ok(Json(room.into()))
}

#[delete("/{home}/{room}")]
async fn delete_room(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String, String)>,
) -> WebResult<Json<WebRoom>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home, room) = path.into_inner();
    let home = home_list.get_home_mut(&home).ok_or(Error::HomeNotFound)?;
    let room = home.del_room(&room).ok_or(Error::RoomNotFound)?;

    Ok(Json(room.into()))
}
