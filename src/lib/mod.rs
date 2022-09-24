use chrono::{NaiveDateTime, NaiveDate, Utc};
use uuid::Uuid;
pub mod db;
pub mod error;
pub mod web;
use std::string::ToString;
use serde::{Serialize, Deserialize};
use strum::Display;

pub trait IntoInner {
    type Output;
    fn into_inner(&self) -> Self::Output;
}

#[derive(Debug, Clone, Serialize, Deserialize)]

/// The main type used to identifiy groups of columns and rows.
pub struct Record {
    pub name: String,
    pub record_id: RecordId,
    pub created_on: NaiveDateTime,
}

impl Record {
    pub fn new(name: String) -> Self {
        Self {
            name,
            record_id: RecordId::new(),
            created_on: Utc::now().naive_utc()
        }
        
    }
}

#[derive(Display)]

/// The types that can be stored in lolbase.
/// Booleans can also be stored as strings, later can be fetched as booleans.
///
/// The variant `File` only stored paths, files are stored seperately.

#[derive(Debug, Clone, Serialize, Deserialize)]

pub enum ValueType {
    #[strum(serialize = "string")]
    String,
    #[strum(serialize = "int")]
    Int,
    #[strum(serialize = "file")]
    File,
}

impl From<String> for ValueType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "string" => Self::String,
            "int" => Self::Int,
            "file" => Self::File,
            _ => panic!("Invalid type, this error is not common and should be immediately reported to maintainers.")
        }
    }
}

pub struct Row {
    pub for_column: ColumnId,
    pub row_id: RowId,
    pub value: String,
    pub value_type: ValueType,
}
#[derive(Debug, Clone)]

pub struct Column {
    pub name: String,
    pub id: ColumnId,
    pub for_record: RecordId,
    pub created_on: NaiveDateTime,
}

impl Column {
    pub fn new(name: &str, id: String) -> Self {
        Self {
            name: name.to_owned(),
            for_record: RecordId::from_uuid(Uuid::parse_str(&id).unwrap()),
            id: ColumnId::new(),
            created_on: Utc::now().naive_utc(),
        }
    }
}

impl ColumnId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
    pub fn from_uuid(a: Uuid) -> Self {
        Self(a)
    }
}

impl RecordId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(a: Uuid) -> Self {
        Self(a)
    }
}

impl RowId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(a: Uuid) -> Self {
        Self(a)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ColumnId(uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RowId(uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RecordId(uuid::Uuid);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordOutput {
    pub columns: Vec<ColumnOutput>,
    pub info: Record,
}

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ColumnOutput {
    pub created_on: NaiveDateTime,
    pub name: String,
    pub id: ColumnId,
    pub rows: Vec<RowOutput>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RowOutput {
    pub value: String,
    pub value_type: ValueType,
    pub for_column: ColumnId,
}

impl IntoInner for RowId {
    type Output = uuid::Uuid;
    fn into_inner(&self) -> Self::Output {
        self.0
    }
}
impl IntoInner for ColumnId {
    type Output = uuid::Uuid;
    fn into_inner(&self) -> Self::Output {
        self.0
    }
}
impl IntoInner for RecordId {
    type Output = uuid::Uuid;
    fn into_inner(&self) -> Self::Output {
        self.0
    }
}
