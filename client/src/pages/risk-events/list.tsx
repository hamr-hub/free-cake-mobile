import React from "react";
import { useCustom } from "@refinedev/core";
import { Table, Card, Tag, Select, Space } from "antd";

const riskLevelColor: Record<string, string> = {
  low: "green",
  medium: "orange",
  high: "red",
};

const riskTypeLabel: Record<string, string> = {
  same_phone_cluster: "重复手机号",
  openid_duplicate: "重复OpenID",
  same_device_cluster: "设备指纹",
  geo_cluster: "地理位置聚类",
  ip_cluster: "IP聚类",
};

export const RiskEventList: React.FC = () => {
  const [riskLevel, setRiskLevel] = React.useState<string | undefined>();
  const [riskType, setRiskType] = React.useState<string | undefined>();

  const params: Record<string, any> = {};
  if (riskLevel) params.risk_level = riskLevel;
  if (riskType) params.risk_type = riskType;

  const { query } = useCustom({ url: "/risk_events", method: "get", config: { query: params } });

  const events = (query.data as any)?.data?.list ?? [];
  const total = (query.data as any)?.data?.total ?? 0;

  const columns = [
    { title: "ID", dataIndex: "id", key: "id", width: 60 },
    { title: "活动ID", dataIndex: "activity_id", key: "activity_id", width: 80 },
    {
      title: "风险类型",
      dataIndex: "risk_type",
      key: "risk_type",
      width: 120,
      render: (v: string) => <Tag>{riskTypeLabel[v] || v}</Tag>,
    },
    {
      title: "风险等级",
      dataIndex: "risk_level",
      key: "risk_level",
      width: 90,
      render: (v: string) => <Tag color={riskLevelColor[v] || "default"}>{v === "low" ? "低" : v === "medium" ? "中" : v === "high" ? "高" : v}</Tag>,
    },
    { title: "详情", dataIndex: "description", key: "description", ellipsis: true },
    { title: "创建时间", dataIndex: "created_at", key: "created_at", width: 160, render: (v: string) => v ? new Date(v).toLocaleString("zh-CN") : "-" },
  ];

  return (
    <Card
      title="风控事件"
      extra={
        <Space>
          <Select
            allowClear
            placeholder="风险等级"
            style={{ width: 120 }}
            value={riskLevel}
            onChange={(v) => setRiskLevel(v)}
            options={[
              { value: "low", label: "低" },
              { value: "medium", label: "中" },
              { value: "high", label: "高" },
            ]}
          />
          <Select
            allowClear
            placeholder="风险类型"
            style={{ width: 140 }}
            value={riskType}
            onChange={(v) => setRiskType(v)}
            options={Object.entries(riskTypeLabel).map(([v, l]) => ({ value: v, label: l }))}
          />
        </Space>
      }
    >
      <Table rowKey="id" dataSource={events} columns={columns} loading={query.isLoading} pagination={{ total, pageSize: 20 }} />
    </Card>
  );
};
