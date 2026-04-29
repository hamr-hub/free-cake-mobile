import React from "react";
import { useForm, Edit } from "@refinedev/antd";
import { Form, Input, InputNumber } from "antd";

export const RegionEdit: React.FC = () => {
  const { formProps, saveButtonProps } = useForm({
    resource: "regions",
    action: "edit",
  });

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical">
        <Form.Item name="name" label="赛区名称" rules={[{ required: true, message: "请输入赛区名称" }]}>
          <Input placeholder="如：XX镇赛区" />
        </Form.Item>
        <Form.Item name="province" label="省份" rules={[{ required: true }]}>
          <Input placeholder="省份" />
        </Form.Item>
        <Form.Item name="city" label="城市" rules={[{ required: true }]}>
          <Input placeholder="城市" />
        </Form.Item>
        <Form.Item name="coverage_radius_km" label="覆盖半径(km)" rules={[{ required: true }]}>
          <InputNumber min={1} max={50} />
        </Form.Item>
        <Form.Item name="center_lat" label="中心纬度" rules={[{ required: true }]}>
          <InputNumber />
        </Form.Item>
        <Form.Item name="center_lng" label="中心经度" rules={[{ required: true }]}>
          <InputNumber />
        </Form.Item>
      </Form>
    </Edit>
  );
};
