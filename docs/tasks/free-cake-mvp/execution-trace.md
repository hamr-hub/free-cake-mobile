# Free Cake MVP - 执行追踪

## 任务信息
- **任务ID**: free-cake-mvp
- **项目**: Free Cake 村镇 AI 蛋糕项目
- **技术栈**: 前端 refine+react+vite / 后端 Rust (Axum)
- **模式**: 单仓 Auto
- **开发者**: codeflicker

## Stage 执行记录

### Stage 0: 前置检查
- **时间**: 2026-04-23
- **Verdict**: PASS
- **检查项**:
  - Node.js: v22.22.1
  - npm: 10.9.4
  - Git: 2.34.1
  - Rust/Cargo: 1.93.0
  - 项目类型: Web (refine+react+vite 前端 + Rust 后端)

### Stage 7: TDD 实现 (迭代续推)
- **时间**: 2026-04-28
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端修复与增强
  1. 修复 `activity_test.rs`: `validate_status_transition` 函数改为 `pub` 以支持外部测试引用
  2. 修复 `settlement_test.rs`: 修复 `SortedBy` trait 定义 (需 `Iterator` 约束) + `sorted_by`/`take` 链式调用修复
  3. 清理所有 test 文件中的 `unused import` warnings (vote/entry/inventory/order/redeem)
  4. 修复 `redeem_test.rs`: 替换 deprecated `NaiveDateTime::from_timestamp_opt` 为 `DateTime::from_timestamp`
  5. 升级 `NotificationService`: 从空 stub 改为基于 audit_log 表的持久化通知记录，更新 `settlement.rs` 和 `inventory.rs` 的调用签名传入 pool
  6. 全部 65 个测试通过，零 warnings

  #### B 端客户端增强
  1. 新增 Login 页面 (`pages/login/index.tsx`): 手机号+验证码登录表单，使用 `useLogin` hook
  2. 添加 `/login` 路由到 `App.tsx`
  3. 新增 3 个 CRUD 创建页面:
     - `pages/regions/create.tsx` (赛区创建表单)
     - `pages/stores/create.tsx` (门店创建表单)
     - `pages/staff/create.tsx` (人员创建表单)
  4. Regions/Stores/Staff 列表页添加 `canCreate` 按钮
  5. App.tsx resources 配置更新，为 regions/stores/staff 添加 `create` 路由
  6. TypeScript 类型检查通过 (0 errors)

- **测试结果**: cargo test 65/65 pass, tsc --noEmit 0 errors
- **产出文件**:
  - `server/src/handlers/activity.rs` (pub 函数)
  - `server/tests/settlement_test.rs` (trait 修复)
  - `server/tests/redeem_test.rs` (deprecated API 修复)
  - `server/tests/{vote,entry,inventory,order}_test.rs` (unused import 清理)
  - `server/src/services/notification.rs` (持久化实现)
  - `server/src/handlers/settlement.rs` (调用签名更新)
  - `server/src/handlers/inventory.rs` (调用签名更新)
  - `client/src/pages/login/index.tsx` (新增)
  - `client/src/pages/regions/create.tsx` (新增)
  - `client/src/pages/stores/create.tsx` (新增)
  - `client/src/pages/staff/create.tsx` (新增)
  - `client/src/App.tsx` (路由更新)
  - `client/src/pages/regions/index.tsx` (canCreate)
  - `client/src/pages/stores/index.tsx` (canCreate)
  - `client/src/pages/staff/index.tsx` (canCreate)

