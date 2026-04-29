import React from "react";
import { useCustom } from "@refinedev/core";
import { Table, Card, Tag, Button, Modal, Form, Input, Select, message } from "antd";
import { PlusOutlined, EditOutlined } from "@ant-design/icons";
import { useCustomMutation } from "@refinedev/core";

export const TemplateList: React.FC = () => {
  const [modalOpen, setModalOpen] = React.useState(false);
  const [editRecord, setEditRecord] = React.useState<any>(null);
  const [form] = Form.useForm();

  const { query } = useCustom({ url: "/templates", method: "get" });
  const { mutateAsync: mutate } = useCustomMutation();

  const templates = (query.data as any)?.data?.list ?? [];
  const total = (query.data as any)?.data?.total ?? 0;

  const handleSubmit = async (values: any) => {
    try {
      if (editRecord) {
        await mutate({ url: `/templates/${editRecord.id}`, method: "put", values });
        message.success("模板已更新");
      } else {
        await mutate({ url: "/templates", method: "post", values });
        message.success("模板已创建");
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
    { title: "名称", dataIndex: "name", key: "name" },
    { title: "装饰参数", dataIndex: "decoration_params", key: "decoration_params", ellipsis: true },
    { title: "尺寸", dataIndex: "cake_size", key: "cake_size" },
    { title: "奶油类型", dataIndex: "cream_type", key: "cream_type" },
    { title: "图片URL", dataIndex: "image_url", key: "image_url", ellipsis: true },
    { title: "可生产等级", dataIndex: "producible_level", key: "producible_level" },
    {
      title: "状态",
      dataIndex: "status",
      key: "status",
      render: (v: string) => <Tag color={v === "active" ? "green" : "red"}>{v === "active" ? "启用" : "停用"}</Tag>,
    },
    {
      title: "操作",
      key: "actions",
      width: 100,
      render: (_: any, record: any) => (
        <Button type="link" size="small" icon={<EditOutlined />} onClick={() => { setEditRecord(record); form.setFieldsValue(record); setModalOpen(true); }}>编辑</Button>
      ),
    },
  ];

  return (
    <Card title="设计模板管理" extra={<Button type="primary" icon={<PlusOutlined />} onClick={() => { setEditRecord(null); form.resetFields(); setModalOpen(true); }}>新增模板</Button>}>
      <Table rowKey="id" dataSource={templates} columns={columns} loading={query.isLoading} pagination={{ total, pageSize: 20 }} />

      <Modal
        title={editRecord ? "编辑模板" : "新增模板"}
        open={modalOpen}
        onCancel={() => { setModalOpen(false); setEditRecord(null); }}
        onOk={() => form.submit()}
      >
        <Form form={form} layout="vertical" onFinish={handleSubmit}>
          <Form.Item name="name" label="名称" rules={[{ required: true }]}><Input /></Form.Item>
          <Form.Item name="image_url" label="图片URL" rules={[{ required: true }]}><Input /></Form.Item>
          <Form.Item name="decoration_params" label="装饰参数 (JSON)"><Input placeholder='{"scene":"birthday","theme":"cute"}' /></Form.Item>
          <Form.Item name="cake_size" label="蛋糕尺寸">
            <Select options={[{ value: "6inch", label: "6 寸" }, { value: "8inch", label: "8 寸" }, { value: "10inch", label: "10 寸" }]} />
          </Form.Item>
          <Form.Item name="cream_type" label="奶油类型">
            <Select options={[{ value: "animal", label: "动物奶油" }, { value: "vegetable", label: "植物奶油" }, { value: "mixed", label: "混合奶油" }]} />
          </Form.Item>
          <Form.Item name="producible_level" label="可生产等级">
            <Select options={[{ value: "high", label: "高" }, { value: "medium", label: "中" }, { value: "low", label: "低" }]} />
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
