Free Cake 村镇 AI 蛋糕项目 —— 云端 B 端统管 + 村镇 C 端落地深度方案

> **版本**: v3.0 | **更新日期**: 2026-04-29 | **变更摘要**: 公开端点IP限流、JWT黑名单+注销+token轮换、B端5个资源详情页补全

一、项目整体定位与核心逻辑
本项目聚焦村镇下沉市场，打造「云端 B 端全域管控 + 单村镇 C 端节点执行」的轻量化 AI 蛋糕运营模式，彻底实现权责分离：
C 端（村镇终端）：扎根单个核心村镇，实体覆盖周边 10km 所有村落、乡镇人群，仅负责线下蛋糕生产、核销自提、基础本地引流，无复杂运营决策；
B 端（云端中台）：部署在云端，作为全域核心大脑，统一管控所有村镇 C 端节点的人事、业务、运营、数据、AI 服务、生产调度，实现「一个云端管遍所有村镇节点」。
核心业务闭环不变：村镇 10km 内用户→云端 AI 生成 5 款蛋糕图→公众投票→云端核算 Top100→云端下发生产指令→村镇 C 端 ROS2 智能生产→免费蛋糕核销自提，全程标准化、无人工干预。
二、双端架构深度拆解（C 端村镇 + 云端 B 端）
（一）C 端：村镇终端节点（执行层，落地周边 10km）
1. 节点选址与覆盖标准
选址要求：单个核心村镇主街、村委旁、村镇便民点 / 商超周边，人流量集中、周边 10km 村落通勤便捷，面积严格控制 30-50㎡（极简生产 + 自提区，无堂食），极大降低房租成本；
覆盖范围：物理锁定半径 10km 以内，通过手机号归属地、IP 定位、线下核销核验，仅限该区域人群参与，杜绝跨区域薅羊毛；
场景适配：主打村镇生日、节庆、婚丧嫁娶、儿童满月等刚需蛋糕场景，贴合村镇消费习惯。
2. 村镇节点硬件配置（ROS2 智能轻量化版）
适配村镇低成本、易维护需求，精简 ROS2+3D 打印设备，满足批量生产 100 个免费蛋糕即可：
核心主控：ROS2 迷你主控单元（Jetson Orin NX），对接云端指令，自动化控制生产；
生产设备：食品级小型 3D 蛋糕打印机 ×1、智能温控烤箱 ×1、冷藏保鲜柜 ×1、基础烘焙操作台；
配套设备：核销扫码枪、云端考勤设备、简易消毒设备；
核心优势：全程 ROS2 自动化生产，1 人即可操作，无需专业烘焙师，设备故障率低、维护简单。
3. 村镇节点人员配置（云端统一管控）
人员数量：1-2 名本地人员（优先招聘本地村民，易管理、通勤成本低）；
岗位职责：仅负责设备基础操作、蛋糕领取核销、店面卫生、原料清点，无运营、决策、营销工作；
人事权限：所有人员招聘、考勤、排班、薪资核算、培训，全部由云端 B 端统一制定、下发、管控，村镇节点仅执行。
4. C 端核心权责（只做执行，不做管理）
线下：按云端下发的生产指令，完成 Top100 免费蛋糕 ROS2 自动化生产、用户到店核销自提；
原料：仅负责接收云端统一调配的原料，上报库存，不自主采购；
引流：仅落地云端下发的线下物料（海报、宣传单），转发云端统一的社群文案，不自主做营销；
数据：每日生产、核销数据自动同步云端，不自主处理数据。
（二）B 端：云端全域管控中台（决策层，统管所有节点）
部署在阿里云 / 腾讯云，搭建全域管理中台系统，实现对所有村镇 C 端节点的人事、业务、运营、生产、数据统一调配，无需线下驻点，全程云端操控。
1. 云端 B 端核心模块一：全域人事管理
人员统筹：统一管理所有村镇节点的招聘标准、人员信息、考勤打卡、薪资核算、绩效考核，自动生成薪资表；
培训管控：云端下发统一的设备操作、核销流程、卫生标准培训视频，村镇人员线上学习、云端考核；
调度管理：可跨节点调配人员（如某节点活动高峰期临时支援），实时查看各节点人员在岗状态。
2. 云端 B 端核心模块二：全域业务管理（核心业务闭环）
AI 蛋糕设计服务：统一搭载 AI 绘图接口，支撑所有村镇用户线上生成 5 款蛋糕效果图，统一管理 AI 提示词、生成规则，控制 AI 成本；
投票系统管控：统一搭建投票平台，按单个村镇节点划分投票赛区（仅限 10km 内用户投票），实时监控投票数据、防刷票、自动核算 Top100 榜单；
订单 / 生产调度：活动结束后，自动将各村镇 Top100 蛋糕设计图、生产指令，精准下发至对应村镇 C 端 ROS2 系统，自动规划生产排期；
核销管控：统一生成核销码，实时监控各村镇蛋糕领取情况，杜绝重复领取、冒领。
3. 云端 B 端核心模块三：全域运营管理
活动统筹：统一制定所有村镇节点的活动周期（每周 / 每两周一期）、活动规则、免费蛋糕标准（6 寸动物奶油固定款），不允许村镇自主调整；
营销管控：统一制作线下海报、社群文案、宣传视频，下发至各村镇节点落地，全域营销口径一致；
成本管控：实时监控各村镇原料消耗、AI 生成成本、活动成本，统一设定成本阈值，超出预警；
数据管控：实时统计各村镇参与人数、投票数、核销率、用户数据，生成全域运营报表，支撑后续决策。
4. 云端 B 端核心模块四：全域原料与供应链调配
统一采购：所有村镇节点的烘焙原料、耗材，由云端统一集中采购，降低采购成本；
统一配送：按各村镇节点生产需求，定期统一配送原料，实现「云端集采、按需配送到村」；
库存监控：实时查看各村镇原料库存，自动提醒补货，避免原料浪费、短缺。
5. 云端 B 端核心权责（只做管理，不做执行）
决策：所有活动、人事、运营、成本规则，全部由云端统一制定；
调度：统一调配全域人、货、资源，精准下发至单个村镇节点；
监控：实时监控各村镇节点运营、生产、核销全流程，异常预警；
优化：根据全域数据，优化活动规则、AI 生成效果、生产流程。
三、全流程业务闭环（双端协同，无断点）
用户端（村镇 10km 内）：通过云端 H5 / 小程序，输入蛋糕需求，AI 一键生成 5 款效果图，发布参赛，分享拉票；
云端投票核算：按村镇赛区统计票数，活动结束自动生成 Top100 获奖名单，同步生产指令；
云端下发指令：将获奖蛋糕设计图、生产要求，精准下发至对应村镇 C 端 ROS2 系统；
C 端生产执行：村镇节点按 ROS2 自动化指令，批量生产蛋糕，全程无需人工设计、裱花；
用户核销领取：获奖用户到村镇节点，扫码核销，领取免费蛋糕；
数据回传云端：生产、核销数据自动同步云端，云端完成成本核算、数据复盘。
四、村镇场景专属运营规则（严控成本、防风险）

