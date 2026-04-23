import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification } from "@refinedev/core";
import { Table, Tag, Statistic, Row, Col, Card, Button, Popconfirm, Space, InputNumber, Modal, Descriptions } from "antd";
import { UnlockOutlined, MinusCircleOutlined, EyeOutlined } from "@ant-design/icons";
import { RiskTag } from "../../components/RiskTag";
import { AuditDrawer } from "../../components/AuditDrawer";

const voteStatusColorMap: Record<string, string> = {
  valid: "green",
  frozen: "blue",
  invalid: "red",
};

export const RiskControlPage: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "votes/risk" });
  const { open } = useNotification();
  const [auditVisible, setAuditVisible] = useState(false);
  const [auditTarget, setAuditTarget] = useState<{ type: string; id: number }>({ type: "vote", id: 0 });
  const [deductModalVisible, setDeductModalVisible] = useState(false);
  const [deductEntry, setDeductEntry] = useState<any>(null);
  const [deductCount, setDeductCount] = useState(1);

  const handleFreeze = async (entryId: number, freeze: boolean) => {
    try {
      const res = await fetch(`/api/entries/${entryId}/freeze`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ freeze }),
      });
      if (!res.ok) throw new Error("操作失败");
      open?.({ type: "success", message: freeze ? "作品已冻结" : "作品已解冻" });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "操作失败", description: e.message });
    }
  };

  const handleDeduct = async () => {
    if (!deductEntry) return;
    try {
      const res = await fetch(`/api/entries/${deductEntry.id}/deduct`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ count: deductCount, reason: "运营扣减异常票" }),
      });
      if (!res.ok) throw new Error("扣减失败");
      open?.({ type: "success", message: `成功扣减 ${deductCount} 票` });
      setDeductModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "扣减失败", description: e.message });
    }
  };

  return (
    <>
      <List>
        <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
          <Col span={6}>
            <Card><Statistic title="总投票数" value={tableProps?.dataSource?.length || 0} /></Card>
          </Col>
          <Col span={6}>
            <Card><Statistic title="风险票数" value={tableProps?.dataSource?.filter?.((v: any) => v.vote_status === "frozen").length || 0} /></Card>
          </Col>
          <Col span={6}>
            <Card><Statistic title="已拦截票数" value={tableProps?.dataSource?.filter?.((v: any) => v.vote_status === "invalid").length || 0} /></Card>
          </Col>
          <Col span={6}>
            <Card>
              <Statistic title="风险占比" value={
                tableProps?.dataSource?.length ? (
                  (tableProps?.dataSource?.filter?.((v: any) => v.vote_status === "frozen" || v.vote_status === "invalid").length / tableProps?.dataSource?.length * 100)
                ) : 0
              } precision={2} suffix="%" />
            </Card>
          </Col>
        </Row>

        <Table {...tableProps} rowKey="id">
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

      <AuditDrawer targetType={auditTarget.type} targetId={auditTarget.id} visible={auditVisible} onClose={() => setAuditVisible(false)} />
    </>
  );
};
