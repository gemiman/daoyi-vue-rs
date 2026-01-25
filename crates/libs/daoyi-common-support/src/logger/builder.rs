use crate::error::ApiResult;
use crate::logger::record_operate_log;
use serde::Serialize;
use serde_json::Value;

/// 操作日志构建器
/// 用于优雅地构建和记录操作日志，自动计算对象差异或记录对象快照
///
/// # 示例 (新增场景)
/// ```rust
/// OperateLogBuilder::new("用户模块", "新增用户")
///     .biz_id(user.id)
///     .action(format!("创建了用户: {}", user.username))
///     .detail(&user) // 记录完整对象快照
///     .record()
///     .await?;
/// ```
///
/// # 示例 (更新场景)
/// ```rust
/// OperateLogBuilder::new("用户模块", "更新用户")
///     .biz_id(new_user.id)
///     .action("修改了用户信息")
///     .diff(&old_user, &new_user) // 自动计算差异并记录
///     .record()
///     .await?;
/// ```
///
/// # 示例 (删除场景)
/// ```rust
/// OperateLogBuilder::new("用户模块", "删除用户")
///     .biz_id(user_id)
///     .action("删除了用户")
///     .detail(&old_user) // 记录被删除前的对象快照
///     .record()
///     .await?;
/// ```
pub struct OperateLogBuilder {
    r#type: String,
    sub_type: String,
    biz_id: String,
    action: String,
    extra: Option<Value>,
}

impl OperateLogBuilder {
    /// 创建一个新的构建器
    ///
    /// # 参数
    /// * `r#type` - 模块类型
    /// * `sub_type` - 操作类型
    pub fn new(r#type: &str, sub_type: &str) -> Self {
        Self {
            r#type: r#type.to_string(),
            sub_type: sub_type.to_string(),
            biz_id: "".to_string(),
            action: "".to_string(),
            extra: None,
        }
    }

    /// 设置业务ID
    pub fn biz_id(mut self, biz_id: impl ToString) -> Self {
        self.biz_id = biz_id.to_string();
        self
    }

    /// 设置操作描述
    pub fn action(mut self, action: impl ToString) -> Self {
        self.action = action.to_string();
        self
    }

    /// 设置额外信息
    pub fn extra(mut self, extra: Value) -> Self {
        self.extra = Some(extra);
        self
    }

    /// 自动计算两个对象的差异并设置为 extra
    /// 仅记录变更的字段，并将变更详情追加到 action 中
    pub fn diff<T: Serialize>(mut self, old: &T, new: &T) -> Self {
        let old_val = serde_json::to_value(old).unwrap_or(Value::Null);
        let new_val = serde_json::to_value(new).unwrap_or(Value::Null);

        let mut diffs = serde_json::Map::new();
        let mut changes = Vec::new();

        if let (Value::Object(old_map), Value::Object(new_map)) = (&old_val, &new_val) {
            for (k, v_new) in new_map {
                // 忽略一些无需比较的字段
                if k == "update_time" || k == "create_time" || k == "update_by" || k == "create_by"
                {
                    continue;
                }

                let v_old = old_map.get(k).unwrap_or(&Value::Null);

                if v_new != v_old {
                    diffs.insert(k.clone(), v_new.clone());

                    let old_str = value_to_simple_string(v_old);
                    let new_str = value_to_simple_string(v_new);
                    changes.push(format!("将{}从{}更新为{}", k, old_str, new_str));
                }
            }
        }

        if !diffs.is_empty() {
            self.extra = Some(Value::Object(diffs));
            if !changes.is_empty() {
                let suffix = changes.join("，");
                if self.action.is_empty() {
                    self.action = suffix;
                } else {
                    self.action = format!("{}，{}", self.action, suffix);
                }
            }
        }
        self
    }

    /// 记录全量详情
    /// 适用于【新增】或【删除】场景，将整个对象记录到 extra 中
    pub fn detail<T: Serialize>(mut self, data: &T) -> Self {
        if let Ok(val) = serde_json::to_value(data) {
            self.extra = Some(val);
        }
        self
    }

    /// 记录日志（异步）
    pub async fn record(self) -> ApiResult<()> {
        record_operate_log(
            &self.r#type,
            &self.sub_type,
            &self.biz_id,
            &self.action,
            self.extra,
        )
        .await
    }
}

fn value_to_simple_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => "空".to_string(),
        _ => v.to_string(),
    }
}

/// 计算两个对象的差异
/// 返回包含变更字段的新值 JSON 对象
pub fn calculate_diff<T: Serialize>(old: &T, new: &T) -> serde_json::Value {
    let old_val = serde_json::to_value(old).unwrap_or(Value::Null);
    let new_val = serde_json::to_value(new).unwrap_or(Value::Null);

    let mut diffs = serde_json::Map::new();

    if let (Value::Object(old_map), Value::Object(new_map)) = (&old_val, &new_val) {
        for (k, v_new) in new_map {
            // 忽略一些无需比较的字段，如 update_time 等（可选，根据需求）
            if k == "update_time" || k == "create_time" || k == "update_by" || k == "create_by" {
                continue;
            }

            if let Some(v_old) = old_map.get(k) {
                if v_new != v_old {
                    diffs.insert(k.clone(), v_new.clone());
                }
            } else {
                // 新增字段（理论上Serialize同一类型不会出现，除非Option None -> Some）
                diffs.insert(k.clone(), v_new.clone());
            }
        }
    }

    Value::Object(diffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct User {
        id: i32,
        name: String,
        age: i32,
        email: Option<String>,
    }

    #[test]
    fn test_calculate_diff() {
        let old = User {
            id: 1,
            name: "Alice".to_string(),
            age: 30,
            email: Some("alice@example.com".to_string()),
        };

        let mut new = old.clone();
        new.age = 31;
        new.email = None;

        let diff = calculate_diff(&old, &new);
        let diff_obj = diff.as_object().unwrap();

        assert_eq!(diff_obj.len(), 2);
        assert_eq!(diff_obj.get("age").unwrap(), &serde_json::json!(31));
        assert_eq!(diff_obj.get("email").unwrap(), &serde_json::json!(null));
        assert!(!diff_obj.contains_key("name"));
    }

    #[test]
    fn test_builder_diff_action_append() {
        let old = User {
            id: 1,
            name: "武大郎".to_string(),
            age: 30,
            email: None,
        };

        let mut new = old.clone();
        new.name = "李世民".to_string();
        new.age = 31;

        let builder = OperateLogBuilder::new("test", "update")
            .action("更新了用户")
            .diff(&old, &new);

        let action = builder.action;
        // 注意：HashMap 迭代顺序不确定，可能先 age 后 name，或反之
        // 因此检查包含关系
        assert!(action.contains("更新了用户"));
        assert!(action.contains("将name从武大郎更新为李世民"));
        assert!(action.contains("将age从30更新为31"));
    }

    #[test]
    fn test_builder_detail() {
        let user = User {
            id: 1,
            name: "Bob".to_string(),
            age: 25,
            email: None,
        };

        let builder = OperateLogBuilder::new("test", "create").detail(&user);

        let extra = builder.extra.unwrap();
        assert_eq!(extra.get("name").unwrap(), "Bob");
        assert_eq!(extra.get("age").unwrap(), 25);
    }
}
