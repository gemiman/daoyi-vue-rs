# Gemini Context & Instructions

## Core Mandates (用户核心指令)
- **语言偏好**：**必须**始终使用**中文**（Chinese）进行思考、回答、编写文档和代码注释。
- **记忆文件**：本文件 (`GEMINI.md`) 用于存储长期记忆和上下文，每次对话开始时应读取此文件。

## Project Context (项目上下文)
- **Project**: daoyi-vue-rs
- **OS**: Darwin (macOS)
- **Key Features**:
  - `#[transactional]` 宏：已实现类似 Spring 的事务传播机制。
    - **注意**：在 `#[transactional]` 方法内，`database::get().await` 返回 `DbGuard` 值类型。调用 SeaORM 方法时需使用引用 `&db` (例如 `User::insert(&db).await`)。

## Previous Memories (历史记忆)
- **Docker**: Created Dockerfile for FreeSWITCH build from source.
- **Environment**: Local Darwin system, SSH tunnel to remote FreeSWITCH.
- **Requirements**: 前后端接口高度对齐，前端界面完全汉化。
