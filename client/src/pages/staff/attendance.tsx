import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag } from "antd";

export const StaffAttendance: React.FC = () => {
  const { tableProps } = useTable({ resource: "attendance" });

  const statusColor: Record<string, string> = {
    normal: "green",
    late: "orange",
    absent: "red",
  };

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="staff_id" title="员工ID" width={80} />
        <Table.Column dataIndex="store_id" title="门店ID" width={80} />
        <Table.Column dataIndex="check_in_at" title="签到时间" width={160} />
        <Table.Column dataIndex="check_out_at" title="签退时间" width={160} />
        <Table.Column dataIndex="status" title="状态" width={80} render={(v: string) => (
          <Tag color={statusColor[v] || "default"}>{v}</Tag>
        )} />
      </Table>
    </List>
  );
};
