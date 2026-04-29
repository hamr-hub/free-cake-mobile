import React from "react";
import { useShow } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag } from "antd";

export const PriceShow: React.FC = () => {
  const { query } = useShow();
  const record = query.data?.data as any;

  return (
    <Show isLoading={query.isLoading}>
      <Descriptions column={2} bordered size="small">
        <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
        <Descriptions.Item label="赛区ID">{record?.region_id}</Descriptions.Item>
        <Descriptions.Item label="蛋糕尺寸">{record?.cake_size || "-"}</Descriptions.Item>
        <Descriptions.Item label="奶油类型">{record?.cream_type || "-"}</Descriptions.Item>
        <Descriptions.Item label="价格">{record?.price != null ? `¥${Number(record.price).toFixed(2)}` : "-"}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record?.status === "active" ? "green" : "default"}>{record?.status ?? "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>{record?.created_at || "-"}</Descriptions.Item>
      </Descriptions>
    </Show>
  );
};
