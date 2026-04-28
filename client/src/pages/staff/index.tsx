import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag } from "antd";

export const StaffList: React.FC = () => {
  const { tableProps } = useTable({ resource: "staff" });

  return (
    <List canCreate>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="name" title="姓名" width={100} />
        <Table.Column dataIndex="phone" title="手机号" width={120} />
        <Table.Column dataIndex="store_id" title="门店ID" width={80} />
        <Table.Column dataIndex="role" title="角色" width={100} render={(v: string) => <Tag>{v || "-"}</Tag>} />
        <Table.Column dataIndex="status" title="状态" width={80} render={(v: string) => (
          <Tag color={v === "active" ? "green" : "red"}>{v || "-"}</Tag>
        )} />
        <Table.Column dataIndex="created_at" title="创建时间" width={140} />
      </Table>
    </List>
  );
};

export { StaffCreate } from "./create";
