# 系统架构

## 一、逻辑架构

### 1. C 端触点
- React Native 移动应用 (iOS/Android)
- 微信小程序 (未来)
- 门店核销端

### 2. B 端中台
- 运营后台
- 活动后台
- 风控后台
- 排产后台
- 库存后台
- 人事后台

### 3. 核心服务
- 用户服务
- 活动服务
- AI 设计服务
- 投票服务
- 风控服务
- 订单服务 + 微信支付
- 履约服务
- 核销服务
- 库存服务
- 排产服务
- 报表服务
- 通知服务 (短信)
- 审计日志服务

### 4. 基础设施
- PostgreSQL 16
- Redis 7
- 对象存储 (Supabase Storage / OSS)
- 任务调度器 (tokio::spawn 后台循环)
- Prometheus 指标导出
- 请求 ID 链路追踪
- tracing-subscriber JSON 结构化日志

### 5. 门店侧
- 门店工作台
- 设备网关
- ROS2 控制节点
- 打印 / 烘烤 / 冷藏设备

## 二、边界划分
### 云端负责
- 活动规则
- AI 生成
- 排行与开奖
- 订单生成
- 排产策略
- 风控决策
- 库存预警
- 报表分析

### 门店负责
- 接收任务
- 执行生产
- 完工确认
- 用户核销
- 异常上报

## 三、部署建议
### MVP 阶段
- 单体应用 + 单 PostgreSQL + 单 Redis
- B 端、C 端 API 共用应用服务
- 对象存储保存图片和模板素材
- Prometheus + Grafana 监控

### 扩展阶段
- 拆分投票、风控、订单、库存服务
- 使用消息队列解耦开奖、通知、排产
- 引入读写分离和多副本部署

## 四、核心接口清单

### 认证
- `POST /api/auth/login`
- `POST /api/auth/wechat-login`
- `POST /api/auth/bind-phone`
- `POST /api/auth/send-verify-code`
- `POST /api/auth/refresh`
- `POST /api/auth/logout`

### 用户
- `GET /api/users/me`
- `PUT /api/users/me`
- `GET /api/users/resolve-region`
- `GET /api/entries/mine`
- `GET /api/votes/mine`
- `GET /api/orders/mine`
- `GET /api/redeem/:code`

### 活动
- `GET /api/activities`
- `GET /api/activities/:id`
- `POST /api/activities` (admin)
- `PUT /api/activities/:id` (admin)
- `POST /api/activities/:id/status` (admin)
- `GET /api/activities/:id/rules`
- `PUT /api/activities/:id/rules` (admin)

### 参赛 & 投票
- `POST /api/activities/:id/entries/generate`
- `POST /api/activities/:id/entries`
- `GET /api/entries/:id`
- `POST /api/entries/:id/vote`
- `GET /api/activities/:id/rank`

### 结算 & 订单
- `POST /api/activities/:id/settle`
- `GET /api/settlement`
- `GET /api/settlement/:id`
- `POST /api/orders` (付费下单)
- `GET /api/orders/:id`
- `POST /api/orders/:id/init-pay`
- `PUT /api/orders/:id/pay-callback`
- `POST /api/orders/:id/cancel`
- `POST /api/orders/:id/resend-code`
- `POST /api/orders/:id/refund`

### 核销 & 排产
- `POST /api/redeem/verify`
- `POST /api/production/tasks/:id/start`
- `POST /api/production/tasks/:id/pause`
- `POST /api/production/tasks/:id/complete`
- `POST /api/production/tasks/:id/error`
- `POST /api/production/tasks/:id/cancel`

### 门店 & 赛区 & 人员
- `GET/POST/PUT /api/stores`
- `GET/POST/PUT /api/regions`
- `GET/POST/PUT /api/staff`
- `POST /api/staff/check-in`
- `POST /api/staff/check-out`
- `GET /api/attendance`

### 库存 & 模板 & 价格
- `POST /api/inventory` / `GET /api/inventory`
- `GET/POST/PUT /api/templates`
- `GET/POST/PUT /api/prices`

### 报表 & 审计 & 风控
- `GET /api/reports/daily|weekly|monthly`
- `GET /api/audit_log` / `GET /api/audit_log/:id`
- `GET /api/risk_events` / `GET /api/risk_events/:id`

### 基础设施
- `GET /api/health`
- `GET /api/metrics`
- `POST /api/upload`
- `GET /api/dashboard/stats`

## 五、事件驱动建议
建议沉淀以下事件：
- entry_created
- vote_casted
- vote_invalidated
- activity_settled
- winner_confirmed
- production_task_created
- production_task_completed
- redeem_success
- inventory_low
- risk_event_detected
