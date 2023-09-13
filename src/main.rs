use crate::config::SiteConfig;
use ::config::Config;
use async_std::task;
use dotenvy::dotenv;
use file::{compress_file, create_sitemap, Link};
use sqlx::postgres::PgPoolOptions;
use std::fs::{rename, File};
mod config;
mod file;
mod query;

fn main() {
    dotenv().ok();

    let builder = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();
    let config: SiteConfig = builder.try_deserialize().unwrap();
    let pool = task::block_on(
        PgPoolOptions::new()
            .max_connections(config.database_pool_size)
            .connect(&config.database),
    )
    .unwrap();

    // get pages
    let mut links = Vec::<Link>::new();
    for p in task::block_on(query::list(&pool, 1)).unwrap() {
        let mut link = Link::new();
        link.loc = p.loc();
        link.lastmod = p.lastmod();
        links.push(link);
    }
    // create sitmap
    create_sitemap(&config.sitemap_name, links).unwrap();
    // compress it
    compress_file(&config.sitemap_name).unwrap();
}

