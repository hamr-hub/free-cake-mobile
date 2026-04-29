import React from "react";
import { Card, Descriptions, Tag, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

const statusColor: Record<string, string> = {
  pending: "orange",
  in_progress: "blue",
  completed: "green",
  paused: "cyan",
  error: "red",
  cancelled: "default",
};

export const ProductionShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/production?id=${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`生产任务 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="订单ID">{record.order_id}</Descriptions.Item>
        <Descriptions.Item label="批次ID">{record.batch_id}</Descriptions.Item>
        <Descriptions.Item label="门店ID">{record.store_id}</Descriptions.Item>
        <Descriptions.Item label="模板ID">{record.template_id}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={statusColor[record.task_status] || "default"}>{record.task_status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="异常描述" span={2}>{record.error_description || "-"}</Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
