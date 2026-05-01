use std::collections::HashMap;


pub fn translate_tag(tag: &str, ns: &str, lang: &str) -> String {
    if lang == "zh-CN" {
        static ZH_CN_TAGS: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();

        let tags_map = ZH_CN_TAGS.get_or_init(|| {
            let json_str = include_str!("tags_zh_cn.json");
            serde_json::from_str::<HashMap<String, String>>(json_str).unwrap_or_default()
        });

        let key = format!("{}:{}", ns, tag.replace(' ', "_"));
        if let Some(translated) = tags_map.get(&key) {
            return translated.clone();
        }
    }
    tag.to_string()
}
