# 技术方案

> **版本**: v2.0 | **更新日期**: 2026-04-28 | **变更摘要**: 同步实际技术栈（Rust/Axum 替代 Java/Node.js），标注各模块生产就绪度

## 一、建设目标
构建一套支持多村镇赛区统一运营的云端中台系统，并连接村镇履约节点，完成用户参与、AI 设计、投票风控、Top100 结算、生产调度、核销自提、数据回流全流程。

## 二、总体架构
系统拆为四层：

1. 用户触达层
- H5
- 微信小程序
- 门店核销端

2. 业务应用层
- 活动中心
- AI 设计中心
- 投票中心
- 订单与履约中心
- 核销中心
- 库存与供应链中心
- B 端运营中台
- 报表中心

3. 平台能力层
- 用户与账号服务
- 区域围栏服务
- 风控服务
- 消息通知服务
- 文件与图片服务
- 任务调度服务
- 审计日志服务

4. 节点执行层
- 门店工作台
- 生产调度终端
- 设备网关
- ROS2 / 打印机 / 烤箱等设备控制接口

## 三、实际技术栈（已落地）

> 以下为代码实际实现，与原始建议（Java/Node.js + MySQL）不同。

### 1. 后端服务
- **语言/框架**: Rust + Axum 0.7 + Tower 中间件
- **数据库**: PostgreSQL 16（sqlx 0.8 + migrate，Supabase 兼容）
- **缓存**: Redis 7（排行榜缓存、限流、幂等控制）
- **认证**: JWT + SHA-256 手机号哈希 + RBAC 三级角色
- **日志**: tracing 框架（结构化 JSON 输出）

### 2. 前端
- **C 端**: React Native 0.76.3 + TypeScript（跨平台 App）
- **B 端**: React 18 + Refine 5 + Ant Design 5 + TypeScript + Vite 6
- **图表**: ECharts 6 + echarts-for-react

### 3. 数据层
- PostgreSQL 16：事务数据（sqlx 异步连接池，最大 10 连接）
- Redis 7：缓存、限流、排行榜加速、幂等控制
- Supabase Storage：图片与活动素材上传（REST API）

### 4. AI 能力
- **现状**: mock 模式（AI_API_URL 为空时返回 placeholder URL）
- **目标**: 接入国内文生图 API（通义万相 / MidJourney API）
- **约束**: 提示词模板标准化 + 尺寸限制 + 风格白名单 + 敏感词过滤 + 内容安全审核
- **首期**: 生成结果仅作为”设计参考图”，最终履约以可生产模板映射为准

### 5. 设备层
- **现状**: 仅 API 层面下发结构化生产指令，未对接实际设备
- **目标**: 门店侧提供设备网关服务，云端只下发生产任务 JSON
- **首期**: 保留人工确认开关，避免云端误调度直接触发生产事故

## 四、核心模块设计
### 1. 活动中心
职责：管理赛区、活动周期、报名时间、投票时间、开奖时间、奖品配置。

关键能力：
- 多赛区隔离
- 活动模板复用
- 自动开奖任务
- 活动状态机管理

活动状态：
- draft
- pending_publish
- registration_open
- voting_open
- voting_closed
- settled
- redeeming
- finished

### 2. AI 设计中心
职责：根据用户输入生成候选蛋糕图，并映射到可生产模板。

流程：
1. 用户输入主题、场景、祝福语、偏好色系
2. 系统生成规范化 prompt
3. 调用模型生成 5 张图
4. 执行内容安全审核
5. 将结果映射到“生产模板 + 装饰参数”
6. 用户选择一个版本提交参赛

关键约束：
- 限制文案长度
- 限制颜色、装饰复杂度
- 不允许超出设备能力边界的造型进入生产环节

### 3. 投票中心
职责：承接投票、计票、排行、反作弊。

