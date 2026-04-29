import React from "react";
import { useForm, Edit } from "@refinedev/antd";
import { Form, Input, InputNumber, Select } from "antd";

export const StaffEdit: React.FC = () => {
  const { formProps, saveButtonProps } = useForm({
    resource: "staff",
    action: "edit",
  });

  return (
    <Edit saveButtonProps={saveButtonProps}>
      <Form {...formProps} layout="vertical">
        <Form.Item name="name" label="姓名" rules={[{ required: true, message: "请输入姓名" }]}>
          <Input />
        </Form.Item>
        <Form.Item name="phone" label="手机号" rules={[{ required: true }]}>
          <Input placeholder="手机号" />
        </Form.Item>
        <Form.Item name="store_id" label="门店ID" rules={[{ required: true }]}>
          <InputNumber min={1} />
        </Form.Item>
        <Form.Item name="role" label="角色" rules={[{ required: true }]}>
          <Select options={[
            { value: "operator", label: "操作员" },
            { value: "manager", label: "店长" },
          ]} />
        </Form.Item>
      </Form>
    </Edit>
  );
};
