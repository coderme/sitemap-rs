use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct SiteConfig {
    pub database: String,
    pub database_pool_size: u32,
    // root
    pub static_root: String,
    pub sitemap_name: String,
}

