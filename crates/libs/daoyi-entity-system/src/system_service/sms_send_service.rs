use crate::system_entity::{prelude::*, system_sms_code, system_sms_log};
use daoyi_common_support::error::{ApiError, ApiResult};
use daoyi_common_support::sms::core::client::{SmsClient, SmsClientFactory};
use daoyi_common_support::sms::core::sms_client_factory;
use daoyi_common_support::vo::system_vo::{SmsCodeSendReqVO, SmsCodeValidateReqVO};
use daoyi_common_support::{database, id_util};
use rand::Rng;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn send_sms_code(req: SmsCodeSendReqVO, client_ip: &str) -> ApiResult<String> {
    let db = database::get_db_async().await;
    let now = sea_orm::sqlx::types::chrono::Local::now().naive_local();
    let today_start = sea_orm::sqlx::types::chrono::Local::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap();

    // 1. Frequency Check: 60s
    let last_code = SystemSmsCode::find()
        .filter(system_sms_code::Column::Mobile.eq(&req.mobile))
        .filter(system_sms_code::Column::Scene.eq(req.scene.to_string()))
        .order_by_desc(system_sms_code::Column::CreateTime)
        .one(&db)
        .await?;

    if let Some(last) = &last_code {
        if (now - last.create_time).num_seconds() < 60 {
            return Err(ApiError::biz("发送过于频繁，请稍后再试"));
        }
    }

    // 2. Daily Limit & Today Index
    let today_count = SystemSmsCode::find()
        .filter(system_sms_code::Column::Mobile.eq(&req.mobile))
        .filter(system_sms_code::Column::Scene.eq(req.scene.to_string()))
        .filter(system_sms_code::Column::CreateTime.gte(today_start))
        .count(&db)
        .await?;

    if today_count >= 10 {
        return Err(ApiError::biz("今日发送次数已达上限"));
    }
    let today_index = (today_count as i16) + 1;

    // 3. Generate Code
    let code: String = rand::rng().random_range(100000..999999).to_string();

    // 4. Save to DB
    let model = system_sms_code::ActiveModel {
        id: Set(id_util::xid()),
        mobile: Set(req.mobile.clone()),
        code: Set(code.clone()),
        scene: Set(req.scene.to_string()),
        today_index: Set(today_index),
        create_ip: Set(client_ip.to_string()),
        create_time: Set(now),
        used: Set(false),
        ..Default::default()
    };
    model.insert(&db).await?;

    // 5. Prepare Template Params
    let mut params = HashMap::new();
    params.insert("code".to_string(), code);

    // 6. Determine Template Code
    let template_code = match req.scene {
        1 => "USER_SMS_LOGIN",
        2 => "USER_SMS_RESET_PASS",
        _ => "USER_SMS_LOGIN",
    };

    // 7. Send
    send_single_sms_to_admin(&req.mobile, None, template_code, &params).await
}

pub async fn use_sms_code(req: SmsCodeValidateReqVO, client_ip: &str) -> ApiResult<()> {
    let db = database::get_db_async().await;
    let now = sea_orm::sqlx::types::chrono::Local::now().naive_local();

    // 1. Find latest unused code
    let last_code = SystemSmsCode::find()
        .filter(system_sms_code::Column::Mobile.eq(&req.mobile))
        .filter(system_sms_code::Column::Scene.eq(req.scene.to_string()))
        .filter(system_sms_code::Column::Used.eq(false))
        .order_by_desc(system_sms_code::Column::Id)
        .one(&db)
        .await?;

    let code_record = last_code.ok_or_else(|| ApiError::biz("验证码不存在或已失效"))?;

    // 2. Validate Code match
    if code_record.code != req.code {
        return Err(ApiError::biz("验证码错误"));
    }

    // 3. Validate Expiration (10 minutes)
    if (now - code_record.create_time).num_minutes() > 10 {
        return Err(ApiError::biz("验证码已过期"));
    }

    // 4. Mark used
    let mut active_model: system_sms_code::ActiveModel = code_record.into();
    active_model.used = Set(true);
    active_model.used_time = Set(Some(now));
    active_model.used_ip = Set(Some(client_ip.to_string()));

    active_model.update(&db).await?;

    Ok(())
}

