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

### Stage 10: P0-P1 全量缺口修复与技术方案实施
- **时间**: 2026-04-28
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 P0 核心基础设施
  1. `db/mod.rs`: 添加 `run_migrations()` 函数，调用 `sqlx::migrate!("./src/db/migrations")` 启动时自动建表
  2. `main.rs`: 在 `create_pool` 后调用 `db::run_migrations(&db_pool).await`
  3. `Cargo.toml`: sqlx 增加 `migrate` feature
  4. `lib.rs`: 注册 `error_handler_middleware` 到 admin_routes 和 user_routes，添加 `tracing::error!` 日志
  5. `redeem.rs`: 提取 `Extension(claims): Extension<Claims>` 替换硬编码 `verifier_staff_id = 0`
  6. `services/sms.rs` (新增): `SmsService::send_verify_code()` — dev 模式 log 到 `tracing::info!`，生产可对接阿里云短信
  7. `query_handlers.rs`: 调用 `SmsService::send_verify_code` 发送验证码
  8. `ai_generator.rs`: mock URL 从 `cdn.example.com/cake_N.png` 改为 `placeholder://cake_design/N`

  #### 服务端 P1 功能补全
  9. `inventory.rs`: 新增 `POST /api/inventory` (create_item)、`POST /api/inventory_txn` (create_txn + update quantity)、`PATCH /api/inventory/:id` (update_item)
  10. `services/storage.rs` (新增): `StorageService::upload()` — POST multipart 到 Supabase Storage REST API (`$SUPABASE_URL/storage/v1/object/{bucket}/{filename}`)
  11. `handlers/upload.rs` (新增): `POST /api/upload` — multipart 解析、类型/大小校验、调用 StorageService 上传
  12. `services/notification.rs`: 改为结构化 `tracing::info!` 日志（含 winner phone + redeem code、库存预警含 manager phone），同时保留 audit_log 写入
  13. `handlers/reports.rs` (新增): `GET /api/reports/summary` — 按日聚合参赛/投票数据、核销率、转化率
  14. `config.rs`: 新增 `supabase_url`、`supabase_api_key`、`supabase_bucket` 配置项

  #### B 端客户端增强
  15. `pages/login/index.tsx`: 添加"获取验证码"按钮 + 60s 冷却倒计时
  16. `pages/regions/edit.tsx` (新增): 赛区编辑表单
  17. `pages/stores/edit.tsx` (新增): 门店编辑表单
  18. `pages/staff/edit.tsx` (新增): 人员编辑表单
  19. `pages/activities/edit.tsx` (新增): 活动编辑表单
  20. `pages/staff/attendance.tsx` (新增): 考勤记录列表页
  21. `pages/reports/index.tsx` (新增): 运营报表页（ECharts趋势图+统计卡片）
  22. `App.tsx`: 添加所有新路由和 resource 配置
  23. `layouts/AdminLayout.tsx`: 添加"运营报表"菜单项，RBAC 按角色过滤菜单

  #### C 端移动端修复
  24. `services/location.ts`: 重写为异步，调用 `api.resolveRegion()` 解析 GPS→region_id，更新 AuthContext.regionId
  25. `services/api.ts`: 添加 `resolveRegion()` 函数；修复 `/auth/send-code` → `/auth/send-verify-code`；401 拦截器增加 token refresh 重试逻辑
  26. `screens/GenerateScreen.tsx`: 替换 `<View>` 图片占位符为 `<Image>`，添加 `placeholder://` URL 回退
  27. `screens/PublishScreen.tsx`: 渲染真实 `<Image>` 替代占位符
  28. `components/SharePoster.tsx`: 渲染真实 `<Image>` 替代占位符

  #### 死代码/重复代码清理
  29. `services/audit_log.rs`: 删除死 `log()` 方法
  30. `Cargo.toml`: 移除未使用的 `bcrypt`、`validator` 依赖
  31. `mobile/src/lib/supabase.ts` + `mobile/src/utils/supabaseConfig.ts`: 删除（未被引用）
  32. `mobile/package.json`: 移除 `@supabase/supabase-js` 依赖
  33. `mobile/src/utils/validators.ts`: 删除重复的 `isInRange`/`toRad`（已在 constants.ts 中）
  34. `mobile/src/__tests__/services/validators.test.ts`: 修改 import 从 constants.ts 引入 isInRange

  #### 服务端新增 UPDATE 路由
  35. `region.rs`: 新增 `PUT /api/regions/:id` (update)
  36. `store.rs`: 新增 `PUT /api/stores/:id` (update)
  37. `staff_handler.rs`: 新增 `PUT /api/staff/:id` (update)
  38. `activity.rs`: 新增 `PUT /api/activities/:id` (update)

- **测试结果**:
  - 服务端: `cargo clippy` 零 warning (1 unused import), `cargo test` 65/65 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

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

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
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

### Stage 11: P2 生产稳定性补全 & Bug 修复
- **时间**: 2026-04-28
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 Bug 修复
  1. `query_handlers.rs`: 修复 winner_list 查询参数重复绑定（push_bind 后多余的 .bind() 导致 PostgreSQL 参数计数不匹配）
  2. `reports.rs`: 修复 redeem_rate/conversion_rate 查询忽略日期和赛区过滤条件，改为 QueryBuilder 动态拼接
  3. `production.rs`: 新增 `POST /api/production/tasks/:id/start` 接口，填补 pending→in_progress 工作流缺口
  4. `lib.rs`: 注册 start_task 路由
  5. `config.rs`: 移除从未调用的 `enable_auto_settle` 死代码配置项

  #### 服务端单元测试（54 个全通过）
  6. `tests/integration_tests.rs` (新增): CryptoService 加解密、不同密文、错误密钥、垃圾输入、手机号脱敏、SHA-256 一致性/差异性、AI prompt 构建/注入防护/长度截断、手机号验证、配置校验、风控阈值、核销锁 TTL

  #### 移动端生产稳定性
  7. `components/ErrorBoundary.tsx` (新增): 全局错误边界，中文友好回退 UI，DEV 模式显示错误堆栈
  8. `services/permissions.ts` (新增): Android 运行时权限请求（location/camera/storage），iOS 透传
  9. `services/crashReporter.ts` (新增): 崩溃上报桩（init/captureException/setUser/clearUser），待接入 Sentry DSN
  10. `App.tsx`: ErrorBoundary 包裹全 App，CrashReporterSetup 子组件绑定用户 ID
  11. `hooks/useLocation.ts`: 集成 permissions.ts，获取位置前先请求 location 权限
  12. `context/ActivityContext.tsx`: 错误捕获调用 captureException，新增 clearError 方法
  13. `navigation/AppNavigator.tsx`: ActivityContext 错误通过 Snackbar 对用户可见

  #### PRD 文档更新
  14. `PRD.md` 新增第 17 节：微信小程序页面结构清单（C 端 10 页 + 门店端 4 页）、微信支付集成清单（10 项）、小程序开发检查清单（10 项）

