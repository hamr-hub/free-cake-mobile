import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag } from "antd";

const statusColorMap: Record<string, string> = {
  active: "green",
  inactive: "red",
};

export const StoreList: React.FC = () => {
  const { tableProps } = useTable({ resource: "stores" });

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="name" title="门店名称" width={140} />
        <Table.Column dataIndex="region_id" title="赛区ID" width={80} />
        <Table.Column dataIndex="address" title="地址" ellipsis />
        <Table.Column dataIndex="lat" title="纬度" width={90} />
        <Table.Column dataIndex="lng" title="经度" width={90} />
        <Table.Column dataIndex="daily_capacity" title="日产能" width={80} />
        <Table.Column dataIndex="contact_name" title="联系人" width={100} />
        <Table.Column dataIndex="contact_phone" title="联系电话" width={120} />
        <Table.Column dataIndex="status" title="状态" width={80} render={(v: string) => (
          <Tag color={statusColorMap[v] || "default"}>{v}</Tag>
        )} />
        <Table.Column dataIndex="created_at" title="创建时间" width={140} />
      </Table>
    </List>
  );
};
