// Author: Gemiman
//
//! 上下文模块 - 用于在请求处理过程中传递 token 信息
//!
//! 注意：在实际应用中，建议通过框架的请求扩展（如 Axum 的 Extension）
//! 来传递上下文，而不是使用 thread_local。这里提供的是一个简单的实现。

use crate::enumeration::UserTypeEnum;
use std::future::Future;
use std::sync::Arc;

tokio::task_local! {
    static CONTEXT: HttpRequestContext;
}

/// http request 上下文 | http request Context
///
/// 用于在请求处理过程中传递 Token 相关信息
/// Used to pass token-related information during request processing
///
/// # 字段说明 | Field Description
/// - `token`: 当前请求的 token | Current request's token
/// - `token_info`: Token 详细信息 | Token detailed information
/// - `login_id`: 登录用户 ID | Logged-in user ID
#[derive(Debug, Clone)]
pub struct HttpRequestContext {
    /// 登录 IP | Login IP
    pub user_ip: Option<Arc<String>>,
    /// Tracing ID
    pub tracing_id: Option<Arc<String>>,
    /// User-Agent
    pub user_agent: Option<Arc<String>>,

    /// Request Method
    pub request_method: Option<Arc<String>>,
    /// Request URL
    pub request_url: Option<Arc<String>>,

    /// 当前请求的 token | Current request's token
    pub token: Option<Arc<String>>,

    /// 当前请求的 租户信息 | Current request's tenant info
    pub tenant_id: Option<Arc<String>>,

    /// 登录 ID | Login ID
    pub login_id: Option<Arc<String>>,

    /// 用户类型 | User Type
    pub user_type: Option<UserTypeEnum>,

    /// 是否忽略租户
    pub ignore_tenant: Option<bool>,
}

/// HttpRequestContext 构建器
#[derive(Debug, Clone, Default)]
pub struct HttpRequestContextBuilder {
    user_ip: Option<Arc<String>>,
    tracing_id: Option<Arc<String>>,
    user_agent: Option<Arc<String>>,
    request_method: Option<Arc<String>>,
    request_url: Option<Arc<String>>,
    token: Option<Arc<String>>,
    tenant_id: Option<Arc<String>>,
    login_id: Option<Arc<String>>,
    user_type: Option<UserTypeEnum>,
    ignore_tenant: Option<bool>,
}

impl HttpRequestContextBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置登录 IP
    pub fn user_ip(mut self, user_ip: impl Into<String>) -> Self {
        self.user_ip = Some(Arc::new(user_ip.into()));
        self
    }

    /// 设置 Tracing ID
    pub fn tracing_id(mut self, tracing_id: impl Into<String>) -> Self {
        self.tracing_id = Some(Arc::new(tracing_id.into()));
        self
    }
    /// 设置 User-Agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(Arc::new(user_agent.into()));
        self
    }

    /// 设置 Request Method
    pub fn request_method(mut self, request_method: impl Into<String>) -> Self {
        self.request_method = Some(Arc::new(request_method.into()));
        self
    }

    /// 设置 Request URL
    pub fn request_url(mut self, request_url: impl Into<String>) -> Self {
        self.request_url = Some(Arc::new(request_url.into()));
        self
    }

    /// 设置 token
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(Arc::new(token.into()));
        self
    }

    /// 设置租户 ID
    pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(Arc::new(tenant_id.into()));
        self
    }

    /// 设置登录 ID
    pub fn login_id(mut self, login_id: impl Into<String>) -> Self {
        self.login_id = Some(Arc::new(login_id.into()));
        self
    }

    /// 设置用户类型
    pub fn user_type(mut self, user_type: UserTypeEnum) -> Self {
        self.user_type = Some(user_type);
        self
    }

    /// 设置是否忽略租户
    pub fn ignore_tenant(mut self, ignore_tenant: bool) -> Self {
        self.ignore_tenant = Some(ignore_tenant);
        self
    }

    /// 构建 HttpRequestContext
    pub fn build(self) -> HttpRequestContext {
        HttpRequestContext {
            user_ip: self.user_ip,
            tracing_id: self.tracing_id,
            user_agent: self.user_agent,
            request_method: self.request_method,
            request_url: self.request_url,
            token: self.token,
            tenant_id: self.tenant_id,
            login_id: self.login_id,
            user_type: self.user_type,
            ignore_tenant: self.ignore_tenant,
        }
    }
}
impl HttpRequestContext {
    pub fn builder() -> HttpRequestContextBuilder {
        HttpRequestContextBuilder::new()
    }
    pub fn new() -> Self {
        Self {
            user_ip: None,
            tracing_id: None,
            user_agent: None,
            request_method: None,
            request_url: None,
            token: None,
            tenant_id: None,
            login_id: None,
            user_type: None,
            ignore_tenant: None,
        }
    }

