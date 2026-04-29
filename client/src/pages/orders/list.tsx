import React from "react";
import { useCustom } from "@refinedev/core";
import { Table, Card, Tag, Space, Button } from "antd";

export const OrderList: React.FC = () => {
  const [payFilter, setPayFilter] = React.useState<string | undefined>(undefined);

  const { query } = useCustom({
    url: "/orders",
    method: "get",
    config: { query: { ...(payFilter ? { pay_status: payFilter } : {}) } },
  });

  const orders = (query.data as any)?.data?.list ?? [];
  const total = (query.data as any)?.data?.total ?? 0;

  const payStatusMap: Record<string, { label: string; color: string }> = {
    free: { label: "免费", color: "green" },
    pending: { label: "待支付", color: "orange" },
    paid: { label: "已支付", color: "blue" },
    closed: { label: "已关闭", color: "default" },
    refunded: { label: "已退款", color: "red" },
  };

  const columns = [
    { title: "订单ID", dataIndex: "id", key: "id", width: 70 },
    { title: "类型", dataIndex: "order_type", key: "order_type", render: (v: string) => <Tag>{v === "paid" ? "付费" : "免费"}</Tag> },
    { title: "金额", dataIndex: "amount", key: "amount", render: (v: number) => v ? `¥${Number(v).toFixed(2)}` : "-" },
    {
      title: "支付状态",
      dataIndex: "pay_status",
      key: "pay_status",
      render: (v: string) => {
        const s = payStatusMap[v] || { label: v, color: "default" };
        return <Tag color={s.color}>{s.label}</Tag>;
      },
    },
    { title: "门店ID", dataIndex: "store_id", key: "store_id", width: 80 },
    {
      title: "生产状态",
      dataIndex: "production_status",
      key: "production_status",
      render: (v: string) => <Tag>{v}</Tag>,
    },
    {
      title: "核销状态",
      dataIndex: "redeem_status",
      key: "redeem_status",
      render: (v: string) => <Tag color={v === "redeemed" ? "green" : "default"}>{v === "redeemed" ? "已核销" : v}</Tag>,
    },
    {
      title: "创建时间",
      dataIndex: "created_at",
      key: "created_at",
      render: (v: string) => v ? new Date(v).toLocaleString("zh-CN") : "-",
    },
  ];

  return (
    <Card
      title="订单管理"
      extra={
        <Space>
          {(["pending", "paid", "closed", "free"] as const).map((s) => (
            <Button key={s} size="small" type={payFilter === s ? "primary" : "default"} onClick={() => setPayFilter((f) => f === s ? undefined : s)}>
              {payStatusMap[s]?.label || s}
            </Button>
          ))}
        </Space>
      }
    >
      <Table rowKey="id" dataSource={orders} columns={columns} loading={query.isLoading} pagination={{ total, pageSize: 20 }} />
    </Card>
  );
};