区域严格限制：单个村镇节点活动，仅限 10km 内手机号 / IP / GPS 定位参与，投票、获奖均锁定区域（三重校验：GPS 经纬度 + 手机号归属地 + IP 地理库）；
免费蛋糕标准化：Top100 免费蛋糕统一为 6 寸动物奶油基础款，AI 生成图案适配 ROS2 批量生产，不接受定制修改；
投票防刷管控：云端统一设置，1 人 1 天限投 3 票，同一设备 / 手机号 / OpenID 限参与 1 次，异常票数自动冻结并标记待复核，风控维度包括：手机号、微信 OpenID、设备指纹、IP、GeoHash、访问频率联合判定；
领取规则：仅限获奖本人到店核销，凭手机号 + 核销码 + 扫码确认三要素，杜绝代领、冒领；
活动周期：云端统一设定每两周一期，控制村镇节点生产压力，避免爆单。

五、双端成本投入深度测算
（一）云端 B 端一次性投入
云端中台系统搭建：含 AI 对接、投票、人事、运营、调度模块，1.2-1.5 万元；
服务器 / 域名：年费 1200 元，支持 10 个以上村镇节点同时运营；
核心优势：一次搭建，全域通用，新增村镇节点无额外系统成本。
（二）单个村镇 C 端节点投入
硬件设备（ROS2+3D 打印 + 烘焙）：3.5-4 万元（轻量化精简版，适配村镇）；
店面装修：极简装修，1 万元以内；
首批原料：5000 元；
备用金：1 万元；
单节点总投入：5-6 万元（远低于城区门店，极致下沉低成本）。
（三）单期活动成本（单个村镇）
AI 生成成本：2000 人参与 ×0.15 元 = 300 元；
100 个免费蛋糕原料成本：22 元 / 个 ×100=2200 元；
单期总成本：2500 元左右（全域云端管控，无额外运营成本）。
六、全域盈利模式（村镇下沉长效盈利）
未获奖用户付费转化：村镇用户未进 Top100，可直接付费下单自己设计的蛋糕，云端自动下发生产指令，C 端制作；
线下刚需转化：获奖用户到店自提，带动村镇日常生日、节庆蛋糕付费订单；
私域长效复购：云端统一将所有参与用户拉入村镇社群，后续推送付费蛋糕、团购活动；
多节点规模化盈利：云端管控多个村镇节点，集中采购、集中运营，摊薄成本，实现全域盈利。

七、实际技术栈与架构（已落地实现）

本节同步代码实际实现状态，与旧版方案文档中 Java/Node.js + MySQL 的建议不同，实际选择如下：

1. 后端服务
- 语言/框架：Rust + Axum 0.7（高性能、内存安全、低资源占用，适配村镇低配服务器）
- 数据库：PostgreSQL 16（sqlx 0.8，支持复杂查询与事务，Supabase 兼容）
- 缓存：Redis 7（排行榜加速、限流、幂等控制、JWT 黑名单）
- 认证：JWT + SHA-256 手机号哈希 + RBAC 三级角色（platform_admin / region_admin / village_admin）
- 中间件：Tower CORS + 错误处理中间件 + 审计日志中间件

