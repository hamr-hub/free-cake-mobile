# Free Cake Mobile - 技术方案

## 一、建设目标
为 Free Cake 村镇 AI 蛋糕项目构建 C端移动应用（iOS + Android），使用 React Native 跨端开发，实现用户参赛、AI 生成、投票、排行、领奖核销的完整移动端体验闭环。

## 二、技术选型

### 2.1 跨端框架
- **React Native 0.76+**（新架构 Fabric + TurboModules）
- 一套代码同时产出 iOS 和 Android 应用
- TypeScript 强类型保障

### 2.2 导航
- **React Navigation 6.x**（Stack + Tab 导航）
- 底部 Tab：首页 / 排行榜 / 我的

### 2.3 状态管理
- **React Context + useReducer**（轻量级，MVP 阶段避免过度设计）
- 认证状态全局管理
- 活动数据按页面局部管理

### 2.4 网络层
- **Axios**（复用现有 Web B端数据层经验）
- JWT token 自动注入与刷新
- 请求重试与超时配置（适配村镇弱网）
- 网络状态检测与离线降级提示

### 2.5 UI 组件库
- **React Native Paper**（Material Design 3，双端适配）
- 自定义业务组件：蛋糕卡片、倒计时横幅、风险标签、领奖二维码

### 2.6 图片处理
- **react-native-fast-image**（CDN 图片缓存与预加载）
- **react-native-view-shot**（分享海报截图生成）
- **react-native-share**（分享到微信/朋友圈）

### 2.7 定位服务
- **react-native-geolocation-service**（获取用户定位匹配赛区）
- **expo-location** 作为备用方案

### 2.8 存储
- **react-native-async-storage**（token 缓存、领奖码离线缓存）
- **react-native-mmkv**（高性能 KV 存储，替代 AsyncStorage）

### 2.9 二维码
- **react-native-qrcode-svg**（领奖码二维码生成）
- **react-native-camera**（门店扫码核销备用）

## 三、项目结构

```
mobile/
├── App.tsx                    # 应用入口（导航配置）
├── package.json               # 依赖声明
├── tsconfig.json              # TypeScript 配置
├── babel.config.js            # Babel 配置
├── metro.config.js            # Metro 打包配置
├── android/                   # Android 原生壳
├── ios/                       # iOS 原生壳
├── src/
│   ├── navigation/
│   │   ├── AppNavigator.tsx   # 根导航
│   │   ├── HomeStack.tsx      # 首页相关页面栈
│   │   ├── TabNavigator.tsx   # 底部 Tab
│   ├── screens/
│   │   ├── HomeScreen.tsx     # 首页
│   │   ├── GenerateScreen.tsx # AI 生成页
│   │   ├── PublishScreen.tsx  # 作品发布页
│   │   ├── DetailScreen.tsx   # 作品详情页
│   │   ├── RankScreen.tsx     # 排行榜页
│   │   ├── RedeemScreen.tsx   # 领奖核销页
│   │   ├── ProfileScreen.tsx  # 我的页面
│   │   ├── LoginScreen.tsx    # 登录页
│   ├── components/
│   │   ├── CakeCard.tsx       # 蛋糕作品卡片
│   │   ├── CountdownBanner.tsx# 倒计时横幅
│   │   ├── VoteButton.tsx     # 投票按钮
│   │   ├── RankBadge.tsx      # 排名徽标
│   │   ├── RiskTag.tsx        # 风险标签
│   │   ├── SharePoster.tsx    # 分享海报
│   │   ├── QRCodeCard.tsx     # 二维码领奖卡
│   │   ├── RegionGuard.tsx    # 赛区校验守卫
│   ├── services/
│   │   ├── api.ts             # API 请求封装
│   │   ├── auth.ts            # 认证服务
│   │   ├── location.ts        # 定位服务
│   │   ├── storage.ts         # 本地存储
│   │   ├── share.ts           # 分享服务
│   ├── hooks/
│   │   ├── useAuth.ts         # 认证 Hook
│   │   ├── useActivity.ts     # 活动数据 Hook
│   │   ├── useVote.ts         # 投票 Hook
│   │   ├── useLocation.ts     # 定位 Hook
│   │   ├── useNetwork.ts      # 网络状态 Hook
│   ├── context/
│   │   ├── AuthContext.tsx     # 认证上下文
│   │   ├── ActivityContext.tsx # 活动上下文
│   ├── types/
│   │   ├── activity.ts        # 活动类型定义
│   │   ├── entry.ts           # 作品类型定义
│   │   ├── vote.ts            # 投票类型定义
│   │   ├── user.ts            # 用户类型定义
│   │   ├── redeem.ts          # 核销类型定义
│   ├── utils/
│   │   ├── constants.ts       # 常量配置
│   │   ├── validators.ts      # 校验工具
│   │   ├── formatters.ts      # 格式化工具
│   ├── theme/
│   │   ├── colors.ts          # 配色（奶油黄/草莓粉/焦糖棕）
│   │   ├── typography.ts      # 字体
│   │   ├── spacing.ts         # 间距
│   │   ├── index.ts           # 主题导出
│   ├── __tests__/
│   │   ├── screens/           # 页面测试
│   │   ├── components/        # 组件测试
│   │   ├── services/          # 服务测试
│   │   ├── hooks/             # Hook 测试
```

