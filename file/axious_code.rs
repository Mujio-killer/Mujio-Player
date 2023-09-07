use reqwest::Client;
use regex::Regex;

#[derive(Debug)]
struct Class {
    tid: String,
    name: String,
}

#[derive(Debug)]
struct Doc {
    class: Vec<Class>,
    page: String,
    pagecount: String,
    pagesize: String,
    recordcount: String,
}

#[tokio::main]
async fn main() {
    let url = "https://example.com";

    let client = Client::new();
    match client.get(url).send().await {
        Ok(res) => {
            let data = res.text().await.unwrap();
            let json = parser::parse(&data, xml_config);
            let json_data = if json.rss.is_none() { Some(json) } else { json.rss };

            if let (Some(json_data), Some(class), Some(list)) =
                (&json_data, &json_data.class, &json_data.list)
            {
                let mut class_arr = Vec::new();

                if let Some(ty) = &class.ty {
                    let regex = regex::Regex::new(r"\{.*\}").unwrap();
                    for i in ty.iter() {
                        let j = Class {
                            tid: i._id.clone(),
                            name: regex.replace_all(&i._t, "").to_string(),
                        };
                        class_arr.push(j);
                    }
                }

                let doc = Doc {
                    class: class_arr,
                    page: list._page.clone(),
                    pagecount: list._pagecount.clone(),
                    pagesize: list._pagesize.clone(),
                    recordcount: list._recordcount.clone(),
                };

                println!("{:?}", doc);
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}