2. B 端管理后台
- 框架：React 18 + Refine 5（低代码 admin 框架） + Ant Design 5 + TypeScript
- 图表：ECharts 6 + echarts-for-react
- 数据层：Supabase JS SDK（auth + data） + REST dataProvider
- 构建：Vite 6，TypeScript 严格模式

3. C 端消费者 App
- 框架：React Native 0.76.3 + TypeScript + React Navigation 6 + React Native Paper 5
- 网络：Axios（JWT 注入、token 自动刷新、指数退避重试）
- 定位：react-native-geolocation-service（GPS → API 区域匹配）
- 存储：react-native-mmkv（高性能 KV，离线核销码缓存）
- 分享：react-native-share + react-native-view-shot + react-native-qrcode-svg
- 图片：react-native-fast-image（CDN 缓存）

4. 基础设施
- 容器化：Docker Compose（server:3000 + client:80 via nginx + postgres:16 + redis:7）
- CI/CD：GitHub Actions（Rust clippy/fmt/test + Node lint/typecheck/build + Docker build+deploy）
- 文件存储：Supabase Storage（REST API 上传）

八、待集成关键能力（从 mock 到生产）

1. AI 蛋糕图生成
- 现状：mock placeholder URL（placeholder://cake_design/N）
- 目标方案：接入国内文生图 API（首选通义万相 / MidJourney API，备选 Stable Diffusion 自部署）
- 集成要点：提示词模板标准化 + 尺寸限制 + 风格白名单 + 敏感词过滤 + 内容安全审核
- 预估成本：0.15 元/张，单期 2000 人参与 × 5 张 × 0.15 = 1500 元

2. 短信验证码
- 现状：dev 模式仅输出 tracing 日志
- 目标方案：阿里云短信服务（SignatureMethod=HMAC-SHA1，国内覆盖好、村镇可达率高）
- 集成要点：验证码 6 位数字、5 分钟有效期、同一手机号 1 分钟 1 条上限、IP 频率限制防轰炸

3. 文件/图片上传
- 现状：Supabase Storage REST API 已接入，需配置真实凭证
- 待办：配置 SUPABASE_URL + SUPABASE_API_KEY + SUPABASE_BUCKET，验证上传链路

4. 支付集成（付费转化核心）
- 现状：无支付流程
- 目标方案：微信支付 H5 支付 + 小程序支付（村镇用户微信覆盖率 > 90%）
- 关键流程：未获奖用户 → 查看作品详情 → 一键付费下单 → 微信支付 → 生成生产指令 → 门店生产 → 到店自提
- 集成要点：微信支付商户号申请、支付回调验签、退款流程、订单状态同步

九、微信生态集成方案

村镇用户微信覆盖率极高（> 90%），微信生态是 C 端触达的核心渠道：

1. 微信小程序（C 端主入口）
- 方案一：React Native 代码通过 Taro/remix 转译为小程序（保留业务逻辑复用）
- 方案二：独立开发微信小程序版本（WXML + WXSS，使用相同 API 接口）
- 推荐方案二：小程序原生性能更好，且微信登录/支付/分享等能力原生集成更顺畅
- 关键集成：wx.login 获取 OpenID → 后端 user_identity 绑定 → 投票/参赛/核销全链路

2. 微信公众号（通知与引流）
- 模板消息推送：活动开始通知、投票提醒、开奖结果、核销提醒
- 公众号菜单：直达小程序、活动规则、客服申诉
- 粉丝沉淀：参赛/投票后引导关注公众号

3. 微信社群（私域复购）
- 云端统一生成社群二维码海报
- 参赛后引导加入「XX 村免费蛋糕群」
- 群内推送：下期活动预告、付费蛋糕优惠、节日特惠
- 群管理工具：企业微信社群管理 API

十、门店端实现方案

PRD 原描述门店端 4 页 UI（工作台/生产任务/核销/库存盘点），目前无代码实现。根据实际部署场景分析：

1. 方案选择：微信小程序门店版（而非独立 App）
- 原因：村镇人员微信使用习惯深、无需安装、开发成本低、可与 C 端小程序共享基础设施
- 门店人员通过微信扫码登录，角色为 village_admin，自动进入门店工作台

2. 门店端核心功能
- 工作台首页：今日待生产数、待核销数、设备状态、低库存提醒
- 生产任务页：任务列表、优先级、模板图预览、开始/暂停/完成/异常上报
- 核销页：扫码输入区、手机号二次确认、核销结果、异常处理
- 库存盘点页：原料列表、实盘数量、差异上报、补货申请

3. 技术方案
- 与 C 端小程序共享后端 API，门店端通过 village_admin JWT 获取不同权限视图
- 核销扫码使用 wx.scanCode API
- 生产任务状态通过 WebSocket/轮询同步

十一、阶段验收与过渡标准

### 阶段 1 → 阶段 2（首镇试点）验收标准
- 用户可正常注册+手机号验证（真实短信）
- AI 出图成功率 > 80%（真实 API）
- 投票流程完整且风控生效（异常票冻结+复核）
- Top100 自动结算准确率 > 99%
- 核销链路端到端可用（生成码→扫码→确认→记录）
- 基础报表数据正确（参与数、投票数、核销率）
- 微信小程序可提审（前端完成）

