import React, { useState } from "react";
import { useCustomMutation } from "@refinedev/core";
import { useTable, List } from "@refinedev/antd";
import { Table, Tag, Card, Statistic, Row, Col, Modal, Form, InputNumber, Select, Input, Button, message } from "antd";
import { AlertOutlined, PlusOutlined, MinusOutlined } from "@ant-design/icons";

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
  const { tableProps, tableQuery } = useTable({ resource: "inventory" });
  const { mutateAsync: customAction } = useCustomMutation();

  const dataSource = tableProps?.dataSource || [];
  const criticalCount = dataSource.filter((r: any) => r.quantity <= r.safety_threshold * 0.5).length;
  const lowCount = dataSource.filter((r: any) => r.quantity <= r.safety_threshold && r.quantity > r.safety_threshold * 0.5).length;

  const [adjustModalVisible, setAdjustModalVisible] = useState(false);
  const [adjustItem, setAdjustItem] = useState<any>(null);
  const [adjustForm] = Form.useForm();
  const [damageModalVisible, setDamageModalVisible] = useState(false);
  const [damageItem, setDamageItem] = useState<any>(null);
  const [damageForm] = Form.useForm();

  const handleAdjust = async () => {
    const values = await adjustForm.validateFields();
    try {
      await customAction({
        url: "/api/inventory_txn",
        method: "post",
        values: {
          store_id: adjustItem.store_id,
          item_id: adjustItem.id,
          txn_type: "replenish",
          quantity: values.quantity,
          reason: values.reason || "补货",
        },
      });
      await customAction({
        url: `/api/inventory/${adjustItem.id}`,
        method: "patch",
        values: { quantity: adjustItem.quantity + values.quantity },
      });
      message.success("补货成功");
      setAdjustModalVisible(false);
      adjustForm.resetFields();
      tableQuery?.refetch();
    } catch {
      message.error("操作失败");
    }
  };

  const handleDamage = async () => {
    const values = await damageForm.validateFields();
    try {
      await customAction({
        url: "/api/inventory_txn",
        method: "post",
        values: {
          store_id: damageItem.store_id,
          item_id: damageItem.id,
          txn_type: "damage",
          quantity: values.quantity,
          reason: values.reason || "报损",
        },
      });
      await customAction({
        url: `/api/inventory/${damageItem.id}`,
        method: "patch",
        values: { quantity: Math.max(0, damageItem.quantity - values.quantity) },
      });
      message.success("报损记录成功");
      setDamageModalVisible(false);
      damageForm.resetFields();
      tableQuery?.refetch();
    } catch {
      message.error("操作失败");
    }
  };

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
        <Table.Column title="操作" width={140} render={(_, record: any) => (
          <>
            <Button type="link" size="small" icon={<PlusOutlined />} onClick={() => { setAdjustItem(record); setAdjustModalVisible(true); }}>补货</Button>
            <Button type="link" size="small" danger icon={<MinusOutlined />} onClick={() => { setDamageItem(record); setDamageModalVisible(true); }}>报损</Button>
          </>
        )} />
      </Table>

      <Modal title="补货" open={adjustModalVisible} onOk={handleAdjust} onCancel={() => setAdjustModalVisible(false)}>
        <Form form={adjustForm} layout="vertical">
          <Form.Item name="quantity" label="补货数量" rules={[{ required: true, message: "请输入数量" }]}>
            <InputNumber min={0.1} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="reason" label="补货原因">
            <Input placeholder="例如：常规补货" />
          </Form.Item>
        </Form>
      </Modal>

      <Modal title="报损" open={damageModalVisible} onOk={handleDamage} onCancel={() => setDamageModalVisible(false)}>
        <Form form={damageForm} layout="vertical">
          <Form.Item name="quantity" label="报损数量" rules={[{ required: true, message: "请输入数量" }]}>
            <InputNumber min={0.1} max={damageItem?.quantity || 999} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="reason" label="报损原因" rules={[{ required: true, message: "请输入原因" }]}>
            <Select placeholder="选择原因" options={[
              { value: "过期变质", label: "过期变质" },
              { value: "运输损坏", label: "运输损坏" },
              { value: "操作失误", label: "操作失误" },
              { value: "其他", label: "其他" },
            ]} />
          </Form.Item>
        </Form>
      </Modal>
    </List>
  );
};

export { InventoryShow } from "./show";
