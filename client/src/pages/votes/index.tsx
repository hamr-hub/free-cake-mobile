import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification, useCustom, useCustomMutation } from "@refinedev/core";
import { Table, Tag, Statistic, Row, Col, Card, Button, Popconfirm, Space, InputNumber, Modal, Descriptions, Tabs, Timeline, Alert, Typography, Spin } from "antd";
import { UnlockOutlined, MinusCircleOutlined, EyeOutlined, WarningOutlined, StopOutlined, ReloadOutlined } from "@ant-design/icons";
import ReactECharts from "echarts-for-react";
import { RiskTag } from "../../components/RiskTag";
import { AuditDrawer } from "../../components/AuditDrawer";

const voteStatusColorMap: Record<string, string> = {
  valid: "green",
  frozen: "blue",
  invalid: "red",
};

const riskEventStatusColor: Record<string, string> = {
  detected: "red",
  confirmed: "volcano",
  resolved: "green",
  ignored: "default",
};

export const RiskControlPage: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "votes/risk" });
  const { open } = useNotification();
  const [auditVisible, setAuditVisible] = useState(false);
  const [auditTarget, setAuditTarget] = useState<{ type: string; id: number }>({ type: "vote", id: 0 });
  const [deductModalVisible, setDeductModalVisible] = useState(false);
  const [deductEntry, setDeductEntry] = useState<any>(null);
  const [deductCount, setDeductCount] = useState(1);
  const [selectedVotes, setSelectedVotes] = useState<number[]>([]);
  const [bulkModalVisible, setBulkModalVisible] = useState(false);
  const [bulkAction, setBulkAction] = useState<"freeze" | "unfreeze" | "invalidate">("freeze");

  const { query: riskEventsQuery } = useCustom({
    url: "/api/risk-events?limit=50",
    method: "get",
  });
  const riskEvents = (() => {
    const raw = riskEventsQuery.data?.data;
    return Array.isArray(raw?.data) ? raw.data : Array.isArray(raw) ? raw : [];
  })();
  const eventsLoading = riskEventsQuery.isLoading;

  const { mutateAsync: freezeMutate } = useCustomMutation();
  const { mutateAsync: deductMutate } = useCustomMutation();
  const { mutateAsync: bulkMutate } = useCustomMutation();

  const dataSource = tableProps?.dataSource || [];

  const handleFreeze = async (entryId: number, freeze: boolean) => {
    try {
      await freezeMutate({
        url: `/api/entries/${entryId}/freeze`,
        method: "post",
        values: { freeze },
      });
      open?.({ type: "success", message: freeze ? "作品已冻结" : "作品已解冻" });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "操作失败", description: e.message });
    }
  };

  const handleDeduct = async () => {
    if (!deductEntry) return;
    try {
      await deductMutate({
        url: `/api/entries/${deductEntry.id}/deduct`,
        method: "post",
        values: { count: deductCount, reason: "运营扣减异常票" },
      });
      open?.({ type: "success", message: `成功扣减 ${deductCount} 票` });
      setDeductModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "扣减失败", description: e.message });
    }
  };

  const handleBulkAction = async () => {
    if (selectedVotes.length === 0) return;
    try {
      const endpoint = bulkAction === "freeze" ? "freeze" : bulkAction === "unfreeze" ? "unfreeze" : "invalidate";
      const result = await bulkMutate({
        url: `/api/votes/bulk/${endpoint}`,
        method: "post",
        values: { vote_ids: selectedVotes },
      });
      const data = result?.data;
      open?.({ type: "success", message: `成功处理 ${data.affected || selectedVotes.length} 条投票` });
      setSelectedVotes([]);
      setBulkModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "批量操作失败", description: e.message });
    }
  };

  const riskPieOption = {
    tooltip: { trigger: "item" },
    legend: { bottom: 0 },
    series: [{
      type: "pie",
      radius: ["40%", "70%"],
      data: [
        { value: dataSource.filter((v: any) => v.vote_status === "valid").length, name: "正常票", itemStyle: { color: "#52c41a" } },
        { value: dataSource.filter((v: any) => v.vote_status === "frozen").length, name: "冻结票", itemStyle: { color: "#1677ff" } },
        { value: dataSource.filter((v: any) => v.vote_status === "invalid").length, name: "无效票", itemStyle: { color: "#ff4d4f" } },
      ],
    }],
  };

  const ipClusterOption = {
    tooltip: { trigger: "axis" },
    xAxis: {
      type: "category",
      data: (() => {
        const ipMap: Record<string, number> = {};
        dataSource.forEach((v: any) => {
          if (v.ip) {
            const prefix = v.ip.split(".").slice(0, 3).join(".");
            ipMap[prefix] = (ipMap[prefix] || 0) + 1;
          }
        });
        return Object.keys(ipMap).sort((a, b) => ipMap[b] - ipMap[a]).slice(0, 10);
      })(),
    },
    yAxis: { type: "value", name: "投票数" },
    series: [{
      type: "bar",
      data: (() => {
        const ipMap: Record<string, number> = {};
        dataSource.forEach((v: any) => {
          if (v.ip) {
            const prefix = v.ip.split(".").slice(0, 3).join(".");
            ipMap[prefix] = (ipMap[prefix] || 0) + 1;
          }
        });
        const keys = Object.keys(ipMap).sort((a, b) => ipMap[b] - ipMap[a]).slice(0, 10);
        return keys.map((k) => ipMap[k]);
      })(),
      itemStyle: { color: "#ff7875" },
    }],
  };

  const tabItems = [
    {
      key: "overview",
      label: "风控总览",
      children: (
        <>
          <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
            <Col span={6}>
              <Card><Statistic title="总投票数" value={dataSource.length} /></Card>
            </Col>
            <Col span={6}>
              <Card><Statistic title="冻结票数" value={dataSource.filter((v: any) => v.vote_status === "frozen").length} valueStyle={{ color: "#1677ff" }} /></Card>
            </Col>
            <Col span={6}>
              <Card><Statistic title="无效票数" value={dataSource.filter((v: any) => v.vote_status === "invalid").length} valueStyle={{ color: "#ff4d4f" }} /></Card>
            </Col>
            <Col span={6}>
              <Card>
                <Statistic title="风险占比" value={
                  dataSource.length ? (
                    (dataSource.filter((v: any) => v.vote_status === "frozen" || v.vote_status === "invalid").length / dataSource.length * 100)
                  ) : 0
                } precision={2} suffix="%" valueStyle={{ color: dataSource.length > 0 && dataSource.filter((v: any) => v.vote_status === "frozen" || v.vote_status === "invalid").length / dataSource.length > 0.1 ? "#ff4d4f" : "#52c41a" }} />
              </Card>
            </Col>
          </Row>

          <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
            <Col span={12}>
              <Card title="风险分布" size="small">
                <ReactECharts option={riskPieOption} style={{ height: 280 }} />
              </Card>
            </Col>
            <Col span={12}>
              <Card title="IP聚类分析 (Top10)" size="small">
                <ReactECharts option={ipClusterOption} style={{ height: 280 }} />
              </Card>
            </Col>
          </Row>

          <Space style={{ marginBottom: 16 }}>
            <Button
              icon={<StopOutlined />}
              disabled={selectedVotes.length === 0}
              onClick={() => { setBulkAction("freeze"); setBulkModalVisible(true); }}
            >
              批量冻结 ({selectedVotes.length})
            </Button>
            <Button
              icon={<UnlockOutlined />}
              disabled={selectedVotes.length === 0}
              style={{ color: "#52c41a" }}
              onClick={() => { setBulkAction("unfreeze"); setBulkModalVisible(true); }}
            >
              批量解冻 ({selectedVotes.length})
            </Button>
            <Button
              icon={<MinusCircleOutlined />}
              disabled={selectedVotes.length === 0}
              danger
              onClick={() => { setBulkAction("invalidate"); setBulkModalVisible(true); }}
            >
              批量作废 ({selectedVotes.length})
            </Button>
            <Button icon={<ReloadOutlined />} onClick={() => tableQuery?.refetch()}>刷新数据</Button>
          </Space>

          <Table
            {...tableProps}
            rowKey="id"
            rowSelection={{
              selectedRowKeys: selectedVotes,
              onChange: (keys) => setSelectedVotes(keys as number[]),
            }}
          >
            <Table.Column dataIndex="id" title="投票ID" width={80} />
            <Table.Column dataIndex="voter_user_id" title="投票者ID" width={80} />
            <Table.Column dataIndex="entry_id" title="作品ID" width={80} />
            <Table.Column dataIndex="ip" title="IP地址" width={120} />
            <Table.Column dataIndex="vote_status" title="票据状态" width={100} render={(v: string) => (
              <Tag color={voteStatusColorMap[v] || "default"}>{v}</Tag>
            )} />
            <Table.Column dataIndex="risk_tags" title="风险标签" width={200} render={(v: any) => {
              if (!v) return "-";
              const tags = typeof v === "string" ? JSON.parse(v) : v;
              return Array.isArray(tags) ? (
                <Space>{tags.map((t: string, i: number) => <RiskTag key={i} level={t.includes("cluster") ? "high" : "medium"} reason={t} />)}</Space>
              ) : "-";
            }} />
            <Table.Column dataIndex="created_at" title="投票时间" width={140} />
            <Table.Column title="操作" width={180} render={(_, record: any) => (
              <Space>
                {record.vote_status === "frozen" && (
                  <Popconfirm title="确认解冻此票？" onConfirm={() => handleFreeze(record.entry_id, false)}>
                    <Button type="link" size="small" icon={<UnlockOutlined />} style={{ color: "#52c41a" }}>解冻</Button>
                  </Popconfirm>
                )}
                <Button type="link" size="small" icon={<MinusCircleOutlined />} danger onClick={() => { setDeductEntry(record); setDeductModalVisible(true); }}>扣票</Button>
                <Button type="link" size="small" icon={<EyeOutlined />} onClick={() => { setAuditTarget({ type: "vote", id: record.id }); setAuditVisible(true); }}>审计</Button>
              </Space>
            )} />
          </Table>
        </>
      ),
    },
    {
      key: "events",
      label: `异常事件 (${riskEvents.length})`,
      children: (
        <Spin spinning={eventsLoading}>
          {riskEvents.length === 0 ? (
            <Card>
              <Alert type="info" message="暂无异常事件记录" showIcon />
            </Card>
          ) : (
            <Timeline
              items={riskEvents.slice(0, 30).map((event: any) => ({
                color: riskEventStatusColor[event.status] || "blue",
                children: (
                  <Card size="small" style={{ marginBottom: 8 }}>
                    <Space direction="vertical" size={4}>
                      <Space>
                        <Tag color={riskEventStatusColor[event.status] || "default"}>
                          {event.status === "detected" ? "新检测" : event.status === "confirmed" ? "已确认" : event.status === "resolved" ? "已解决" : "已忽略"}
                        </Tag>
                        <Tag color="red"><WarningOutlined /> {event.risk_type || event.event_type}</Tag>
                        <Typography.Text type="secondary">{event.created_at || event.detected_at}</Typography.Text>
                      </Space>
                      <Typography.Text>
                        作品 #{event.entry_id} - {event.description || event.reason || "异常行为检测"}
                      </Typography.Text>
                      {event.related_user_ids && (
                        <Typography.Text type="secondary">
                          关联用户: {Array.isArray(event.related_user_ids) ? event.related_user_ids.join(", ") : event.related_user_ids}
                        </Typography.Text>
                      )}
                      {event.ip_list && (
                        <Typography.Text type="secondary">
                          关联IP: {Array.isArray(event.ip_list) ? event.ip_list.join(", ") : event.ip_list}
                        </Typography.Text>
                      )}
                    </Space>
                  </Card>
                ),
              }))}
            />
          )}
        </Spin>
      ),
    },
  ];

  return (
    <>
      <List>
        <Tabs items={tabItems} defaultActiveKey="overview" />
      </List>

      <Modal
        title={`扣减异常票 - 作品 #${deductEntry?.entry_id}`}
        open={deductModalVisible}
        onOk={handleDeduct}
        onCancel={() => setDeductModalVisible(false)}
      >
        <Descriptions column={1} size="small">
          <Descriptions.Item label="作品ID">{deductEntry?.entry_id}</Descriptions.Item>
          <Descriptions.Item label="当前得票数">{deductEntry?.vote_count}</Descriptions.Item>
        </Descriptions>
        <div style={{ marginTop: 16 }}>
          <span>扣减票数：</span>
          <InputNumber min={1} max={deductEntry?.vote_count || 10} value={deductCount} onChange={(v) => setDeductCount(v || 1)} />
        </div>
      </Modal>

      <Modal
        title={`批量${bulkAction === "freeze" ? "冻结" : bulkAction === "unfreeze" ? "解冻" : "作废"} - ${selectedVotes.length} 条投票`}
        open={bulkModalVisible}
        onOk={handleBulkAction}
        onCancel={() => setBulkModalVisible(false)}
      >
        <Alert
          type={bulkAction === "invalidate" ? "error" : "warning"}
          message={`确认要对 ${selectedVotes.length} 条投票执行${bulkAction === "freeze" ? "冻结" : bulkAction === "unfreeze" ? "解冻" : "作废"}操作？`}
          showIcon
        />
      </Modal>

      <AuditDrawer targetType={auditTarget.type} targetId={auditTarget.id} visible={auditVisible} onClose={() => setAuditVisible(false)} />
    </>
  );
};
