import React from "react";
import { useForm, Edit } from "@refinedev/antd";
import { Form, Input, InputNumber } from "antd";

export const StoreEdit: React.FC = () => {
  const { formProps, saveButtonProps } = useForm({
    resource: "stores",
    action: "edit",
  });

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical">
        <Form.Item name="name" label="门店名称" rules={[{ required: true, message: "请输入门店名称" }]}>
          <Input placeholder="如：XX镇蛋糕自提点" />
        </Form.Item>
        <Form.Item name="region_id" label="赛区ID" rules={[{ required: true }]}>
          <InputNumber min={1} />
        </Form.Item>
        <Form.Item name="address" label="地址" rules={[{ required: true }]}>
          <Input placeholder="详细地址" />
        </Form.Item>
        <Form.Item name="lat" label="纬度" rules={[{ required: true }]}>
          <InputNumber />
        </Form.Item>
        <Form.Item name="lng" label="经度" rules={[{ required: true }]}>
          <InputNumber />
        </Form.Item>
        <Form.Item name="daily_capacity" label="日产能" rules={[{ required: true }]}>
          <InputNumber min={1} />
        </Form.Item>
        <Form.Item name="contact_name" label="联系人" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
        <Form.Item name="contact_phone" label="联系电话" rules={[{ required: true }]}>
          <Input />
        </Form.Item>
      </Form>
    </Edit>
  );
};
