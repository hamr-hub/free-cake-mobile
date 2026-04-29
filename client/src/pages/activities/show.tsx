import React, { useState } from "react";
import { useShow, useNotification, useCustom, useCustomMutation } from "@refinedev/core";
import { Show } from "@refinedev/antd";
import { Descriptions, Tag, Button, Popconfirm, Space, Row, Col, Spin, Tabs, Table, Card, Statistic } from "antd";
import { PlayCircleOutlined, ThunderboltOutlined } from "@ant-design/icons";
import { CountdownBanner } from "../../components/CountdownBanner";
import { AuditDrawer } from "../../components/AuditDrawer";
import { EntryCard } from "../../components/EntryCard";

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

const entryStatusColor: Record<string, string> = {
  pending: "orange",
  approved: "green",
  rejected: "red",
  active: "green",
  frozen: "blue",
};

export const ActivityShow: React.FC = () => {
  const { query } = useShow({ resource: "activities" });
  const record = query.data?.data;
  const isLoading = query.isLoading;
  const { open } = useNotification();
  const { mutateAsync: postAction } = useCustomMutation();
  const [auditVisible, setAuditVisible] = useState(false);

  const { query: entriesQuery } = useCustom({
    url: `/api/entries?activity_id=${record?.id}&limit=100`,
    method: "get",
  });

  const { query: voteStatsQuery } = useCustom({
    url: `/api/activities/${record?.id}/votes/stats`,
    method: "get",
  });

  const { query: rulesQuery } = useCustom({
    url: `/api/activities/${record?.id}/rules`,
    method: "get",
    config: { query: { enabled: !!record?.id } },
  });
  const rules = (rulesQuery.data as any)?.data ?? null;

  const { query: rankQuery } = useCustom({
    url: `/api/activities/${record?.id}/rank`,
    method: "get",
  });

  const rawEntries = entriesQuery.data?.data;
  const displayEntries: any[] = Array.isArray(rawEntries?.data) ? rawEntries.data
    : Array.isArray(rawEntries?.list) ? rawEntries.list
    : Array.isArray(rawEntries) ? rawEntries : [];

  const voteStats = voteStatsQuery.data?.data;
  const rawRank = rankQuery.data?.data;
  const rankList: any[] = Array.isArray(rawRank?.data) ? rawRank.data
    : Array.isArray(rawRank) ? rawRank : [];

  const handleStatusTransition = async (newStatus: string) => {
    try {
      await postAction({
        url: `/api/activities/${record?.id}/status`,
        method: "post",
        values: { new_status: newStatus },
      });
      open?.({ type: "success", message: "状态切换成功" });
      query.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "状态切换失败", description: e.message });
    }
  };

  const handleSettle = async () => {
    try {
      const result = await postAction({
        url: `/api/activities/${record?.id}/settle`,
        method: "post",
        values: { force: false },
      });
      const data = result?.data;
      open?.({ type: "success", message: `结算完成：${data.winner_count} 名获奖者，${data.order_count} 个订单` });
      query.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "结算失败", description: e.message });
    }
  };

  const handleEntryStatus = async (entryId: number, newStatus: string) => {
    try {
      await postAction({
        url: `/api/entries/${entryId}/status`,
        method: "post",
        values: { new_status: newStatus },
      });
      open?.({ type: "success", message: "作品状态更新成功" });
    } catch (e: any) {
      open?.({ type: "error", message: "操作失败", description: e.message });
    }
  };

  const next = record?.status ? nextStatusMap[record.status] : null;

  const tabItems = [
    {
      key: "info",
      label: "活动信息",
      children: (
        <>
          <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
            {record?.voting_end_at && record.status === "voting_open" && (
              <Col span={24}><CountdownBanner targetTime={record.voting_end_at} label="投票截止" /></Col>
            )}
            {record?.registration_end_at && record.status === "registration_open" && (
              <Col span={24}><CountdownBanner targetTime={record.registration_end_at} label="报名截止" /></Col>
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
        </>
      ),
    },
    {
      key: "entries",
      label: `作品列表 (${displayEntries.length})`,
      children: (
        <Spin spinning={entriesQuery.isLoading}>
          <Row gutter={[12, 12]}>
            {displayEntries.map((entry: any) => (
              <Col key={entry.id} xs={24} sm={12} md={8} lg={6}>
                <EntryCard
                  id={entry.id}
                  imageUrl={entry.image_url || entry.photo_url || ""}
                  title={entry.title || entry.name}
                  userName={entry.user_name || entry.submitter_name}
                  voteCount={entry.vote_count}
                  rank={entry.rank}
                  status={entry.status}
                  aiGenerated={entry.ai_generated || false}
                  onClick={(id) => {
                    if (entry.status === "pending") handleEntryStatus(id, "approved");
                  }}
                />
              </Col>
            ))}
            {displayEntries.length === 0 && !entriesQuery.isLoading && (
              <Col span={24} style={{ textAlign: "center", padding: 40, color: "#999" }}>暂无参赛作品</Col>
            )}
          </Row>
        </Spin>
      ),
    },
    {
      key: "votes",
      label: "投票数据",
      children: (
        <Spin spinning={voteStatsQuery.isLoading || rankQuery.isLoading}>
          {voteStats && (
            <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
              <Col span={6}><Card><Statistic title="总投票数" value={voteStats.total_votes || 0} /></Card></Col>
              <Col span={6}><Card><Statistic title="有效票数" value={voteStats.valid_votes || 0} /></Card></Col>
              <Col span={6}><Card><Statistic title="参与人数" value={voteStats.unique_voters || 0} /></Card></Col>
              <Col span={6}><Card><Statistic title="风险票占比" value={voteStats.risk_ratio || 0} precision={2} suffix="%" /></Card></Col>
            </Row>
          )}
          <Table dataSource={rankList} rowKey="id" loading={rankQuery.isLoading} pagination={{ pageSize: 20 }} size="small">
            <Table.Column dataIndex="rank" title="排名" width={60} render={(v: number) => v <= 3 ? <Tag color="gold">{v}</Tag> : v} />
            <Table.Column dataIndex="entry_id" title="作品ID" width={80} />
            <Table.Column dataIndex="title" title="作品名称" ellipsis />
            <Table.Column dataIndex="vote_count" title="得票数" width={100} sorter={(a: any, b: any) => a.vote_count - b.vote_count} />
            <Table.Column dataIndex="user_name" title="作者" width={120} />
            <Table.Column dataIndex="status" title="状态" width={80} render={(v: string) => <Tag color={entryStatusColor[v] || "default"}>{v}</Tag>} />
          </Table>
        </Spin>
      ),
    },
    {
      key: "rules",
      label: "活动规则",
      children: (
        <Spin spinning={rulesQuery.isLoading}>
          {rules ? (
            <Descriptions column={2} bordered>
              <Descriptions.Item label="每人每天投票上限">{rules.max_votes_per_day ?? "-"}</Descriptions.Item>
              <Descriptions.Item label="蛋糕尺寸">{rules.cake_size ?? "-"}</Descriptions.Item>
              <Descriptions.Item label="奶油类型">{rules.cream_type ?? "-"}</Descriptions.Item>
              <Descriptions.Item label="AI生成速率限制">{rules.decoration_params ? JSON.parse(rules.decoration_params).ai_generation_rate_limit ?? "-" : "-"}</Descriptions.Item>
            </Descriptions>
          ) : (
            <div style={{ textAlign: "center", padding: 40, color: "#999" }}>暂无规则配置</div>
          )}
        </Spin>
      ),
    },
  ];

  return (
    <Spin spinning={isLoading}>
      <Show isLoading={isLoading}>
        <Tabs items={tabItems} defaultActiveKey="info" />
      </Show>
      <AuditDrawer targetType="activity" targetId={Number(record?.id) || 0} visible={auditVisible} onClose={() => setAuditVisible(false)} />
    </Spin>
  );
};
