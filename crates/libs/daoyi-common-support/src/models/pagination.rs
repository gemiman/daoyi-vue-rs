use crate::serde::deserialize_numer;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
pub struct PaginationParams {
    #[validate(range(min = 1, message = "页码必须大于0"))]
    #[serde(default = "default_page", deserialize_with = "deserialize_numer")]
    pub page: u64,
    #[validate(range(min = 1, max = 100, message = "分页大小必须在1~100之间"))]
    #[serde(default = "default_size", deserialize_with = "deserialize_numer")]
    pub size: u64,
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}

fn default_size() -> u64 {
    DEFAULT_SIZE
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    page: u64,
    size: u64,
    total: u64,
    items: Vec<T>,
    total_page: u64,
}

impl<T> Page<T> {
    pub fn new(page: u64, size: u64, total: u64, items: Vec<T>) -> Self {
        Page {
            page,
            size,
            total,
            items,
            total_page: if size == 0 {
                0
            } else {
                total / size + if total % size == 0 { 0 } else { 1 }
            },
        }
    }
    pub fn from_pagination(pagination: PaginationParams, total: u64, items: Vec<T>) -> Self {
        Page::new(pagination.page, pagination.size, total, items)
    }
}