- **测试结果**:
  - 服务端: `cargo clippy` 零 error, `cargo test` 54/54 pass
  - 移动端: `tsc --noEmit` 0 errors

### Stage 12: 微信认证 / 支付准备 / 调度器 / 移动端稳定性
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 微信 OpenID 认证
  1. `auth.rs` (重写): 新增 `POST /auth/wechat-login` — 用 code 换取 openid，已绑定用户返回 JWT，未绑定返回 `need_bind_phone: true`
  2. `auth.rs`: 新增 `POST /auth/bind-phone` — 接收 openid + phone + verify_code，绑定 user_identity 表，更新 app_user.open_id，签发 JWT
  3. `lib.rs`: 注册 wechat-login 和 bind-phone 为公开路由

  #### 服务端 — 后台定时调度器
  4. `main.rs`: 新增 `tokio::spawn` 后台调度任务，每 60 秒执行一次 tick
  5. `main.rs::tick_activity_transitions`: 自动流转 draft→registration_open→voting_open→voting_closed（基于 registration_start_at / voting_start_at / voting_end_at）
  6. `main.rs::tick_order_timeouts`: 自动关闭超过 30 分钟未支付的 pending 订单

  #### 服务端 — 支付数据库迁移
  7. `migrations/004_payment_schema.sql` (新增): reward_order 添加 amount/pay_status/pay_transaction_id/paid_at/closed_at/refund 列
  8. 新增 `price_config` 表（按赛区/规格/奶油类型配置价格）
  9. 新增 `payment_record` 表（支付审计追踪）
  10. design_template 补充 status + updated_at 列及触发器

  #### 服务端 — 更多修复
  11. `query_handlers.rs`: 修复 winner_list 参数重复绑定 bug
  12. `reports.rs`: 修复 redeem_rate/conversion_rate 忽略日期和赛区过滤
  13. `production.rs`: 新增 start_task 接口
  14. `template.rs` + `mod.rs` + `lib.rs`: 新增 DesignTemplate CRUD API
  15. `config.rs`: 移除死代码 enable_auto_settle

  #### 移动端 — 稳定性
  16. `services/api.ts`: Token 刷新互斥锁（promise-based），并发 401 共享同一个 refresh 请求
  17. `hooks/useVote.ts`: 投票状态持久化到 MMKV（usedToday + lastDate），重启后不丢失日限
  18. `services/storage.ts`: 新增 VOTE_STATE key + setVoteState/getVoteState 访问器
  19. `components/ErrorBoundary.tsx` (新增): 全局 React 错误边界
  20. `services/permissions.ts` (新增): Android 运行时权限请求
  21. `services/crashReporter.ts` (新增): 崩溃上报桩
  22. `App.tsx`: ErrorBoundary + CrashReporterSetup 集成
  23. `navigation/AppNavigator.tsx`: Snackbar 展示 ActivityContext 错误
  24. `index.js` + `app.json` + `scripts/setup-native.sh` (新增)

  #### PRD 文档
  25. `PRD.md` v2.1: 更新进展到 Stage 12，新增端到端差距分析表

- **测试结果**:
  - 服务端: `cargo clippy` 零 error, `cargo test` 83/83 pass
  - 移动端: `tsc --noEmit` 0 errors

### Stage 17: Redis TTL 修复 + 软删除端点
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — Redis 缓存修复
  1. `rank_cache.rs`: ZADD 后新增 EXPIRE 300（5分钟 TTL），防止排行榜缓存键永不过期导致内存泄漏和数据过时

  #### 服务端 — 软删除状态端点
  2. `template.rs`: 新增 `POST /templates/:id/status` — active/inactive 状态切换
  3. `store.rs`: 新增 `POST /stores/:id/status` — active/inactive 状态切换
  4. `lib.rs`: 注册 /templates/:id/status 和 /stores/:id/status 路由

- **测试结果**:
  - 服务端: `cargo check` 通过, `cargo test` 9/9 pass
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — API 补全
  1. `entry.rs`: 新增 `GET /entries/:id` — 返回单条作品详情（EntryDetail struct）
  2. `lib.rs`: 注册 `/entries/:id` 路由
  3. `user.rs`: 新增 `PUT /users/me` — 更新昵称（1-32字符校验）
  4. `lib.rs`: 注册 `PUT /users/me` 路由
  5. `activity.rs`: 新增 `GET /activities/:id/rules` — 返回活动规则（activity_rule）
  6. `activity.rs`: 新增 `PUT /activities/:id/rules` — 更新活动规则（max_votes_per_day/cake_size/cream_type）
  7. `lib.rs`: 注册 `/activities/:id/rules` 路由

  #### B 端 — 4 新管理页
  8. `pages/templates/list.tsx` (新增): 设计模板 CRUD 列表（新增/编辑弹窗）
  9. `pages/audit-log/list.tsx` (新增): 审计日志查看器（动作/日期过滤）
  10. `pages/risk-events/list.tsx` (新增): 风控事件查看器（风险等级/类型过滤）
  11. `pages/orders/show.tsx` (新增): 订单详情页
  12. `App.tsx`: 新增 /templates、/audit-log、/risk-events 资源和路由
  13. `AdminLayout.tsx`: 新增"设计模板""审计日志""风控事件"菜单项
  14. `RoleGuard.tsx`: 新增 3 路由到 adminOnlyRoutes

  #### C 端 — 2 新屏幕
  15. `screens/BindPhoneScreen.tsx` (新增): 微信 OpenID→手机号绑定（验证码+冷却倒计时）
  16. `screens/RulesScreen.tsx` (新增): 活动规则图文说明（5 节 FAQ）
  17. `navigation/AppNavigator.tsx`: 注册 BindPhone、Rules 路由

- **测试结果**:
  - 服务端: `cargo check` 通过
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 订单 API
  1. `order.rs`: 新增 `GET /orders` list — 支持 pay_status 过滤、分页（QueryBuilder），返回 order_type/amount/pay_status/refund_status
  2. `order.rs`: 新增 `GET /orders/:id` detail — LEFT JOIN redeem_code 获取核销码，返回完整订单+退款+核销信息
  3. `lib.rs`: 注册 GET /orders、GET /orders/:id 路由

  #### 服务端 — 支付回调 stub
  4. `order.rs`: 新增 `PUT /orders/:id/pay-callback` — 校验订单状态，更新 pay_status=paid+paid_at，写入 payment_record 审计行（TODO: 微信签名验证）
  5. `lib.rs`: 注册 /orders/:id/pay-callback 路由

  #### 服务端 — winner_list 增强
  6. `query_handlers.rs`: winner_list 查询新增 order_type/amount/pay_status/refund_status 字段，WinnerWithOrder 结构体扩展

