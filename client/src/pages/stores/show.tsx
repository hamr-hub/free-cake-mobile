import React from "react";
import { useShow } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag } from "antd";

const statusColorMap: Record<string, string> = {
  active: "green",
  inactive: "red",
  maintenance: "orange",
};

const statusLabelMap: Record<string, string> = {
  active: "正常运营",
  inactive: "停业",
  maintenance: "设备维护",
};

export const StoreShow: React.FC = () => {
  const { query } = useShow({ resource: "stores" });
  const record = query.data?.data;

  return (
    <Show isLoading={query.isLoading}>
      <Descriptions column={2} bordered>
        <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
        <Descriptions.Item label="门店名称">{record?.name}</Descriptions.Item>
        <Descriptions.Item label="赛区ID">{record?.region_id}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={statusColorMap[record?.status] || "default"}>
            {statusLabelMap[record?.status] || record?.status}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="地址" span={2}>{record?.address || "-"}</Descriptions.Item>
        <Descriptions.Item label="经度">{record?.center_lng ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="纬度">{record?.center_lat ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="日产能">{record?.daily_capacity ?? "-"}</Descriptions.Item>
        <Descriptions.Item label="设备状态">
          <Tag color={record?.device_status === "online" ? "green" : "red"}>
            {record?.device_status === "online" ? "在线" : record?.device_status === "warning" ? "告警" : "离线"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="联系电话">{record?.contact_phone || "-"}</Descriptions.Item>
        <Descriptions.Item label="创建时间">{record?.created_at}</Descriptions.Item>
      </Descriptions>
    </Show>
  );
};
