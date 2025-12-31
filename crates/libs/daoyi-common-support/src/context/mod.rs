// Author: Gemiman
//
//! 上下文模块 - 用于在请求处理过程中传递 token 信息
//!
//! 注意：在实际应用中，建议通过框架的请求扩展（如 Axum 的 Extension）
//! 来传递上下文，而不是使用 thread_local。这里提供的是一个简单的实现。

use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<HttpRequestContext >> = RefCell::new(None);
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
    /// 当前请求的 token | Current request's token
    pub token: Option<String>,

    /// 当前请求的 租户信息 | Current request's tenant info
    pub tenant_id: Option<String>,

    /// 登录 ID | Login ID
    pub login_id: Option<String>,

    /// 是否忽略租户
    pub ignore_tenant: Option<bool>,
}

/// HttpRequestContext 构建器
#[derive(Debug, Clone, Default)]
pub struct HttpRequestContextBuilder {
    token: Option<String>,
    tenant_id: Option<String>,
    login_id: Option<String>,
    ignore_tenant: Option<bool>,
}

impl HttpRequestContextBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置 token
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// 设置租户 ID
    pub fn tenant_id(mut self, tenant_id: impl Into<String>) -> Self {
        self.tenant_id = Some(tenant_id.into());
        self
    }

    /// 设置登录 ID
    pub fn login_id(mut self, login_id: impl Into<String>) -> Self {
        self.login_id = Some(login_id.into());
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
            token: self.token,
            tenant_id: self.tenant_id,
            login_id: self.login_id,
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
            token: None,
            tenant_id: None,
            login_id: None,
            ignore_tenant: None,
        }
    }

    /// let result = HttpRequestContext::execute_with_other_context(
    /// ctx,
    /// || {
    /// // 这里的代码会在指定的上下文中执行
    /// do_something()
    /// }
    /// );
    ///
    pub fn execute_with_other_context<F, T>(ctx: HttpRequestContext, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        // 保存当前上下文
        let old_ctx = Self::get_current();

        // 设置新上下文
        Self::set_current(ctx);

        // 执行函数
        let result = f();

        // 恢复原来的上下文
        if let Some(old) = old_ctx {
            Self::set_current(old);
        } else {
            Self::clear();
        }

        result
    }

    /// 在指定的上下文中执行异步函数
    ///
    /// # 参数
    /// - `ctx`: 要临时设置的上下文
    /// - `f`: 要执行的异步闭包或函数
    ///
    /// # 返回
    /// 返回异步闭包的执行结果
    ///
    /// # 示例
    ///
    pub async fn execute_with_other_context_async<F, Fut, T>(ctx: HttpRequestContext, f: F) -> T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        // 保存当前上下文
        let old_ctx = Self::get_current();

        // 设置新上下文
        Self::set_current(ctx);

        // 执行异步函数
        let result = f().await;

        // 恢复原来的上下文
        if let Some(old) = old_ctx {
            Self::set_current(old);
        } else {
            Self::clear();
        }

        result
    }

    /// 设置当前上下文 | Set Current Context
    ///
    /// # 参数 | Parameters
    /// - `ctx`: 要设置的上下文 | Context to set
    pub fn set_current(ctx: HttpRequestContext) {
        CONTEXT.with(|c| {
            *c.borrow_mut() = Some(ctx);
        });
    }

    /// 获取当前上下文 | Get Current Context
    ///
    /// # 返回 | Returns
    /// 当前线程的上下文，如果不存在则返回 None
    /// Current thread's context, or None if not exists
    pub fn get_current() -> Option<HttpRequestContext> {
        CONTEXT.with(|c| c.borrow().clone())
    }
    pub async fn get_login_id() -> Option<String> {
        if let Ok(login_id) = Self::get_login_id_as_string().await {
            return Some(login_id);
        }
        None
    }

    pub async fn get_tenant_id() -> Option<String> {
        if let Ok(tenant_id) = Self::get_tenant_id_as_string().await {
            return Some(tenant_id);
        }
        None
    }
    pub async fn get_login_id_as_string() -> anyhow::Result<String> {
        Self::get_current()
            .and_then(|ctx| ctx.login_id)
            .ok_or_else(|| anyhow::anyhow!("login_id is None"))
    }

    pub async fn get_tenant_id_as_string() -> anyhow::Result<String> {
        Self::get_current()
            .and_then(|ctx| ctx.tenant_id)
            .ok_or_else(|| anyhow::anyhow!("tenant_id is None"))
    }

    pub fn get_ignore_tenant() -> bool {
        Self::get_current()
            .and_then(|ctx| ctx.ignore_tenant)
            .ok_or_else(|| anyhow::anyhow!("ignore_tenant is None"))
            .unwrap_or(false)
    }

    /// 清除当前上下文 | Clear Current Context
    ///
    /// 清除当前线程的上下文信息
    /// Clear current thread's context information
    pub fn clear() {
        CONTEXT.with(|c| {
            *c.borrow_mut() = None;
        });
    }
}

impl Default for HttpRequestContext {
    fn default() -> Self {
        Self::new()
    }
}