- **测试结果**:
  - 服务端: `cargo check` 通过, `cargo test` 9/9 pass
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors

  #### 移动端 — 付费下单流程
  7. `screens/OrderScreen.tsx` (新增): 蛋糕规格选择（6/8/10寸+动物/植物/混合奶油），动态价格展示，确认下单按钮
  8. `screens/OrderDetailScreen.tsx` (新增): 订单详情展示（金额/支付状态/退款状态/核销码）
  9. `navigation/AppNavigator.tsx`: 注册 Order、OrderDetail 路由
  10. `screens/DetailScreen.tsx`: 非获奖作品新增"我要同款蛋糕"按钮，导航到 Order 页面
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — Bug 修复
  1. `order.rs`: 修复 NaiveDateTime 时区数据损坏 — `scheduled_date` 解析为 NaiveDateTime 后用 `Utc.from_utc_datetime()` 转为 `DateTime<Utc>` 再绑定 TIMESTAMPTZ 列

  #### 服务端 — 自动结算调度器
  2. `main.rs`: 新增 `tick_auto_settle()` 函数 — 查询 `voting_closed` 超过 5 分钟的活动，执行结算逻辑（插入 winner_record + reward_order + redeem_code），更新状态为 `settled`，调用审计日志和通知服务
  3. `main.rs`: 在调度循环中调用 `tick_auto_settle()`

  #### 服务端 — 退款 API
  4. `order.rs`: 新增 `POST /orders/:id/refund` — 校验付费+已支付+未退款状态，更新 `refund_status/refund_reason/refund_txn_id/refunded_at`，写入 payment_record 审计行，作废核销码
  5. `lib.rs`: 注册 `/orders/:id/refund` 路由

  #### B 端客户端修复
  6. `pages/prices/list.tsx`: 替换原始 `fetch` 为 `useCustomMutation`，创建后自动 refetch 列表

  #### B 端结算页增强
  7. `pages/settlement/index.tsx`: 新增订单类型/金额/退款列，付费订单退款按钮，展开行显示 pay_status/refund_status，统计卡片新增"付费订单"计数，详情弹窗新增付费订单字段

- **测试结果**:
  - 服务端: `cargo check` 通过, `cargo test` 9/9 pass
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 付费下单
  1. `order.rs`: 新增 `POST /orders` create_paid_order — 按赛区/规格从 price_config 查询价格，验证用户归属、门店状态，创建 paid 订单并生成核销码
  2. `lib.rs`: 注册 `POST /orders` 路由

  #### 服务端 — 价格配置 CRUD
  3. `price.rs` + `mod.rs` + `lib.rs` (新增): GET/POST /prices（按赛区/状态列表、创建），PUT /prices/:id（更新价格/状态）

  #### 服务端 — Schema 修复
  4. `002_foreign_keys.sql`: 移除不存在的 production_batch.order_id FK（会导致迁移失败）
  5. `002_foreign_keys.sql`: 新增 production_task.order_id → reward_order(id) FK
  6. `004_payment_schema.sql`: 新增 order_type 列到 reward_order

  #### B 端管理页
  7. `pages/prices/list.tsx` + `pages/prices/index.ts` (新增): 价格配置列表页+新增弹窗
  8. `pages/orders/list.tsx` + `pages/orders/index.ts` (新增): 订单管理页+支付状态筛选
  9. `App.tsx`: 新增 /prices、/orders 资源和路由
  10. `AdminLayout.tsx`: 新增"价格配置"和"订单管理"菜单项
  11. `RoleGuard.tsx`: 新增 /prices、/orders 到 adminOnlyRoutes

  #### 移动端 API 扩展
  12. `services/api.ts`: 新增 createPaidOrder、getOrderDetail、wechatLogin、bindPhone、getPrices

- **测试结果**:
  - 服务端: `cargo clippy` 零 error, `cargo test` 83/83 pass
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors

### Stage 18: 移动端投票限制动态化 + 微信登录流程
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 移动端 — useVote 硬编码修复
  1. `services/api.ts`: 新增 `getActivityRules(activityId)` 调用 `GET /activities/:id/rules`
  2. `hooks/useVote.ts`: 新增 `activityId` 参数，useEffect 内获取服务端 `max_votes_per_day` 替代硬编码常量 `MAX_VOTES_PER_DAY=3`
  3. `screens/DetailScreen.tsx`: 传入 `entry?.activity_id` 给 `useVote(entry?.activity_id)`

  #### 移动端 — 微信登录流程串联
  4. `types/user.ts`: 新增 `WechatLoginResponse`、`BindPhoneResponse` 类型
  5. `services/auth.ts`: 新增 `loginWithWechat(code)` 和 `bindPhoneAndLogin(openid, phone, verifyCode)` 函数
  6. `context/AuthContext.tsx`: 新增 `wechatLogin`、`bindPhone` action，绑定成功后正确更新 `isAuthenticated` 状态
  7. `hooks/useAuth.ts`: 导出 `wechatLogin`、`bindPhone`
  8. `screens/LoginScreen.tsx`: 新增"微信登录"按钮 + 分隔线；预留 `wx.login()` → `wechatLogin(code)` → `navigate('BindPhone', { openid })` 流程
  9. `screens/BindPhoneScreen.tsx`: 修复直接写 storage 的 bug，改用 `useAuth().bindPhone()` 通过 AuthContext 更新全局状态
  10. `navigation/AppNavigator.tsx`: 将 `BindPhone` 路由移到未认证区域（微信登录时用户尚未认证需可达）

- **测试结果**:
  - 移动端: `tsc --noEmit` 0 errors
  - 服务端: `cargo clippy --all-targets -- -D warnings` 零警告
  - 服务端: `cargo test --all` 83/83 pass

### Stage 19: 服务端 Clippy 修复 + 支付验签脚手架 + Mobile CI
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — Clippy 零警告
  1. `handlers/auth.rs`: 修复2处 `redundant_closure`（`|e| AppError::Internal(e)` → `AppError::Internal`）
  2. `services/sms.rs`: 新增 `#[allow(clippy::too_many_arguments)]` 于 `send_tencent_cloud`
  3. `tests/inventory_test.rs`: `vec![]` → 数组 `[]`
  4. `tests/auth_test.rs`: 字符串常量 → `String::new()` / `String::from()` 避免 `const_is_empty`
  5. `tests/activity_test.rs`: 同上修复 `const_is_empty`
  6. `tests/redeem_test.rs`: 同上修复 `const_is_empty`

  #### 服务端 — 支付验签脚手架
  7. `config.rs`: 新增 `wechat_pay_api_key`、`wechat_pay_mch_id` 配置项
  8. `handlers/order.rs`: `pay_callback` 新增条件验签逻辑（有 API key 时记录日志提示待实现，无 key 时 dev 模式 warn）

  #### CI — Mobile CI Job
  9. `.github/workflows/ci.yml`: 新增 `mobile-ci` job（TypeCheck + Lint）

