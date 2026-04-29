import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const RedeemShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/redeem?id=${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`核销记录 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="订单ID">{record.order_id}</Descriptions.Item>
        <Descriptions.Item label="门店ID">{record.store_id}</Descriptions.Item>
        <Descriptions.Item label="核销码">{record.redeem_code || "-"}</Descriptions.Item>
        <Descriptions.Item label="核销状态">
          <Tag color={record.redeem_status === "redeemed" ? "green" : "default"}>{record.redeem_status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="生产状态">
          <Tag>{record.production_status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
