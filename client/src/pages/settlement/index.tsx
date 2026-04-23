import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag, Typography } from "antd";

const statusColorMap: Record<string, string> = {
  pending: "orange",
  confirmed: "green",
  cancelled: "red",
};

const orderStatusColorMap: Record<string, string> = {
  pending: "orange",
  scheduled: "blue",
  producing: "cyan",
  completed: "green",
  cancelled: "red",
};

const redeemStatusColorMap: Record<string, string> = {
  pending: "orange",
  redeemed: "green",
  expired: "red",
};

export const SettlementList: React.FC = () => {
  const { tableProps } = useTable({ resource: "settlement" });

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="activity_id" title="活动ID" width={80} />
        <Table.Column dataIndex="entry_id" title="作品ID" width={80} />
        <Table.Column dataIndex="rank" title="排名" width={70} render={(v: number) => (
          <Typography.Text strong style={{ color: v <= 10 ? "#faad14" : undefined }}>#{v}</Typography.Text>
        )} />
        <Table.Column dataIndex="valid_vote_count" title="有效票数" width={80} />
        <Table.Column dataIndex="status" title="状态" width={100} render={(v: string) => (
          <Tag color={statusColorMap[v] || "default"}>{v}</Tag>
        )} />
        <Table.Column dataIndex="store_id" title="门店ID" width={80} />
        <Table.Column dataIndex="production_status" title="订单状态" width={100} render={(v: string) => (
          <Tag color={orderStatusColorMap[v] || "default"}>{v || "-"}</Tag>
        )} />
        <Table.Column dataIndex="redeem_status" title="核销状态" width={100} render={(v: string) => (
          <Tag color={redeemStatusColorMap[v] || "default"}>{v || "-"}</Tag>
        )} />
        <Table.Column dataIndex="created_at" title="创建时间" width={140} />
      </Table>
    </List>
  );
};
