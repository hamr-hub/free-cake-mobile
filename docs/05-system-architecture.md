# 系统架构

## 一、逻辑架构

### 1. C 端触点
- 微信小程序
- H5 活动页
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
- 订单服务
- 履约服务
- 核销服务
- 库存服务
- 报表服务
- 通知服务

### 4. 基础设施
- MySQL
- Redis
- 对象存储
- 消息队列
- 任务调度器
- 监控告警

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
- 单体应用 + 单 MySQL + 单 Redis
- B 端、C 端 API 共用应用服务
- 对象存储保存图片和模板素材

### 扩展阶段
- 拆分投票、风控、订单、库存服务
- 使用消息队列解耦开奖、通知、排产
- 引入读写分离和多副本部署

## 四、核心接口清单
- `POST /api/auth/login`
- `POST /api/activities/{id}/entries/generate`
- `POST /api/activities/{id}/entries`
- `POST /api/entries/{id}/vote`
- `GET /api/activities/{id}/rank`
- `POST /api/activities/{id}/settle`
- `POST /api/orders/{id}/schedule`
- `POST /api/redeem/verify`
- `GET /api/stores/{id}/inventory`
- `POST /api/device/tasks/report`

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
