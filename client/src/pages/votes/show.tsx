import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const VoteShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/votes/risk?id=${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`投票记录 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="活动ID">{record.activity_id}</Descriptions.Item>
        <Descriptions.Item label="作品ID">{record.entry_id}</Descriptions.Item>
        <Descriptions.Item label="投票用户ID">{record.voter_user_id}</Descriptions.Item>
        <Descriptions.Item label="投票状态">
          <Tag color={record.vote_status === "valid" ? "green" : record.vote_status === "frozen" ? "orange" : "red"}>
            {record.vote_status ?? "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="风险标签">{record.risk_tags || "-"}</Descriptions.Item>
        <Descriptions.Item label="IP">{record.ip || "-"}</Descriptions.Item>
        <Descriptions.Item label="GeoHash">{record.geohash || "-"}</Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
