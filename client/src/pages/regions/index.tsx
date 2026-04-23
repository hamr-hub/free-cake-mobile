import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag } from "antd";

const statusColorMap: Record<string, string> = {
  active: "green",
  inactive: "default",
};

export const RegionList: React.FC = () => {
  const { tableProps } = useTable({ resource: "regions" });

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="name" title="赛区名称" width={140} />
        <Table.Column dataIndex="province" title="省份" width={100} />
        <Table.Column dataIndex="city" title="城市" width={100} />
        <Table.Column dataIndex="coverage_radius_km" title="覆盖半径(km)" width={110} />
        <Table.Column dataIndex="status" title="状态" width={80} render={(v: string) => (
          <Tag color={statusColorMap[v] || "default"}>{v || "-"}</Tag>
        )} />
        <Table.Column dataIndex="created_at" title="创建时间" width={140} />
      </Table>
    </List>
  );
};