- **测试结果**:
  - 服务端: `cargo clippy --all-targets -- -D warnings` 零警告
  - 服务端: `cargo test --all` 83/83 pass

### Stage 20: C端用户路由 + 移动端API规范 + react-native-config + B端CRUD补全
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — C端用户路由
  1. `handlers/user.rs`: 新增 `my_entries`、`my_votes`、`my_orders` 三个handler
  2. `lib.rs`: 将 `GET /activities`、`GET /activities/:id`、`GET /entries/:id`、`GET /orders`、`GET /orders/:id`、`GET /stores`、`GET /regions`、`GET /templates`、`GET /prices` 等公开读取端点从 `admin_routes` 移至 `user_routes`
  3. `lib.rs`: 在 `user_routes` 注册 `GET /entries/mine`、`GET /votes/mine`、`GET /orders/mine`

  #### 移动端 — API规范使用
  4. `screens/DetailScreen.tsx`: 替换原始 `apiClient.get()` 为 `api.getEntryDetail(entryId)`
  5. `screens/RulesScreen.tsx`: 从纯静态硬编码改为动态获取 `GET /activities/:id/rules`

  #### 移动端 — react-native-config
  6. `package.json`: 新增 `react-native-config` 依赖
  7. `utils/constants.ts`: `API_BASE_URL` 改为从 `react-native-config` 读取
  8. `.env.example` + `.env` (新增)
  9. `types/react-native-config.d.ts` (新增)

  #### B端 — CRUD 补全
  10. `pages/prices/list.tsx`: 新增编辑功能（编辑按钮+复用Modal）
  11. `layouts/AdminLayout.tsx`: 人员考勤改为子菜单（人员列表 + 考勤记录），添加 `openKeys` 展开

- **测试结果**:
  - 服务端: `cargo clippy --all-targets -- -D warnings` 零警告
  - 服务端: `cargo test --all` 83/83 pass
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors

### Stage 21: 死路由接通 + 生产任务扩展 + 对账服务 + 隐私设置 + B端详情页
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 死路由接通
  1. `lib.rs`: 接通 `region::update`、`store::update`、`template::update` PUT 路由
  2. `lib.rs`: 接通 `staff_handler::list`、`staff_handler::show`、`staff_handler::update` 路由
  3. `lib.rs`: 接通 `query_handlers::entry_list` GET 路由

  #### 服务端 — 生产任务扩展
  4. `production.rs`: 新增 `pause_task`（in_progress → paused）、`report_error`（→ error + description）、`cancel_task`（→ cancelled + 同步 reward_order）
  5. `005_production_task_extensions.sql` (新增): error_description、paused_at、cancelled_at 列
  6. `lib.rs`: 注册 /production/tasks/:id/pause、/error、/cancel 三条路由

  #### 服务端 — 对账服务
  7. `services/reconciliation.rs` (新增): 支付-订单对账、投票计数漂移、库存余额漂移
  8. `handlers/reports.rs`: 新增 `reconciliation` handler
  9. `lib.rs`: 注册 GET /reports/reconciliation

  #### 移动端 — 隐私设置
  10. `ProfileScreen.tsx`: 新增隐私设置区（推送通知/排行榜展示开关、数据导出、注销账户）
  11. `api.ts`: 删除未使用的 getDashboardStats、getRiskEvents

  #### B端 — 详情页补全
  12. `stores/show.tsx` (新增): 门店详情页
  13. `staff/show.tsx` (新增): 员工详情页
  14. `App.tsx`: 新增 StoreShow、StaffShow 路由和资源定义

- **测试结果**:
  - 服务端: `cargo clippy --all-targets -- -D warnings` 零警告
  - 服务端: `cargo test --all` 9/9 pass
  - B端: `tsc --noEmit` 0 errors
  - 移动端: `tsc --noEmit` 0 errors

### Stage 22: 可观测性与测试质量
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — Prometheus 指标中间件
  1. `Cargo.toml`: 新增 `metrics` 0.24 + `metrics-exporter-prometheus` 0.16
  2. `app_middleware/metrics.rs` (新增): `metrics_middleware` — http_requests_total、http_responses_total、http_request_duration_seconds
  3. `app_middleware/metrics.rs`: `normalize_path` — 数字段替换为 `:id`（前4段），单元测试覆盖
  4. `lib.rs`: 全局注册 metrics_middleware、`/api/metrics` 端点暴露 Prometheus 文本格式

  #### 服务端 — 单元测试补全（25个 lib 测试）
  5. `services/crypto.rs`: 10 个测试 — encrypt/decrypt 往返、不同 nonce、错误 key、无效 base64、过短密文、手机号脱敏、空字符串、Unicode
  6. `handlers/production.rs`: `validate_transition` 函数抽取 + 6 个状态机测试 — 有效转换、completed 不可取消、无效转换、同状态拒绝、error_description 非空检查
  7. `handlers/production.rs`: 重构 5 个 handler 统一调用 `validate_transition`，消除重复逻辑
  8. `services/reconciliation.rs`: 5 个测试 — vote/inventory drift 计算、空结果一致性、PaymentMismatch 字段
  9. `config.rs`: 4 个测试 — insecure JWT 检测、dev 数据库检测，新增 `is_insecure_jwt`/`is_dev_database` 方法

  #### B端 — 报表页对账标签
  10. `reports/index.tsx`: 新增「数据对账」Tab，含 3 张对账表格（支付-订单不一致/投票计数漂移/库存余额漂移）

- **测试结果**:
  - 服务端: `cargo clippy` 零警告
  - 服务端: `cargo test --lib` 25/25 pass（含新增 16 个）
  - 服务端: `cargo test --all` 总计 99 pass
  - B端: `tsc --noEmit` 0 errors

### Stage 23: 生产就绪加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 真实健康检查
  1. `lib.rs`: `health()` 替换为 `health_check()`，检查 PostgreSQL (`SELECT 1`) 和 Redis (`PING`)
  2. `lib.rs`: 依赖失败返回 503 + JSON 详情，成功返回 200 + 各组件状态

  #### 服务端 — 请求 ID 中间件
  3. `app_middleware/request_id.rs` (新增): 生成 UUID 请求 ID，支持上游传播（X-Request-Id）
  4. `lib.rs`: 全局注册 request_id_middleware（metrics 之前）

  #### 服务端 — IP 限流中间件
  5. `app_middleware/rate_limit.rs` (新增): Redis INCR + EXPIRE 实现每 IP 每分钟 100 次限流
  6. `lib.rs`: 用户路由层注册 ip_rate_limit_middleware（auth_middleware 之前）

  #### 服务端 — 分页修复
  7. `user.rs`: `my_votes`/`my_orders` 新增 `page`/`page_size` 查询参数，替换硬编码 LIMIT
  8. `activity.rs`: `list` 的 `page_size` 增加 `.clamp(1, 100)` 上限
  9. `vote.rs`: `rank` 的 `page_size` 增加 `.clamp(1, 100)` 上限

  #### 服务端 — 输入校验强化
  10. `services/validation.rs` (新增): 共享校验工具 — `validate_phone`、`validate_verify_code`、`validate_string_max`、`validate_cake_size`、`validate_cream_type`
  11. `validation.rs`: 13 个单元测试覆盖各校验器
  12. `auth.rs`: `login` 使用 `validate_phone` + `validate_verify_code`（含数字校验、6位校验）
  13. `order.rs`: `create_paid_order` 使用 `validate_cake_size` + `validate_cream_type` 枚举白名单
  14. `entry.rs`: `submit` 新增 `selected_generation_id > 0` 校验 + `validate_string_max("title", 100)`
  15. `vote.rs`: `cast` 新增跨字段一致性校验（entry.activity_id == request.activity_id）

