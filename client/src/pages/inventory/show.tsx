import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const InventoryShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/inventory/items/${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`库存 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="门店ID">{record.store_id}</Descriptions.Item>
        <Descriptions.Item label="名称">{record.name || "-"}</Descriptions.Item>
        <Descriptions.Item label="数量">{record.quantity ?? 0}</Descriptions.Item>
        <Descriptions.Item label="单位">{record.unit || "-"}</Descriptions.Item>
        <Descriptions.Item label="安全库存">{record.safety_threshold ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record.status === "active" ? "green" : "default"}>{record.status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间">{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
