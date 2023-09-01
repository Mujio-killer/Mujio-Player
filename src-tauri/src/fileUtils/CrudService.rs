// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use rusqlite::{params, Connection, Result};
use crate::models::site_info::SiteInfo;
use std::path::Path;

pub fn get_conn() -> Result<Connection> {
    let path = Path::new("D:\\VsCodeProjects\\myapp\\testRust\\file\\test.db");

    println!("{:?}", path);
    Ok(Connection::open(path)?)

}


fn create_db(conn: &Connection) {
    let drop_res = conn.execute("DROP TABLE IF EXISTS site_info", []);
    println!("{:?}", drop_res);

    let create_res = conn.execute("CREATE TABLE site_info(
        id INTEGER PRIMARY KEY,
        key VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        api VARCHAR(255) NOT NULL,
        analysis_url VARCHAR(255) NOT NULL,
        group_name VARCHAR(255) NOT NULL,
        active tiny default 0,
        reverse_order tiny default 0
    )",
                                  []
    );
    println!("{:?}", create_res);
}

fn insert_data(conn: &Connection) -> Result<()> {
    let p1 = SiteInfo{
        key: "apibdzy".to_string(),
        id: 2,
        name: "百度云资源".to_string(),
        api: "https://m3u8.apibdzy.com/api.php/provide/vod/at/xml/".to_string(),
        analysis_url: "https://jx.444662.cn/m3u8/?url=".to_string(),
        group_name: "默认".to_string(),
        is_active: true,
        reverse_order: true
    };
    let _ = conn.execute("INSERT INTO site_info (id, key, name, api, analysis_url, group_name, active, reverse_order)
    VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);",
                         params![p1.id, p1.key, p1.name, p1.api, p1.analysis_url, p1.group_name, p1.is_active, p1.reverse_order]);

    Ok(())
}

fn query_data(conn: &Connection) -> Result<Vec<SiteInfo>> {
    let mut stmt = conn.prepare("SELECT * FROM site_info")?;
    let site_info_it = stmt.query_map([], |row|{
        Ok(SiteInfo{
            id: row.get(0)?,
            key: row.get(1)?,
            name: row.get(2)?,
            api: row.get(3)?,
            analysis_url: row.get(4)?,
            group_name: row.get(5)?,
            is_active: row.get(6)?,
            reverse_order: row.get(8)?
        })
    })?;

    let mut site_info_list = Vec::new();
    for site in site_info_it {
        site_info_list.push(site?);
    }

    Ok(site_info_list)
}

fn main() {
    let conn = get_conn().unwrap();
    let _ = create_db(&conn);

    let _ = insert_data(&conn);
    let sites = query_data(&conn);

    for site in &sites {
        println!("here is: {:?}", site);
    }

}
