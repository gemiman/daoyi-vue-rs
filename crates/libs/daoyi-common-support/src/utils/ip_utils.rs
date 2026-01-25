use std::sync::{OnceLock, Mutex};
use std::fs::File;
use std::io::Write;
use ip2region::Searcher;

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

    /// 获得 IP 对应的地址
    /// 返回格式：国家|区域|省份|城市|ISP
    pub fn get_region(ip: &str) -> Option<String> {
        let searcher_lock = Self::get_searcher();
        let searcher = searcher_lock.lock().unwrap();
        // memory_search might return a Result<String> or Result<Location>
        match searcher.search(ip) {
            Ok(region) => Some(region), // Assuming it returns String
            Err(e) => {
                tracing::error!("IP search failed for {}: {}", ip, e);
                None
            }
        }
    }

    /// 获取简化的地址（省份 城市）
    pub fn get_simple_region(ip: &str) -> String {
        if let Some(region) = Self::get_region(ip) {
            // Region format: Country|Region|Province|City|ISP
            // e.g., 中国|0|上海|上海市|联通
            let parts: Vec<&str> = region.split('|').collect();
            if parts.len() >= 5 {
                let province = parts[2];
                let city = parts[3];
                // Simplify: if province == city, just return city.
                // remove "0"
                let mut res = String::new();
                if province != "0" && province != city {
                    res.push_str(province);
                    res.push(' ');
                }
                if city != "0" {
                    res.push_str(city);
                }
                if res.is_empty() {
                     return region; // Fallback
                }
                return res.trim().to_string();
            }
            region
        } else {
            "未知".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ip_region() {
        // Use a known public IP, e.g., 114.114.114.114 (Nanjing, China)
        // or 8.8.8.8 (US)
        // Note: this test depends on the xdb data quality.
        let region = IPUtils::get_region("114.114.114.114");
        println!("Region: {:?}", region);
        assert!(region.is_some());
    }
}
