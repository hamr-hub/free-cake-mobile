import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification } from "@refinedev/core";
import { Table, Tag, Typography, Button, Popconfirm, Space, Descriptions, Modal, Statistic, Row, Col, Card } from "antd";
import { ThunderboltOutlined, EyeOutlined, SendOutlined } from "@ant-design/icons";

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

export const SettlementList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "settlement" });
  const { open } = useNotification();
  const [detailVisible, setDetailVisible] = useState(false);
  const [detailRecord, setDetailRecord] = useState<any>(null);
  const [detailData, setDetailData] = useState<any>(null);

  const dataSource = tableProps?.dataSource || [];
  const pendingCount = dataSource.filter((r: any) => r.status === "pending").length;
  const completedCount = dataSource.filter((r: any) => r.production_status === "completed").length;
  const redeemedCount = dataSource.filter((r: any) => r.redeem_status === "redeemed").length;

  const handleSettle = async (activityId: number) => {
    try {
      const res = await fetch(`/api/activities/${activityId}/settle`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ force: false }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "结算失败");
      open?.({ type: "success", message: `结算完成：${data.winner_count} 名获奖者，${data.order_count} 个订单` });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "结算失败", description: e.message });
    }
  };

  const handleResendCode = async (orderId: number) => {
    try {
      const res = await fetch(`/api/orders/${orderId}/resend-code`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
      });
      if (!res.ok) throw new Error("重发失败");
      open?.({ type: "success", message: "核销码已重发" });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "重发失败", description: e.message });
    }
  };

  const showDetail = async (record: any) => {
    setDetailRecord(record);
    setDetailVisible(true);
    setDetailVisible(true);
    try {
      const res = await fetch(`/api/orders/${record.id}`, {
        headers: { Authorization: `Bearer ${localStorage.getItem("token")}` },
      });
      const data = await res.json();
      setDetailData(data?.data || data);
    } catch {
      setDetailData(null);
    }
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
          <Card><Statistic title="总订单数" value={dataSource.length} /></Card>
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
          <Table.Column title="操作" width={220} render={(_, record: any) => (
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
        </Descriptions>
      </Modal>
    </>
  );
};
