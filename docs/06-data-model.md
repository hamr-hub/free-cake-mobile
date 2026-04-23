# 数据模型

## 一、实体关系

### 1. 区域与门店
- Region：赛区 / 村镇区域
- Store：履约节点
- 一个 Region 可绑定一个主 Store，也可扩展多个 Store

### 2. 用户与身份
- User：用户主档
- UserIdentity：手机号、OpenID、设备指纹等身份索引

### 3. 活动与作品
- Activity：活动主表
- ContestEntry：参赛作品
- AIGenerationRecord：AI 出图记录
- DesignTemplate：可生产模板

### 4. 投票与风控
- VoteRecord：投票流水
- RiskEvent：风险事件

### 5. 获奖与履约
- WinnerRecord：获奖记录
- RewardOrder：奖励订单
- ProductionBatch：生产批次
- ProductionTask：生产任务
- RedeemCode：核销码
- RedeemRecord：核销流水

### 6. 库存与人员
- InventoryItem：库存台账
- InventoryTxn：库存流水
- Staff：人员档案
- AttendanceRecord：考勤流水

## 二、关键字段建议
### Activity
- id
- region_id
- name
- registration_start_at
- registration_end_at
- voting_start_at
- voting_end_at
- max_winner_count
- status

### ContestEntry
- id
- activity_id
- user_id
- selected_generation_id
- selected_template_id
- title
- share_code
- raw_vote_count
- valid_vote_count
- risk_score
- status

### VoteRecord
- id
- activity_id
- entry_id
- voter_user_id
- voter_open_id
- voter_phone_hash
- voter_device_id
- ip
- geohash
- vote_status
- risk_tags
- created_at

### RewardOrder
- id
- winner_id
- store_id
- order_type
- template_id
- scheduled_date
- production_status
- redeem_status

### ProductionTask
- id
- batch_id
- order_id
- store_id
- device_task_payload
- task_status
- started_at
- completed_at
- fail_reason

### RedeemRecord
- id
- order_id
- redeem_code_id
- store_id
- verifier_staff_id
- redeem_result
- redeem_at

## 三、索引建议
- VoteRecord(activity_id, entry_id, created_at)
- VoteRecord(voter_open_id, activity_id)
- ContestEntry(activity_id, valid_vote_count desc)
- RewardOrder(store_id, scheduled_date, production_status)
- RedeemRecord(store_id, redeem_at)
- InventoryTxn(store_id, item_id, created_at)

## 四、审计要求
以下动作必须记审计日志：
- 扣票
- 改票
- 手动加名额
- 重置核销码
- 人工改排产
- 库存报损
- 人员薪资确认
