# Free Cake - AI Flow Project Guide

## WHY
Free Cake 是一个免费蛋糕领券平台，连接蛋糕店和消费者，通过投票机制分配免费蛋糕名额。

## WHAT
- **client**: 管理后台（蛋糕店管理、活动审核、库存管理、结算、数据看板）
- **mobile**: C端消费者应用（浏览活动、投票、领券、兑换）
- **server**: Rust/Axum 后端 API 服务
- **DB**: PostgreSQL (Supabase) - 数据存储、认证、RLS

## HOW

### Architecture
```
[Mobile App] → [Axum API] → [Supabase PostgreSQL]
[Admin Web]  → [Axum API] → [Supabase PostgreSQL]
                              ↓
                        [Redis Cache]
```

### Commands
| Stack | Dev | Build | Test | Lint | TypeCheck |
|-------|-----|-------|------|------|-----------|
| client | `cd client && pnpm dev` | `cd client && pnpm build` | `cd client && pnpm test` | `cd client && pnpm lint` | `cd client && pnpm typecheck` |
| mobile | `cd mobile && pnpm start` | - | `cd mobile && pnpm test` | `cd mobile && pnpm lint` | `cd mobile && pnpm typecheck` |
| server | `cd server && cargo run` | `cd server && cargo build` | `cd server && cargo test` | `cd server && cargo clippy` | `cd server && cargo check` |

### Key Decisions
- **DB**: Supabase (PostgreSQL) - 提供认证、实时订阅、RLS、边缘函数
- **Backend**: Rust/Axum - 高性能、内存安全
- **Client**: refine + Ant Design - 快速构建管理后台 CRUD
- **Mobile**: React Native + Paper - 跨端移动应用
- **Auth**: Supabase Auth (JWT) - 统一认证方案
- **Cache**: Redis - 投票排名缓存

### File Structure
- `client/src/` - React admin pages (refine)
- `mobile/src/` - React Native screens & components
- `server/src/` - Rust handlers, services, middleware
- `server/src/db/migrations/` - Supabase PostgreSQL migrations
- `docs/` - 项目文档（PRD、架构、数据模型等）
