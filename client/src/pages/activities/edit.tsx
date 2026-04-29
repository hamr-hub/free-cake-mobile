import React from "react";
import dayjs from "dayjs";
import { useForm, Edit } from "@refinedev/antd";
import { Form, Input, DatePicker, InputNumber, Row, Col, Switch, Divider } from "antd";

const { RangePicker } = DatePicker;

export const ActivityEdit: React.FC = () => {
  const { formProps, saveButtonProps, query } = useForm({
    resource: "activities",
    action: "edit",
  });

  const onFinish = (values: any) => {
    const registrationRange = values.time_ranges;
    const votingRange = values.time_ranges_voting;
    const transformed = {
      region_id: values.region_id,
      name: values.name,
      registration_start_at: registrationRange?.[0]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      registration_end_at: registrationRange?.[1]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      voting_start_at: votingRange?.[0]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      voting_end_at: votingRange?.[1]?.format("YYYY-MM-DDTHH:mm:ss") || "",
      max_winner_count: values.max_winner_count,
      rules: values.rules || {},
    };
    formProps.onFinish?.(transformed);
  };

  const data = query?.data?.data;

  const initialValues = data ? {
    name: data.name,
    region_id: data.region_id,
    time_ranges: data.registration_start_at && data.registration_end_at
      ? [dayjs(data.registration_start_at), dayjs(data.registration_end_at)]
      : undefined,
    time_ranges_voting: data.voting_start_at && data.voting_end_at
      ? [dayjs(data.voting_start_at), dayjs(data.voting_end_at)]
      : undefined,
    max_winner_count: data.max_winner_count,
    rules: data.rules || {},
  } : {};

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical" onFinish={onFinish} initialValues={initialValues}>
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name="name" label="活动名称" rules={[{ required: true, message: "请输入活动名称" }]}>
              <Input placeholder="例如：XX镇第一届蛋糕大赛" />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item name="region_id" label="赛区ID" rules={[{ required: true }]}>
              <InputNumber min={1} style={{ width: "100%" }} />
            </Form.Item>
          </Col>
        </Row>

        <Form.Item name="time_ranges" label="报名时间范围" rules={[{ required: true }]}>
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

        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name="max_winner_count" label="获奖人数上限" rules={[{ required: true }]}>
              <InputNumber min={1} max={500} style={{ width: "100%" }} />
            </Form.Item>
          </Col>
        </Row>

        <Divider>活动规则</Divider>
        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name={["rules", "max_votes_per_user_per_day"]} label="每日投票上限">
              <InputNumber min={1} max={100} />
            </Form.Item>
          </Col>
          <Col span={12}>
            <Form.Item name={["rules", "ai_generation_rate_limit"]} label="AI生成频率限制(次/时)">
              <InputNumber min={1} max={20} />
            </Form.Item>
          </Col>
        </Row>
        <Row gutter={16}>
          <Col span={8}>
            <Form.Item name={["rules", "region_restricted"]} label="赛区限制" valuePropName="checked">
              <Switch />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item name={["rules", "allow_ai_generated"]} label="允许AI作品" valuePropName="checked">
              <Switch />
            </Form.Item>
          </Col>
          <Col span={8}>
            <Form.Item name={["rules", "vote_weight_by_region"]} label="加权投票" valuePropName="checked">
              <Switch />
            </Form.Item>
          </Col>
        </Row>
      </Form>
    </Edit>
  );
};
