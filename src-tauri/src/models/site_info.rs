// #[drive(...)]用于生成常用的特质(Trait),例如Debug、Clone、PartialEq、PartialOrd
#[derive(Debug)]
pub struct SiteInfo {
    pub id: i32,
    pub key: String,
    pub name: String,
    pub api: String,
    pub analysis_url: String,
    pub group_name: String,
    pub is_active: bool,
    pub reverse_order: bool
}