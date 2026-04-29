import React from "react";
import { useShow } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag } from "antd";

export const StaffShow: React.FC = () => {
  const { query } = useShow({ resource: "staff" });
  const record = query.data?.data;

  return (
    <Show isLoading={query.isLoading}>
      <Descriptions column={2} bordered>
        <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
        <Descriptions.Item label="姓名">{record?.name}</Descriptions.Item>
        <Descriptions.Item label="手机号">{record?.phone || "-"}</Descriptions.Item>
        <Descriptions.Item label="门店ID">{record?.store_id}</Descriptions.Item>
        <Descriptions.Item label="角色">
          <Tag>{record?.role || "-"}</Tag>
        </Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record?.status === "active" ? "green" : "red"}>
            {record?.status === "active" ? "在职" : "离职"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间">{record?.created_at}</Descriptions.Item>
        <Descriptions.Item label="更新时间">{record?.updated_at}</Descriptions.Item>
      </Descriptions>
    </Show>
  );
};
