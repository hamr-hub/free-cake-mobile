# Free Cake 项目文档总览

## 1. 项目目标
Free Cake 面向村镇下沉市场，采用“云端 B 端统管 + 村镇 C 端执行”模式，完成 AI 生成蛋糕设计、区域投票、Top100 生产调度、到店核销自提的完整闭环。

## 2. 文档目录
- `docs/01-product-overview.md`：产品定位、目标用户、业务闭环
- `docs/02-market-research.md`：市场调研、行业规律、竞品与机会判断
- `docs/03-business-rules.md`：活动规则、风控规则、履约规则
- `docs/04-technical-solution.md`：完整技术方案
- `docs/05-system-architecture.md`：系统架构与模块边界
- `docs/06-data-model.md`：核心数据模型
- `docs/07-delivery-plan.md`：分期实施与里程碑
- `docs/08-risk-and-compliance.md`：风险、食品安全、合规要求
- `docs/09-ui-design.md`：C 端、B 端、门店端界面方案

## 3. 核心结论
- 该项目本质是“低成本乡镇履约节点 + 云端统一运营中台”。
- 成败关键不在单点 AI 能力，而在区域限制、反作弊、生产标准化、供应链稳定性。
- MVP 应优先验证：单镇获客成本、Top100 核销率、产线稳定性、付费转化率。
- 技术上应先以“模板化 AI 设计 + 半自动生产调度 + 人工兜底履约”为第一阶段，不宜首期全量押注复杂 ROS2 自主生产。
