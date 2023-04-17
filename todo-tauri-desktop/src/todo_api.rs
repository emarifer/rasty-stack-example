use reqwasm::{http::Request, Error};
use std::collections::VecDeque;

// use crate::helpers::display_data;
use crate::model::*;

#[allow(dead_code)]
const BASE_URL: &str = "http://localhost:8080";

pub async fn fetch_tasks() -> Result<VecDeque<Task>, Error> {
    // display_data(format!("{BASE_URL}/todos"));

    let response = Request::get(&format!("{BASE_URL}/todos")).send().await?;

    response.json().await
}

pub async fn create_task(title: &str) -> Result<Task, Error> {
    let response = Request::post(&format!("{BASE_URL}/todo/{title}"))
        .send()
        .await?;

    response.json().await
}

pub async fn toggle_task(id: String) -> Result<AffectedRows, Error> {
    let response = Request::patch(&format!("{BASE_URL}/todo/{id}"))
        .send()
        .await?;

    response.json().await
}

pub async fn delete_task(id: String) -> Result<AffectedRows, Error> {
    let response = Request::delete(&format!("{BASE_URL}/todo/{id}"))
        .send()
        .await?;

    response.json().await
}
