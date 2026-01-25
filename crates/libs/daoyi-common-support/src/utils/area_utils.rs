use std::collections::HashMap;
use std::sync::OnceLock;
use serde::Deserialize;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize, Clone)]
pub struct Area {
    pub id: i32,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(rename = "parentId")]
    pub parent_id: i32,
}

pub struct AreaUtils;

static AREA_DATA: OnceLock<(HashMap<i32, Area>, HashMap<i32, Vec<i32>>)> = OnceLock::new();

// Embed the CSV file
const AREA_CSV: &str = include_str!("../resources/area.csv");

impl AreaUtils {
    fn get_data() -> &'static (HashMap<i32, Area>, HashMap<i32, Vec<i32>>) {
        AREA_DATA.get_or_init(|| {
            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(AREA_CSV.as_bytes());

            let mut areas = HashMap::new();
            let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();

            for result in reader.deserialize() {
                match result {
                    Ok(area) => {
                        let area: Area = area;
                        children_map.entry(area.parent_id).or_default().push(area.id);
                        areas.insert(area.id, area);
                    }
                    Err(e) => {
                        eprintln!("Failed to parse area CSV record: {}", e);
                    }
                }
            }
            (areas, children_map)
        })
    }

    pub fn get_area(id: i32) -> Option<&'static Area> {
        Self::get_data().0.get(&id)
    }

    pub fn get_children(id: i32) -> Option<&'static Vec<i32>> {
        Self::get_data().1.get(&id)
    }

    /// 格式化区域，例如：北京市 北京市 东城区
    pub fn format(id: i32) -> String {
        Self::format_with_separator(id, " ")
    }

    pub fn format_with_separator(id: i32, separator: &str) -> String {
        let mut current_id = id;
        let mut names = Vec::new();
        let (areas, _) = Self::get_data();

        // Prevent infinite loop
        let mut count = 0;
        while let Some(area) = areas.get(&current_id) {
            if area.id == 0 { break; }
            
            // 排除国家（type=1），通常不需要显示 "中国"
            if area.type_ == 1 {
                break;
            }

            names.push(area.name.as_str());
            current_id = area.parent_id;
            
            count += 1;
            if count > 10 { break; }
        }
        
        names.reverse();
        names.join(separator)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_area_load() {
        let area = AreaUtils::get_area(110101); // Beijing Dongcheng
        assert!(area.is_some());
        assert_eq!(area.unwrap().name, "东城区");
    }

    #[test]
    fn test_format() {
        // 110101 -> 东城区 -> 北京市(City) -> 北京市(Province) -> 中国
        let formatted = AreaUtils::format(110101);
        println!("Formatted: {}", formatted);
        assert_eq!(formatted, "北京市 北京市 东城区");
    }
}