- **测试结果**:
  - 服务端: `cargo clippy` 零警告
  - 服务端: `cargo test --lib` 39/39 pass（含新增 14 个）
  - 服务端: `cargo test --all` 总计 113 pass
  - B端: `tsc --noEmit` 0 errors

### Stage 24: 交易安全与运维加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 订单幂等性
  1. `006_order_traceability.sql` (新增): entry_id、user_id 列 + 索引
  2. `order.rs`: create_paid_order 新增幂等性检查（entry_id + pending/paid 去重），返回 409 Conflict
  3. `order.rs`: INSERT 写入 entry_id、user_id 字段

  #### 服务端 — 数据库连接池配置
  4. `db/mod.rs`: acquire_timeout(5s)、idle_timeout(600s)、max_lifetime(1800s)

  #### 服务端 — 结构化日志
  5. `Cargo.toml`: tracing-subscriber 启用 env-filter + json features
  6. `main.rs`: fmt().json() + EnvFilter（RUST_LOG 环境变量，默认 info）

  #### 服务端 — 优雅关机
  7. `main.rs`: scheduler JoinHandle 保存，shutdown 后 abort() 取消后台任务

  #### 移动端 — 导航类型安全
  8. `AppNavigator.tsx`: 导出 RootStackParamList 类型定义（各路由参数类型化）
  9. `hooks/useTypedRoute.ts` (新增): 类型安全的 useTypedRoute hook
  10. 5 个屏幕替换 `useRoute<any>` → `useTypedRoute<'ScreenName'>()`：Detail、Order、OrderDetail、BindPhone、Rules
  11. 7 个屏幕替换 `useNavigation<any>` → `useNavigation<NativeStackNavigationProp<RootStackParamList>>()`
  12. ProfileScreen、DetailScreen 修复 `navigate('Redeem')` → `navigate('Redeem', { code: '' })`

- **测试结果**:
  - 服务端: `cargo clippy` 零警告
  - 服务端: `cargo test --lib` 39/39 pass
  - 服务端: `cargo test --all` 总计 113 pass
  - 移动端: `tsc --noEmit` 0 errors

