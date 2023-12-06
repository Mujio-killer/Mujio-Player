// #[drive(...)]用于生成常用的特质(Trait),例如Debug、Clone、PartialEq、PartialOrd
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub update_time: String,
    pub id: String,
    pub tid: String,
    pub name: String,
    pub type_str: String,
    pub pic: String,
    pub lang: String,
    pub area: String,
    pub year: String,
    pub note: String,
    pub actor: String,
    pub director: String,
    pub desc: String,
    pub video_source: Vec<VideoSource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoSource {
    // 资源标签
    pub flag: String,
    // 资源详情
    pub episodes: Vec<Episode>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    // 集数
    pub episode: String,
    // 链接
    pub link: String,
}
