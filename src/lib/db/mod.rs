pub mod model;

use std::vec;

use sqlx::{postgres::PgPoolOptions, Postgres};

use crate::{
    error::ApiError, Column, ColumnId, ColumnOutput, IntoInner, Record, RecordId, RecordOutput,
    Row, RowId, RowOutput, ValueType,
};

#[derive(Clone, Copy)]
pub struct DatabaseArm {}
#[derive(Clone, Debug)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub async fn new(uri: &str) -> Self {
        Self {
            pool: PgPoolOptions::new().connect(uri).await.unwrap(),
        }
    }
}
type Pool = sqlx::Pool<Postgres>;

impl DatabaseArm {
    pub async fn get_all_rows(pool: &Pool, id: &ColumnId) -> Result<Vec<RowOutput>, ApiError> {
        let pool = pool.clone();
        let rows = sqlx::query!("SELECT * FROM rows where for_column = $1", id.into_inner())
            .fetch_all(&pool)
            .await?;
        let mut row_output = vec![];
        for row in rows {
            let row = RowOutput {
                for_column: ColumnId(row.for_column),
                value: row.value,
                value_type: row.value_type.into(),
            };
            row_output.push(row);
        }
        Ok(row_output)
    }
    pub async fn get_all_columns(
        pool: &Pool,
        id: &RecordId,
    ) -> Result<Vec<ColumnOutput>, ApiError> {
        let pool = pool.clone();
        let columns = sqlx::query!(
            "SELECT * FROM columns WHERE for_record = $1",
            id.into_inner()
        )
        .fetch_all(&pool)
        .await?;
        let mut column_output = vec![];
        for column in columns {
            let rows = DatabaseArm::get_all_rows(&pool, &ColumnId(column.column_id)).await?;
            let column = ColumnOutput {
                created_on: column.created_on,
                name: column.name,
                rows: rows,
                id: ColumnId(column.column_id),
            };
            column_output.push(column);
        }
        Ok(column_output)
    }
    pub async fn get_all_records(pool: &Pool) -> Result<Vec<RecordOutput>, ApiError> {
        let pool = pool.clone();
        let records = sqlx::query!("SELECT * FROM records")
            .fetch_all(&pool)
            .await?;
        let mut records_output = vec![];
        for record in records {
            let columns = DatabaseArm::get_all_columns(&pool, &RecordId(record.record_id)).await?;
            let record = RecordOutput {
                columns: columns,
                info: Record {
                    name: record.name,
                    record_id: RecordId(record.record_id),
                    created_on: record.created_on,
                },
            };
            records_output.push(record);
        }
        Ok(records_output)
    }
    pub async fn create_record(record: &Record, pool: &Pool) -> Result<(), ApiError> {
        let pool = pool.clone();
        sqlx::query!(
            "INSERT INTO records(name, record_id, created_on) VALUES($1, $2, $3)",
            record.name,
            record.record_id.into_inner(),
            record.created_on
        )
        .execute(&pool)
        .await?;

        Ok(())
    }

    pub async fn create_column(column: &Column, pool: &Pool) -> Result<(), ApiError> {
        let pool = pool.clone();
        sqlx::query!(
            "INSERT INTO columns(name, for_record, column_id, created_on) VALUES($1, $2, $3, $4)",
            column.name,
            column.for_record.into_inner(),
            column.id.into_inner(),
            column.created_on
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn create_row(row: &Row, pool: &Pool) -> Result<(), ApiError> {
        let pool = pool.clone();
        sqlx::query!(
            "INSERT INTO rows(for_column, row_id, value, value_type) VALUES($1, $2, $3, $4)",
            row.for_column.into_inner(),
            row.row_id.into_inner(),
            row.value,
            row.value_type.to_string()
        )
        .execute(&pool)
        .await?;
        Ok(())
    }

    pub async fn store_value(pool: &Pool, values: &mut Vec<Row>) -> Result<(), ApiError> {
        let pool = pool.clone();
        let id = RowId::from_uuid(uuid::Uuid::new_v4());
        for value in values {
            value.row_id = id.clone();
            DatabaseArm::create_row(&value, &pool).await?;
        }
        Ok(())
    }
}
