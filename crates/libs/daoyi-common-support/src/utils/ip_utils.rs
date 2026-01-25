use std::sync::{OnceLock, Mutex};
use std::fs::File;
use std::io::Write;
use ip2region::Searcher;
use crate::utils::area_utils::{Area, AreaUtils};

const IP2REGION_XDB: &[u8] = include_bytes!("../resources/ip2region.xdb");
static IP_SEARCHER: OnceLock<Mutex<Searcher>> = OnceLock::new();

pub struct IPUtils;

impl IPUtils {
    fn get_searcher() -> &'static Mutex<Searcher> {
        IP_SEARCHER.get_or_init(|| {
            let xid = xid::new().to_string();
            let mut temp_path = std::env::temp_dir();
            temp_path.push(format!("daoyi_ip2region_{}.xdb", xid));

            let mut file = File::create(&temp_path).expect("Failed to create temp ip2region file");
            file.write_all(IP2REGION_XDB).expect("Failed to write temp ip2region file");
            
            let searcher = Searcher::new(temp_path.to_str().unwrap()).expect("Failed to create Ip2Region Searcher");
            Mutex::new(searcher)
        })
    }

    /// 查询 IP 对应的地区编号
    pub fn get_area_id(ip: &str) -> Option<i32> {
        let searcher_lock = Self::get_searcher();
        let searcher = searcher_lock.lock().unwrap();
        match searcher.search(ip) {
            Ok(region_str) => {
                // The XDB in this project returns an Area ID string (e.g. "320100")
                match region_str.trim().parse::<i32>() {
                    Ok(id) => Some(id),
                    Err(_) => {
                        // If parsing fails, maybe it IS a standard region string?
                        // But based on Java code `Integer.parseInt`, it expects a number.
                        tracing::warn!("IP search result '{}' is not a valid Area ID for IP: {}", region_str, ip);
                        None
                    }
                }
            }
            Err(e) => {
                tracing::error!("IP search failed for {}: {}", ip, e);
                None
            }
        }
    }

    /// 查询 IP 对应的地区
    pub fn get_area(ip: &str) -> Option<&'static Area> {
        if let Some(id) = Self::get_area_id(ip) {
            AreaUtils::get_area(id)
        } else {
            None
        }
    }

    /// 获得 IP 对应的地址 (格式化后的字符串)
    /// 对应 Java 中可能没有直接对应的方法，但一般业务需要显示地址。
    /// Java AreaUtils.format(id)
    pub fn get_region(ip: &str) -> Option<String> {
        if let Some(id) = Self::get_area_id(ip) {
            Some(AreaUtils::format(id))
        } else {
            None
        }
    }

    /// 获取简化的地址（这里我们复用 format，因为 AreaUtils::format 已经处理了中国/全球的隐藏）
    pub fn get_simple_region(ip: &str) -> String {
        Self::get_region(ip).unwrap_or_else(|| "未知".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_area_id() {
        // 114.114.114.114 -> Nanjing, Jiangsu.
        // ID should be 320100 (Nanjing City) or similar.
        let id = IPUtils::get_area_id("114.114.114.114");
        println!("IP Area ID: {:?}", id);
        assert!(id.is_some());
    }

    #[test]
    fn test_ip_area_format() {
        let region = IPUtils::get_region("114.114.114.114");
        println!("IP Region: {:?}", region);
        // Expect "江苏省 南京市" or similar depending on AreaUtils::format logic
        assert!(region.is_some());
    }
}