## 四、核心模块设计

### 4.1 认证模块
- AuthContext 提供 login/logout/getToken 方法
- 登录流程：手机号 → 发送验证码 → 验证 → JWT 存储 → 自动赛区匹配
- Token 过期自动刷新，401 自动跳转登录页
- MMKV 持久化 token，支持冷启动免登录

### 4.2 赛区校验（RegionGuard）
- 登录后自动获取定位
- 定位 + 手机号归属地联合判定赛区
- 超出 10km 范围展示"暂未开放"页面
- 每次进入活动页面重新校验

### 4.3 AI 生成模块
- 5 个表单项（场景/主题/祝福/色系/风格）
- 生成过程展示 loading 动画
- 5 张效果图宫格展示
- 支持重新生成（限频 5 次/小时）
- 选择后进入发布确认页

### 4.4 投票模块
- 每日 3 票限制，前端实时计数
- 投票成功动效反馈
- 风控拦截时展示冻结原因
- 排行榜实时刷新（3-10s 缓存）

### 4.5 领奖核销模块
- 获奖通知 → 生成二维码领奖码
- 领奖码支持 MMKV 离线缓存（弱网场景）
- 展示领奖门店地址 + 导航
- 核销状态实时查询

### 4.6 分享模块
- 分享海报：作品图 + 标题 + 排名 + 二维码
- react-native-view-shot 截图生成
- react-native-share 分享到微信/朋友圈/社群

## 五、网络层设计

### 5.1 API 对接
复用现有 Axum 后端接口：
- POST /api/auth/login → 登录
- POST /api/activities/{id}/entries/generate → AI 生成
- POST /api/activities/{id}/entries → 参赛提交
- POST /api/entries/{id}/vote → 投票
- GET /api/activities/{id}/rank → 排行榜
- POST /api/redeem/verify → 核销
- GET /api/dashboard/stats → 统计数据

### 5.2 弱网适配
- 请求超时：10 秒（默认），30 秒（AI 生成）
- 自动重试：3 次（指数退避）
- 网络状态检测：无网络时展示离线提示
- 领奖码本地缓存：弱网时仍可展示二维码

## 六、视觉设计
### 6.1 配色
- 主色：奶油黄 #FFD54F / 草莓粉 #FF8A80 / 焦糖棕 #8D6E63
- 背景：浅奶油 #FFF8E1
- 强调色：免费领 #FF6D00 / Top100 #FFD600

### 6.2 字体
- iOS：系统字体 SF Pro
- Android：Roboto
- 标题 20sp / 正文 14sp / 辅助 12sp

### 6.3 间距
- 8px 基础间距单位
- 页面边距 16px
- 卡片间距 12px

## 七、风险与应对
| 风险 | 级别 | 应对 |
|-----|-----|------|
| iOS/Android 原生编译环境缺失 | P1 | 使用 Expo 或云编译服务 |
| 弱网环境 API 超时 | P1 | 请求重试 + 离线缓存降级 |
| React Native 新架构兼容 | P2 | 锁定 0.76+ 版本，使用 Fabric 组件 |
| 定位权限用户拒绝 | P1 | 提供手动选择赛区降级方案 |
| 二维码分享海报生成性能 | P2 | view-shot 异步生成，缓存海报 |

## 八、验收指标
- iOS & Android 双端功能完全一致
- 冷启动 ≤ 3 秒
- 核心交互路径 ≤ 3-5 步
- 弱网（3G）核心操作可用
- 所有 AC 对应测试通过
