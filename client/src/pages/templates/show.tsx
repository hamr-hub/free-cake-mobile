import React from "react";
import { useShow } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag } from "antd";

export const TemplateShow: React.FC = () => {
  const { query } = useShow();
  const record = query.data?.data as any;

  return (
    <Show isLoading={query.isLoading}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
        <Descriptions.Item label="名称">{record?.name}</Descriptions.Item>
        <Descriptions.Item label="图片URL" span={2}>{record?.image_url || "-"}</Descriptions.Item>
        <Descriptions.Item label="尺寸">{record?.cake_size || "-"}</Descriptions.Item>
        <Descriptions.Item label="奶油类型">{record?.cream_type || "-"}</Descriptions.Item>
        <Descriptions.Item label="装饰参数" span={2}>{record?.decoration_params ? JSON.stringify(record.decoration_params) : "-"}</Descriptions.Item>
        <Descriptions.Item label="可生产等级">{record?.producible_level || "-"}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record?.status === "active" ? "green" : "default"}>{record?.status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间">{record?.created_at || "-"}</Descriptions.Item>
        <Descriptions.Item label="更新时间">{record?.updated_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Show>
  );
};