### 阶段 2 → 阶段 3（运营优化）验收标准
- 首期参与人数 ≥ 500（单镇）
- 异常票识别率 > 60%
- 核销成功率 > 85%
- 免费→付费转化形成首批样本（≥ 10 例）
- 无重大食品安全或合规事故

### 阶段 3 → 阶段 4（多镇复制）验收标准
- 风控策略迭代 2 次以上
- 库存管理完整闭环（采购→配送→消耗→预警→盘点）
- 支付转化率 > 5%
- 单期活动毛利 > 0（或获客成本 < 10 元）
- 门店端小程序上线可用

十二、监控与可观测性

生产环境必须具备的基础监控能力：

1. 应用监控
- 结构化日志：tracing 框架输出 JSON 格式日志（已有）
- 关键指标：请求延迟 P50/P95/P99、错误率、活跃用户数、投票 QPS
- 告警规则：错误率 > 1% 触发告警、核销失败率 > 5% 触发告警

2. 业务监控
- 活动状态流转监控：活动创建→报名→投票→结算→核销各阶段停留时间
- 风控事件实时统计：异常票数/总票数比率、黑名单命中率
- 库存安全阈值告警：任一门店任一原料低于安全值立即通知

3. 基础设施监控
- PostgreSQL 连接池状态、慢查询监控
- Redis 内存使用、命中率
- 服务器 CPU/内存/磁盘使用率

十三、微信小程序详细架构

### 1. C 端小程序技术架构

```
┌─────────────────────────────────────────────────┐
│              微信小程序 C 端                        │
├─────────────┬─────────────┬───────────────────────┤
│   首页      │  AI 生成页  │  排行榜页              │
│  - 活动Banner│  - 场景选择│  - 实时排名            │
│  - 倒计时   │  - 关键词  │  - 我的排名             │
│  - 热门作品 │  - 色系偏好│  - 申诉入口             │
│  - 规则说明 │  - 5图生成 │                        │
├─────────────┼─────────────┼───────────────────────┤
│  作品详情页  │  我的页    │  领奖核销页             │
│  - 大图展示  │  - 我的作品│  - 核销码二维码          │
│  - 投票按钮  │  - 投票记录│  - 门店信息            │
│  - 分享海报  │  - 领奖码 │  - 截止时间            │
├─────────────┴─────────────┴───────────────────────┤
│              公共服务层                             │
│  - wx.login → OpenID 绑定                         │
│  - wx.request 合规封装（自动带 token）              │
│  - wx.getLocation（GPS 区域校验）                   │
│  - wx.requestPayment（付费转化）                    │
│  - wx.scanCode（核销扫码，门店端）                  │
└─────────────────────────────────────────────────────┘
```

### 2. OpenID 绑定流程

```
用户打开小程序 → wx.login() → 获取 code
       ↓
调用后端 /api/auth/wechat-login { code }
       ↓
后端用 code 换取 session_key + openid
       ↓
若 openid 已绑定用户 → 签发 JWT，返回 token
若 openid 未绑定 → 返回 need_bind_phone: true
       ↓
前端展示手机号绑定页 → 用户输入手机号 + 验证码
       ↓
调用 /api/auth/bind-phone { openid, phone, verify_code }
       ↓
后端绑定 openid 到 app_user.user_identity 表 → 签发 JWT
```

### 3. 门店端小程序

门店端与 C 端共享同一个微信小程序，通过角色区分视图：
- `village_admin` 角色登录后进入门店工作台
- 普通用户进入 C 端首页
- 核销使用 `wx.scanCode` 扫描用户二维码

### 4. 小程序提审要点

- 类目选择：餐饮服务 → 蛋糕烘焙
- 隐私协议：需声明收集位置、手机号用途
- 支付能力：需申请微信支付商户号
- 内容安全：AI 生成图需接入微信内容安全检测 API

十四、支付流程详细时序

### 未获奖用户付费下单流程

```
用户浏览作品详情 → 点击"我要同款"
       ↓
选择蛋糕规格（6/8寸，奶油类型）→ 确认价格
       ↓
调用后端 POST /api/orders { entry_id, spec, price }
       ↓
后端创建订单（status=pending）→ 返回 order_id + prepay_id
       ↓
前端调用 wx.requestPayment(prepay_id)
       ↓
用户在微信支付弹窗完成支付
       ↓
微信支付回调 → 后端 PUT /api/orders/:id/pay-callback
       ↓
后端验签 → 更新订单 status=paid → 生成生产指令 → 通知门店
       ↓
门店生产 → 用户到店自提 → 扫码核销
```

### 关键技术约束

- 微信支付回调必须验签：使用商户 API 证书验 SHA256-RSA 签名
- 订单超时：30 分钟未支付自动关闭（Redis key 过期 + 定时扫描兜底）
- 退款流程：用户申请 → 运营审核 → 调用微信退款 API → 回调确认
- 价格配置：B 端后台可按赛区配置不同规格价格

十五、数据隐私与合规

### 1. 个人信息保护法合规

