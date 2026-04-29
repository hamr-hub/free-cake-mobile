import React, { useState } from "react";
import { useForm, Create } from "@refinedev/antd";
import { Form, Input, DatePicker, InputNumber, Row, Col, Card, Select, Switch, Divider, Space, Tag } from "antd";
import { useCustom } from "@refinedev/core";

const { RangePicker } = DatePicker;

interface ActivityRules {
  max_votes_per_user_per_day: number;
  region_restricted: boolean;
  ai_generation_rate_limit: number;
  min_entry_age: number;
  allow_ai_generated: boolean;
  vote_weight_by_region: boolean;
}

interface ActivityTemplate {
  id: number;
  name: string;
  rules: ActivityRules;
}

const defaultRules: ActivityRules = {
  max_votes_per_user_per_day: 10,
  region_restricted: true,
  ai_generation_rate_limit: 3,
  min_entry_age: 0,
  allow_ai_generated: true,
  vote_weight_by_region: false,
};

export const ActivityCreate: React.FC = () => {
  const { formProps, saveButtonProps } = useForm({
    resource: "activities",
    action: "create",
  });

  const [rules, setRules] = useState<ActivityRules>(defaultRules);
  const [selectedTemplate, setSelectedTemplate] = useState<number | null>(null);

  const { query: templatesQuery } = useCustom({
    url: "/api/activities/templates",
    method: "get",
  });
  const templates = (templatesQuery.data?.data || []) as ActivityTemplate[];

  const handleTemplateSelect = (templateId: number) => {
    const tpl = templates.find((t) => t.id === templateId);
    if (tpl) {
      setRules(tpl.rules);
      setSelectedTemplate(templateId);
    }
  };

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
      rules: rules,
    };
    formProps.onFinish?.(transformed);
  };

  const handleRuleChange = (key: keyof ActivityRules, value: any) => {
    setRules((prev) => ({ ...prev, [key]: value }));
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

        {templates.length > 0 && (
          <Card title="模板复用" size="small" style={{ marginBottom: 16 }}>
            <Space>
              <Select
                style={{ width: 300 }}
                placeholder="选择已有活动模板快速复用规则"
                value={selectedTemplate}
                onChange={handleTemplateSelect}
                allowClear
                options={templates.map((t) => ({ label: t.name, value: t.id }))}
              />
              {selectedTemplate && <Tag color="blue">已加载模板规则</Tag>}
            </Space>
          </Card>
        )}

        <Card title="时间配置" size="small" style={{ marginBottom: 16 }}>
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
        </Card>

        <Row gutter={16}>
          <Col span={12}>
            <Form.Item name="max_winner_count" label="获奖人数上限" initialValue={100} rules={[{ required: true }]}>
              <InputNumber min={1} max={500} style={{ width: "100%" }} />
            </Form.Item>
          </Col>
        </Row>

        <Divider>活动规则配置</Divider>

        <Card title="投票规则" size="small" style={{ marginBottom: 16 }}>
          <Row gutter={16}>
            <Col span={12}>
              <Form.Item label="每人每日投票上限">
                <InputNumber
                  min={1}
                  max={100}
                  value={rules.max_votes_per_user_per_day}
                  onChange={(v) => handleRuleChange("max_votes_per_user_per_day", v || 10)}
                  style={{ width: "100%" }}
                  addonAfter="票/天"
                />
              </Form.Item>
            </Col>
            <Col span={12}>
              <Form.Item label="AI生成速率限制">
                <InputNumber
                  min={1}
                  max={20}
                  value={rules.ai_generation_rate_limit}
                  onChange={(v) => handleRuleChange("ai_generation_rate_limit", v || 3)}
                  style={{ width: "100%" }}
                  addonAfter="次/小时"
                />
              </Form.Item>
            </Col>
          </Row>
          <Row gutter={16}>
            <Col span={8}>
              <Form.Item label="赛区投票限制">
                <Switch
                  checked={rules.region_restricted}
                  onChange={(v) => handleRuleChange("region_restricted", v)}
                  checkedChildren="仅本赛区"
                  unCheckedChildren="不限赛区"
                />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item label="允许AI生成作品">
                <Switch
                  checked={rules.allow_ai_generated}
                  onChange={(v) => handleRuleChange("allow_ai_generated", v)}
                  checkedChildren="允许"
                  unCheckedChildren="禁止"
                />
              </Form.Item>
            </Col>
            <Col span={8}>
              <Form.Item label="赛区加权投票">
                <Switch
                  checked={rules.vote_weight_by_region}
                  onChange={(v) => handleRuleChange("vote_weight_by_region", v)}
                  checkedChildren="加权"
                  unCheckedChildren="等权"
                />
              </Form.Item>
            </Col>
          </Row>
        </Card>
      </Form>
    </Create>
  );
};
