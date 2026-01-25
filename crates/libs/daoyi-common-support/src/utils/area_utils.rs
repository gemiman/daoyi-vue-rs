use crate::enumeration::AreaTypeEnum;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Area {
    pub id: String,
    pub name: String,
    pub r#type: AreaTypeEnum,
    pub parent_id: String,
}

pub struct AreaUtils;

static AREA_DATA: OnceLock<(HashMap<String, Area>, HashMap<String, Vec<String>>)> = OnceLock::new();

// Embed the CSV file
const AREA_CSV: &str = include_str!("../resources/area.csv");

// Constants matching Java Area.java
pub const ID_GLOBAL: &str = "0";
pub const ID_CHINA: &str = "1";

impl AreaUtils {
    fn get_data() -> &'static (HashMap<String, Area>, HashMap<String, Vec<String>>) {
        AREA_DATA.get_or_init(|| {
            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(AREA_CSV.as_bytes());

            let mut areas = HashMap::new();
            let mut children_map: HashMap<String, Vec<String>> = HashMap::new();

            // Add Global node if not present in CSV (Java code adds it manually)
            let global = Area {
                id: ID_GLOBAL.to_string(),
                name: "全球".to_string(),
                r#type: AreaTypeEnum::EARTH,
                parent_id: "-1".to_string(), // No parent
            };
            areas.insert(ID_GLOBAL.to_string(), global);

            for result in reader.deserialize() {
                match result {
                    Ok(area) => {
                        let area: Area = area;
                        children_map
                            .entry(area.parent_id.clone())
                            .or_default()
                            .push(area.id.clone());
                        areas.insert(area.id.clone(), area);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse area CSV record: {}", e);
                    }
                }
            }
            (areas, children_map)
        })
    }

    pub fn get_area(id: &str) -> Option<&'static Area> {
        Self::get_data().0.get(id)
    }

    pub fn get_children(id: &str) -> Option<&'static Vec<String>> {
        Self::get_data().1.get(id)
    }

    /// 获得指定区域对应的编号
    /// path_str 区域路径，例如说：河南省/石家庄市/新华区
    pub fn parse_area(path_str: &str) -> Option<&'static Area> {
        let paths: Vec<&str> = path_str.split('/').collect();
        let (areas, children_map) = Self::get_data();

        let mut current_area: Option<&Area> = None;

        for path in paths {
            if let Some(area) = current_area {
                // Search in children
                if let Some(children) = children_map.get(&area.id) {
                    current_area = children
                        .iter()
                        .filter_map(|child_id| areas.get(child_id))
                        .find(|child| child.name == path);
                } else {
                    return None;
                }
            } else {
                // Search in all areas for the first node
                // Prioritize areas with lower area_type (e.g. Province before City)
                // because paths usually start from the top.
                let mut candidates: Vec<&Area> =
                    areas.values().filter(|a| a.name == path).collect();

                if candidates.is_empty() {
                    return None;
                }

                // Sort by area_type ascending (1=Country, 2=Province, ...)
                candidates.sort_by_key(|a| a.r#type);

                current_area = candidates.first().copied();
            }

            if current_area.is_none() {
                return None;
            }
        }

        current_area
    }

    /// 格式化区域，例如：北京市 北京市 东城区
    pub fn format(id: &str) -> String {
        Self::format_with_separator(id, " ")
    }

    pub fn format_with_separator(id: &str, separator: &str) -> String {
        let mut current_id = id.to_string();
        let mut names = Vec::new();
        let (areas, _) = Self::get_data();

        // Max depth check (Java uses enum length which is 4-5)
        let mut count = 0;
        while let Some(area) = areas.get(&current_id) {
            // Java: if area == null return null (here we return partial string or empty)

            names.push(area.name.as_str());

            // "递归"父节点
            let parent_id = &area.parent_id;

            // Java: if parent is null or ID_GLOBAL or ID_CHINA -> break
            if parent_id == ID_GLOBAL || parent_id == ID_CHINA {
                break;
            }

            // Also need to check if parent exists
            if !areas.contains_key(parent_id) {
                break;
            }

            current_id = parent_id.clone();

            count += 1;
            if count > 10 {
                break;
            }
        }

        names.reverse();
        names.join(separator)
    }

    pub fn get_by_type(area_type: AreaTypeEnum) -> Vec<&'static Area> {
        let (areas, _) = Self::get_data();
        areas.values().filter(|a| a.r#type == area_type).collect()
    }

    pub fn get_parent_id_by_type(id: &str, area_type: AreaTypeEnum) -> Option<String> {
        let (areas, _) = Self::get_data();
        let mut current_id = id.to_string();
        let target_type = area_type;

        // Loop max 127 times (Java: Byte.MAX_VALUE)
        for _ in 0..127 {
            if let Some(area) = areas.get(&current_id) {
                if area.r#type == target_type {
                    return Some(area.id.clone());
                }
                // Check if root
                if area.parent_id == ID_GLOBAL || area.parent_id == "-1" {
                    return None;
                }
                current_id = area.parent_id.clone();
            } else {
                return None;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_load() {
        let area = AreaUtils::get_area("110101"); // Beijing Dongcheng
        assert!(area.is_some());
        assert_eq!(area.unwrap().name, "东城区");
    }

    #[test]
    fn test_format() {
        // 110101 -> 东城区 -> 北京市(City) -> 北京市(Province) -> 中国
        let formatted = AreaUtils::format("110101");
        println!("Formatted: {}", formatted);
        assert_eq!(formatted, "北京市 北京市 东城区");
    }

    #[test]
    fn test_parse_area() {
        let area = AreaUtils::parse_area("北京市/北京市/东城区");
        assert!(area.is_some());
        assert_eq!(area.unwrap().id, "110101");

        let area_prov = AreaUtils::parse_area("河南省");
        assert!(area_prov.is_some());
        assert_eq!(area_prov.unwrap().name, "河南省");
    }

    #[test]
    fn test_get_parent_id_by_type() {
        // 110101 (District) -> Province (2) -> 110000
        let parent = AreaUtils::get_parent_id_by_type("110101", AreaTypeEnum::PROVINCE);
        assert_eq!(parent, Some("110000".to_string()));
    }
}
