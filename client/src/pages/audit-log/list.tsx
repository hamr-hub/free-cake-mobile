import React from "react";
import { useCustom } from "@refinedev/core";
import { Table, Card, Tag, DatePicker, Select, Space } from "antd";

export const AuditLogList: React.FC = () => {
  const [action, setAction] = React.useState<string | undefined>();
  const [dateRange, setDateRange] = React.useState<[string, string] | undefined>();

  const { RangePicker } = DatePicker;

  const params: Record<string, any> = {};
  if (action) params.action = action;
  if (dateRange) {
    params.start_date = dateRange[0];
    params.end_date = dateRange[1];
  }

  const { query } = useCustom({ url: "/audit_log", method: "get", config: { query: params } });

  const logs = (query.data as any)?.data?.list ?? [];
  const total = (query.data as any)?.data?.total ?? 0;

  const columns = [
    { title: "ID", dataIndex: "id", key: "id", width: 60 },
    { title: "操作者ID", dataIndex: "operator_id", key: "operator_id", width: 90 },
    { title: "动作", dataIndex: "action", key: "action", render: (v: string) => <Tag>{v}</Tag> },
    { title: "目标类型", dataIndex: "target_type", key: "target_type", width: 100 },
    { title: "目标ID", dataIndex: "target_id", key: "target_id", width: 80 },
    { title: "详情", dataIndex: "detail", key: "detail", ellipsis: true },
    { title: "时间", dataIndex: "created_at", key: "created_at", width: 160, render: (v: string) => v ? new Date(v).toLocaleString("zh-CN") : "-" },
  ];

  return (
    <Card
      title="审计日志"
      extra={
        <Space>
          <Select
            allowClear
            placeholder="动作类型"
            style={{ width: 140 }}
            value={action}
            onChange={(v) => setAction(v)}
            options={[
              { value: "activity_settled", label: "活动结算" },
              { value: "auto_settle", label: "自动结算" },
              { value: "refund_order", label: "退款" },
              { value: "pay_callback", label: "支付回调" },
              { value: "entry_frozen", label: "作品冻结" },
              { value: "votes_deducted", label: "扣票" },
              { value: "create_paid_order", label: "付费下单" },
              { value: "order_scheduled", label: "订单排产" },
              { value: "resend_redeem_code", label: "重发码" },
            ]}
          />
          <RangePicker onChange={(_, ds) => { setDateRange(ds as any); }} />
        </Space>
      }
    >
      <Table rowKey="id" dataSource={logs} columns={columns} loading={query.isLoading} pagination={{ total, pageSize: 20 }} />
    </Card>
  );
};
