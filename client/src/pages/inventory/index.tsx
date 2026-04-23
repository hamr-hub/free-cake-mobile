import React from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag, Card, Statistic, Row, Col } from "antd";
import { AlertOutlined } from "@ant-design/icons";

const statusColorMap: Record<string, string> = {
  normal: "green",
  low: "orange",
  critical: "red",
};

const statusLabelMap: Record<string, string> = {
  normal: "正常",
  low: "偏低",
  critical: "紧急",
};

export const InventoryList: React.FC = () => {
  const { tableProps } = useTable({ resource: "inventory" });

  const criticalCount = tableProps?.dataSource?.filter?.((r: any) => r.quantity <= r.safety_threshold * 0.5).length || 0;
  const lowCount = tableProps?.dataSource?.filter?.((r: any) => r.quantity <= r.safety_threshold && r.quantity > r.safety_threshold * 0.5).length || 0;

  return (
    <List>
      <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
        <Col span={8}>
          <Card>
            <Statistic title="紧急库存" value={criticalCount} valueStyle={{ color: "#ff4d4f" }} prefix={<AlertOutlined />} />
          </Card>
        </Col>
        <Col span={8}>
          <Card>
            <Statistic title="偏低库存" value={lowCount} valueStyle={{ color: "#faad14" }} />
          </Card>
        </Col>
      </Row>

      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="name" title="原料名称" width={120} />
        <Table.Column dataIndex="category" title="类别" width={100} />
        <Table.Column dataIndex="store_id" title="门店ID" width={80} />
        <Table.Column dataIndex="quantity" title="库存量" width={80} render={(v: number, record: any) => {
          const isLow = v <= record.safety_threshold;
          return <span style={{ color: isLow ? "#ff4d4f" : undefined, fontWeight: isLow ? 600 : 400 }}>{v}</span>;
        }} />
        <Table.Column dataIndex="safety_threshold" title="安全阈值" width={80} />
        <Table.Column dataIndex="unit" title="单位" width={60} />
        <Table.Column title="状态" width={80} render={(_, record: any) => {
          let status = "normal";
          if (record.quantity <= (record.safety_threshold || 0) * 0.5) status = "critical";
          else if (record.quantity <= (record.safety_threshold || 0)) status = "low";
          return <Tag color={statusColorMap[status]}>{statusLabelMap[status]}</Tag>;
        }} />
      </Table>
    </List>
  );
};
