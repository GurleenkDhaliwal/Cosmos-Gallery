use crate::AppError;
use anyhow::Result;
use chrono::NaiveDate;
use sqlx::Error;
use sqlx::PgPool;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Apod {
    pub date: NaiveDate,
    pub explanation: String,
    pub hdurl: Option<String>,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
}

pub async fn insert_apod(pool: &PgPool, apod: &Apod) -> Result<i32, Error> {

    sqlx::query!(
        "INSERT INTO apods (date, explanation, hdurl, media_type, service_version, title, url) 
         VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING id",
        apod.date,
        apod.explanation,
        apod.hdurl,
        apod.media_type,
        apod.service_version,
        apod.title,
        apod.url
    )
    .fetch_one(pool)
    .await
    .map(|record| record.id)
}
