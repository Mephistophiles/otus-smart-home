use actix_web::{middleware::Logger, web, App, HttpServer};
use types::{WebDevice, WebHome, WebRoom};

use self::{
    device_list::{
        create_socket, create_thermometer, delete_device, get_current_power,
        get_current_temperature, read_device, read_device_list, socket_off, socket_on,
    },
    home_list::{create_home, delete_home, read_home, read_home_list, GlobalContext},
    room_list::{create_room, delete_room, read_room, read_room_list},
};

mod device_list;
mod error;
mod home_list;
mod room_list;
mod types;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let ctx = GlobalContext::default();

    HttpServer::new(move || {
        let ctx = ctx.clone();

        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(ctx))
            .service(
                web::scope("/home")
                    .service(read_home_list)
                    .service(read_home)
                    .service(create_home)
                    .service(delete_home)
                    .service(read_room_list)
                    .service(read_room)
                    .service(create_room)
                    .service(delete_room)
                    .service(read_device_list)
                    .service(read_device)
                    .service(create_thermometer)
                    .service(create_socket)
                    .service(delete_device)
                    .service(get_current_temperature)
                    .service(get_current_power)
                    .service(socket_on)
                    .service(socket_off),
            )
    })
    .bind(("127.0.0.1", 4080))?
    .run()
    .await
}
