use crate::serde::deserialize_numer;
use serde::{Deserialize, Serialize};
use validator::Validate;

const DEFAULT_PAGE: u64 = 1;
const DEFAULT_SIZE: u64 = 10;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    #[validate(range(min = 1, message = "页码必须大于0"))]
    #[serde(default = "default_page", deserialize_with = "deserialize_numer")]
    pub page_no: u64,
    #[validate(range(min = 1, max = 100, message = "分页大小必须在1~100之间"))]
    #[serde(default = "default_size", deserialize_with = "deserialize_numer")]
    pub page_size: u64,
}

fn default_page() -> u64 {
    DEFAULT_PAGE
}

pub fn default_size() -> u64 {
    DEFAULT_SIZE
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResult<T> {
    page_no: u64,
    page_size: u64,
    total: u64,
    list: Vec<T>,
    total_page: u64,
}

impl<T> PageResult<T> {
    pub fn new(page_no: u64, page_size: u64, total: u64, list: Vec<T>) -> Self {
        PageResult {
            page_no,
            page_size,
            total,
            list,
            total_page: if page_size == 0 {
                0
            } else {
                total / page_size + if total % page_size == 0 { 0 } else { 1 }
            },
        }
    }
    pub fn from_pagination(pagination: &PaginationParams, total: u64, list: Vec<T>) -> Self {
        PageResult::new(pagination.page_no, pagination.page_size, total, list)
    }
    pub fn empty(pagination: &PaginationParams) -> Self {
        PageResult::new(pagination.page_no, pagination.page_size, 0, vec![])
    }

    pub fn map<U, F>(self, f: F) -> PageResult<U>
    where
        F: FnMut(T) -> U,
    {
        let new_list: Vec<U> = self.list.into_iter().map(f).collect();
        PageResult {
            page_no: self.page_no,
            page_size: self.page_size,
            total: self.total,
            list: new_list,
            total_page: self.total_page,
        }
    }
}
