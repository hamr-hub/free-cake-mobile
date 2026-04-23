import React from "react";
import { useCustom, useList } from "@refinedev/core";
import { Row, Col, Card, Statistic, Table, Tag, Spin } from "antd";
import {
  EnvironmentOutlined,
  PictureOutlined,
  LikeOutlined,
  WarningOutlined,
  ScheduleOutlined,
  CheckCircleOutlined,
  AlertOutlined,
} from "@ant-design/icons";

interface DashboardStats {
  active_regions: number;
  today_entries: number;
  today_votes: number;
  risk_vote_ratio: number;
  pending_production: number;
  today_redeem_rate: number;
  low_inventory_stores: number;
}

export const DashboardPage: React.FC = () => {
  const statsResult = useCustom({
    url: "/api/dashboard/stats",
    method: "get",
  });

  const statsLoading = statsResult.query.isLoading;
  const stats: DashboardStats = (statsResult.query.data?.data as any) || {
    active_regions: 0,
    today_entries: 0,
    today_votes: 0,
    risk_vote_ratio: 0,
    pending_production: 0,
    today_redeem_rate: 0,
    low_inventory_stores: 0,
  };

  const activitiesResult = useList({
    resource: "activities",
    pagination: { pageSize: 5 },
  });

  const recentActivities = activitiesResult.result?.data || [];

  const cardStyle: React.CSSProperties = {
    borderRadius: 8,
    boxShadow: "0 1px 2px rgba(0,0,0,0.06)",
  };

  const statCards = [
    { title: "活动中赛区数", value: stats.active_regions, icon: <EnvironmentOutlined style={{ fontSize: 24, color: "#1677ff" }} /> },
    { title: "今日新增参赛", value: stats.today_entries, icon: <PictureOutlined style={{ fontSize: 24, color: "#52c41a" }} /> },
    { title: "今日投票数", value: stats.today_votes, icon: <LikeOutlined style={{ fontSize: 24, color: "#722ed1" }} /> },
    { title: "风险票占比", value: stats.risk_vote_ratio, icon: <WarningOutlined style={{ fontSize: 24, color: "#faad14" }} />, precision: 2, suffix: "%" },
    { title: "待排产订单", value: stats.pending_production, icon: <ScheduleOutlined style={{ fontSize: 24, color: "#eb2f96" }} /> },
    { title: "今日核销率", value: stats.today_redeem_rate, icon: <CheckCircleOutlined style={{ fontSize: 24, color: "#13c2c2" }} />, precision: 2, suffix: "%" },
    { title: "低库存门店", value: stats.low_inventory_stores, icon: <AlertOutlined style={{ fontSize: 24, color: "#ff4d4f" }} /> },
  ];

  const statusColorMap: Record<string, string> = {
    draft: "default",
    registration_open: "blue",
    voting_open: "green",
    voting_closed: "orange",
    settled: "gold",
    redeeming: "purple",
    finished: "red",
  };

  return (
    <Spin spinning={statsLoading}>
      <Row gutter={[16, 16]}>
        {statCards.map((card, idx) => (
          <Col span={6} key={idx}>
            <Card style={cardStyle} styles={{ body: { padding: "16px 24px" } }}>
              <Statistic title={card.title} value={card.value} precision={card.precision} suffix={card.suffix} prefix={card.icon} />
            </Card>
          </Col>
        ))}
      </Row>
      <Row gutter={[16, 16]} style={{ marginTop: 16 }}>
        <Col span={24}>
          <Card title="近期活动" style={cardStyle}>
            <Table dataSource={recentActivities} rowKey="id" pagination={false} size="small">
              <Table.Column dataIndex="id" title="ID" width={60} />
              <Table.Column dataIndex="name" title="活动名称" ellipsis />
              <Table.Column dataIndex="status" title="状态" width={140} render={(v: string) => <Tag color={statusColorMap[v] || "default"}>{v}</Tag>} />
              <Table.Column dataIndex="region_id" title="赛区ID" width={80} />
              <Table.Column dataIndex="max_winner_count" title="获奖上限" width={90} />
            </Table>
          </Card>
        </Col>
      </Row>
    </Spin>
  );
};