关键设计：
- 写入投票流水表
- 实时分为原始票、待审票、有效票、废票
- 排行榜优先展示有效票
- 通过 Redis 维护热榜缓存

风控策略：
- 手机号、OpenID、设备指纹、IP、定位、访问频率联合判定
- 同一设备批量切号投票识别
- 异常地域集中投票识别
- 短时间票数突增冻结
- 黑名单与人工复核台

### 4. 订单与履约中心
职责：将获奖结果转成生产任务，并绑定门店与领取时间。

关键流程：
- 结算 Top100
- 生成奖励订单
- 按门店产能自动排产
- 生成生产批次与任务单
- 同步到门店工作台 / 设备网关

### 5. 核销中心
职责：生成核销码、校验到店领取、回传领取结果。

关键设计：
- 一单一码
- 有效期控制
- 核销幂等控制
- 核销失败原因枚举化
- 支持离线短时缓存，恢复后补传

### 6. 库存与供应链中心
职责：维护门店库存、消耗、补货与预警。

关键设计：
- 原料主数据统一维护
- 每个蛋糕模板绑定 BOM
- 每次排产自动预扣库存
- 低库存预警
- 报损与盘点流程

### 7. B 端运营中台
职责：总部统一配置规则和监控业务。

主要页面：
- 赛区管理
- 活动管理
- 作品管理
- 风控审核台
- 开奖与订单台
- 门店管理
- 排产中心
- 库存中心
- 人员与考勤
- 经营报表

## 五、关键数据流
### 1. 用户参赛
用户登录 → 校验赛区归属 → AI 出图 → 选择设计 → 提交作品 → 进入投票池

### 2. 投票结算
用户投票 → 风控评分 → 票据入账 → 活动截止 → 计算有效票 → 生成 Top100

### 3. 生产履约
Top100 → 奖励订单 → 排产 → 生成生产批次 → 门店执行 / 设备执行 → 完工入库 → 用户核销领取

### 4. 数据回流
投票日志、生产日志、核销日志、库存日志统一回传报表中心

## 六、核心数据表
建议至少包含：
- region
- store
- activity
- activity_rule
- user
- user_identity
- ai_generation_record
- design_template
- contest_entry
- vote_record
- risk_event
- winner_record
- reward_order
- production_batch
- production_task
- redeem_code
- redeem_record
- inventory_item
- inventory_txn
- staff
- attendance_record
- audit_log

## 七、设备与 ROS2 接入策略
不建议首期直接让业务系统深度耦合设备控制细节，建议采用网关模式：

1. 云端生成标准生产任务 JSON
2. 门店网关拉取任务
3. 网关把任务翻译为 ROS2 topic / service 指令
4. 设备回传阶段状态：accepted / running / paused / failed / completed
5. 云端只认任务状态，不直接依赖设备私有协议

任务 JSON 示例字段：
- task_id
- store_id
- template_id
- cake_size
- cream_type
- decoration_params
- scheduled_time
- priority
- operator_required

## 八、非功能要求
### 1. 可用性
- 活动核心链路 SLA 99.9%
- 开奖任务支持重试与人工补偿
- 核销链路必须支持幂等

### 2. 性能
- 单赛区支持 1 万级作品浏览
- 投票峰值具备秒级响应
- 排行榜缓存刷新延迟控制在 3-10 秒内

### 3. 安全
- 管理后台 RBAC
- 敏感操作双重审计
- 用户手机号脱敏展示
- 核销码防撞库与时效控制

### 4. 可运营
- 所有规则可配置
- 活动模板可复制
- 风控阈值可在线调整

## 九、分阶段建设建议
### Phase 1：MVP
- 小程序 / H5 参赛
- AI 出图
- 投票 + 基础风控
- Top100 结算
- 人工排产 + 门店核销
- 基础报表

### Phase 2：规模化运营
- 多赛区管理
- 自动排产
- 库存中心
- 完整 B 端运营后台
- 更强风控与审计