pub async fn send_single_sms_to_admin(
    mobile: &str,
    user_id: Option<&str>,
    template_code: &str,
    template_params: &HashMap<String, String>,
) -> ApiResult<String> {
    let db = database::get_db_async().await;

    // 1. Get Template
    let template = super::system_sms_template_service::get_sms_template_by_code(template_code)
        .await?
        .ok_or_else(|| ApiError::biz(format!("短信模板({})不存在", template_code)))?;

    // 2. Get Client
    let client = get_sms_client(&template.channel_id).await?;

    // 3. Prepare Log Content
    let content = format_template_content(&template.content, template_params);
    let log_id = id_util::xid();
    let now = sea_orm::sqlx::types::chrono::Local::now().naive_local();

    // 4. Create Log (Sending)
    let log_model = system_sms_log::ActiveModel {
        id: Set(log_id.clone()),
        channel_id: Set(template.channel_id.clone()),
        channel_code: Set(template.channel_code.clone()),
        template_id: Set(template.id.clone()),
        template_code: Set(template.code.clone()),
        template_type: Set(template.r#type.clone()),
        template_content: Set(content),
        template_params: Set(serde_json::to_value(template_params).unwrap_or_default()),
        api_template_id: Set(template.api_template_id.clone()),
        mobile: Set(mobile.to_string()),
        user_id: Set(user_id.map(|s| s.to_string())),
        user_type: Set(Some("1".to_string())), // Admin
        send_status: Set(0), // Init
        send_time: Set(Some(now)),
        receive_status: Set(false),
        ..Default::default()
    };
    log_model.insert(&db).await?;

    // 5. Send SMS
    let send_result = client
        .send_sms(
            0, 
            mobile,
            &template.api_template_id,
            template_params,
        )
        .await;

    // 6. Update Log Status
    let mut log_update: system_sms_log::ActiveModel = SystemSmsLog::find_by_id(&log_id)
        .one(&db)
        .await?
        .unwrap()
        .into();

    let serial_no = match send_result {
        Ok(res) => {
            log_update.send_status = Set(10); // Success
            log_update.api_send_code = Set(Some("SUCCESS".to_string()));
            log_update.api_send_msg = Set(Some("发送成功".to_string()));
            log_update.api_serial_no = Set(res.serial_no.clone());
            log_update.api_request_id = Set(res.api_request_id.clone());
            log_update.update(&db).await?;
            res.serial_no.unwrap_or_default()
        }
        Err(e) => {
            log_update.send_status = Set(20); // Fail
            log_update.api_send_code = Set(Some("FAIL".to_string()));
            log_update.api_send_msg = Set(Some(e.to_string()));
            log_update.update(&db).await?;
            return Err(e);
        }
    };

    Ok(serial_no)
}

async fn get_sms_client(channel_id: &str) -> ApiResult<Arc<dyn SmsClient>> {
    let factory = sms_client_factory::get();
    if let Some(client) = factory.get_sms_client(channel_id) {
        return Ok(client);
    }

    // Load from DB
    let channel = super::system_sms_channel_service::get_sms_channel(channel_id)
        .await?
        .ok_or_else(|| ApiError::biz("短信渠道不存在"))?;

    let client = factory.create_or_update_sms_client(channel.into());
    Ok(client)
}

fn format_template_content(content: &str, params: &HashMap<String, String>) -> String {
    let mut result = content.to_string();
    for (k, v) in params {
        let placeholder = format!("${{{}}}", k);
        result = result.replace(&placeholder, v);
    }
    result
}
