import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag, Button, Modal, Row, Col, Card, Statistic } from "antd";
import { InboxOutlined } from "@ant-design/icons";

const statusColorMap: Record<string, string> = {
  active: "green",
  inactive: "red",
  maintenance: "orange",
};

const statusLabelMap: Record<string, string> = {
  active: "正常运营",
  inactive: "停业",
  maintenance: "设备维护",
};

const deviceStatusColor: Record<string, string> = {
  online: "green",
  offline: "red",
  warning: "orange",
};

export const StoreList: React.FC = () => {
  const { tableProps } = useTable({ resource: "stores" });
  const [inventoryVisible, setInventoryVisible] = useState(false);
  const [inventoryData, setInventoryData] = useState<any[]>([]);
  const [inventoryLoading, setInventoryLoading] = useState(false);
  const [selectedStore, setSelectedStore] = useState<any>(null);

  const dataSource = tableProps?.dataSource || [];
  const activeCount = dataSource.filter((r: any) => r.status === "active").length;
  const totalCapacity = dataSource.reduce((sum: number, r: any) => sum + (r.daily_capacity || 0), 0);

  const showStoreInventory = async (store: any) => {
    setSelectedStore(store);
    setInventoryVisible(true);
    setInventoryLoading(true);
    try {
      const res = await fetch(`/api/stores/${store.id}/inventory`, {
        headers: { Authorization: `Bearer ${localStorage.getItem("token")}` },
      });
      const data = await res.json();
      setInventoryData(Array.isArray(data?.data) ? data.data : Array.isArray(data) ? data : []);
    } catch {
      setInventoryData([]);
    } finally {
      setInventoryLoading(false);
    }
  };

  return (
    <>
      <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
        <Col span={6}>
          <Card><Statistic title="活跃门店" value={activeCount} valueStyle={{ color: "#52c41a" }} /></Card>
        </Col>
        <Col span={6}>
          <Card><Statistic title="总门店数" value={dataSource.length} /></Card>
        </Col>
        <Col span={6}>
          <Card><Statistic title="总日产能" value={totalCapacity} suffix="个/天" /></Card>
        </Col>
      </Row>

      <List canCreate>
        <Table {...tableProps} rowKey="id">
          <Table.Column dataIndex="id" title="ID" width={60} />
          <Table.Column dataIndex="name" title="门店名称" width={140} />
          <Table.Column dataIndex="region_id" title="赛区ID" width={80} />
          <Table.Column dataIndex="address" title="地址" ellipsis />
          <Table.Column dataIndex="daily_capacity" title="日产能" width={80} render={(v: number) => `${v || 0}个/天`} />
          <Table.Column dataIndex="contact_name" title="联系人" width={100} />
          <Table.Column dataIndex="contact_phone" title="联系电话" width={120} />
          <Table.Column dataIndex="status" title="运营状态" width={100} render={(v: string) => (
            <Tag color={statusColorMap[v] || "default"}>{statusLabelMap[v] || v}</Tag>
          )} />
          <Table.Column dataIndex="device_status" title="设备状态" width={100} render={(v: string) => (
            v ? <Tag color={deviceStatusColor[v] || "default"}>{v}</Tag> : "-"
          )} />
          <Table.Column title="操作" width={120} render={(_, record: any) => (
            <Button type="link" size="small" icon={<InboxOutlined />} onClick={() => showStoreInventory(record)}>库存</Button>
          )} />
        </Table>
      </List>

      <Modal
        title={`${selectedStore?.name || "门店"} - 库存详情`}
        open={inventoryVisible}
        onCancel={() => setInventoryVisible(false)}
        footer={null}
        width={700}
      >
        <Table
          dataSource={inventoryData}
          rowKey="id"
          loading={inventoryLoading}
          pagination={{ pageSize: 10 }}
          size="small"
        >
          <Table.Column dataIndex="name" title="原料" width={120} />
          <Table.Column dataIndex="quantity" title="库存量" width={80} render={(v: number, record: any) => {
            const isLow = v <= (record.safety_threshold || 0);
            return <span style={{ color: isLow ? "#ff4d4f" : undefined }}>{v}</span>;
          }} />
          <Table.Column dataIndex="safety_threshold" title="安全阈值" width={80} />
          <Table.Column dataIndex="unit" title="单位" width={60} />
          <Table.Column dataIndex="category" title="类别" width={100} />
          <Table.Column title="状态" width={80} render={(_, record: any) => {
            let status = "正常";
            let color = "green";
            if (record.quantity <= (record.safety_threshold || 0) * 0.5) { status = "紧急"; color = "red"; }
            else if (record.quantity <= (record.safety_threshold || 0)) { status = "偏低"; color = "orange"; }
            return <Tag color={color}>{status}</Tag>;
          }} />
        </Table>
      </Modal>
    </>
  );
};

export { StoreCreate } from "./create";