### Phase 3：设备深度接入
- 设备网关标准化
- ROS2 自动调度
- 生产状态细粒度回传
- 异常自恢复 / 人工接管

## 十、验收指标
- 单赛区有效参与人数
- AI 出图成功率
- 投票作弊拦截率
- Top100 自动结算准确率
- 核销成功率
- 免费用户转付费率
- 单节点产能利用率
- 单期活动毛利 / 获客成本

## 十一、生产就绪度评估（2026-04-28 审计，同日修复更新）

### 后端服务 — 关键阻断项

| 问题 | 严重度 | 状态 |
|------|--------|------|
| JWT_SECRET 默认值 `dev-secret` | **致命** | ✅ 已修复：启动时若为默认值则 panic |
| CORS 默认允许所有源 | **致命** | ✅ 已修复：生产模式空值则 panic + 启动警告 |
| DATABASE_URL 默认硬编码密码 | **致命** | ✅ 已修复：生产模式默认凭证则 panic |
| SMS 为 stub（仅日志） | **阻断** | ✅ 已修复：集成阿里云/腾讯云短信（SMS_PROVIDER 环境变量切换） |
| AI 生成默认返回 placeholder | **阻断** | ✅ 已修复：集成通义万相/OpenAI 兼容接口（AI_PROVIDER 环境变量切换，含异步轮询） |
| 文件上传默认返回 placeholder | **阻断** | ⬜ 待配置：需真实 SUPABULE_URL/KEY |
| 风控维度依赖客户端上报 | **严重** | ✅ 部分修复：IP 从 X-Forwarded-For 提取；device/geohash 仍需客户端 |
| 无外键约束 | **高** | ⬜ 待迁移 |
| 无请求体大小限制 | **高** | ✅ 已修复：1MB 限制中间件 |
| 无优雅关机 | **高** | ✅ 已修复：with_graceful_shutdown + SIGINT |
| 无健康检查端点 | **高** | ✅ 已修复：/api/health |
| 无生产配置校验 | **高** | ✅ 已修复：APP_ENV=production 时校验 3 项 |
| 手机号明文存储 | **中** | ✅ 已修复：AES-256-GCM 加密 + SHA-256 哈希 |
| Redis 分布式锁 DEL 竞态 | **中** | ✅ 已修复：Lua script compare-and-delete |
| AI 提示词注入风险 | **中** | ✅ 已修复：sanitize 过滤 + 长度截断 |
| winner_list 查询参数重复绑定 | **中** | ✅ 已修复：移除 push_bind 后多余的 .bind() |
| 报表 redeem_rate/conversion_rate 不响应过滤 | **中** | ✅ 已修复：QueryBuilder 加入 date/region 条件 |
| 生产任务缺少 start 接口 | **中** | ✅ 已修复：新增 POST /production/tasks/:id/start |
| enable_auto_settle 配置项无实际调用 | **低** | ✅ 已修复：移除死代码 |
| NotificationService 仅日志 | **低** | ⬜ 待集成：需微信模板消息/推送 |

### B 端管理后台 — 关键阻断项

| 问题 | 严重度 | 状态 |
|------|--------|------|
| 无路由级权限守卫 | **严重** | ✅ 已修复：RoleGuard 组件 |
| 8 个页面使用 raw fetch() | **高** | ✅ 已修复：全部 23 个 fetch 替换为 useCustom/useCustomMutation |
| 活动 edit 表单日期字段映射错误 | **高** | ✅ 已修复：字段名对齐 create + onFinish ISO 转换 |
| 库存操作非原子 | **中** | ✅ 已修复：服务端 DB 事务 create_txn |
| 零测试覆盖 | **中** | ✅ 已添加：54 个单元测试全部通过 |
| 考勤页只读无操作 | **低** | ⬜ 待添加 |

### C 端移动 App — 关键阻断项

