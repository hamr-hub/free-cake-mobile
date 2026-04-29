import React from "react";
import { useCustom, useCustomMutation } from "@refinedev/core";
import { Table, Card, Button, Tag, InputNumber, Select, Form, Modal, message } from "antd";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";

export const PriceList: React.FC = () => {
  const [modalOpen, setModalOpen] = React.useState(false);
  const [editRecord, setEditRecord] = React.useState<any>(null);
  const [form] = Form.useForm();

  const { query } = useCustom({
    url: "/prices",
    method: "get",
  });

  const { mutateAsync: mutate } = useCustomMutation();

  const prices = (query.data as any)?.data?.list ?? [];
  const total = (query.data as any)?.data?.total ?? 0;

  const handleSubmit = async (values: any) => {
    try {
      if (editRecord) {
        await mutate({ url: `/prices/${editRecord.id}`, method: "put", values });
        message.success("价格配置已更新");
      } else {
        await mutate({ url: "/prices", method: "post", values });
        message.success("价格配置已创建");
      }
      setModalOpen(false);
      setEditRecord(null);
      form.resetFields();
      query.refetch();
    } catch {
      message.error("操作失败");
    }
  };

  const columns = [
    { title: "ID", dataIndex: "id", key: "id", width: 60 },
    { title: "赛区ID", dataIndex: "region_id", key: "region_id", width: 80 },
    { title: "蛋糕尺寸", dataIndex: "cake_size", key: "cake_size" },
    { title: "奶油类型", dataIndex: "cream_type", key: "cream_type" },
    { title: "价格 (元)", dataIndex: "price", key: "price", render: (v: number) => `¥${v.toFixed(2)}` },
    {
      title: "状态",
      dataIndex: "status",
      key: "status",
      render: (v: string) => <Tag color={v === "active" ? "green" : "red"}>{v === "active" ? "启用" : "停用"}</Tag>,
    },
    {
      title: "操作",
      key: "actions",
      width: 80,
      render: (_: any, record: any) => (
        <Button type="link" size="small" icon={<EditOutlined />} onClick={() => { setEditRecord(record); form.setFieldsValue(record); setModalOpen(true); }}>编辑</Button>
      ),
    },
  ];

  return (
    <Card title="价格配置" extra={<Button type="primary" icon={<PlusOutlined />} onClick={() => { setEditRecord(null); form.resetFields(); setModalOpen(true); }}>新增价格</Button>}>
      <Table rowKey="id" dataSource={prices} columns={columns} loading={query.isLoading} pagination={{ total, pageSize: 20 }} />

      <Modal
        title={editRecord ? "编辑价格配置" : "新增价格配置"}
        open={modalOpen}
        onCancel={() => { setModalOpen(false); setEditRecord(null); }}
        onOk={() => form.submit()}
      >
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="region_id" label="赛区ID" rules={[{ required: true }]}>
            <InputNumber style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="cake_size" label="蛋糕尺寸" rules={[{ required: true }]}>
            <Select options={[{ value: "6inch", label: "6 寸" }, { value: "8inch", label: "8 寸" }, { value: "10inch", label: "10 寸" }]} />
          </Form.Item>
          <Form.Item name="cream_type" label="奶油类型" rules={[{ required: true }]}>
            <Select options={[{ value: "animal", label: "动物奶油" }, { value: "vegetable", label: "植物奶油" }, { value: "mixed", label: "混合奶油" }]} />
          </Form.Item>
          <Form.Item name="price" label="价格 (元)" rules={[{ required: true }]}>
            <InputNumber min={0} step={0.5} style={{ width: "100%" }} />
          </Form.Item>
          {editRecord && (
            <Form.Item name="status" label="状态">
              <Select options={[{ value: "active", label: "启用" }, { value: "inactive", label: "停用" }]} />
            </Form.Item>
          )}
        </Form>
      </Modal>
    </Card>
  );
};