| 数据类型 | 收集目的 | 存储方式 | 保留期限 | 用户权利 |
|----------|----------|----------|----------|----------|
| 手机号 | 身份验证 | SHA-256 哈希 + 加密原文 | 活动结束后 6 个月 | 可要求删除 |
| GPS 位置 | 区域围栏校验 | 不持久化，仅内存校验 | 请求结束后丢弃 | 默认不存储 |
| 微信 OpenID | 用户标识 | 原文存储 | 账号存续期 | 可要求解绑 |
| 投票行为 | 风控审计 | 加密存储 | 活动结束后 2 年 | 可申请查阅 |

### 2. 数据安全措施

- 手机号原文 AES-256 加密存储，密钥与数据库分离
- API 返回用户信息时手机号中间 4 位脱敏（138****1234）
- 审计日志仅记录操作者 ID，不记录原始数据
- Redis 中的验证码 5 分钟自动过期，无持久化
- 定期清理过期用户数据（Rust 后端定时任务）

### 3. 小程序隐私合规

- 收集手机号需用户主动点击授权按钮
- 位置信息需 `wx.authorize` 弹窗授权
- 隐私协议需在首次打开时弹窗确认
- 用户可随时在「我的 → 隐私设置」中撤回授权

十七、微信小程序页面结构与支付集成清单

### 1. C 端小程序页面清单

| 页面 | 路径 | 核心功能 | 对应 API |
|------|------|----------|----------|
| 首页 | /pages/home/index | 活动 Banner、倒计时、热门作品、规则说明 | GET /activities, GET /entries |
| AI 生成 | /pages/generate/index | 场景选择、关键词输入、色系偏好、5 图生成 | POST /entries/generate |
| 作品详情 | /pages/detail/index | 大图展示、投票按钮、分享海报生成 | GET /entries/:id, POST /vote |
| 排行榜 | /pages/rank/index | 实时排名、我的排名、申诉入口 | GET /activities/:id/rank |
| 我的 | /pages/profile/index | 我的作品、投票记录、领奖码、隐私设置 | GET /users/me, GET /entries/mine |
| 领奖核销 | /pages/redeem/index | 核销码二维码、门店信息、截止时间 | GET /redeem/:code |
| 付费下单 | /pages/order/index | 蛋糕规格选择、价格确认、微信支付 | POST /orders, wx.requestPayment |
| 订单详情 | /pages/order-detail/index | 订单状态、支付结果、自提信息 | GET /orders/:id |
| 手机号绑定 | /pages/bind-phone/index | 手机号 + 验证码输入、OpenID 绑定 | POST /auth/bind-phone |
| 活动规则 | /pages/rules/index | 图文规则说明、FAQ | 静态页 |

### 2. 门店端小程序页面清单

| 页面 | 路径 | 核心功能 | 对应 API |
|------|------|----------|----------|
| 工作台 | /pages/store/dashboard | 待生产/待核销数、设备状态、低库存提醒 | GET /dashboard/stats |
| 生产任务 | /pages/store/production | 任务列表、模板图预览、状态更新 | GET /production-tasks, PATCH /tasks/:id |
| 核销扫码 | /pages/store/redeem-scan | wx.scanCode 扫码、手机号二次确认、核销确认 | POST /redeem/:code/confirm |
| 库存盘点 | /pages/store/inventory | 原料列表、实盘录入、差异上报 | GET /inventory, POST /inventory-txn |

### 3. 微信支付集成清单

| 序号 | 集成项 | 状态 | 说明 |
|------|--------|------|------|
| 1 | 商户号申请 | ⬜ | 微信支付商户号（个体户/企业） |
| 2 | API 证书配置 | ⬜ | apiclient_cert.p12 + apiclient_key.pem |
| 3 | 后端统一下单接口 | ✅ | POST /api/orders → 创建paid订单+核销码 |
| 4 | 前端 wx.requestPayment | 🔧 | 需微信 SDK native 集成，React Native 端暂用 Alert 提示 |
| 5 | 支付回调验签 | 🔧 | PUT /api/orders/:id/pay-callback，脚手架已就位(WECHAT_PAY_API_KEY条件日志) |
| 6 | 订单超时自动关闭 | ✅ | Redis key TTL 30min + 定时扫描兜底（已实现） |
| 7 | 退款流程 | ✅ | API已实现 + B端退款按钮 |
| 8 | 价格配置管理 | ✅ | B 端后台按赛区/规格配置价格 + 编辑功能 |
| 9 | 对账与结算 | ⬜ | 每日自动对账，T+1 结算到商户号 |
| 10 | 支付异常处理 | ⬜ | 网络超时重查、重复支付幂等、余额不足引导 |

### 4. 小程序开发检查清单