| 问题 | 严重度 | 状态 |
|------|--------|------|
| 无 android/ios 原生项目目录 | **阻断** | ⬜ 待创建（运行 scripts/setup-native.sh） |
| 无 index.js 入口 | **阻断** | ✅ 已创建：index.js + app.json |
| API_BASE_URL 硬编码 localhost | **阻断** | ⬜ 待 react-native-config |
| 无运行时权限请求 | **高** | ✅ 已修复：location/camera/storage 权限请求 |
| PublishScreen 发送 stub ID | **高** | ✅ 已修复：传递真实 generation_id/template_id |
| token 刷新竞态 | **中** | ✅ 已修复：promise-based mutex，并发 401 共享同一刷新请求 |
| useVote 状态重启丢失 | **中** | ✅ 已修复：投票状态持久化到 MMKV（usedToday + lastDate） |
| 无错误边界组件 | **中** | ✅ 已添加：ErrorBoundary 包裹全 App |
| 无崩溃上报/分析/推送 | **中** | ✅ 桩已添加：crashReporter stub，待配置 Sentry DSN |

### 三端共同缺失

| 能力 | 严重度 | 状态 |
|------|--------|------|
| 微信小程序版本 | 阻断 | ⬜ 待开发（需开放平台注册） |
| 微信登录（OpenID 绑定） | 阻断 | ✅ 已实现：POST /auth/wechat-login + /auth/bind-phone |
| 支付集成 | 阻断 | ⬜ 待申请商户号+集成（数据库迁移已就绪） |
| 后台定时调度 | 高 | ✅ 已实现：活动状态自动流转 + 订单超时关闭 |
| E2E 测试 | 高 | ⬜ 待搭建 |
| 监控告警 | 高 | ⬜ 待搭建 |
| 推送通知 | 中 | ⬜ 待集成 |

## 十二、生产化修复优先级路线图（更新版）

### ✅ P0 — 安全致命项（已完成）
1. JWT_SECRET 强制校验
2. CORS_ORIGIN + DATABASE_URL 强制校验
3. 服务端 IP 提取（X-Forwarded-For）
4. 请求体大小限制中间件

### ✅ P1 — 核心业务集成（已完成）
5. 阿里云/腾讯云短信服务集成
6. AI 文生图 API 集成（通义万相 + OpenAI 兼容）

### ✅ P1 — 客户端阻断修复（已完成）
7. B 端 RoleGuard 路由权限守卫
8. B 端全量 raw fetch() 替换为 Refine hooks
9. B 端活动 edit 日期字段对齐
10. 移动端 PublishScreen 真实 ID 传递

### P2 — 生产稳定性（大部分完成）
11. 健康检查端点 /api/health ✅
12. 优雅关机 ✅
13. 安全响应头中间件 ✅
14. DB 外键约束迁移 ⬜
15. Redis 分布式锁 Lua script ✅
16. B 端库存操作原子化 ✅
17. 移动端 token 刷新互斥锁 ✅
18. 移动端错误边界 + 崩溃上报 ✅（crashReporter stub 待接入 Sentry）
19. 手机号 AES-256 加密存储 ✅
20. 服务端单元测试 ✅（83 个测试通过）
21. winner_list 查询参数绑定修复 ✅
22. 报表过滤修复 ✅
23. 生产任务 start 接口 ✅
24. DesignTemplate CRUD API ✅
25. 微信 OpenID 认证端点 ✅
26. 后台定时调度器 ✅
27. 支付数据库迁移 ✅（004_payment_schema.sql）
28. 投票状态 MMKV 持久化 ✅

### P3 — 规模化准备（2-4 周）
29. 微信小程序开发（C 端主入口）
30. 微信支付集成（需商户号）
31. 推送通知
32. 监控告警
33. E2E 测试
34. 门店端小程序开发
35. 移动端原生项目创建（运行 setup-native.sh）
36. 价格配置 CRUD + B 端管理页
