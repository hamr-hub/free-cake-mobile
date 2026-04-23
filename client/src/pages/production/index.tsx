import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification } from "@refinedev/core";
import { Table, Tag, Button, Popconfirm, Space, Modal, Form, InputNumber, DatePicker, Select, Card, Statistic, Row, Col } from "antd";
import { ScheduleOutlined, CheckCircleOutlined } from "@ant-design/icons";

const taskStatusColorMap: Record<string, string> = {
  pending: "orange",
  in_progress: "blue",
  completed: "green",
  cancelled: "red",
};

export const ProductionList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "production" });
  const { open } = useNotification();
  const [scheduleModalVisible, setScheduleModalVisible] = useState(false);
  const [selectedOrder, setSelectedOrder] = useState<any>(null);

  const handleSchedule = async (values: any) => {
    try {
      const res = await fetch(`/api/orders/${selectedOrder?.id}/schedule`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({
          store_id: values.store_id,
          scheduled_date: values.scheduled_date?.format("YYYY-MM-DDTHH:mm:ss"),
          priority: values.priority,
        }),
      });
      const data = await res.json();
      if (!res.ok) throw new Error(data.error || "排产失败");
      open?.({ type: "success", message: "排产成功", description: `批次 ${data.batch_id}，任务 ${data.task_ids.join(",")}` });
      setScheduleModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "排产失败", description: e.message });
    }
  };

  const pendingCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "pending").length || 0;
  const inProgressCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "in_progress").length || 0;
  const completedCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "completed").length || 0;

  return (
    <>
      <List>
        <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
          <Col span={8}><Card><Statistic title="待排产" value={pendingCount} valueStyle={{ color: "#faad14" }} /></Card></Col>
          <Col span={8}><Card><Statistic title="生产中" value={inProgressCount} valueStyle={{ color: "#1677ff" }} /></Card></Col>
          <Col span={8}><Card><Statistic title="已完成" value={completedCount} valueStyle={{ color: "#52c41a" }} /></Card></Col>
        </Row>

        <Table {...tableProps} rowKey="id">
          <Table.Column dataIndex="id" title="ID" width={60} />
          <Table.Column dataIndex="order_id" title="订单ID" width={80} />
          <Table.Column dataIndex="batch_id" title="批次ID" width={80} />
          <Table.Column dataIndex="store_id" title="门店ID" width={80} />
          <Table.Column dataIndex="template_id" title="模板ID" width={80} />
          <Table.Column dataIndex="task_status" title="任务状态" width={100} render={(v: string) => (
            <Tag color={taskStatusColorMap[v] || "default"}>{v || "pending"}</Tag>
          )} />
          <Table.Column dataIndex="created_at" title="创建时间" width={140} />
          <Table.Column title="操作" width={160} render={(_, record: any) => (
            <Space>
              {record.task_status === "pending" && (
                <Button type="link" size="small" icon={<ScheduleOutlined />} onClick={() => { setSelectedOrder(record); setScheduleModalVisible(true); }}>排产</Button>
              )}
              {record.task_status === "in_progress" && (
                <Popconfirm title="确认完工？" onConfirm={async () => {
                  try {
                    await fetch(`/api/production/tasks/${record.id}/complete`, {
                      method: "POST",
                      headers: { Authorization: `Bearer ${localStorage.getItem("token")}` },
                    });
                    open?.({ type: "success", message: "已标记完工" });
                    tableQuery?.refetch();
                  } catch { open?.({ type: "error", message: "操作失败" }); }
                }}>
                  <Button type="link" size="small" icon={<CheckCircleOutlined />} style={{ color: "#52c41a" }}>完工</Button>
                </Popconfirm>
              )}
            </Space>
          )} />
        </Table>
      </List>

      <Modal
        title="排产调度"
        open={scheduleModalVisible}
        onCancel={() => setScheduleModalVisible(false)}
        onOk={() => {}}
        footer={null}
      >
        <Form layout="vertical" onFinish={handleSchedule}>
          <Form.Item name="store_id" label="门店ID" rules={[{ required: true }]}>
            <InputNumber min={1} style={{ width: "100%" }} />
          </Form.Item>
          <Form.Item name="scheduled_date" label="排产日期" rules={[{ required: true }]}>
            <DatePicker showTime style={{ width: "100%" }} format="YYYY-MM-DD HH:mm" />
          </Form.Item>
          <Form.Item name="priority" label="优先级" initialValue={1}>
            <Select options={[{ label: "普通", value: 1 }, { label: "紧急", value: 2 }, { label: "最高", value: 3 }]} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" icon={<ScheduleOutlined />}>确认排产</Button>
          </Form.Item>
        </Form>
      </Modal>
    </>
  );
};
