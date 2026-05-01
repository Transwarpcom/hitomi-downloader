use std::collections::HashMap;
use serde_json::Value;

pub fn translate_tag(tag: &str, ns: &str, lang: &str) -> String {
    if lang == "zh-CN" {
        static ZH_CN_TAGS: std::sync::OnceLock<HashMap<String, String>> = std::sync::OnceLock::new();

        let tags_map = ZH_CN_TAGS.get_or_init(|| {
            let json_str = include_str!("tags_zh_cn.json");
            if let Ok(Value::Object(map)) = serde_json::from_str(json_str) {
                map.into_iter()
                    .filter_map(|(k, v)| {
                        if let Value::String(s) = v {
                            Some((k, s))
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                HashMap::new()
            }
        });

        let key = format!("{}:{}", ns, tag.replace(' ', "_"));
        if let Some(translated) = tags_map.get(&key) {
            return translated.clone();
        }
    }
    tag.to_string()
}
