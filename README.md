# Daoyi Vue RS (道一管理系统)

[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![SeaORM](https://img.shields.io/badge/SeaORM-1.1-green.svg)](https://www.sea-ql.org/SeaORM/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

基于 **Rust** + **Axum** + **SeaORM** 构建的高性能、模块化、现代化企业级管理后台系统后端。本项目致力于在保持 Rust 极致性能与安全性的同时，提供类似 Java Spring Boot 的高效开发体验。

## 📖 目录

- [系统架构](#-系统架构)
- [主要技术栈](#-主要技术栈)
- [核心技术亮点](#-核心技术亮点)
- [模块说明](#-模块说明)
- [快速开始](#-快速开始)
- [部署指南](#-部署指南)

## 🏗 系统架构

本项目采用 **Rust Workspace (Monorepo)** 结构进行管理，实现了高度的模块化与解耦。系统遵循 **领域驱动设计 (DDD)** 的分层思想，将核心业务逻辑、通用基础设施和应用入口分离。

### 目录结构概览

```
daoyi-vue-rs/
├── crates/
│   ├── bins/               # 应用入口 (Application Layer)
│   │   ├── daoyi-server/   # 主服务器入口，组装各模块
│   │   ├── daoyi-demo/     # 演示应用
│   │   └── ...
│   └── libs/               # 库模块 (Domain & Infrastructure Layer)
│       ├── daoyi-api-system/    # 系统模块 API 路由与控制器
│       ├── daoyi-entity-system/ # 系统模块 实体定义与 Service 层
│       ├── daoyi-common-support/# 通用基础设施 (DB, Redis, Auth, Log...)
│       ├── daoyi-macros/        # 自定义过程宏 (核心黑科技)
│       └── ...
├── docs/                   # 文档与数据库脚本
└── resources/              # 配置文件
```

## 🛠 主要技术栈

| 类别 | 技术 | 说明 |
| --- | --- | --- |
| **编程语言** | Rust (2024 Edition) | 内存安全，无 GC，极致性能 |
| **Web 框架** | Axum 0.8 | 基于 Tokio 的人体工学且模块化的 Web 框架 |
| **ORM 框架** | SeaORM 1.1 | 异步动态 ORM，支持 SQLx，类型安全 |
| **数据库** | PostgreSQL | 强大的开源关系型数据库 |
| **缓存/队列** | Redis / Deadpool | 高性能缓存与连接池管理 |
| **异步运行时** | Tokio | Rust 最流行的异步运行时 |
| **配置中心** | Nacos (SDK) | (可选) 支持集成 Nacos 进行配置管理与服务发现 |
| **日志监控** | Tracing | 结构化日志收集与分布式追踪 |
| **API 文档** | OpenAPI / Swagger | (规划中) 自动生成 API 文档 |

## ✨ 核心技术亮点

本项目不仅仅是一个简单的 CRUD 后台，我们在**开发体验 (Developer Experience)** 上做了大量优化：

1.  **类 Spring 的事务管理 (`#[transactional]`)**:
    *   自主研发了 `#[transactional]` 过程宏，支持类似 Spring 的声明式事务。
    *   **自动传播**：支持事务嵌套与上下文传播 (Propagation.REQUIRED)。
    *   **智能回滚**：方法返回 `Err` 自动回滚，`Ok` 自动提交。
    *   **无感替换**：自动将 `database::get().await` 替换为当前事务上下文，业务代码零侵入。

2.  **极简 Entity 开发 (`daoyi-macros`)**:
    *   `#[daoyi_model]`: 自动注入审计字段（`create_time`, `update_time`, `creator`, `updater`, `deleted`, `tenant_id`）。
    *   `ActiveModelBehavior`: 自动处理主键生成 (Snowflake/UUID)、密码加密、审计时间更新，无需手动编写重复代码。
    *   `insert_many_auto`: 支持批量插入时自动填充上述默认字段。

3.  **高度模块化**:
    *   业务模块 (如 `system`, `demo`) 物理隔离，通过 Cargo 依赖组合。
    *   API 定义 (`daoyi-api-*`) 与 业务实现 (`daoyi-entity-*`) 分离，清晰的依赖边界。

4.  **企业级基础设施**:
    *   统一的 `ApiResult` 错误处理与响应封装。
    *   内置 RBAC 权限控制 (用户-角色-菜单-部门-岗位)。
    *   多租户架构设计 (Tenant Support)。
    *   集成 Nacos 配置中心，支持动态配置。

## 📦 模块说明

*   **daoyi-common-support**: 系统的基石。包含数据库连接池封装 (`DbGuard`)、Redis 客户端、全局错误定义、JWT 认证、日志配置、工具类等。
*   **daoyi-macros**: 核心宏库。提供了 `transactional`, `daoyi_model` 等过程宏，极大简化代码。
*   **daoyi-entity-system**: 系统核心领域的 Entity 定义 (Model) 和 Service 实现 (DAO)。包含用户、角色、菜单等核心逻辑。
*   **daoyi-api-system**: 系统核心模块的 Web 层。定义 Router 和 Controller (Handler)，处理 HTTP 请求与参数校验。

## 🚀 快速开始

### 1. 环境准备
*   **Rust**: 安装最新版 Rust (`rustup update`)
*   **Database**: 准备 PostgreSQL 数据库。
*   **Redis**: 准备 Redis 服务。

### 2. 初始化数据库
执行 `docs/db/system-schema.sql` (以及 `demo-schema.sql` 如需) 初始化表结构和基础数据。

### 3. 配置
在项目根目录或 `resources/` 下创建配置文件 (参考 `resources/application.yaml`)。
设置环境变量 `DY__SERVER__PORT` 等或直接修改配置文件中的数据库/Redis 连接信息。

### 4. 运行
```bash
# 开发模式运行 demo
cargo run --bin daoyi-demo

# 或者运行主 server
cargo run --bin daoyi-server
```

## 🚢 部署指南

### Docker 构建

```dockerfile
# (示例 Dockerfile)
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin daoyi-server

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/daoyi-server /usr/local/bin/
CMD ["daoyi-server"]
```

### 手动编译
```bash
cargo build --release --bin daoyi-server
# 产物位于 target/release/daoyi-server
```

### 注意事项
*   在使用 `#[transactional]` 宏的方法内部，获取到的 `db` 对象为 `DbGuard` 值类型。传给 SeaORM 的方法（如 `insert`, `find`）时，**必须使用引用** (`&db`)。

## 🤝 贡献
欢迎提交 Issue 和 PR！

## 📄 License
MIT License