    /// 在指定的上下文中执行异步函数
    ///
    /// # 参数
    /// - `ctx`: 要临时设置的上下文
    /// - `f`: 要执行的异步闭包或函数
    ///
    /// # 返回
    /// 返回异步闭包的执行结果
    pub async fn scope<F, Fut, T>(ctx: HttpRequestContext, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        CONTEXT.scope(ctx, f()).await
    }

    /// 兼容旧方法名，但在 task_local 模式下等同于 scope
    pub async fn execute_with_other_context_async<F, Fut, T>(ctx: HttpRequestContext, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        Self::scope(ctx, f).await
    }

    /// 获取当前上下文 | Get Current Context
    ///
    /// # 返回 | Returns
    /// 当前任务的上下文，如果不存在则返回 None
    pub fn get_current() -> Option<HttpRequestContext> {
        CONTEXT.try_with(|c| c.clone()).ok()
    }

    pub fn get_login_id() -> Option<String> {
        CONTEXT
            .try_with(|c| c.login_id.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_user_ip() -> Option<String> {
        CONTEXT
            .try_with(|c| c.user_ip.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }
    pub fn get_tracing_id() -> Option<String> {
        CONTEXT
            .try_with(|c| c.tracing_id.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }
    pub fn get_user_agent() -> Option<String> {
        CONTEXT
            .try_with(|c| c.user_agent.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_request_method() -> Option<String> {
        CONTEXT
            .try_with(|c| c.request_method.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_request_url() -> Option<String> {
        CONTEXT
            .try_with(|c| c.request_url.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_token() -> Option<String> {
        CONTEXT
            .try_with(|c| c.token.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_login_id_arc() -> Option<Arc<String>> {
        CONTEXT.try_with(|c| c.login_id.clone()).ok().flatten()
    }

    pub fn get_tenant_id() -> Option<String> {
        CONTEXT
            .try_with(|c| c.tenant_id.as_ref().map(|s| s.as_ref().clone()))
            .ok()
            .flatten()
    }

    pub fn get_tenant_id_arc() -> Option<Arc<String>> {
        CONTEXT.try_with(|c| c.tenant_id.clone()).ok().flatten()
    }
    pub fn get_login_id_as_string() -> anyhow::Result<String> {
        Self::get_login_id().ok_or_else(|| anyhow::anyhow!("login_id is None"))
    }

    pub fn get_user_ip_as_string() -> String {
        Self::get_user_ip().unwrap_or(String::from("0.0.0.0"))
    }

    pub fn get_tracing_id_as_string() -> String {
        Self::get_tracing_id().unwrap_or(String::from("0"))
    }
    pub fn get_user_agent_as_string() -> String {
        Self::get_user_agent().unwrap_or(String::from("unknown"))
    }
    pub fn get_request_method_as_string() -> String {
        Self::get_request_method().unwrap_or(String::from("GET"))
    }
    pub fn get_request_url_as_string() -> String {
        Self::get_request_url().unwrap_or(String::from("/"))
    }

    pub fn get_token_as_string() -> anyhow::Result<String> {
        Self::get_token().ok_or_else(|| anyhow::anyhow!("login_id is None"))
    }

    pub fn get_tenant_id_as_string() -> anyhow::Result<String> {
        Self::get_tenant_id().ok_or_else(|| anyhow::anyhow!("tenant_id is None"))
    }

    pub fn get_ignore_tenant() -> bool {
        CONTEXT
            .try_with(|c| c.ignore_tenant.unwrap_or(false))
            .unwrap_or(false)
    }

    pub fn get_user_type() -> UserTypeEnum {
        CONTEXT
            .try_with(|c| c.user_type.unwrap_or(UserTypeEnum::Admin))
            .unwrap_or(UserTypeEnum::Admin)
    }
}

impl Default for HttpRequestContext {
    fn default() -> Self {
        Self::new()
    }
}