### Stage 25: 认证安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 公开端点 IP 限流
  1. `app_middleware/public_rate_limit.rs` (新增): 按 IP + 路径 Redis INCR 限流
     - /auth/send-verify-code: 10 次/分钟
     - /auth/* 其他: 20 次/分钟
     - 其他公开路由: 100 次/分钟
  2. `lib.rs`: public_routes 注册 public_rate_limit_middleware

  #### 服务端 — JWT 黑名单 + 注销
  3. `app_middleware/auth.rs`: Claims 新增 jti 字段 + 黑名单 Redis 检查
  4. `handlers/auth.rs`: login/wechat_login/bind_phone 新增 jti = uuid
  5. `handlers/auth.rs`: refresh 方法实现 token 轮换（黑名单旧 jti，签发新 jti）
  6. `handlers/auth.rs` (新增): `logout` handler — 黑名单当前 jti，TTL = 剩余过期时间
  7. `lib.rs`: user_routes 注册 POST /auth/logout
  8. `tests/auth_test.rs`: 3 处 Claims 构造补充 jti 字段

  #### B端 — 详情页补全
  9. `pages/votes/show.tsx` (新增): 投票记录详情
  10. `pages/settlement/show.tsx` (新增): 获奖记录详情
  11. `pages/production/show.tsx` (新增): 生产任务详情
  12. `pages/redeem/show.tsx` (新增): 核销记录详情
  13. `pages/templates/show.tsx` (新增): 设计模板详情
  14. `App.tsx`: 5 个新 show 路由注册 + resource 定义补充 show 路径

- **测试结果**:
  - 服务端: `cargo clippy` 零警告
  - 服务端: `cargo test --all` 总计 121 pass
  - B端: `tsc --noEmit` 0 errors

### Stage 26: 输入验证 + 安全修复 + C端订单流程 (迭代续推)
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 输入验证补全
  1. `services/validation.rs` 已有函数现在被 6 个 handler 复用
  2. `handlers/store.rs`: 新增 validate_string_max(name, 100), validate_string_max(address, 500), validate_phone(contact_phone), daily_capacity>=0, lat/lng 范围检查
  3. `handlers/region.rs`: 新增 validate_string_max(name, 100), validate_string_max(province, 50), validate_string_max(city, 50), center_lat/center_lng 范围检查
  4. `handlers/staff_handler.rs`: create 新增 validate_phone(phone); update 新增 validate_phone(phone)
  5. `handlers/template.rs`: create 新增 validate_string_max(name, 100), validate_cake_size, validate_cream_type
  6. `handlers/price.rs`: create 新增 validate_cake_size, validate_cream_type
  7. `handlers/activity.rs`: create 新增 validate_string_max(name, 200), validate_cake_size/cream_type (可选和默认值); update_rules 新增 validate_cake_size/cream_type

  #### 服务端 — unwrap() 恐慌修复
  8. `handlers/query_handlers.rs`: and_hms_opt().unwrap() 改为 ok_or_else(AppError::BadRequest) — 防止恶意日期参数导致进程崩溃
  9. `handlers/reports.rs`: conversion_rate 查询的 unwrap_or(0) 改为 map_err(AppError::Internal) — 不再静默吞掉 DB 错误

  #### C端 — 订单流程修复
  10. `mobile/src/services/api.ts`: 新增 getStores(params) API 函数
  11. `mobile/src/screens/OrderScreen.tsx`: 新增 store 列表获取 + store 选择器 UI + 选择后才能下单 — 修复 storeId=0 导致订单永远无法提交的阻断 bug

  #### B端 — 详情页路由注册
  12. `client/src/App.tsx`: inventory/audit-log/risk-events/prices 4 个资源补充 show 路径 + 嵌套路由 (show/:id)

  #### 服务端 — 微信支付签名验证脚手架
  13. `services/wechat_pay.rs` (新增): WechatPayHeaders 头解析 + 5 分钟重放保护 + verify_signature RSA-SHA256 验证 + verify_if_configured 开发模式降级
  14. `handlers/order.rs`: pay_callback 改为接收 HeaderMap + String body, 先验证签名再解析 JSON
  15. `config.rs`: 新增 wechat_pay_platform_cert 配置字段 + env 读取
  16. `Cargo.toml`: 新增 pem/x509-parser/rsa/num-bigint-dig 依赖
  17. 5 个新增单元测试 (header 解析 + 重放保护 + dev mode + message format)

  #### 服务端 — 微信退款 API 脚手架
  18. `services/wechat_pay.rs`: 新增 RefundRequest/RefundAmount/RefundResponse 结构体 + submit_refund() 异步方法
  19. `handlers/order.rs`: refund handler 调用 wechat_pay::submit_refund() 替代 stub — 无商户凭证时仍返回 stub ID
  20. `lib.rs`: AppState 新增 http_client 字段
  21. `main.rs`: 初始化 reqwest::Client 并传入 AppState

  #### 服务端 — 微信退款 API 脚手架
  18. `services/wechat_pay.rs`: 新增 RefundRequest/RefundAmount/RefundResponse 结构体 + submit_refund() 异步方法
  19. `handlers/order.rs`: refund handler 调用 wechat_pay::submit_refund() 替代 stub — 无商户凭证时仍返回 stub ID
  20. `lib.rs`: AppState 新增 http_client 字段
  21. `main.rs`: 初始化 reqwest::Client 并传入 AppState

  #### C端 — 导航类型安全 + ProfileScreen 改进
  22. `mobile/src/navigation/AppNavigator.tsx`: RootStackParamList.Publish 补全 imageUrl/imageIndex/templateId 三个缺失参数
  23. `mobile/src/screens/ProfileScreen.tsx`: 隐私开关持久化到 MMKV storage（不再每次启动重置为 true）
  24. `mobile/src/screens/ProfileScreen.tsx`: 3 个硬编码 URL/电话号码提取到 constants.ts
  25. `mobile/src/services/storage.ts`: 新增 set/getPushNotifications + set/getRankingVisible 方法

  #### B端 — barrel export 一致性
  26. `client/src/pages/orders/index.ts` 等 5 个文件补全 Show 组件 re-export

  #### 服务端 — 模型修复 (Runtime Crash 修复)
  27. `db/models.rs`: DesignTemplate 补全 status + updated_at 字段 (migration 004)
  28. `db/models.rs`: ProductionTask 补全 error_description + paused_at + cancelled_at 字段 (migration 005)
  29. `db/models.rs`: RewardOrder 补全 amount/pay_status/pay_transaction_id/paid_at/closed_at/refund_*/entry_id/user_id 字段 (migrations 004+006)

  #### 服务端 — 通知服务修复
  30. `services/notification.rs`: redeem_code status 'generated' 改为 'valid' — 修复获奖通知永远不会发送的死代码

  #### 服务端 — 单记录 GET 端点
  31. `handlers/query_handlers.rs`: 新增 show_audit_log handler (GET /api/audit_log/:id)
  32. `handlers/query_handlers.rs`: 新增 show_winner handler (GET /api/settlement/:id)
  33. `handlers/inventory.rs`: 新增 show_item handler (GET /api/inventory/items/:id)
  34. `lib.rs`: 注册 3 个新路由

  #### B端 — 详情页 API 端点修复
  35. `pages/settlement/show.tsx`: /api/settlement?id= 改为 /api/settlement/:id
  36. `pages/audit-log/show.tsx`: /api/audit_log?id= 改为 /api/audit_log/:id
  37. `pages/inventory/show.tsx`: /api/stores/0/inventory?id= 改为 /api/inventory/items/:id

  #### C端 — ErrorBoundary + 导航修复
  38. `navigation/AppNavigator.tsx`: ErrorBoundary 包裹 NavigationContainer
  39. `screens/ProfileScreen.tsx`: 隐私开关持久化到 MMKV (set/getPushNotifications, set/getRankingVisible)
  40. `screens/ProfileScreen.tsx`: 3 个硬编码 URL/电话号码提取到 constants.ts
  41. `utils/constants.ts`: 新增 CUSTOMER_SERVICE_PHONE/APPEAL_URL/HELP_URL

  #### B端 — ErrorBoundary
  42. `client/src/components/ErrorBoundary.tsx` (新增): React ErrorBoundary 包裹 authenticated routes，渲染失败时显示 antd Result 而非白屏

- **测试结果**:
  - 服务端: `cargo clippy` 零新增警告
  - 服务端: `cargo test --lib` 44 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

