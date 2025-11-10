# PubChat - Rust即时通讯系统 / PubChat - Rust Instant Messaging System

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/github/license/casparcao/pubchat)

[中文文档](#pubchat---rust即时通讯系统) • [English Version](#pubchat---rust-instant-messaging-system)

## 项目简介 / Project Overview

PubChat 是一个基于 Rust 语言开发的高性能即时通讯系统，采用了现代化的异步架构和微服务设计理念。该系统提供了完整的用户认证、好友管理、实时消息传输以及文件分享功能。

PubChat is a high-performance instant messaging system built with Rust, featuring modern asynchronous architecture and microservices design philosophy. It provides complete user authentication, contact management, real-time messaging, and file sharing capabilities.

## 功能特性 / Features

### 已实现功能 / Implemented Features
- 🔄 微服务架构 / Microservice Architecture
- 👤 用户注册与登录 (JWT 认证) / User Registration and Login (JWT Authentication)
- 💬 实时消息传输 / Real-time Message Transmission
- 📇 好友管理系统 / Contact Management System
- 📁 文件上传与下载 / File Upload and Download
- 🔐 权限验证中间件 / Authentication Middleware
- 🗃️ 消息持久化存储 / Message Persistence Storage
- 📱 终端用户界面 (TUI) / Terminal User Interface (TUI)
- 📡 RabbitMQ 消息队列 / RabbitMQ Message Queue
- ⚡ 异步非阻塞 I/O / Asynchronous Non-blocking I/O
- 🧠 Redis 缓存支持 / Redis Cache Support

### 待开发功能 / Upcoming Features
- 📤 完善文件发送功能 / Complete File Sending Functionality
- 🔄 好友在线状态同步 / Friend Online Status Synchronization
- 🔌 插件系统架构 / Plugin System Architecture
- 📜 消息历史记录滚动 / Message History Scrolling
- 🛡️ 统一异常处理机制 / Unified Exception Handling Mechanism

## 技术栈 / Tech Stack

- **语言 / Language**: Rust 2024 Edition
- **框架 / Framework**: Axum + Tokio
- **数据库 / Database**: MySQL / SQLite (SQLx)
- **缓存 / Cache**: Redis
- **消息队列 / Message Queue**: RabbitMQ (Lapin)
- **序列化 / Serialization**: Protocol Buffers (Prost), Serde
- **前端 / Frontend**: Ratatui (Terminal UI)
- **认证 / Authentication**: JWT
- **日志 / Logging**: Tracing + Color-Eyre

## 系统架构 / System Architecture

```
┌─────────────────┐         ┌──────────────────┐
│   Client(TUI)   │◄───────►│   Connection     │
└─────────────────┘         └──────────────────┘
                                     │
                            ┌──────────────────┐
                            │   RabbitMQ       │
                            └──────────────────┘
                                     │
       ┌────────────┬─────────────────┼─────────────────┬──────────────┐
       │            │                 │                 │              │
┌────────────┐ ┌────────────┐  ┌────────────┐   ┌────────────┐  ┌────────────┐
│   User     │ │  Session   │  │   Blob     │   │            │  │            │
│  Service   │ │  Service   │  │  Service   │   │            │  │            │
└────────────┘ └────────────┘  └────────────┘   └────────────┘  └────────────┘
     ▲               ▲              ▲
     │               │              │
     ▼               ▼              ▼
┌────────────┐ ┌────────────┐  ┌────────────┐
│   MySQL    │ │   MySQL    │  │   Disk     │
└────────────┘ └────────────┘  └────────────┘
```

## 项目结构 / Project Structure

- `client/` - 终端客户端 / Terminal Client (TUI)
- `user/` - 用户服务 / User Service
- `session/` - 会话与消息服务 / Session & Message Service
- `blob/` - 文件服务 / File Service
- `connection/` - WebSocket 连接管理器 / WebSocket Connection Manager
- `core/` - 公共库 / Shared Core Library
- `docker/` - Docker 配置文件 / Docker Configuration Files

## 快速开始 / Quick Start

### 环境要求 / Prerequisites

- Rust 1.75+
- Docker (用于运行依赖服务) / Docker (for dependency services)
- SQLx CLI (可选) / SQLx CLI (optional)

### 安装依赖服务 / Install Dependencies

```bash
# 启动 MySQL, Redis, RabbitMQ / Start MySQL, Redis, RabbitMQ
docker-compose -f docker/mysql.yml up -d
docker-compose -f docker/redis.yml up -d
docker-compose -f docker/rabbitmq.yml up -d
```

### 数据库初始化 / Database Initialization

```bash
# 进入各服务目录并运行迁移 / Enter each service directory and run migrations
cd user && sqlx migrate run
cd session && sqlx migrate run
cd blob && sqlx migrate run
cd client && sqlx migrate run
```

### 运行服务 / Run Services

```bash
# 构建所有服务 / Build all services
cargo build

# 运行用户服务 / Run user service
cargo run -p user

# 运行会话服务 / Run session service
cargo run -p session

# 运行文件服务 / Run blob service
cargo run -p blob

# 运行连接服务 / Run connection service
cargo run -p connection

# 运行客户端 / Run client
cargo run -p client
```

## 开发指南 / Development Guide

### 代码规范 / Coding Standards

- 使用 `rustfmt` 格式化代码 / Format code with `rustfmt`
- 使用 `clippy` 检查代码质量 / Check code quality with `clippy`
- 错误处理遵循 `anyhow` / `thiserror` 规范 / Error handling follows `anyhow` / `thiserror` conventions

### 项目配置 / Configuration

环境变量通过 `.env` 文件配置，参考各服务下的 `.env.example` 文件。

Environment variables are configured via `.env` files. Refer to the `.env.example` files in each service.

## API 文档 / API Documentation

使用 Bruno API 测试工具，配置文件位于各服务的 `bruno/api/` 目录下。

Use the Bruno API testing tool. Configuration files are located in the `bruno/api/` directory of each service.

## 贡献 / Contributing

欢迎提交 Issue 和 Pull Request 来帮助改进项目。

Feel free to submit Issues and Pull Requests to help improve the project.

## 许可证 / License

版权所有 (c) caohailong

本项目采用 MIT 许可证授权。详情请见 [LICENSE](./LICENSE) 文件或访问 <http://opensource.org/licenses/MIT>

Copyright (c) caohailong

This project is licensed under the MIT license. See the [LICENSE](./LICENSE) file or visit <http://opensource.org/licenses/MIT> for details.