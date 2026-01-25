use std::collections::HashMap;
use std::sync::OnceLock;
use serde::Deserialize;
use csv::ReaderBuilder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AreaType {
    Country = 1,
    Province = 2,
    City = 3,
    District = 4,
}

impl AreaType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::Country),
            2 => Some(Self::Province),
            3 => Some(Self::City),
            4 => Some(Self::District),
            _ => None,
        }
    }
}

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

// Constants matching Java Area.java
pub const ID_GLOBAL: i32 = 0;
pub const ID_CHINA: i32 = 1;

impl AreaUtils {
    fn get_data() -> &'static (HashMap<i32, Area>, HashMap<i32, Vec<i32>>) {
        AREA_DATA.get_or_init(|| {
            let mut reader = ReaderBuilder::new()
                .has_headers(true)
                .from_reader(AREA_CSV.as_bytes());

            let mut areas = HashMap::new();
            let mut children_map: HashMap<i32, Vec<i32>> = HashMap::new();

            // Add Global node if not present in CSV (Java code adds it manually)
            // But CSV might not have it. Java adds ID_GLOBAL=0 manually.
            let global = Area {
                id: ID_GLOBAL,
                name: "全球".to_string(),
                type_: 0,
                parent_id: -1, // No parent
            };
            areas.insert(ID_GLOBAL, global);


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
                    current_area = children.iter()
                        .filter_map(|child_id| areas.get(child_id))
                        .find(|child| child.name == path);
                } else {
                    return None;
                }
            } else {
                // Search in all areas for the first node
                // Prioritize areas with lower type_ (e.g. Province before City)
                // because paths usually start from the top.
                let mut candidates: Vec<&Area> = areas.values()
                    .filter(|a| a.name == path)
                    .collect();
                
                if candidates.is_empty() {
                    return None;
                }
                
                // Sort by type_ ascending (1=Country, 2=Province, ...)
                candidates.sort_by_key(|a| a.type_);
                
                current_area = candidates.first().copied();
            }

            if current_area.is_none() {
                return None;
            }
        }

        current_area
    }


    /// 格式化区域，例如：北京市 北京市 东城区
    pub fn format(id: i32) -> String {
        Self::format_with_separator(id, " ")
    }

    pub fn format_with_separator(id: i32, separator: &str) -> String {
        let mut current_id = id;
        let mut names = Vec::new();
        let (areas, _) = Self::get_data();

        // Max depth check (Java uses enum length which is 4-5)
        let mut count = 0;
        while let Some(area) = areas.get(&current_id) {
            // Java: if area == null return null (here we return partial string or empty)
            
            names.push(area.name.as_str());
            
            // "递归"父节点
            let parent_id = area.parent_id;
            
            // Java: if parent is null or ID_GLOBAL or ID_CHINA -> break
            if parent_id == ID_GLOBAL || parent_id == ID_CHINA {
                 // But wait, Java adds current area BEFORE checking parent breaks.
                 // "sb.insert(0, area.getName()); area = area.getParent(); if (area == null || ... ID_CHINA) break;"
                 // So if current is China, we added China, then parent is 0, we break.
                 // But typically format is called for a leaf.
                 // If we are at Beijing (110000), parent is 1 (China). We add Beijing. Parent is 1. Break.
                 // So China is NOT added.
                 break;
            }
            
            // Also need to check if parent exists
            if !areas.contains_key(&parent_id) {
                break;
            }

            current_id = parent_id;
            
            count += 1;
            if count > 10 { break; }
        }
        
        names.reverse();
        names.join(separator)
    }

    pub fn get_by_type(type_: AreaType) -> Vec<&'static Area> {
         let (areas, _) = Self::get_data();
         areas.values()
             .filter(|a| a.type_ == type_ as i32)
             .collect()
    }
    
    pub fn get_parent_id_by_type(id: i32, type_: AreaType) -> Option<i32> {
        let (areas, _) = Self::get_data();
        let mut current_id = id;
        let target_type = type_ as i32;

        // Loop max 127 times (Java: Byte.MAX_VALUE)
        for _ in 0..127 {
             if let Some(area) = areas.get(&current_id) {
                 if area.type_ == target_type {
                     return Some(area.id);
                 }
                 // Check if root
                 if area.parent_id == 0 || area.parent_id == -1 {
                     return None;
                 }
                 current_id = area.parent_id;
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
    
    #[test]
    fn test_parse_area() {
        let area = AreaUtils::parse_area("北京市/北京市/东城区");
        assert!(area.is_some());
        assert_eq!(area.unwrap().id, 110101);
        
        let area_prov = AreaUtils::parse_area("河南省");
        assert!(area_prov.is_some());
        assert_eq!(area_prov.unwrap().name, "河南省");
    }
    
    #[test]
    fn test_get_parent_id_by_type() {
        // 110101 (District) -> Province (2) -> 110000
        let parent = AreaUtils::get_parent_id_by_type(110101, AreaType::Province);
        assert_eq!(parent, Some(110000));
    }
}