# Free Cake

村镇 AI 蛋糕活动与履约平台 —— 云端 B 端统管 + 村镇 C 端执行

## 项目简介

Free Cake 是面向乡镇/村镇下沉市场的蛋糕活动平台，核心业务闭环：

1. 用户通过 H5/小程序输入需求 → AI 生成 5 款蛋糕效果图
2. 用户发布参赛 → 分享拉票 → 系统结算 Top100
3. 云端下发生产指令 → 村镇节点 ROS2 自动化生产
4. 用户到店扫码核销 → 领取免费蛋糕 → 数据回传云端

## 技术栈

| 端 | 技术 |
|---|---|
| **Server** | Rust / Axum / SQLx (MySQL) / Redis / JWT |
| **Admin (B端)** | React 18 / Refine / Ant Design 5 / Vite / TypeScript |
| **Mobile (C端)** | React Native 0.76 / React Navigation / React Native Paper / TypeScript |

## 项目结构

```
free-cake/
├── server/          # Rust 后端服务
│   ├── src/
│   │   ├── handlers/    # API 路由处理（activity, auth, entry, vote, redeem, store, dashboard...）
│   │   ├── services/    # 业务服务（AI生成, 风控, 排名缓存, 通知, 审计日志）
│   │   ├── db/          # 数据库迁移 & 模型
│   │   └── app_middleware/  # 认证 & 错误处理中间件
│   └── Cargo.toml
├── client/          # B 端管理后台
│   ├── src/
│   │   ├── pages/       # 页面（activities, entries, votes, stores, staff, inventory, settlement...）
│   │   ├── components/  # 组件（EntryCard, CountdownBanner, RiskTag, AuditDrawer）
│   │   ├── providers/   # authProvider / dataProvider
│   │   └ layouts/       # AdminLayout
│   └── package.json
├── mobile/          # C 端移动应用
│   ├── src/
│   │   ├── screens/     # 页面（Home, Detail, Generate, Login, Redeem, Rank, Profile, Publish）
│   │   ├── components/  # 组件（CakeCard, QRCodeCard, VoteButton, SharePoster, CountdownBanner...）
│   │   ├── hooks/       # useAuth, useActivity, useVote, useLocation, useNetwork
│   │   ├── context/     # ActivityContext, AuthContext
│   │   ├── navigation/  # AppNavigator, TabNavigator
│   │   ├── services/    # api, auth, location, share, storage
│   │   ├── theme/       # colors, spacing, typography
│   │   ├── types/       # activity, entry, redeem, user, vote
│   │   └ utils/         # constants, formatters, validators
│   └ package.json
├── docs/            # 项目文档
│   ├── 01-product-overview.md
│   ├── 02-market-research.md
│   ├── 03-business-rules.md
│   ├── 04-technical-solution.md
│   ├── 05-system-architecture.md
│   ├── 06-data-model.md
│   ├── 07-delivery-plan.md
│   ├── 08-risk-and-compliance.md
│   ├── 09-ui-design.md
│   └ tasks/
   └ img/
└── PRD.md
```

## 快速开始

### 后端

```bash
cd server
cp .env.example .env   # 编辑数据库与 Redis 连接信息
cargo run
```

### B 端管理后台

```bash
cd client
npm install
npm run dev
```

### C 端移动应用

```bash
cd mobile
npm install
# Android
npm run android
# iOS
npm run ios
```

## 开发命令

| 命令 | 说明 |
|---|---|
| `cd server && cargo test` | 后端测试 |
| `cd client && npm run test` | B端测试 |
| `cd client && npm run lint` | B端代码检查 |
| `cd client && npm run typecheck` | B端类型检查 |
| `cd mobile && npm run test` | C端测试 |
| `cd mobile && npm run lint` | C端代码检查 |
| `cd mobile && npm run typecheck` | C端类型检查 |

## 核心业务规则

- **区域限制**：单个村镇节点活动仅限 10km 内用户参与（手机号/IP 定位校验）
- **免费蛋糕标准化**：Top100 获奖蛋糕统一为 6 寸动物奶油基础款
- **投票防刷**：1人1天限投3票，同设备/手机号限参与1次
- **核销校验**：仅获奖本人到店，手机号+核销码双重验证
- **活动周期**：云端统设定每两周一期

## 文档

完整项目文档见 [docs/](./docs/) 目录，涵盖产品概览、市场调研、业务规则、技术方案、系统架构、数据模型、交付计划、风险合规、UI 设计。

## License

MIT License - 详见 [LICENSE](./LICENSE)
