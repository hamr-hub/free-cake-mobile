import React, { useState } from "react";
import { Card, Descriptions, Tag, Button, Modal, Input, Space, message, Spin } from "antd";
import { ArrowLeftOutlined, MoneyCollectOutlined, CloseCircleOutlined } from "@ant-design/icons";
import { useShow } from "@refinedev/core";

const payStatusLabel: Record<string, { label: string; color: string }> = {
  free: { label: "免费", color: "green" },
  pending: { label: "待支付", color: "orange" },
  paid: { label: "已支付", color: "blue" },
  closed: { label: "已关闭", color: "default" },
  refunded: { label: "已退款", color: "red" },
};

async function apiPost(url: string, body: any): Promise<any> {
  const token = localStorage.getItem("token") || "";
  const res = await fetch(`/api${url}`, {
    method: "POST",
    headers: { "Content-Type": "application/json", Authorization: `Bearer ${token}` },
    body: JSON.stringify(body),
  });
  if (!res.ok) {
    const err = await res.json().catch(() => ({ message: res.statusText }));
    throw new Error(err.message || err.error || "Request failed");
  }
  return res.json();
}

export const OrderShow: React.FC = () => {
  const { query } = useShow({ meta: { resource: "orders" } });
  const record = query.data?.data as any;
  const [refundModalOpen, setRefundModalOpen] = useState(false);
  const [refundReason, setRefundReason] = useState("");
  const [loading, setLoading] = useState(false);

  if (query.isLoading) return <Card loading />;

  const handleRefund = async () => {
    setLoading(true);
    try {
      await apiPost(`/orders/${record.id}/refund`, { reason: refundReason || "管理员发起退款" });
      message.success("退款成功");
      setRefundModalOpen(false);
      query.refetch();
    } catch (e: any) {
      message.error(e.message || "退款失败");
    } finally {
      setLoading(false);
    }
  };

  const handleCancel = () => {
    Modal.confirm({
      title: "确认取消订单？",
      content: "取消后订单将关闭，核销码将失效。",
      onOk: async () => {
        try {
          await apiPost(`/orders/${record.id}/cancel`, { reason: "管理员取消订单" });
          message.success("订单已取消");
          query.refetch();
        } catch (e: any) {
          message.error(e.message || "取消失败");
        }
      },
    });
  };

  const canRefund = record?.order_type === "paid" && record?.pay_status === "paid" && record?.refund_status !== "refunded";
  const canCancel = record?.pay_status === "pending";

  return (
    <Spin spinning={loading}>
      <Card
        title={`订单 #${record?.id ?? "-"}`}
        extra={
          <Space>
            {canRefund && (
              <Button icon={<MoneyCollectOutlined />} danger onClick={() => setRefundModalOpen(true)}>
                退款
              </Button>
            )}
            {canCancel && (
              <Button icon={<CloseCircleOutlined />} onClick={handleCancel}>
                取消订单
              </Button>
            )}
            <Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>
          </Space>
        }
      >
        <Descriptions column={2} bordered>
          <Descriptions.Item label="订单ID">{record?.id}</Descriptions.Item>
          <Descriptions.Item label="类型">
            <Tag color={record?.order_type === "paid" ? "blue" : "green"}>
              {record?.order_type === "paid" ? "付费" : "免费"}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="金额">
            {record?.amount != null ? `¥${Number(record.amount).toFixed(2)}` : "-"}
          </Descriptions.Item>
          <Descriptions.Item label="支付状态">
            <Tag color={payStatusLabel[record?.pay_status]?.color || "default"}>
              {payStatusLabel[record?.pay_status]?.label || record?.pay_status || "-"}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="门店ID">{record?.store_id}</Descriptions.Item>
          <Descriptions.Item label="模板ID">{record?.template_id}</Descriptions.Item>
          <Descriptions.Item label="生产状态">
            <Tag>{record?.production_status ?? "-"}</Tag>
          </Descriptions.Item>
          <Descriptions.Item label="核销状态">
            <Tag color={record?.redeem_status === "redeemed" ? "green" : "default"}>
              {record?.redeem_status === "redeemed" ? "已核销" : record?.redeem_status ?? "-"}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="核销码">{record?.redeem_code || "-"}</Descriptions.Item>
          <Descriptions.Item label="退款状态">
            <Tag color={record?.refund_status === "refunded" ? "red" : "default"}>
              {record?.refund_status || "-"}
            </Tag>
          </Descriptions.Item>
          {record?.refund_reason && (
            <Descriptions.Item label="退款原因" span={2}>{record.refund_reason}</Descriptions.Item>
          )}
          <Descriptions.Item label="创建时间" span={2}>
            {record?.created_at ? new Date(record.created_at).toLocaleString("zh-CN") : "-"}
          </Descriptions.Item>
          {record?.paid_at && (
            <Descriptions.Item label="支付时间" span={2}>
              {new Date(record.paid_at).toLocaleString("zh-CN")}
            </Descriptions.Item>
          )}
        </Descriptions>

        <Modal
          title="退款确认"
          open={refundModalOpen}
          onOk={handleRefund}
          onCancel={() => setRefundModalOpen(false)}
          confirmLoading={loading}
        >
          <p>订单 #{record?.id}，金额 ¥{record?.amount?.toFixed(2)}</p>
          <Input.TextArea
            rows={3}
            placeholder="退款原因（可选）"
            value={refundReason}
            onChange={(e) => setRefundReason(e.target.value)}
          />
        </Modal>
      </Card>
    </Spin>
  );
};
