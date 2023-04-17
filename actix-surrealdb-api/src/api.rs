use actix_web::{
    delete,
    get,
    patch,
    post,
    web::{Json, Path},
    // HttpResponse,
};

use crate::db::*;
use crate::model::*;
use crate::prelude::*;

#[post("/todo/{title}")]
pub async fn create(title: Path<String>) -> Result<Json<Task>> {
    let todo = add_task(title.into_inner()).await?;

    Ok(Json(todo))

    // match todo_id {
    //     Ok(id) => HttpResponse::Ok().json(id),
    //     Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    // }
}

#[get("/todo/{id}")]
pub async fn get(id: Path<String>) -> Result<Json<Task>> {
    let task = get_task(id.into_inner()).await?;

    Ok(Json(task))
}

#[patch("/todo/{id}")]
pub async fn update(id: Path<String>) -> Result<Json<AffectedRows>> {
    let updated = toggle_task(id.into_inner()).await?;

    Ok(Json(updated))
}

#[delete("/todo/{id}")]
pub async fn delete(id: Path<String>) -> Result<Json<AffectedRows>> {
    let deleted = delete_task(id.into_inner()).await?;

    Ok(Json(deleted))
}

#[get("/todos")]
pub async fn list() -> Result<Json<Vec<Task>>> {
    let todos = get_all_tasks().await?;

    Ok(Json(todos))
}
