import React, { useState } from "react";
import { useShow, useNotification } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag, Button, Popconfirm, Space, Row, Col, Spin } from "antd";
import { PlayCircleOutlined, ThunderboltOutlined } from "@ant-design/icons";
import { CountdownBanner } from "../../components/CountdownBanner";
import { AuditDrawer } from "../../components/AuditDrawer";

const statusColorMap: Record<string, string> = {
  draft: "default",
  registration_open: "blue",
  voting_open: "green",
  voting_closed: "orange",
  settled: "gold",
  redeeming: "purple",
  finished: "red",
};

const statusLabelMap: Record<string, string> = {
  draft: "草稿",
  registration_open: "报名开放",
  voting_open: "投票开放",
  voting_closed: "投票截止",
  settled: "已结算",
  redeeming: "核销中",
  finished: "已结束",
};

const nextStatusMap: Record<string, { status: string; label: string }> = {
  draft: { status: "registration_open", label: "开放报名" },
  registration_open: { status: "voting_open", label: "开放投票" },
  voting_open: { status: "voting_closed", label: "截止投票" },
  voting_closed: { status: "settled", label: "结算" },
  settled: { status: "redeeming", label: "开启核销" },
  redeeming: { status: "finished", label: "结束活动" },
};

export const ActivityShow: React.FC = () => {
  const { query } = useShow({ resource: "activities" });
  const record = query.data?.data;
  const isLoading = query.isLoading;
  const { open } = useNotification();
  const [auditVisible, setAuditVisible] = useState(false);

  const handleStatusTransition = async (newStatus: string) => {
    try {
      await fetch(`/api/activities/${record?.id}/status`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ new_status: newStatus }),
      });
      open?.({ type: "success", message: "状态切换成功" });
      query.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "状态切换失败", description: e.message });
    }
  };

  const handleSettle = async () => {
    try {
      const res = await fetch(`/api/activities/${record?.id}/settle`, {
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
      query.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "结算失败", description: e.message });
    }
  };

  const next = record?.status ? nextStatusMap[record.status] : null;

  return (
    <Spin spinning={isLoading}>
      <Show isLoading={isLoading}>
        <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
          {record?.voting_end_at && record.status === "voting_open" && (
            <Col span={24}>
              <CountdownBanner targetTime={record.voting_end_at} label="投票截止" />
            </Col>
          )}
          {record?.registration_end_at && record.status === "registration_open" && (
            <Col span={24}>
              <CountdownBanner targetTime={record.registration_end_at} label="报名截止" />
            </Col>
          )}
        </Row>

        <Descriptions column={2} bordered>
          <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
          <Descriptions.Item label="活动名称">{record?.name}</Descriptions.Item>
          <Descriptions.Item label="状态">
            <Tag color={statusColorMap[record?.status] || "default"}>
              {statusLabelMap[record?.status] || record?.status}
            </Tag>
          </Descriptions.Item>
          <Descriptions.Item label="赛区ID">{record?.region_id}</Descriptions.Item>
          <Descriptions.Item label="报名开始">{record?.registration_start_at}</Descriptions.Item>
          <Descriptions.Item label="报名截止">{record?.registration_end_at}</Descriptions.Item>
          <Descriptions.Item label="投票开始">{record?.voting_start_at}</Descriptions.Item>
          <Descriptions.Item label="投票截止">{record?.voting_end_at}</Descriptions.Item>
          <Descriptions.Item label="获奖人数上限">{record?.max_winner_count}</Descriptions.Item>
          <Descriptions.Item label="创建时间">{record?.created_at}</Descriptions.Item>
        </Descriptions>

        <Space style={{ marginTop: 16 }}>
          {next && (
            <Popconfirm title={`确认切换到「${next.label}」？`} onConfirm={() => handleStatusTransition(next.status)}>
              <Button type="primary" icon={<PlayCircleOutlined />}>{next.label}</Button>
            </Popconfirm>
          )}
          {record?.status === "voting_closed" && (
            <Popconfirm title="确认执行结算？将生成 Top100 获奖名单和核销码" onConfirm={handleSettle}>
              <Button type="primary" danger icon={<ThunderboltOutlined />}>执行结算</Button>
            </Popconfirm>
          )}
          <Button onClick={() => setAuditVisible(true)}>查看审计日志</Button>
        </Space>
      </Show>

      <AuditDrawer targetType="activity" targetId={Number(record?.id) || 0} visible={auditVisible} onClose={() => setAuditVisible(false)} />
    </Spin>
  );
};
