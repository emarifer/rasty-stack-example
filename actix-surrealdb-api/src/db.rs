use chrono::offset::Local;

use crate::model::*;
use crate::prelude::*;
use crate::DB;

#[allow(dead_code)]
const TASK: &str = "task";

pub async fn add_task(title: String) -> Result<Task> {
    let created: Task = DB
        .create(TASK)
        .content(Task {
            id: None,
            title,
            completed: false,
            created_at: Local::now(),
        })
        .await?;

    Ok(created)
}

pub async fn get_task(id: String) -> Result<Task> {
    let th = id.split_once(":").unwrap();
    let rec: Task = DB.select(th).await?;

    Ok(rec)
}

pub async fn delete_task(id: String) -> Result<AffectedRows> {
    let th = id.split_once(":").unwrap();
    let _rec: Record = DB.delete(th).await?;

    Ok(AffectedRows { rows_affected: 1 })
}

pub async fn toggle_task(id: String) -> Result<AffectedRows> {
    let (tb, id) = id.split_once(":").unwrap();
    let sql =
        "UPDATE type::thing($tb, $id) SET completed = function() { return !this.completed; };";

    let mut response = DB.query(sql).bind(("tb", tb)).bind(("id", id)).await?;

    let _task_updated = response
        .take::<Vec<Task>>(0)?
        .into_iter()
        .next()
        .ok_or(Error::Generic("Failed to update record".into()))?;

    Ok(AffectedRows { rows_affected: 1 })
}

pub async fn get_all_tasks() -> Result<Vec<Task>> {
    // let tasks: Vec<Task> = DB.select(TASK).await?;

    // Ok(tasks)
    let sql = "SELECT * FROM type::table($table) ORDER BY created_at DESC;";

    let mut response = DB.query(sql).bind(("table", TASK)).await?;

    let tasks: Vec<Task> = response.take(0)?;

    Ok(tasks)
}

/*
 * https://surrealdb.com/docs/surrealql/functions/type#thing
 * https://surrealdb.com/docs/surrealql/functions/script
 * https://stackoverflow.com/questions/36876570/return-first-item-of-vector
 * https://stackoverflow.com/questions/57707966/how-to-get-timestamp-of-the-current-date-and-time-in-rust
 */

/*
 * RAZÓN POR LA QUE USAR EL VERBO "PATCH" Y NO "PUT". VER:
 * https://www.abstractapi.com/guides/put-vs-patch
 * https://www.baeldung.com/cs/http-put-vs-patch
 * https://github.com/letsgetrusty/rsty-stack-example/tree/main/todo_api
 * https://www.google.com/search?q=difference+between+patch+and+put&oq=dif&aqs=chrome.0.69i59j69i57j35i39j69i60l2j69i65l3.2889j0j7&sourceid=chrome&ie=UTF-8
 *
 * SOBRE ok_or. VER:
 * Is there a way to simplify converting an Option into a Result without a macro?
 * https://stackoverflow.com/questions/37890405/is-there-a-way-to-simplify-converting-an-option-into-a-result-without-a-macro
 */
