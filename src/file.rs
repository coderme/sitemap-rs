use sailfish::TemplateOnce;
use std::{
    error::Error,
    fs::{rename, File},
    io::{Read, Write},
    os::unix::prelude::PermissionsExt,
    process::{Command, Stdio},
};

#[derive(Default)]
pub struct Link {
    pub loc: String,
    pub lastmod: String,
    pub changefreq: String,
    pub priority: f32,
}

impl Link {
    pub fn new() -> Self {
        Link::default()
    }
}

#[derive(TemplateOnce, Default)]
#[template(path = "sitemap.xml", rm_whitespace = true)]
struct Sitemap {
    links: Vec<Link>,
}

#[derive(TemplateOnce, Default)]
#[template(path = "sitemap_index.xml", rm_whitespace = true)]
struct SitemapIndex {
    links: Vec<Link>,
}

/// Creates a sitemap from links.
pub fn create_sitemap(name: &str, links: Vec<Link>) -> Result<(), Box<dyn Error>> {
    let t = format!("{}.1", name);
    let m = Sitemap { links }.render_once()?;
    let mut f = File::create(&t)?;
    f.metadata()?.permissions().set_mode(0o644);
    f.write_all(m.as_bytes())?;
    Ok(rename(t, name)?)
}

/// Creates a sitemap index from links.
pub fn create_sitemap_index(name: &str, links: Vec<Link>) -> Result<(), Box<dyn Error>> {
    let t = format!("{}.1", name);
    let m = SitemapIndex { links }.render_once()?;
    let mut f = File::create(&t)?;
    f.metadata()?.permissions().set_mode(0o644);
    f.write_all(m.as_bytes())?;
    Ok(rename(t, name)?)
}

/// Compresses file using gzip format.
pub fn compress_file(name: &str) -> Result<(), Box<dyn Error>> {
    Command::new("7z")
        .args([
            "a",
            "-tgzip",
            "-mx9",
            format!("{}.gz.1", name).as_str(),
            &name,
        ])
        .stdout(Stdio::null())
        .status()?;

    Ok(rename(format!("{}.gz.1", name), format!("{}.gz", name))?)
}

const SITEMAP_NAME: &'static str = "/tmp/.___sitemap.xml";

#[cfg(debug_assertions)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sitemap() {
        const RESULT: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">

<url>
<loc>https://coderme.com/page-1</loc>

<lastmod>2023-11-11</lastmod>





</url>

<url>
<loc>https://coderme.com/page-2</loc>

<lastmod>2023-11-11</lastmod>





</url>

<url>
<loc>https://coderme.com/page-3</loc>

<lastmod>2023-11-11</lastmod>





</url>

</urlset> "#;
        let links: Vec<Link> = vec![
            Link {
                loc: "https://coderme.com/page-1".to_string(),
                priority: 0.0,
                changefreq: String::from(""),
                lastmod: String::from("2023-11-11"),
            },
            Link {
                loc: "https://coderme.com/page-2".to_string(),
                priority: 0.0,
                changefreq: String::from(""),
                lastmod: String::from("2023-11-11"),
            },
            Link {
                loc: "https://coderme.com/page-3".to_string(),
                priority: 0.0,
                changefreq: String::from(""),
                lastmod: String::from("2023-11-11"),
            },
        ];

        create_sitemap(SITEMAP_NAME, links).unwrap();
        let mut f = File::open(SITEMAP_NAME).unwrap();
        let mut buf = vec![];
        f.read_to_end(&mut buf).unwrap();
        let out = String::from_utf8(buf).unwrap();
        assert_eq!(out, RESULT);
    }

    #[test]
    fn test_create_sitemap_index() {
        const NAME: &'static str = "/tmp/.___sitemap_index.xml";
        const RESULT: &'static str = r#"<?xml version="1.0" encoding="UTF-8"?>
<sitemapindex xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">

<sitemap>
<loc>https://coderme.com/page-1</loc>

<lastmod>2023-11-11</lastmod>


</sitemap>

<sitemap>
<loc>https://coderme.com/page-2</loc>

<lastmod>2023-11-11</lastmod>


</sitemap>

<sitemap>
<loc>https://coderme.com/page-3</loc>

<lastmod>2023-11-11</lastmod>


</sitemap>
