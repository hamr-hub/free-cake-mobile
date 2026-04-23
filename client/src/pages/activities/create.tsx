import React from "react";
import { useForm, Create } from "@refinedev/antd";
import { Form, Input, DatePicker, InputNumber, Row, Col, Card } from "antd";

const { RangePicker } = DatePicker;

export const ActivityCreate: React.FC = () => {
  const { formProps, saveButtonProps } = useForm({
    resource: "activities",
    action: "create",
  });

  const onFinish = (values: any) => {
    const [registrationRange, votingRange] = values.time_ranges || [];
    const transformed = {
      region_id: values.region_id,
      name: values.name,
      registration_start_at: registrationRange?.[0]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      registration_end_at: registrationRange?.[1]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      voting_start_at: votingRange?.[0]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      voting_end_at: votingRange?.[1]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      max_winner_count: values.max_winner_count,
    };
    formProps.onFinish?.(transformed);
  };

  return (
    <Create saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical" onFinish={onFinish}>
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name="name" label="活动名称" rules={[{ required: true, message: "请输入活动名称" }]}>
              <Input placeholder="例如：XX镇第一届蛋糕大赛" />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item name="region_id" label="赛区ID" rules={[{ required: true, message: "请选择赛区" }]}>
              <InputNumber min={1} style={{ width: "100%" }} placeholder="输入赛区ID" />
            </Form.Item>
          </Col>
        </Row>

        <Card title="时间配置" size="small" style={{ marginBottom: 16 }}>
          <Form.Item name="time_ranges" label="活动时间范围" rules={[{ required: true }]}>
            <RangePicker
              style={{ width: "100%" }}
              showTime
              format="YYYY-MM-DD HH:mm"
              placeholder={["报名开始", "报名截止"]}
            />
          </Form.Item>
          <Form.Item name="time_ranges_voting" label="投票时间范围" rules={[{ required: true }]}>
            <RangePicker
              style={{ width: "100%" }}
              showTime
              format="YYYY-MM-DD HH:mm"
              placeholder={["投票开始", "投票截止"]}
            />
          </Form.Item>
        </Card>

        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name="max_winner_count" label="获奖人数上限" initialValue={100} rules={[{ required: true }]}>
              <InputNumber min={1} max={500} style={{ width: "100%" }} />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Create>
  );
};
