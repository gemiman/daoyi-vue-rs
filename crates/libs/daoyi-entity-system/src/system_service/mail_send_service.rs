use daoyi_common_support::enumeration::UserTypeEnum;
use daoyi_common_support::error::ApiResult;
use std::collections::HashMap;

pub async fn send_single_mail_to_admin(
    user_id: &str,
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_mail(
        to_mails,
        cc_mails,
        bcc_mails,
        user_id,
        UserTypeEnum::Admin,
        template_code,
        template_params,
    )
    .await
}

pub async fn send_single_mail_to_member(
    user_id: &str,
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    send_single_mail(
        to_mails,
        cc_mails,
        bcc_mails,
        user_id,
        UserTypeEnum::Member,
        template_code,
        template_params,
    )
    .await
}

pub async fn send_single_mail(
    to_mails: &Vec<String>,
    cc_mails: &Vec<String>,
    bcc_mails: &Vec<String>,
    user_id: &str,
    user_type: UserTypeEnum,
    template_code: &str,
    template_params: &Option<HashMap<String, String>>,
) -> ApiResult<String> {
    // 1.1 校验邮箱模版是否合法
    // 1.2 校验邮箱账号是否合法
    // 1.3 校验邮件参数是否缺失
    // 2. 组装邮箱
    // 创建发送日志。如果模板被禁用，则不发送短信，只记录日志
    // 发送 MQ 消息，异步执行发送短信
    todo!()
}
