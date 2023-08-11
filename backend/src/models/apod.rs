use sqlx::PgPool;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Apod {
    pub date: String,
    pub explanation: String,
    pub hdurl: Option<String>,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
}

impl Apod {
    pub async fn create(apod: Apod, pool: &PgPool) -> Result<Apod> {
        sqlx::query!(
            r#"
            INSERT INTO apods (date, explanation, hdurl, media_type, service_version, title, url)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            apod.date, apod.explanation, apod.hdurl, apod.media_type, apod.service_version, apod.title, apod.url
        )
        .fetch_one(pool)
        .await?;

        Ok(apod)
    }

}

