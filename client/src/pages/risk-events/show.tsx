import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const RiskEventShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/risk_events/${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`风控事件 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="活动ID">{record.activity_id}</Descriptions.Item>
        <Descriptions.Item label="作品ID">{record.entry_id}</Descriptions.Item>
        <Descriptions.Item label="风险类型">{record.risk_type || "-"}</Descriptions.Item>
        <Descriptions.Item label="风险等级">
          <Tag color={record.risk_level === "high" ? "red" : record.risk_level === "medium" ? "orange" : "blue"}>
            {record.risk_level || "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record.status === "open" ? "orange" : record.status === "resolved" ? "green" : "default"}>
            {record.status ?? "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="描述" span={2}>{record.description || "-"}</Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
