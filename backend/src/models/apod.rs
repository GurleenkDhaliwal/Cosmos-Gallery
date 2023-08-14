use crate::AppError;
use anyhow::Result;
use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};
use sqlx::Error;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Apod {
    pub date: NaiveDate,
    pub explanation: String,
    pub hdurl: Option<String>,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
    pub id: i64,
    pub upvotes: Option<i32>,
    pub downvotes: Option<i32>,
}

pub async fn insert_apod(pool: &PgPool, apod: &Apod) -> Result<i32, Error> {
    sqlx::query!(
        "INSERT INTO apods (date, explanation, hdurl, media_type, service_version, title, url, upvotes, downvotes)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         RETURNING id",
        apod.date,
        apod.explanation,
        apod.hdurl,
        apod.media_type,
        apod.service_version,
        apod.title,
        apod.url,
        apod.upvotes.unwrap_or(0),  // default to 0 if None
        apod.downvotes.unwrap_or(0) // default to 0 if None
    )
    .fetch_one(pool)
    .await
    .map(|record| record.id)
}

