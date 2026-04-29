import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

const statusColorMap: Record<string, string> = {
  active: "green",
  inactive: "default",
};

export const RegionShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/regions/${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card
      title={`赛区 #${id}`}
      extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}
    >
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="赛区名称">{record.name || "-"}</Descriptions.Item>
        <Descriptions.Item label="省份">{record.province || "-"}</Descriptions.Item>
        <Descriptions.Item label="城市">{record.city || "-"}</Descriptions.Item>
        <Descriptions.Item label="中心纬度">{record.center_lat ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="中心经度">{record.center_lng ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="覆盖半径(km)">{record.coverage_radius_km ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={statusColorMap[record.status] || "default"}>{record.status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