- **测试结果**:
  - 服务端: `cargo clippy` 零警告 (1 pre-existing borrow warning)
  - 服务端: `cargo test --lib` 44 pass (含 5 个新 wechat_pay 测试)
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 27: P0/P1 运行时崩溃与字段对齐修复
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — P0 迁移与类型修复
  1. `002_foreign_keys.sql`: 修复 `fk_inventory_txn_item` 列名 `inventory_item_id` → `item_id`（实际列名）
  2. `002_foreign_keys.sql`: 移除不存在的 `staff.region_id` 外键（会导致迁移失败）
  3. `007_activity_rule_decoration.sql` (新增): `activity_rule` 表添加 `decoration_params JSONB` 列
  4. `query_handlers.rs`: `WinnerWithOrder.created_at` 和 `EntryWithUserInfo.created_at/updated_at` 从 `String` 改为 `chrono::DateTime<Utc>`（TIMESTAMPTZ 无法 decode 为 String）
  5. `handlers/user.rs`: `UserVote.created_at` 从 `String` 改为 `chrono::DateTime<Utc>`
  6. `handlers/user.rs`: `MyOrder.created_at` 从 `String` 改为 `chrono::DateTime<Utc>`
  7. `handlers/user.rs`: `RedeemDetailResponse.expires_at` 从 `String` 改为 `chrono::DateTime<Utc>`（原用 NaiveDateTime 解 TIMESTAMPTZ 也不正确）
  8. 所有 `r.get::<String, _>("created_at")` / `("updated_at")` / `("expires_at")` 替换为 `r.get::<chrono::DateTime<chrono::Utc>, _>(...)`
  9. `services/wechat_pay.rs`: 移除未使用 `rsa::signature::Verifier` import
  10. `services/crypto.rs`: `&[0u8; 5]` → `[0u8; 5]` 消除 needless_borrow warning

  #### 服务端 — P1 排行榜端点字段对齐
  11. `vote.rs`: `RankResponse.entries` 从 `Vec<ContestEntry>`（无 rank/user_name/is_winner）改为 `Vec<RankedEntry>` 新结构体
  12. `RankedEntry` 包含 id/title/image_url/user_name/valid_vote_count/rank/is_winner — 匹配移动端 `RankedEntry` 接口
  13. `rank` handler 改用 SQL 窗口函数 `RANK() OVER` 计算 rank，JOIN app_user 获取 user_name，EXISTS 子查询判断 is_winner

  #### B端 — 字段名对齐
  14. `pages/inventory/show.tsx`: `record.safety_stock` → `record.safety_threshold`（匹配服务端 InventoryItem 模型）
  15. `pages/audit-log/show.tsx`: `record.description` → `record.detail`（匹配服务端 AuditLog 模型）

  #### C端 — ActivityRules 类型对齐
  16. `mobile/src/types/activity.ts`: `ActivityRules` 从 `ai_generation_rate_limit/region_radius_km/free_cake_size/allow_re_entry` 改为 `max_votes_per_day/cake_size/cream_type/decoration_params`
  17. `mobile/src/screens/HomeScreen.tsx`: 规则展示从 `rules.free_cake_size` → `rules.cake_size`，`rules.ai_generation_rate_limit` → `rules.decoration_params?.ai_generation_rate_limit`，移除 `region_radius_km` 硬编码

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 44/44 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 28: PRD 对齐 + API 路由修复 + 类型对齐
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### PRD/文档更新
  1. `requirement.yaml`: 新增 `mobile` 技术栈 (React Native 0.76)，修正 `database: PostgreSQL 16` (原 MySQL)，新增 `implementation_status` 节详列三端覆盖率
  2. `requirement.md`: 新增「当前实现状态」节，列明服务端 45+ 端点、B端 14 资源模块、C端 12 屏幕
  3. `tech-solution.md`: 修正数据库为 PostgreSQL，新增 React Native C端，新增「已实现功能清单」完整端点表
  4. `05-system-architecture.md`: 修正基础设施栈 (PostgreSQL/Redis 7/tracing/Prometheus)，扩展接口清单含微信认证/支付/对账

  #### C端类型对齐
  5. `mobile/src/types/redeem.ts`: `RedeemCode` 字段对齐服务端 `RedeemDetailResponse`（expire_at→expires_at, cake_name→cake_size+cream_type, 移除 store_distance/cake_image_url）
  6. `mobile/src/components/QRCodeCard.tsx`: 移除 `formatDistance` 引用与 `store_distance` 展示，改用 `cake_size + cream_type` 组合名称

  #### B端页面补全
  7. `client/src/pages/regions/show.tsx` (新增): 赛区详情页
  8. `client/src/pages/regions/index.tsx`: 新增 RegionShow re-export
  9. `client/src/App.tsx`: 注册 RegionShow 组件和 /regions/show/:id 路由

  #### 服务端 API 路由别名
  10. `lib.rs`: 新增 `/risk-events` 别名路由（客户端用连字符，服务端用下划线）
  11. `lib.rs`: 新增 `/audit-log` + `/audit-log/:id` 别名路由
  12. `lib.rs`: 新增 `/activities/templates` 别名路由（创建活动页查模板）

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 44/44 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 29: JSAPI 支付集成 + 配置补全
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 微信 JSAPI 支付集成
  1. `wechat_pay.rs`: 新增 `create_jsapi_order()` — 构造 JSAPI 预付单请求，调用 `POST /v3/pay/transactions/jsapi`，解析 `prepay_id`，生成 `PrepayParams`
  2. `order.rs`: `CreatePaidOrderResponse` 新增 `prepay_id`/`prepay_params`，付费下单后自动调用 `create_jsapi_order` 获取微信支付参数
  3. `order.rs`: 从 `app_user.open_id` 查询用户 OpenID 传入 JSAPI 请求
  4. `config.rs`: 新增 `wechat_app_id`/`wechat_pay_notify_url` 配置 + 环境变量读取 + 测试默认值

  #### 服务端 — Claims 扩展
  5. `auth.rs`: `Claims` 新增 `open_id: Option<String>`，微信登录签发携带 `open_id`，刷新保留原值
  6. `tests/auth_test.rs`: 3 处 Claims 构造补充 `open_id: None`

  #### 服务端 — 报表修复
  7. `reports.rs`: 修复 `NaiveDate - Duration` 编译错误，分步计算月末日
  8. `reports.rs`: 修复 `period` 类型不匹配 → `.to_string()`

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 32: B端+移动端 API 对齐 + 字段修复
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### B端 — dataProvider URL 前缀修复
  1. `dataProvider.ts`: 修复 `custom` 方法中 URL 以 `/` 开头时跳过 `/api` 前缀的 bug

  #### B端 — 模板页面字段对齐
  2. `templates/list.tsx`: `style`→`decoration_params`，`preview_url`→`image_url`，新增 `producible_level`
  3. `templates/list.tsx`: 表单字段对齐服务器 API
  4. `templates/show.tsx`: 展示完整服务器字段，移除不存在的 `style`

  #### B端 — 风控事件页面修复
  5. `risk-events/list.tsx`: `detail`→`description`，风控类型 label 对齐服务器
  6. `risk-events/show.tsx`: 使用正确的 `GET /api/risk_events/:id` 端点

  #### 服务端 — Risk Event Show 端点
  7. `query_handlers.rs`: 新增 `risk_event_show` handler
  8. `lib.rs`: 注册 `GET /risk_events/:id` 路由

  #### 服务端 — UserRedeemCode 完整数据
  9. `user.rs`: `UserRedeemCode` 扩展完整字段
  10. `user.rs`: 查询 JOIN store + activity_rule 填充领奖码数据

  #### 服务端 — 兑换状态一致性
  11. `user.rs`: `/redeem/:code` 返回 `status:"unused"` 统一与 `/users/me` 一致

  #### C端 — DetailScreen 修复
  12. `DetailScreen.tsx`: `entry.rank != null` 修复 rank=0 不显示
  13. `DetailScreen.tsx`: 用 `entry.image_url` 判断 AI 生成标签

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 30: P0 API 数据完整性补全
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — resolve-region 补充 is_in_range
  1. `user.rs`: `ResolveRegionResponse` 新增 `is_in_range: bool`，命中赛区返回 `true`，未命中返回 `false`

  #### 服务端 — Activity 详情增强
  2. `activity.rs`: `show` 返回值从 `Json<Activity>` 改为 `Json<ActivityDetailResponse>`，包含 `region_name`、`current_entry_count`、`current_vote_count`、`rules` (ActivityRuleResponse)

  #### 服务端 — Entry 详情增强
  3. `entry.rs`: `EntryDetail` 新增 `user_name`、`rank`、`is_winner`，SQL 使用 `RANK() OVER` + `EXISTS` 子查询

  #### 服务端 — 报表 region_id 过滤
  4. `reports.rs`: 实现 `apply_region_filter()` 替换空壳，对 reward_order/contest_entry/vote_record/store 表分别注入 region 条件
  5. `reports.rs`: `daily_report`/`weekly_report`/`monthly_report` 全部 7 个统计查询 + top_stores 查询应用 region_id 过滤

  #### B端客户端 — barrel export 补全
  6. `inventory/index.tsx`: 补充 `export { InventoryShow } from "./show"`

