mod api;
mod db;
mod error;
mod model;
mod prelude;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use api::*;

static DB: Surreal<Client> = Surreal::init();

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    DB.connect::<Ws>("localhost:8000").await?;

    DB.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    DB.use_ns("namespace").use_db("database").await?;

    println!("✅ Database connected successfully!!");

    println!("✅ Server running at http://localhost:{PORT}");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .send_wildcard();

        App::new()
            .wrap(cors)
            .service(create)
            .service(get)
            .service(update)
            .service(delete)
            .service(list)
    })
    .bind(("localhost", PORT))?
    .run()
    .await?;

    Ok(())
}

/*
 * ARRANCAR UN CONTENEDOR DOCKER DE SURREALDB CON UN FICHERO docker-compose.yml:
 * sudo docker compose up -d
 *
 * ENTRAR EN LA CLI DE SURREALDB EN EL CONTENEDOR CREADO:
 * sudo docker exec -it surrealdb /surreal sql -c http://localhost:8000 -u root -p root --ns namespace --db database --pretty
 *
 * VER SI EL CONTENEDOR DOCKER ESTÁ INICIADO:
 * sudo docker ps  (CON EL FLAG --a SE LISTAN TODOS LOS CONTENEDORES, ACTIVOS Y NO ACTIVOS)
 *
 * DETENER EL CONTENEDOR DE DOCKER:
 * sudo docker stop surrealdb
 *
 * VOLVER A INICIAR EL CONTENEDOR DE DOCKER:
 * sudo docker start surrealdb
 */

/*
 * CORS EN ACTIX-WEB. VER:
 * https://stackoverflow.com/questions/73098788/access-control-allow-origin-missing-using-actix-web
 * https://github.com/actix/examples/blob/master/cors/backend/src/main.rs
 * https://docs.rs/actix-cors/0.6.4/actix_cors/
 * https://github.com/actix/actix-extras/blob/master/actix-cors/examples/cors.rs
 */
