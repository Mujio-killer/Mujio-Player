// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
// fn main() {
//     // 配置窗口
//     // 这里 `"quit".to_string()` 定义菜单项 ID，第二个参数是菜单项标签。
//     // let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//     // let close = CustomMenuItem::new("close".to_string(), "Close");
//     // let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
//     // let menu = Menu::new()
//     //         .add_native_item(MenuItem::Copy)
//     //         .add_item(CustomMenuItem::new("hide", "Hide"))
//     //         .add_submenu(submenu);

//     tauri::Builder::default()
//         .invoke_handler(tauri::generate_handler![greet])
//         // .menu(menu)
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
    
// }
use rusqlite::{params, Connection, Result};
use crate::models::site_info::SiteInfo;

pub fn get_conn() -> Result<Connection> {
    let database_file = "../../../file/sqlite.db";
    
    Ok(Connection::open(database_file)?)

}


fn create_db(conn: &Connection) {
    let _ = conn.execute("DROP TABLE site_info", []);

    let _ = conn.execute("CREATE TABLE site_info(
        id INTEGER PRIMARY KEY,
        key VARCHAR(255) NOT NULL,
        name VARCHAR(255) NOT NULL,
        api VARCHAR(255) NOT NULL,
        analysis_url VARCHAR(255) NOT NULL,
        group VARCHAR(255) NOT NULL,
        active tiny default 0,
        reverse_order tiny default 0
    )",
     []
    );

}

fn insert_data(conn: &Connection) -> Result<()> {
    let p1 = SiteInfo{
      key: "apibdzy".to_string(),
      id: 2,
      name: "百度云资源".to_string(),
      api: "https://m3u8.apibdzy.com/api.php/provide/vod/at/xml/".to_string(),
      analysis_url: "https://jx.444662.cn/m3u8/?url=".to_string(),
      group: "默认".to_string(),
      is_active: true,
      reverse_order: true
    };
    let _ = conn.execute("INSERT INTO site_info (id, key, name, api, analysis_url, group, active, reverse_order)
    VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);", 
    params![p1.id, p1.key, p1.name, p1.api, p1.analysis_url, p1.group, p1.is_active, p1.reverse_order]);

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
            group: row.get(5)?,
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
    create_db(&conn);
    let _ = insert_data(&conn);
    let sites = query_data(&conn);

    for site in &sites {
        println!("here is: {:?}", site);
    }

}