- **测试结果**:
  - 服务端: `cargo check` 编译通过
  - 服务端: `cargo clippy --all-targets` 零警告
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 31: API 数据对齐 + 活动列表增强
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — Price Show 端点
  1. `price.rs`: 新增 `show` handler，按 ID 查询 `price_config` 返回 `PriceConfigItem`
  2. `lib.rs`: 注册 `GET /prices/:id` 路由

  #### 服务端 — Activity 列表增强
  3. `activity.rs`: 新增 `ActivityListItem` 结构体，包含 `region_name`、`current_entry_count`、`current_vote_count`、`banner_url`、`rules`
  4. `activity.rs`: `list` handler 改用 JOIN + 子查询，返回丰富的活动列表数据

  #### 服务端 — activity_rule 表补全
  5. 新增 migration `008_activity_rule_status.sql`，添加 `status` 和 `updated_at` 列
  6. `activity.rs`: `ActivityRuleResponse` 新增 `status` 字段，所有 SQL 查询和更新同步适配

  #### 服务端 — activity banner_url
  7. 新增 migration `009_activity_banner_url.sql`，添加 `banner_url` 列

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 32: B端+移动端 API 对齐 + 字段修复
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### B端 — dataProvider URL 前缀修复
  1. `dataProvider.ts`: 修复 `custom` 方法中 URL 以 `/` 开头时跳过 `/api` 前缀的 bug

  #### B端 — 模板页面字段对齐
  2. `templates/list.tsx`: `style`→`decoration_params`，`preview_url`→`image_url`，新增 `producible_level`
  3. `templates/list.tsx`: 表单字段对齐服务器 API
  4. `templates/show.tsx`: 展示完整服务器字段，移除不存在的 `style`

  #### B端 — 风控事件页面修复
  5. `risk-events/list.tsx`: `detail`→`description`，风控类型 label 对齐服务器
  6. `risk-events/show.tsx`: 使用正确的 `GET /api/risk_events/:id` 端点

  #### 服务端 — Risk Event Show 端点
  7. `query_handlers.rs`: 新增 `risk_event_show` handler
  8. `lib.rs`: 注册 `GET /risk_events/:id` 路由

  #### 服务端 — UserRedeemCode 完整数据
  9. `user.rs`: `UserRedeemCode` 扩展完整字段
  10. `user.rs`: 查询 JOIN store + activity_rule 填充领奖码数据

  #### 服务端 — 兑换状态一致性
  11. `user.rs`: `/redeem/:code` 返回 `status:"unused"` 统一与 `/users/me` 一致

  #### C端 — DetailScreen 修复
  12. `DetailScreen.tsx`: `entry.rank != null` 修复 rank=0 不显示
  13. `DetailScreen.tsx`: 用 `entry.image_url` 判断 AI 生成标签

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 33: 移动端类型对齐 + 订单创建安全加固
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### C端 — 类型系统对齐服务器 API
  1. `types/entry.ts`: `template_id`→`selected_template_id`，`generation_id`→`selected_generation_id`，移除 `vote_count`，`image_url` 改为 `string|null`，新增 `user_name`/`status`
  2. `types/vote.ts`: `status`→`vote_status`，移除服务器不返回的 `activity_id`/`user_id`
  3. `types/user.ts`: 移除 `LoginResponse`/`WechatLoginResponse`/`BindPhoneResponse` 中不存在的 `region_id`，`User` 移除 `avatar_url`
  4. `types/order.ts`: 新增文件，定义 `Order`/`OrderDetail`/`CreateOrderRequest`/`CreateOrderResponse`/`OrderListResponse`
  5. `context/AuthContext.tsx`: 登录后调用 `getUserProfile()` 获取 regionId，而非从登录响应读取（服务器不返回）
  6. `components/CakeCard.tsx`: `imageUrl` 改为 `string|null`
  7. `screens/OrderDetailScreen.tsx`: 使用 `OrderDetail` 类型替代 `any`，展示 `store_id`/`created_at`/`paid_at`

  #### 服务端 — 订单创建安全加固
  8. `order.rs`: 新增 activity status 检查（仅 redeeming/finished/settled/voting_closed 可下单）
  9. `order.rs`: 新增 entry status 检查（仅 active 可下单）
  10. `order.rs`: 幂等检查 + 订单创建 + 领奖码生成包装在 DB 事务中，使用 `FOR UPDATE` 防止并发竞态

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告
  - 服务端: `cargo test --lib` 46/46 pass
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors

### Stage 34: WeChat Pay JSAPI 签名实现 + 深度链接 + 文档同步
- **时间**: 2026-04-29
- **Verdict**: PASS
- **本轮变更**:

  #### 服务端 — 微信支付 JSAPI 签名
  1. `wechat_pay.rs`: 实现 `compute_jsapi_sign()` — RSA-SHA256 (PKCS1v15) 签名，解析 PKCS8 PEM 私钥
  2. `wechat_pay.rs`: 实现 `build_v3_auth_header()` — WeChat Pay v3 Authorization 头生成
  3. `wechat_pay.rs`: `create_jsapi_order()` 添加 Authorization 头 + 序列化请求体
  4. `wechat_pay.rs`: `submit_refund()` 同步添加 v3 auth 头
  5. `config.rs`: 新增 `wechat_pay_private_key` / `wechat_pay_serial_no` 配置项
  6. `Cargo.toml`: rsa crate 添加 `pem` feature

  #### C端 — 深度链接
  7. `AppNavigator.tsx`: 添加 `linking` 配置，注册 `freecake://` scheme + universal link 前缀
  8. `AppNavigator.tsx`: `Detail` screen 映射 `entry/:entryId` 路径
  9. `constants.ts`: 新增 `DEEP_LINK_SCHEME` / `UNIVERSAL_LINK_HOST` / `buildEntryDeepLink()`
  10. `app.json`: 添加 `"scheme": "freecake"`
  11. `DetailScreen.tsx`: 分享消息包含深度链接 URL

  #### 文档同步
  12. `requirement.yaml`: implementation_context 更新至 Stage 34, 132 tests
  13. `requirement.yaml`: implementation_status 从 Stage 27 同步至 Stage 34
  14. `tech-solution.md`: MySQL→PostgreSQL, H5→React Native 全量修正
  15. `05-system-architecture.md`: C端触点、核心服务、接口清单全面更新

- **测试结果**:
  - 服务端: `cargo clippy --all-targets` 零警告 (1 too_many_arguments acceptable)
  - 服务端: `cargo test` 132/132 pass (49 lib + 83 integration)
  - B端: `tsc --noEmit` 0 errors
  - C端: `tsc --noEmit` 0 errors
