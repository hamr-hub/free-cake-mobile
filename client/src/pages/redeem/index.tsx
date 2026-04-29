import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification, useCustomMutation } from "@refinedev/core";
import { Table, Tag, Space, Button, Popconfirm, Card, Statistic, Row, Col, Modal, Descriptions, Input, InputNumber, Typography } from "antd";
import { CheckCircleOutlined, RedoOutlined, SearchOutlined } from "@ant-design/icons";

const statusColorMap: Record<string, string> = {
  valid: "blue",
  used: "green",
  expired: "red",
};

const statusLabelMap: Record<string, string> = {
  valid: "待核销",
  used: "已核销",
  expired: "已过期",
};

export const RedeemList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "redeem" });
  const { open } = useNotification();
  const [verifyModalVisible, setVerifyModalVisible] = useState(false);
  const [verifyCode, setVerifyCode] = useState("");
  const [verifyPhone, setVerifyPhone] = useState("");
  const [verifyStoreId, setVerifyStoreId] = useState<number>(0);

  const validCount = tableProps?.dataSource?.filter?.((r: any) => r.status === "valid").length || 0;
  const usedCount = tableProps?.dataSource?.filter?.((r: any) => r.status === "used").length || 0;
  const expiredCount = tableProps?.dataSource?.filter?.((r: any) => r.status === "expired").length || 0;

  const { mutateAsync: verifyMutate } = useCustomMutation();
  const { mutateAsync: resendMutate } = useCustomMutation();

  const handleVerify = async () => {
    try {
      const result = await verifyMutate({
        url: "/api/redeem/verify",
        method: "post",
        values: { redeem_code: verifyCode, phone: verifyPhone, store_id: verifyStoreId },
      });
      const data = result?.data;
      if (data.success) {
        open?.({ type: "success", message: "核销成功", description: `订单 #${data.order_id} 已核销` });
      } else {
        open?.({ type: "error", message: "核销失败", description: data.fail_reason });
      }
      setVerifyModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "核销失败", description: e.message });
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

  return (
    <>
      <List>
        <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
          <Col span={8}><Card><Statistic title="待核销" value={validCount} valueStyle={{ color: "#1677ff" }} /></Card></Col>
          <Col span={8}><Card><Statistic title="已核销" value={usedCount} valueStyle={{ color: "#52c41a" }} /></Card></Col>
          <Col span={8}><Card><Statistic title="已过期" value={expiredCount} valueStyle={{ color: "#ff4d4f" }} /></Card></Col>
        </Row>

        <Space style={{ marginBottom: 12 }}>
          <Button type="primary" icon={<CheckCircleOutlined />} onClick={() => setVerifyModalVisible(true)}>手动核销</Button>
        </Space>

        <Table {...tableProps} rowKey="id">
          <Table.Column dataIndex="id" title="ID" width={60} />
          <Table.Column dataIndex="code" title="核销码" width={120} render={(v: string) => (
            <Typography.Text copyable style={{ fontSize: 13 }}>{v}</Typography.Text>
          )} />
          <Table.Column dataIndex="order_id" title="订单ID" width={80} />
          <Table.Column dataIndex="expires_at" title="过期时间" width={140} />
          <Table.Column dataIndex="status" title="状态" width={100} render={(v: string) => (
            <Tag color={statusColorMap[v] || "default"}>{statusLabelMap[v] || v}</Tag>
          )} />
          <Table.Column title="操作" width={120} render={(_, record: any) => (
            <Space>
              {record.status === "expired" && (
                <Popconfirm title="确认重发核销码？" onConfirm={() => handleResendCode(record.order_id)}>
                  <Button type="link" size="small" icon={<RedoOutlined />}>重发</Button>
                </Popconfirm>
              )}
            </Space>
          )} />
        </Table>
      </List>

      <Modal title="手动核销" open={verifyModalVisible} onOk={handleVerify} onCancel={() => setVerifyModalVisible(false)}>
        <Descriptions column={1} size="small" style={{ marginBottom: 16 }}>
          <Descriptions.Item label="说明">输入核销码和手机号完成核销</Descriptions.Item>
        </Descriptions>
        <div style={{ marginBottom: 12 }}>
          <Input placeholder="核销码" value={verifyCode} onChange={(e) => setVerifyCode(e.target.value)} prefix={<SearchOutlined />} />
        </div>
        <div style={{ marginBottom: 12 }}>
          <Input placeholder="手机号" value={verifyPhone} onChange={(e) => setVerifyPhone(e.target.value)} />
        </div>
        <div>
          <InputNumber placeholder="门店ID" value={verifyStoreId} onChange={(v) => setVerifyStoreId(v || 0)} style={{ width: "100%" }} />
        </div>
      </Modal>
    </>
  );
};
