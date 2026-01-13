# Daoyi Vue RS (道一管理系统)

[![Rust](https://img.shields.io/badge/Rust-1.92%2B-orange.svg)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-0.8-blue.svg)](https://github.com/tokio-rs/axum)
[![SeaORM](https://img.shields.io/badge/SeaORM-1.1-green.svg)](https://www.sea-ql.org/SeaORM/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

基于 **Rust** + **Axum** + **SeaORM** 构建的高性能、模块化、现代化企业级管理后台系统后端。本项目致力于在保持 Rust
极致性能与安全性的同时，提供类似 Java Spring Boot (RuoYi-Vue-Pro) 的高效开发体验。

## 📖 目录

- [系统架构](#-系统架构)
- [主要技术栈](#-主要技术栈)
- [核心技术亮点](#-核心技术亮点)
- [模块说明](#-模块说明)
- [快速开始](#-快速开始)
- [开发手册](#-开发手册)

## 🏗 系统架构

本项目采用 **Rust Workspace (Monorepo)** 结构，实现了高度的模块化。遵循 **领域驱动设计 (DDD)** 思想，确保代码结构清晰、易于扩展。

### 目录结构概览

```
daoyi-vue-rs/
├── crates/
│   ├── bins/               # 应用入口 (daoyi-server, daoyi-demo, daoyi-module-*)
│   └── libs/               # 核心库模块
│       ├── daoyi-api-infra/     # 基础设施模块 API (文件、配置、WebSocket)
│       ├── daoyi-entity-infra/  # 基础设施模块 Service (S3/FTP/DB 文件处理)
│       ├── daoyi-api-system/    # 系统模块 API (用户、权限、字典)
│       ├── daoyi-common-support/# 通用基础设施 (DB, Redis, Auth, WebSocket 框架)
│       └── daoyi-macros/        # 自定义过程宏 (事务、模型增强)
├── docs/                   # 文档与 SQL 脚本
└── resources/              # 配置文件 (application.yaml)
```

## 🛠 主要技术栈

* **后端核心**: Rust (2024 Edition) + Axum 0.8
* **数据库层**: SeaORM 1.1 (PostgreSQL)
* **缓存/消息**: Redis + Redis Pub/Sub (集群广播)
* **文件存储**: 支持 Local, Amazon S3 (MinIO, Aliyun OSS), FTP, SFTP, Database
* **即时通讯**: WebSocket (集成鉴权、集群广播)
* **基础设施**: Nacos (配置/注册), Tracing (日志), Snowflake (ID 生成)

## ✨ 核心技术亮点

### 1. 类 Spring 声明式事务 (`#[transactional]`)

通过自研过程宏实现。在 Service 方法上标记后，系统自动处理事务的开启、提交、回滚及上下文传播。业务逻辑只需专注于处理数据。

### 2. WebSocket 集群化消息框架

* **对标 RuoYi-Vue-Pro**: 采用 `type` + `content` 消息协议。
* **集群广播**: 基于 Redis Pub/Sub，推送消息自动分发至集群所有节点，确保分布式环境下客户端连接的透明性。
* **易扩展**: 简单的 `WebSocketMessageListener` trait 即可实现业务监听。

### 3. 多协议通用文件系统 (`FileClient`)

内置强大的文件操作客户端，只需在后台简单配置即可无缝切换存储方式：

* **云存储**: 深度集成 S3 协议（阿里云、华为云、MinIO 等）。
* **传统协议**: 支持 FTP, SFTP。
* **本地/DB**: 支持本地文件系统存储及数据库二进制存储。

### 4. 极简 Entity 开发

使用 `#[daoyi_model]` 自动填充审计字段，通过过程宏减少 80% 的 CRUD 样板代码，让 Rust 开发像 Java 一样高效。

## 📦 模块说明

* **daoyi-common-support**: 包含底层设施。**WebSocket 核心框架**、**Auth 鉴权**、**Redis 封装**。
* **daoyi-api-infra**: 基础设施 Web 层。包含文件上传下载路由、**WebSocket 消息端点 (`/infra/ws`)**。
* **daoyi-entity-infra**: 基础设施业务层。实现多种 **FileClient** 逻辑及配置管理。
* **daoyi-api-system**: 系统 Web 层。处理用户、角色、菜单、通知公告（含 **WebSocket 实时推送**）等逻辑。

## 🚀 快速开始

### 1. 运行环境

* **Rust**: 1.92+
* **PostgreSQL**: 12+
* **Redis**: 6+

### 2. 启动项目
```bash
# 1. 初始化数据库
psql -f docs/db/system-schema.sql
psql -f docs/db/infra-schema.sql

# 2. 修改配置 (resources/application.yaml)

# 3. 运行主程序
cargo run --bin daoyi-server
```

## 📚 开发手册

详细的开发指南（如何增加模块、添加 WebSocket 监听器、使用事务等）请参考：
👉 [**开发手册.md**](./开发手册.md)

## 🤝 参与贡献

欢迎提交 Issue 或 Pull Request，一起打造最强 Rust 后台系统！

## 📄 License
MIT License
