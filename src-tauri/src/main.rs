// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::sync::Arc;

use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::Name;
use serde_json::Value;

use crate::models::movie::{Episode, Movie, VideoSource};

mod models;

// 获取保存的站点
#[tauri::command]
fn read_json_file(file_path: String) -> Vec<Value> {
    // 读取文件内容
    let file_content = fs::read_to_string(file_path).unwrap();
    // 解析为JSON数组
    let json_array: Vec<Value> = serde_json::from_str(&file_content).unwrap();

    json_array
}


/*
根据名称查询资源：https://api.xinlangapi.com/xinlangapi.php/provide/vod/at/xml?ac=videolist&wd=1988
 */
#[tauri::command]
fn search_by_site(site: String, keyword: String) -> Vec<Movie> {
    let mut movies = Vec::new();
    // 发起查询请求
    let response = reqwest::blocking::get(&format!("{}?ac=videolist&wd={}", site, keyword)).unwrap();

    // 解析响应数据
    let html_data = response.text().unwrap();
    let document = Document::from(html_data.as_str());

    for node in document.find(Name("video")) {
        let mut video_source = Vec::new();
        let dd_list = node.find(Name("dd"));
        for dd in dd_list {
            let mut episodes = Vec::new();
            let flag = dd.attr("flag").unwrap().to_string();
            let source_info = parse_comment(dd.html());

            let parts: Vec<&str> = source_info.split("#").collect();
            for part in parts {
                let sub_parts: Vec<&str> = part.split("$").collect();
                if sub_parts.len() == 2 {
                    let episode = sub_parts[0].to_string();
                    let link = sub_parts[1].to_string();
                    episodes.push(Episode { episode, link });
                }
            }
            video_source.push(VideoSource { flag, episodes });
        }

        let movie = Movie {
            update_time: get_text(node, "last"),
            id: get_text(node, "id"),
            tid: get_text(node, "tid"),
            name: get_text(node, "name"),
            type_str: get_text(node, "type"),
            pic: get_text(node, "pic"),
            lang: get_text(node, "lang"),
            area: get_text(node, "area"),
            year: get_text(node, "year"),
            note: get_text(node, "note"),
            actor: get_text(node, "actor"),
            director: get_text(node, "director"),
            desc: get_text(node, "des"),
            video_source,
        };

        movies.push(movie);
    }

    movies
}

// 避免空指针
fn get_text(node: Node, target_node_name: &str) -> String {
    let start_tag = format!("<{}>", target_node_name).to_string();
    let end_tag = format!("</{}>", target_node_name);
    let comment_start_tag = "<!--[CDATA[";
    let comment_end_tag = "]]-->";
    let extra_tag = "]]&gt;";

    let target_node: Option<Node> = node.find(Name(target_node_name)).next();
    if target_node.is_none() {
        return "".to_string();
    }
    let node_html_str = target_node.unwrap().html();
    node_html_str
        .replace(&start_tag, "")
        .replace(&end_tag, "")
        .replace(&comment_start_tag, "")
        .replace(&comment_end_tag, "")
        .replace(&extra_tag, "")
        .trim().to_string()
}

// 提取注释内容
fn parse_comment(text: String) -> String {
    let re_comment = Regex::new(r#"<!--\[CDATA\[(.*?)\]\]-->"#).unwrap();
    if let Some(captures) = re_comment.captures(&text) {
        let comment = captures.get(1).unwrap().as_str();
        return clean_comment(comment.to_string());
    }
    clean_comment(text)
}

// 移除内容异常情况下末尾的']]>'
fn clean_comment(text: String) -> String {
    let re_comment = Regex::new(r#"(.*?)\]\]>"#).unwrap();
    if let Some(captures) = re_comment.captures(&text) {
        let comment = captures.get(1).unwrap().as_str();
        return comment.to_string();
    }
    text
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_json_file, search_by_site])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}