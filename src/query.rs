use async_std::task;
use chrono::{DateTime, Local};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, Error, FromRow, Pool, Postgres};
use std::env;

#[derive(FromRow, Default)]
pub struct PageResult {
    pub slug: String,
    pub modtime: Option<DateTime<Local>>,
}

impl PageResult {
    pub fn loc(&self) -> String {
        format!("https://coderme.com/{}", self.slug)
    }

    pub fn lastmod(&self) -> String {
        self.modtime.unwrap().date_naive().to_string()
    }
}

pub async fn list(pool: &Pool<Postgres>, page: u32) -> Result<Vec<PageResult>, Error> {
    let p: i64;

    if page == 0 {
        p = 1;
    } else {
        p = page as i64
    }

    const PER_PAGE: i64 = 50_000;
    let offset: i64 = PER_PAGE * (p - 1i64);
    let rows: Vec<PageResult> = sqlx::query_as(include_str!("./sql/page.sql"))
        .bind(&PER_PAGE)
        .bind(&offset)
        .fetch_all(pool)
        .await?;

    Ok(rows)
}

#[cfg(debug_assertions)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        dotenv().ok();
        let uri = env::var("DATABASE").unwrap();
        let pool = task::block_on(PgPoolOptions::new().connect(&uri)).unwrap();
        task::block_on(list(&pool, 1)).unwrap();
    }
}

