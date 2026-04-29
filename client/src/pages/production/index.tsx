import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification, useCustomMutation } from "@refinedev/core";
import { Table, Tag, Button, Popconfirm, Space, Modal, Form, InputNumber, DatePicker, Select, Card, Statistic, Row, Col, Input } from "antd";
import { ScheduleOutlined, CheckCircleOutlined, PauseCircleOutlined, WarningOutlined, StopOutlined } from "@ant-design/icons";

const taskStatusColorMap: Record<string, string> = {
  pending: "orange",
  in_progress: "blue",
  completed: "green",
  paused: "cyan",
  error: "red",
  cancelled: "default",
};

const taskStatusLabel: Record<string, string> = {
  pending: "待排产",
  in_progress: "生产中",
  completed: "已完成",
  paused: "已暂停",
  error: "异常",
  cancelled: "已取消",
};

export const ProductionList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "production" });
  const { open } = useNotification();
  const [scheduleModalVisible, setScheduleModalVisible] = useState(false);
  const [selectedOrder, setSelectedOrder] = useState<any>(null);

  const { mutateAsync: scheduleMutate } = useCustomMutation();
  const { mutateAsync: completeMutate } = useCustomMutation();
  const { mutateAsync: actionMutate } = useCustomMutation();

  const [errorModalVisible, setErrorModalVisible] = useState(false);
  const [errorTaskId, setErrorTaskId] = useState<number | null>(null);
  const [errorForm] = Form.useForm();

  const handleSchedule = async (values: any) => {
    try {
      const result = await scheduleMutate({
        url: `/api/orders/${selectedOrder?.id}/schedule`,
        method: "post",
        values: {
          store_id: values.store_id,
          scheduled_date: values.scheduled_date?.format("YYYY-MM-DDTHH:mm:ss"),
          priority: values.priority,
        },
      });
      const data = result?.data;
      open?.({ type: "success", message: "排产成功", description: `批次 ${data.batch_id}，任务 ${data.task_ids.join(",")}` });
      setScheduleModalVisible(false);
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "排产失败", description: e.message });
    }
  };

  const pendingCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "pending").length || 0;
  const inProgressCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "in_progress").length || 0;
  const pausedCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "paused").length || 0;
  const errorCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "error").length || 0;
  const completedCount = tableProps?.dataSource?.filter?.((r: any) => r.task_status === "completed").length || 0;

  return (
    <>
      <List>
        <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
          <Col span={6}><Card><Statistic title="待排产" value={pendingCount} valueStyle={{ color: "#faad14" }} /></Card></Col>
          <Col span={6}><Card><Statistic title="生产中" value={inProgressCount} valueStyle={{ color: "#1677ff" }} /></Card></Col>
          <Col span={6}><Card><Statistic title="已暂停" value={pausedCount} valueStyle={{ color: "#13c2c2" }} /></Card></Col>
          <Col span={6}><Card><Statistic title="已完成" value={completedCount} valueStyle={{ color: "#52c41a" }} /></Card></Col>
        </Row>
        {errorCount > 0 && (
          <Row style={{ marginBottom: 16 }}>
            <Col span={24}><Card size="small" style={{ borderColor: "#ff4d4f" }}><Statistic title="异常任务" value={errorCount} valueStyle={{ color: "#ff4d4f" }} prefix={<WarningOutlined />} /></Card></Col>
          </Row>
        )}

        <Table {...tableProps} rowKey="id">
          <Table.Column dataIndex="id" title="ID" width={60} />
          <Table.Column dataIndex="order_id" title="订单ID" width={80} />
          <Table.Column dataIndex="batch_id" title="批次ID" width={80} />
          <Table.Column dataIndex="store_id" title="门店ID" width={80} />
          <Table.Column dataIndex="template_id" title="模板ID" width={80} />
          <Table.Column dataIndex="task_status" title="任务状态" width={100} render={(v: string) => (
            <Tag color={taskStatusColorMap[v] || "default"}>{taskStatusLabel[v] || v}</Tag>
          )} />
          <Table.Column dataIndex="error_description" title="异常描述" width={200} ellipsis render={(v: string) => v || "-"} />
          <Table.Column dataIndex="created_at" title="创建时间" width={140} />
          <Table.Column title="操作" width={220} render={(_, record: any) => (
            <Space size="small" wrap>
              {record.task_status === "pending" && (
                <Button type="link" size="small" icon={<ScheduleOutlined />} onClick={() => { setSelectedOrder(record); setScheduleModalVisible(true); }}>排产</Button>
              )}
              {record.task_status === "in_progress" && (
                <>
                  <Popconfirm title="确认完工？" onConfirm={async () => {
                    try {
                      await completeMutate({ url: `/api/production/tasks/${record.id}/complete`, method: "post", values: {} });
                      open?.({ type: "success", message: "已标记完工" });
                      tableQuery?.refetch();
                    } catch { open?.({ type: "error", message: "操作失败" }); }
                  }}>
                    <Button type="link" size="small" icon={<CheckCircleOutlined />} style={{ color: "#52c41a" }}>完工</Button>
                  </Popconfirm>
                  <Popconfirm title="确认暂停？" onConfirm={async () => {
                    try {
                      await actionMutate({ url: `/api/production/tasks/${record.id}/pause`, method: "post", values: {} });
                      open?.({ type: "success", message: "已暂停" });
                      tableQuery?.refetch();
                    } catch { open?.({ type: "error", message: "操作失败" }); }
                  }}>
                    <Button type="link" size="small" icon={<PauseCircleOutlined />} style={{ color: "#13c2c2" }}>暂停</Button>
                  </Popconfirm>
                </>
              )}
              {(record.task_status === "in_progress" || record.task_status === "paused") && (
                <Button type="link" size="small" icon={<WarningOutlined />} danger onClick={() => { setErrorTaskId(record.id); errorForm.resetFields(); setErrorModalVisible(true); }}>异常</Button>
              )}
              {record.task_status !== "completed" && record.task_status !== "cancelled" && (
                <Popconfirm title="确认取消此任务？此操作不可撤销" onConfirm={async () => {
                  try {
                    await actionMutate({ url: `/api/production/tasks/${record.id}/cancel`, method: "post", values: {} });
                    open?.({ type: "success", message: "任务已取消" });
                    tableQuery?.refetch();
                  } catch { open?.({ type: "error", message: "操作失败" }); }
                }}>
                  <Button type="link" size="small" icon={<StopOutlined />}>取消</Button>
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

      <Modal
        title="异常上报"
        open={errorModalVisible}
        onCancel={() => setErrorModalVisible(false)}
        onOk={() => errorForm.submit()}
      >
        <Form form={errorForm} layout="vertical" onFinish={async (values) => {
          try {
            await actionMutate({
              url: `/api/production/tasks/${errorTaskId}/error`,
              method: "post",
              values: { error_description: values.error_description },
            });
            open?.({ type: "success", message: "异常已上报" });
            setErrorModalVisible(false);
            tableQuery?.refetch();
          } catch { open?.({ type: "error", message: "操作失败" }); }
        }}>
          <Form.Item name="error_description" label="异常描述" rules={[{ required: true, message: "请输入异常描述" }]}>
            <Input.TextArea rows={3} placeholder="请描述异常情况..." />
          </Form.Item>
        </Form>
      </Modal>
    </>
  );
};
