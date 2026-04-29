import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification, useCustom, useCustomMutation } from "@refinedev/core";
import { Table, Tag, Typography, Button, Popconfirm, Space, Descriptions, Modal, Statistic, Row, Col, Card } from "antd";
import { ThunderboltOutlined, EyeOutlined, SendOutlined, UndoOutlined } from "@ant-design/icons";

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

const orderStatusLabel: Record<string, string> = {
  pending: "待排产",
  scheduled: "已排产",
  producing: "生产中",
  completed: "已完成",
  cancelled: "已取消",
};

const redeemStatusLabel: Record<string, string> = {
  pending: "待核销",
  redeemed: "已核销",
  expired: "已过期",
};

const payStatusLabel: Record<string, { label: string; color: string }> = {
  free: { label: "免费", color: "green" },
  pending: { label: "待支付", color: "orange" },
  paid: { label: "已支付", color: "blue" },
  closed: { label: "已关闭", color: "default" },
  refunded: { label: "已退款", color: "red" },
};

const refundStatusColorMap: Record<string, string> = {
  refunded: "red",
  rejected: "default",
  pending: "orange",
};

export const SettlementList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "settlement" });
  const { open } = useNotification();
  const { mutateAsync: settleMutate } = useCustomMutation();
  const { mutateAsync: resendMutate } = useCustomMutation();
  const { mutateAsync: refundMutate } = useCustomMutation();

  const [detailVisible, setDetailVisible] = useState(false);
  const [detailRecord, setDetailRecord] = useState<any>(null);

  const { query: detailQuery } = useCustom({
    url: `/api/orders/${detailRecord?.id || 0}`,
    method: "get",
    queryOptions: { enabled: !!detailRecord && detailVisible },
  });
  const detailData = detailQuery.data?.data;

  const dataSource = tableProps?.dataSource || [];
  const pendingCount = dataSource.filter((r: any) => r.status === "pending").length;
  const completedCount = dataSource.filter((r: any) => r.production_status === "completed").length;
  const redeemedCount = dataSource.filter((r: any) => r.redeem_status === "redeemed").length;
  const paidCount = dataSource.filter((r: any) => r.order_type === "paid").length;

  const handleSettle = async (activityId: number) => {
    try {
      const result = await settleMutate({
        url: `/api/activities/${activityId}/settle`,
        method: "post",
        values: { force: false },
      });
      const data = result?.data;
      open?.({ type: "success", message: `结算完成：${data.winner_count} 名获奖者，${data.order_count} 个订单` });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "结算失败", description: e.message });
    }
  };

  const handleResendCode = async (orderId: number) => {
    try {
      await resendMutate({
        url: `/api/orders/${orderId}/resend-code`,
        method: "post",
        values: {},
      });
      open?.({ type: "success", message: "核销码已重发" });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "重发失败", description: e.message });
    }
  };

  const handleRefund = async (orderId: number) => {
    try {
      await refundMutate({
        url: `/api/orders/${orderId}/refund`,
        method: "post",
        values: { reason: "Admin initiated refund" },
      });
      open?.({ type: "success", message: "退款已提交" });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "退款失败", description: e.message });
    }
  };

  const showDetail = (record: any) => {
    setDetailRecord(record);
    setDetailVisible(true);
  };

  const expandable = {
    expandedRowRender: (record: any) => (
      <Descriptions column={3} size="small" bordered>
        <Descriptions.Item label="订单ID">{record.id}</Descriptions.Item>
        <Descriptions.Item label="活动ID">{record.activity_id}</Descriptions.Item>
        <Descriptions.Item label="作品ID">{record.entry_id}</Descriptions.Item>
        <Descriptions.Item label="排名">{record.rank}</Descriptions.Item>
        <Descriptions.Item label="有效票数">{record.valid_vote_count}</Descriptions.Item>
        <Descriptions.Item label="门店ID">{record.store_id}</Descriptions.Item>
        <Descriptions.Item label="订单类型">
          <Tag color={record.order_type === "paid" ? "blue" : "green"}>
            {record.order_type === "paid" ? "付费" : "免费"}
          </Tag>
        </Descriptions.Item>
        {record.order_type === "paid" && (
          <>
            <Descriptions.Item label="金额">
              {record.amount != null ? `¥${Number(record.amount).toFixed(2)}` : "-"}
            </Descriptions.Item>
            <Descriptions.Item label="支付状态">
              <Tag color={payStatusLabel[record.pay_status]?.color || "default"}>
                {payStatusLabel[record.pay_status]?.label || record.pay_status || "-"}
              </Tag>
            </Descriptions.Item>
            <Descriptions.Item label="退款状态">
              <Tag color={refundStatusColorMap[record.refund_status] || "default"}>
                {record.refund_status || "-"}
              </Tag>
            </Descriptions.Item>
          </>
        )}
        <Descriptions.Item label="订单状态">
          <Tag color={orderStatusColorMap[record.production_status] || "default"}>
            {orderStatusLabel[record.production_status] || record.production_status || "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="核销状态">
          <Tag color={redeemStatusColorMap[record.redeem_status] || "default"}>
            {redeemStatusLabel[record.redeem_status] || record.redeem_status || "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="结算状态">
          <Tag color={statusColorMap[record.status] || "default"}>{record.status}</Tag>
        </Descriptions.Item>
      </Descriptions>
    ),
  };

  return (
    <>
      <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
        <Col span={6}>
          <Card><Statistic title="待结算" value={pendingCount} valueStyle={{ color: "#faad14" }} /></Card>
        </Col>
        <Col span={6}>
          <Card><Statistic title="已完成生产" value={completedCount} valueStyle={{ color: "#52c41a" }} /></Card>
        </Col>
        <Col span={6}>
          <Card><Statistic title="已核销" value={redeemedCount} /></Card>
        </Col>
        <Col span={6}>
          <Card><Statistic title="付费订单" value={paidCount} valueStyle={{ color: "#1890ff" }} /></Card>
        </Col>
      </Row>

      <List>
        <Table {...tableProps} rowKey="id" expandable={expandable}>
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
            <Tag color={orderStatusColorMap[v] || "default"}>{orderStatusLabel[v] || v || "-"}</Tag>
          )} />
          <Table.Column dataIndex="redeem_status" title="核销状态" width={100} render={(v: string) => (
            <Tag color={redeemStatusColorMap[v] || "default"}>{redeemStatusLabel[v] || v || "-"}</Tag>
          )} />
          <Table.Column dataIndex="order_type" title="类型" width={70} render={(v: string) => (
            <Tag color={v === "paid" ? "blue" : "green"}>{v === "paid" ? "付费" : "免费"}</Tag>
          )} />
          <Table.Column dataIndex="amount" title="金额" width={80} render={(v: number) => v ? `¥${Number(v).toFixed(2)}` : "-"} />
          <Table.Column dataIndex="refund_status" title="退款" width={80} render={(v: string) => (
            v ? <Tag color={refundStatusColorMap[v] || "default"}>{v}</Tag> : "-"
          )} />
          <Table.Column title="操作" width={260} render={(_, record: any) => (
            <Space>
              <Button type="link" size="small" icon={<EyeOutlined />} onClick={() => showDetail(record)}>详情</Button>
              {record.status === "pending" && (
                <Popconfirm title="确认结算此活动？" onConfirm={() => handleSettle(record.activity_id)}>
                  <Button type="link" size="small" icon={<ThunderboltOutlined />} danger>结算</Button>
                </Popconfirm>
              )}
              {record.redeem_status === "expired" && (
                <Popconfirm title="确认重发核销码？" onConfirm={() => handleResendCode(record.id)}>
                  <Button type="link" size="small" icon={<SendOutlined />}>重发码</Button>
                </Popconfirm>
              )}
              {record.order_type === "paid" && record.pay_status === "paid" && !record.refund_status && (
                <Popconfirm title="确认退款此订单？" onConfirm={() => handleRefund(record.id)}>
                  <Button type="link" size="small" icon={<UndoOutlined />} danger>退款</Button>
                </Popconfirm>
              )}
            </Space>
          )} />
        </Table>
      </List>

      <Modal
        title={`订单详情 #${detailRecord?.id}`}
        open={detailVisible}
        onCancel={() => setDetailVisible(false)}
        footer={null}
        width={600}
      >
        <Descriptions column={2} bordered>
          <Descriptions.Item label="订单ID">{detailData?.id}</Descriptions.Item>
          <Descriptions.Item label="活动ID">{detailData?.activity_id}</Descriptions.Item>
          <Descriptions.Item label="作品ID">{detailData?.entry_id}</Descriptions.Item>
          <Descriptions.Item label="排名">{detailData?.rank}</Descriptions.Item>
          <Descriptions.Item label="门店ID">{detailData?.store_id}</Descriptions.Item>
          <Descriptions.Item label="核销码">{detailData?.redeem_code || "-"}</Descriptions.Item>
          <Descriptions.Item label="订单类型">
            <Tag color={detailData?.order_type === "paid" ? "blue" : "green"}>
              {detailData?.order_type === "paid" ? "付费" : "免费"}
            </Tag>
          </Descriptions.Item>
          {detailData?.order_type === "paid" && (
            <>
              <Descriptions.Item label="金额">
                {detailData?.amount != null ? `¥${Number(detailData.amount).toFixed(2)}` : "-"}
              </Descriptions.Item>
              <Descriptions.Item label="支付状态">
                <Tag color={payStatusLabel[detailData?.pay_status]?.color || "default"}>
                  {payStatusLabel[detailData?.pay_status]?.label || detailData?.pay_status || "-"}
                </Tag>
              </Descriptions.Item>
              <Descriptions.Item label="退款状态">
                <Tag color={refundStatusColorMap[detailData?.refund_status] || "default"}>
                  {detailData?.refund_status || "-"}
                </Tag>
              </Descriptions.Item>
            </>
          )}
        </Descriptions>
      </Modal>
    </>
  );
};
