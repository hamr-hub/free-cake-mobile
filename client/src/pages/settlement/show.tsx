import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const SettlementShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/settlement/${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`获奖记录 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="活动ID">{record.activity_id}</Descriptions.Item>
        <Descriptions.Item label="作品ID">{record.entry_id}</Descriptions.Item>
        <Descriptions.Item label="用户ID">{record.user_id}</Descriptions.Item>
        <Descriptions.Item label="排名">{record.rank}</Descriptions.Item>
        <Descriptions.Item label="有效票数">{record.valid_vote_count}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record.status === "confirmed" ? "green" : "orange"}>{record.status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
