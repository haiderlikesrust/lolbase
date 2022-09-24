use std::sync::Arc;

use super::{media::MediaType, Name, NameId, RowReq};
use crate::{db, error::ApiError, Record, Column, RecordOutput, Row, RowId};
use axum::{extract::Multipart, Extension, Json};
use http::header::CONTENT_TYPE;

use super::{State, Status};

pub async fn save_media(
    Extension(state): Extension<Arc<State>>,
    mut form: Multipart,
) -> Result<Json<Status>, ApiError> {
    while let Some(a) = form.next_field().await.unwrap() {
        let headers = a.headers();
        let content_type =
            MediaType::try_from(headers.get(CONTENT_TYPE).unwrap().to_str().unwrap())?;
    }
    Ok(Json(Status { status: true }))
}

pub async fn create_record(
    Extension(state): Extension<Arc<State>>,
    name: Json<Name>,
) -> Result<Json<Status>, ApiError> {
    let pool = state.database.pool.clone();
    db::DatabaseArm::create_record(&Record::new(name.0.clone().name), &pool).await?;
    Ok(Json(Status { status: true }))
}

pub async fn create_column(
    Extension(state): Extension<Arc<State>>,
    name: Json<NameId>,
) -> Result<Json<Status>, ApiError> {
    let pool = state.database.pool.clone();
    let column = Column::new(&name.0.clone().name, name.0.clone().id);
    db::DatabaseArm::create_column(&column, &pool).await?;
    Ok(Json(Status { status: true }))
}


pub async fn create_row(
    Extension(state): Extension<Arc<State>>,
    rows: Json<Vec<RowReq>>
) ->  Result<Json<Status>, ApiError> {
    let pool = state.database.pool.clone();
    let mut values = vec![];
    for row in rows.0.clone() {
        let row = Row {
            for_column: crate::ColumnId::from_uuid(uuid::Uuid::parse_str(&row.for_column).unwrap()),
            row_id: RowId::new(),
            value: row.value,
            value_type: crate::ValueType::String,
        };
        values.push(row);
    }
    db::DatabaseArm::store_value(&pool, &mut values).await?;
    Ok(Json(Status { status: true }))

}

pub async fn get_all_records(Extension(state): Extension<Arc<State>>) -> Result<Json<Vec<RecordOutput>>, ApiError> {
    let pool = state.database.pool.clone();
    let records = db::DatabaseArm::get_all_records(&pool).await?;
    Ok(Json(records))
}
