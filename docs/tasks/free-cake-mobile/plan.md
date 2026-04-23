# Free Cake Mobile - 任务计划

## 总览
- **任务ID**: free-cake-mobile
- **平台**: Mobile (React Native iOS + Android)
- **技术栈**: React Native 0.76+ / TypeScript / React Navigation / React Native Paper
- **测试覆盖率目标**: 80%
- **TDD 模式**: RED → GREEN → REFACTOR

## 阶段划分

### Phase 1: 项目基础设施 (T0-infra)
- 初始化 React Native 项目
- 配置 TypeScript / Babel / Metro
- 定义主题配色（奶油黄/草莓粉/焦糖棕）
- App.tsx 入口渲染

### Phase 2: 类型定义 (T1-types)
- Activity / Entry / Vote / User / Redeem 类型
- 手机号校验与赛区范围校验
- 日期/票数/排名格式化

### Phase 3: 服务层 (T2-service-test → P0-services)
- API 请求封装（Axios + JWT + 重试 + 超时）
- 认证服务（登录/验证码/token）
- 定位服务（GPS/赛区匹配）
- 本地存储（MMKV/领奖码离线缓存）
- 分享服务（海报/微信）

### Phase 4: 认证 (T3-auth-test → P0-auth)
- AuthContext 全局认证管理
- 登录页（手机号+验证码）
- 赛区校验守卫（RegionGuard）

### Phase 5: 导航与首页 (T4-nav-test → P0-navigation-home)
- 根导航（登录/主页切换）
- Tab 导航（首页/排行/我的）
- 首页（Banner + CTA + 热门作品 + 规则）
- 倒计时横幅 + 蛋糕卡片组件

### Phase 6: AI生成与发布 (T5-generate-test → P0-generate-publish)
- AI 生成页（表单 → 5图 → 选图）
- 作品发布页（确认 + 分享海报）

### Phase 7: 投票与排行 (T6-vote-test → P0-vote-rank)
- 作品详情页（大图 + 投票 + 排名）
- 排行榜页（Top榜单 + 赛区筛选）
- 投票 Hook + 投票按钮组件

### Phase 8: 领奖核销与我的 (T7-redeem-test → P0-redeem)
- 领奖核销页（二维码 + 门店 + 状态）
- 我的页面（作品/投票/领奖状态）
- 二维码领奖卡组件

## 关键依赖链
```
T0-infra → T1-types → T2-service-test → P0-services → T3-auth-test → P0-auth
→ T4-nav-test → P0-navigation-home → T5-generate-test → P0-generate-publish
→ T6-vote-test → P0-vote-rank → T7-redeem-test → P0-redeem
```

## 测试命令
```bash
cd mobile && npx jest              # 运行所有测试
cd mobile && npx jest -- __tests__/screens/LoginScreen.test.tsx  # 指定测试
cd mobile && npx tsc --noEmit      # 类型检查
cd mobile && npx eslint src/ --fix # Lint检查
```
