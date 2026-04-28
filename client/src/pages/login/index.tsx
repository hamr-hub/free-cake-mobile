import React, { useState } from "react";
import { useLogin } from "@refinedev/core";
import { Card, Form, Input, Button, Typography, message, Row, Col } from "antd";
import { UserOutlined, LockOutlined } from "@ant-design/icons";

const { Title, Text } = Typography;

export const LoginPage: React.FC = () => {
  const { mutate: login } = useLogin();
  const [loading, setLoading] = useState(false);

  const onFinish = async (values: { username: string; password: string }) => {
    setLoading(true);
    login(values, {
      onSuccess: () => {
        message.success("登录成功");
      },
      onError: () => {
        message.error("登录失败，请检查手机号和验证码");
        setLoading(false);
      },
      onSettled: () => {
        setLoading(false);
      },
    });
  };

  return (
    <Row justify="center" align="middle" style={{ minHeight: "100vh", background: "#f0f2f5" }}>
      <Col>
        <Card style={{ width: 400, borderRadius: 8 }} bordered={false}>
          <div style={{ textAlign: "center", marginBottom: 32 }}>
            <Title level={3}>Free Cake 运营后台</Title>
            <Text type="secondary">云端 B 端全域管控中台</Text>
          </div>
          <Form onFinish={onFinish} size="large">
            <Form.Item name="username" rules={[{ required: true, message: "请输入手机号" }]}>
              <Input prefix={<UserOutlined />} placeholder="手机号" />
            </Form.Item>
            <Form.Item name="password" rules={[{ required: true, message: "请输入验证码" }]}>
              <Input prefix={<LockOutlined />} placeholder="验证码" />
            </Form.Item>
            <Form.Item>
              <Button type="primary" htmlType="submit" loading={loading} block>
                登录
              </Button>
            </Form.Item>
          </Form>
        </Card>
      </Col>
    </Row>
  );
};

export default LoginPage;
