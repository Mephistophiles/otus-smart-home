use std::sync::Arc;

use actix_web::{delete, get, put, web, web::Json};
use smart_home_lib::{Home, SmartHub};
use tokio::sync::Mutex;

use crate::{
    error::{Error, WebResult},
    WebHome,
};

#[derive(Default)]
pub struct GlobalContext {
    pub home_list: Arc<Mutex<SmartHub>>,
}

impl Clone for GlobalContext {
    fn clone(&self) -> Self {
        Self {
            home_list: self.home_list.clone(),
        }
    }
}

#[get("/")]
async fn read_home_list(ctx: web::Data<GlobalContext>) -> Json<Vec<WebHome>> {
    let home_list = ctx.home_list.lock().await;

    Json(
        home_list
            .iter()
            .map(|home| WebHome {
                name: home.name().to_owned(),
            })
            .collect(),
    )
}

#[get("/{home}")]
async fn read_home(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String,)>,
) -> WebResult<Json<WebHome>> {
    let home_list = ctx.home_list.lock().await;
    let (name,) = path.into_inner();
    let home = home_list.get_home(&name).ok_or(Error::HomeNotFound)?;

    Ok(Json(home.into()))
}

#[put("/{home}")]
async fn create_home(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String,)>,
) -> WebResult<Json<WebHome>> {
    let mut home_list = ctx.home_list.lock().await;
    let (home,) = path.into_inner();

    let home = home_list
        .add_home(Home::new(&home))
        .map_err(|_| Error::HomeAlreadyExists)?;

    Ok(Json(home.into()))
}

#[delete("/{home}")]
async fn delete_home(
    ctx: web::Data<GlobalContext>,
    path: web::Path<(String,)>,
) -> WebResult<Json<WebHome>> {
    let home_list = &mut ctx.home_list.lock().await;
    let (name,) = path.into_inner();

    let home = home_list.del_home(&name).ok_or(Error::HomeNotFound)?;

    Ok(Json(home.into()))
}