| 序号 | 检查项 | 状态 | 说明 |
|------|--------|------|------|
| 1 | 微信开放平台注册 + 小程序创建 | ⬜ | 需企业/个体户资质 |
| 2 | 服务器域名白名单配置 | ⬜ | request 合法域名 + socket 合法域名 |
| 3 | 业务域名配置（H5 跳转） | ⬜ | 如需 web-view 内嵌页 |
| 4 | 类目选择（餐饮→蛋糕烘焙） | ⬜ | 影响可用 API 能力 |
| 5 | 隐私协议 + 用户信息授权弹窗 | ⬜ | 手机号、位置、相册 |
| 6 | 内容安全检测接入 | ⬜ | AI 生成图接入 msg_sec_check |
| 7 | 订阅消息模板申请 | ⬜ | 开奖通知、核销提醒、订单状态 |
| 8 | 小程序码生成（分享 + 线下） | ⬜ | wxacode.getUnlimited API |
| 9 | 性能优化（分包加载） | ⬜ | 主包 < 2MB，门店端独立分包 |
| 10 | 体验版 + 审核提审 | ⬜ | 首次审核约 2-7 天 |

十六、当前迭代进展（2026-04-29 更新）

### 已完成
- Server: 18 handler 模块、8 service 模块、自动迁移、RBAC、审计日志、JWT 刷新、健康检查、优雅关机、请求体限制、服务端 IP 提取
- Server: 阿里云/腾讯云短信服务集成（SMS_PROVIDER 环境变量切换）
- Server: AI 文生图集成（AI_PROVIDER 环境变量切换，含异步任务轮询、提示词注入防护）
- Server: 83 个单元测试全部通过（覆盖 crypto、AI prompt、phone 验证、config 校验、风控、核销、库存、结算、投票、活动、订单等）- Server: 微信登录 / 绑定手机端点（POST /auth/wechat-login、POST /auth/bind-phone）
- Server: 后台定时调度器（活动状态自动流转、未支付订单 30 分钟自动关闭、投票结束后自动结算）
- Server: 支付数据库迁移（004：reward_order 支付列、price_config 表、payment_record 表、design_template 补列、order_type 列）
- Server: 付费下单 API（POST /orders，含 price_config 价格查询、验证、订单创建、核销码生成）
- Server: 退款 API（POST /orders/:id/refund，含状态校验、payment_record 审计、核销码作废）
- Server: 价格配置 CRUD API（GET/POST /prices、PUT /prices/:id）
- Server: 报表过滤修复、winner_list 参数绑定修复、生产任务 start 端点、DesignTemplate CRUD
- Server: 002 foreign_keys 迁移修复（移除不存在的 production_batch.order_id FK）
- Server: order.rs 时区数据损坏修复（NaiveDateTime → DateTime<Utc>）
- Server: 3处竞态条件修复（结算双击、核销双击、调度自动结算，均使用 SELECT FOR UPDATE）
- Server: Redis 投票排行缓存更新改为非致命（失败仅 warn 不阻断请求）
- Server: Redis 排行缓存 key 新增 5 分钟 TTL
- Server: AI 生成时自动创建 design_template 行
- Server: 支付回调验签脚手架（WECHAT_PAY_API_KEY/MCH_ID 配置项 + 条件日志）
- Server: Clippy 零警告（修复 redundant_closure、const_is_empty、useless_vec、too_many_arguments）
- Server: Prometheus 指标中间件（http_requests_total、http_responses_total、http_request_duration_seconds + /api/metrics 端点）
- Server: 39 个 lib 单元测试（crypto 10个、production 状态机 6个、reconciliation 5个、config 4个、validation 13个、metrics 1个）
- Server: 请求 ID 中间件（X-Request-Id 生成/传播）
- Server: IP 限流中间件（Redis INCR，每 IP 每分钟 100 次）
- Server: 真实健康检查（PostgreSQL SELECT 1 + Redis PING，503 JSON）
- Server: 输入校验模块（phone/verify_code/string_max/cake_size/cream_type + 跨字段一致性）
- Server: 分页修复（my_votes/my_orders 支持 page/page_size、所有 list page_size 上限 100）
- Server: 订单幂等性（entry_id+pending/paid 去重 → 409 Conflict，006迁移加 entry_id/user_id 列）
- Server: DB连接池超时（acquire 5s、idle 600s、max_lifetime 1800s）
- Server: 结构化JSON日志（RUST_LOG 环境变量控制级别，默认 info）
- Server: 优雅关机取消后台scheduler（abort() JoinHandle）
- Mobile: 导航类型安全（RootStackParamList、useTypedRoute hook、消除7处 useNavigation<any>、5处 useRoute<any>）
- Server: 公开端点 IP 限流（/auth/send-verify-code 10/min、/auth/* 20/min、其他 100/min）
- Server: JWT 黑名单 + 注销（jti 字段、Redis SET+EX 黑名单、refresh 自动轮换、POST /auth/logout）
- B-end: 5 个资源详情页（投票/获奖/生产/核销/模板）
- Client: 21 页面（新增设计模板、审计日志、风控事件、订单详情）、RoleGuard + sidebar RBAC、所有 23 个 fetch 替换为 Refine hooks
- Client: 结算页新增付费订单字段（类型/金额/退款）、退款操作按钮
- Client: dataProvider 401 修复（不再绕过 authProvider 的 token 刷新逻辑）
- Mobile: 10 屏幕（新增付费下单、订单详情、绑定手机号、活动规则）、真实 generation_id/template_id 传递、GPS 区域匹配
- Mobile: ErrorBoundary、权限请求、crashReporter stub、Snackbar、token 互斥锁、投票 MMKV 持久化
- Mobile: index.js 入口、app.json、setup-native.sh 脚本
- Mobile: useVote 动态获取投票上限（GET /activities/:id/rules → max_votes_per_day）
- Mobile: 微信登录流程串联（LoginScreen按钮 → wechatLogin → BindPhone导航，AuthContext全局状态更新）
- Server: C端用户路由 GET /entries/mine、GET /votes/mine、GET /orders/mine
- Server: 公开读取端点解禁（activities/entries/orders/stores/regions/templates/prices GET 从 admin_routes 移至 user_routes）
- Mobile: DetailScreen 使用 getEntryDetail 替代原始 apiClient 调用
- Mobile: RulesScreen 从静态硬编码改为动态获取活动规则 API
- Mobile: react-native-config 环境变量系统（.env.example + 类型声明）
- B-end: 价格配置编辑功能补全（编辑按钮 + 复用 Modal）
- B-end: 人员考勤改为子菜单（人员列表 + 考勤记录，ClockCircleOutlined）
- Server: 模型补全（DesignTemplate补status+updated_at, ProductionTask补error_description+paused_at+cancelled_at, RewardOrder补全部支付/退款/溯源列）— 修复 SELECT * 运行时崩溃
- Server: 通知服务修复（redeem_code status 'generated'→'valid'）— 修复获奖通知永不发出的死代码
- Server: 微信支付签名验证脚手架 + 退款 API 脚手架（含 RSA-SHA256 验证、5 分钟重放保护、dev mode 降级）
- Server: 6 handler 输入校验补全（复用 shared validation 模块）
- Server: unwrap() 恐慌修复（query_handlers 日期解析、reports DB 错误传播）
- Server: 3 个单记录 GET 端点（audit_log/:id, settlement/:id, inventory/items/:id）
- Client: 4 详情页路由注册 + 5 barrel export 补全
- Client: 3 详情页 API 端点修复（settlement/audit-log/inventory show）
- Mobile: 订单门店选择器修复（storeId=0 阻断 → 获取门店列表+选择器）
- Mobile: RootStackParamList.Publish 类型补全 + ErrorBoundary 包裹导航
- Mobile: ProfileScreen 隐私开关 MMKV 持久化 + URL/电话提取到 constants
- Server: 死路由接通6个（region::update、store::update、template::update、staff list/show/update、entry_list）
- Server: 生产任务扩展（pause、error report、cancel 端点 + migration 005）
- Server: 对账服务（支付-订单对账、投票计数漂移、库存余额漂移检测 + GET /reports/reconciliation）
- Mobile: ProfileScreen 新增隐私设置区（推送通知/排行榜展示开关、数据导出、注销账户）
- Mobile: 删除未使用的 getDashboardStats、getRiskEvents 函数
- B-end: 门店详情页 + 员工详情页
- B-end: 报表页新增「数据对账」标签（3张对账表格 + 一致性通过提示）
- Server: 生产状态机 validate_transition 统一抽取（handler 复用，消除重复 if 分支）

### 待完成（Phase 1 → Phase 2 阻断项）
- [ ] 配置真实 AI_API_URL + AI_API_KEY
- [ ] 配置真实 SMS 凭证
- [ ] 配置 Supabase Storage 凭证
- [ ] 运行 scripts/setup-native.sh 创建 android/ios 原生项目
- [ ] react-native-config 环境变量系统
- [ ] 微信小程序开发 — 需微信开放平台注册
- [ ] 微信支付商户号申请 — 需营业执照
- [ ] 支付回调签名验证（脚手架已就位，需商户证书后替换 stub）
- [ ] E2E 测试框架
- [ ] 监控告警

### 端到端差距（PRD 第 14 节支付流程）
| 缺失能力 | 影响 | 前置条件 | 进展 |
|----------|------|----------|------|
| POST /api/orders（付费下单） | 阻断付费转化收入流 | price_config 数据写入 | ✅ 已实现 |
| PUT /api/orders/:id/pay-callback | 阻断微信支付闭环 | 商户号 + API 证书 | ✅ 签名验证脚手架已就位(需WECHAT_PAY_PLATFORM_CERT) |
| 退款 API + 流程 | 阻断售后闭环 | 商户号 | ✅ 退款API脚手架已就位(需商户证书后替换stub) |
| 微信小程序前端 | 阻断 C 端主入口 | 开放平台注册 | ⬜ 待开发 |
| wx.requestPayment 前端调用 | 阻断支付 UI | prepay_id 返回 | ⬜ 待实现 |
| 价格配置 B 端管理页 | 阻断运营配置 | 004 迁移 + CRUD API | ✅ 已实现 |
| 订单管理 B 端管理页 | 阻断运营查看 | API 可用 | ✅ 已实现 |
| 订单详情 B 端页 | 阻断运营查看详情 | GET /orders/:id API | ✅ 已实现 |
| 设计模板 B 端管理页 | 阻断模板运营 | Template CRUD API | ✅ 已实现 |
| 审计日志 B 端页 | 阻断安全审计 | Audit log API + 过滤 | ✅ 已实现 |
| 风控事件 B 端页 | 阻断风控运营 | Risk event API | ✅ 已实现 |
| C端绑定手机号页 | 阻断微信登录闭环 | /auth/bind-phone API | ✅ 已实现 |
| C端活动规则页 | 阻断用户知情 | 静态页 | ✅ 已实现 |
| 作品详情 API | 阻断B端+移动端 | entries 表 | ✅ 已实现 |
| 用户资料更新 API | 阻断昵称修改 | app_user 表 | ✅ 已实现 |
| 活动规则 API | 阻断规则动态配置 | activity_rule 表 | ✅ 已实现 |
| useVote 动态投票上限 | 阻断不同活动不同规则 | GET /activities/:id/rules | ✅ 已实现 |
| 微信登录流程串联 | 阻断微信登录闭环 | wechatLogin + bindPhone + AuthContext | ✅ 已实现 |
| 支付验签脚手架 | 安全基础 | WECHAT_PAY_API_KEY 配置 | ✅ 已实现(待商户证书) |
| Clippy 零警告 | 代码质量 | - | ✅ 已实现 |
| Mobile CI | 持续集成 | GitHub Actions | ✅ 已实现 |
| C端用户路由(3个) | 阻断C端"我的"页面 | GET entries/votes/orders/mine | ✅ 已实现 |
| 公开端点解禁 | 阻断C端浏览活动/作品 | 从admin_routes移至user_routes | ✅ 已实现 |
| DetailScreen API规范 | 代码质量 | 使用getEntryDetail替代原始apiClient | ✅ 已实现 |
| RulesScreen动态规则 | 数据一致性 | 从API获取而非硬编码 | ✅ 已实现 |
| react-native-config | 多环境部署 | .env + 类型声明 | ✅ 已实现 |
| B端价格编辑 | 运营配置 | 编辑按钮+复用Modal | ✅ 已实现 |
| B端考勤子菜单 | 发现性 | ClockCircleOutlined + openKeys | ✅ 已实现 |
| C端用户路由(3个) | 阻断C端"我的"页面 | GET entries/votes/orders/mine | ✅ 已实现 |
| 公开端点解禁 | 阻断C端浏览活动/作品 | 从admin_routes移至user_routes | ✅ 已实现 |
| DetailScreen API规范 | 代码质量 | 使用getEntryDetail替代原始apiClient | ✅ 已实现 |
| RulesScreen动态规则 | 数据一致性 | 从API获取而非硬编码 | ✅ 已实现 |
| react-native-config | 多环境部署 | .env + 类型声明 | ✅ 已实现 |
| B端价格编辑 | 运营配置 | 编辑按钮+复用Modal | ✅ 已实现 |
| 死路由接通(6个) | 运营操作 | region/store/template update, staff list/show/update | ✅ 已实现 |
| 生产任务扩展 | 生产异常处理 | pause/error/cancel端点 + migration 005 | ✅ 已实现 |
| 对账服务 | 数据一致性保障 | 3维检测 + GET /reports/reconciliation | ✅ 已实现 |
| 移动端隐私设置 | 用户权利 | 推送/排行榜开关+数据导出+注销 | ✅ 已实现 |
| B端门店/员工详情页 | 运营查看 | StoreShow + StaffShow | ✅ 已实现 |
| Prometheus 指标 | 生产可观测性 | metrics + metrics-exporter-prometheus crate | ✅ 已实现 |
| 25个lib单元测试 | 核心逻辑质量 | crypto/production状态机/reconciliation/config | ✅ 已实现 |
| 请求ID中间件 | 分布式追踪 | X-Request-Id 生成/传播 | ✅ 已实现 |
| IP限流中间件 | 防刷防爬 | Redis INCR 100/min | ✅ 已实现 |
| 真实健康检查 | 编排器正确性 | PG SELECT 1 + Redis PING | ✅ 已实现 |
| 输入校验模块 | 安全防线 | phone/verify_code/string_max/cake_size/cream_type | ✅ 已实现 |
| 分页修复 | 防止无限制查询 | my_votes/my_orders + page_size 上限 | ✅ 已实现 |
| 订单幂等性 | 防重复付款 | entry_id+pending/paid 去重 → 409 | ✅ 已实现 |
| DB连接池超时 | 生产稳定性 | acquire 5s/idle 600s/lifetime 1800s | ✅ 已实现 |
| JSON结构化日志 | 生产可观测 | RUST_LOG 级别控制 | ✅ 已实现 |
| 优雅关机 | 资源清理 | scheduler abort() | ✅ 已实现 |
| 移动端导航类型安全 | 代码质量 | RootStackParamList + useTypedRoute | ✅ 已实现 |
| 公开端点 IP 限流 | 防暴力破解 | Redis INCR 按 IP+路径 | ✅ 已实现 |
| JWT 黑名单+注销 | Token 安全 | jti + Redis 黑名单 + refresh 轮换 | ✅ 已实现 |
| B端5个详情页 | 运营可操作性 | 投票/获奖/生产/核销/模板 show | ✅ 已实现 |
| B端报表对账标签 | 运营数据校验 | 支付-订单/投票漂移/库存漂移 3张表 | ✅ 已实现 |
