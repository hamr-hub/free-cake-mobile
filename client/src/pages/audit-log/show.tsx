import React from "react";
import { Card, Descriptions, Button } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";

export const AuditLogShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const { query } = useCustom({ url: `/api/audit_log/${id}`, method: "get" });
  const record = (query.data as any)?.data ?? {};

  if (query.isLoading) return <Card loading />;

  return (
    <Card title={`审计日志 #${id}`} extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record.id ?? id}</Descriptions.Item>
        <Descriptions.Item label="操作者ID">{record.operator_id}</Descriptions.Item>
        <Descriptions.Item label="操作">{record.action || "-"}</Descriptions.Item>
        <Descriptions.Item label="目标类型">{record.target_type || "-"}</Descriptions.Item>
        <Descriptions.Item label="目标ID">{record.target_id}</Descriptions.Item>
        <Descriptions.Item label="描述" span={2}>{record.detail || "-"}</Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