### Stage 8: P0/P1 功能缺口修复 (三端迭代)
- **时间**: 2026-04-28
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 P0 安全修复
  1. `query_handlers.rs`: 修复 SQL 注入漏洞，改为参数化查询 (6处拼接改为bind参数)
  2. `auth.rs`: 添加 RBAC 中间件 (role+scope 双层鉴权: platform_admin > region_admin > village_admin)
  3. `main.rs`: 添加 RBAC 中间件层、risk_event 路由、risk_control_enabled 配置项
  4. `entry.rs`: 使用认证用户 ID 替代请求体中的 user_id (防止伪造)
  5. `settlement.rs`: 使用事务、修复 LAST_INSERT_ID、添加分润逻辑
  6. `risk_control.rs`: 添加 OpenID/geohash 维度风控、risk_event 持久化
  7. `vote.rs`: 传入 risk_enabled、添加 openid 字段、Redis 排名缓存快速路径

  #### B 端客户端 P0/P1 功能增强
  1. `activities/create.tsx`: 添加活动规则配置 (投票限制/AI速率限制/赛区限制/AI作品允许/加权投票) + 模板复用功能
  2. `activities/show.tsx`: 添加 Tabs 切换 (活动信息/作品列表/投票数据)，使用 EntryCard 组件展示作品，排行榜表格+投票统计卡片
  3. `votes/index.tsx`: 添加 Tabs (风控总览/异常事件)，风险分布饼图 + IP聚类柱状图 (echarts)，批量冻结/解冻/作废操作，异常事件 Timeline feed
  4. `settlement/index.tsx`: 添加统计卡片、结算触发按钮、订单展开行详情、重发核销码按钮
  5. `stores/index.tsx`: 添加统计卡片、设备状态列、库存查看按钮 + Modal
  6. `inventory/index.tsx`: 修复 useTable/import 错误，改为 fetch 方式调用 API
  7. `dashboard/index.tsx`: 已在之前迭代中完成 echarts 图表

  #### C 端移动端 P0/P1 功能修复
  1. `HomeScreen.tsx`: 移除 globalThis.__navigation hack，改用 useNavigation hook；从 API 获取真实热门作品数据；添加 banner_url 展示；活动规则动态读取
  2. `DetailScreen.tsx`: 从 API 获取真实作品数据 (entries/:id)；添加投票后刷新排名；实现 Share 拉票功能；添加获奖提示+领奖导航
  3. `RankScreen.tsx`: 使用 currentActivity.id 替代硬编码 0；添加"我的排名"浮层按钮 + Modal；添加"有效票说明"弹窗；作品可点击跳转详情
  4. `ProfileScreen.tsx`: 添加投票记录列表展示；添加客服与申诉入口 (电话/链接)；作品/领奖项添加导航跳转
  5. `RegionGuard.tsx`: 添加可参与赛区列表 (从 voting_open 活动中获取)；替代纯"暂未开放"提示
  6. `api.ts`: 添加 getEntryDetail/getRiskEvents 函数，export apiClient
  7. 修复预已有类型错误: location.ts (添加 isInRange 函数), share.ts (移除 WECHAT 引用), PublishScreen.tsx (参数类型), constants.test.ts/formatters.test.ts/validators.test.ts (import路径)
  8. package.json: 移除废弃的 react-navigation 包，修复依赖版本
  9. 添加 @types/jest devDependency

- **测试结果**:
  - 服务端: `cargo check` 通过, `cargo test` 9项通过
  - B端: `npm run build` (tsc + vite) 通过
  - C端: `npx tsc --noEmit` 0 errors

### Stage 9: P0-P2 全量安全修复与功能补全
- **时间**: 2026-04-28
- **Verdict**: PASS
- **本轮变更**:

  #### P0 安全与架构修复
  1. 合并 `lib.rs`/`main.rs` 重复的 `AppState` 和 `create_router`：`main.rs` 改为引用 `lib.rs` 导出，避免双份代码不同步
  2. 修复 `auth.rs` 中 `md5_hash` 用 `DefaultHasher` 伪装 MD5：改为 SHA-256 (`sha2`) 哈希手机号
  3. 修复 `vote.rs` rank 缓存路径 SQL 注入：`format!("... WHERE id IN ({})", id_str)` 改为 `ANY($1)` 参数化查询

  #### P1 功能补全
  4. `activity.rs`/`region.rs`/`staff_handler.rs`：全部 `list` 函数改用 `sqlx::QueryBuilder`，消除 `format!` 拼接参数索引
  5. 活动创建时自动创建 `activity_rule` 记录（含 max_votes_per_day/cake_size/cream_type/ai_generation_rate_limit）
  6. 修复 `order.rs` `resend_code` 硬编码过期时间 `"2099-12-31"`：改为投票结束后 +7 天
  7. 新增考勤 API：`POST /staff/check-in`、`POST /staff/check-out`、`GET /attendance`，含重复签到检查、员工-门店关联校验、审计日志

  #### P2 优化
  8. `store.rs` list 添加分页参数 (page/page_size/region_id/status)
  9. 生产环境 CORS 配置化：新增 `CORS_ORIGIN` 环境变量，支持逗号分隔域名白名单，空值回退为 `Any`
  10. Dashboard 7 次独立查询合并为 1 条 SQL (子查询合并)，减少 DB 往返
  11. 新增 `/api/users/me` 和 `/api/redeem/:code` 端点，支撑 C 端 ProfileScreen 展示用户作品、投票记录、领奖码及详情

- **测试结果**:
  - 服务端: `cargo check` + `cargo clippy` 零 warning, `cargo test` 9项通过
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors
- **产出文件**:
  - `server/src/lib.rs` (RBAC 分层 + 新路由)
  - `server/src/main.rs` (精简为仅 main 函数)
  - `server/src/config.rs` (新增 cors_origin)
  - `server/src/handlers/auth.rs` (SHA-256 哈希)
  - `server/src/handlers/vote.rs` (参数化查询 + clippy 修复)
  - `server/src/handlers/activity.rs` (QueryBuilder + activity_rule 创建)
  - `server/src/handlers/region.rs` (QueryBuilder)
  - `server/src/handlers/store.rs` (分页 + QueryBuilder)
  - `server/src/handlers/staff_handler.rs` (QueryBuilder + 考勤 API)
  - `server/src/handlers/order.rs` (动态过期时间)
  - `server/src/handlers/dashboard.rs` (单条 SQL 优化)
  - `server/src/handlers/user.rs` (新增: profile + redeem detail)
  - `server/src/handlers/mod.rs` (新增 user 模块)
  - `server/Cargo.toml` (新增 sha2 依赖)
  - `server/.env.example` (新增 CORS_ORIGIN)
