#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "apods"]
pub struct APOD {
    pub id: i32,
    pub date: String,
    pub explanation: String,
    pub hdurl: Option<String>,
    pub media_type: String,
    pub service_version: String,
    pub title: String,
    pub url: String,
}

