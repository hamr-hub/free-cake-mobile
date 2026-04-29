import React, { useState, useRef, useCallback } from "react";
import { useLogin, useCustomMutation } from "@refinedev/core";
import { Card, Form, Input, Button, Typography, message, Row, Col, Space } from "antd";
import { UserOutlined, LockOutlined } from "@ant-design/icons";

const { Title, Text } = Typography;

const COOLDOWN_SECONDS = 60;

export const LoginPage: React.FC = () => {
  const { mutate: login } = useLogin();
  const { mutateAsync: sendCodeAction } = useCustomMutation();
  const [loading, setLoading] = useState(false);
  const [sendingCode, setSendingCode] = useState(false);
  const [cooldown, setCooldown] = useState(0);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const startCooldown = useCallback(() => {
    setCooldown(COOLDOWN_SECONDS);
    timerRef.current = setInterval(() => {
      setCooldown((prev) => {
        if (prev <= 1) {
          if (timerRef.current) clearInterval(timerRef.current);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  }, []);

  const handleSendCode = async (phone: string) => {
    if (!phone || !/^1\d{10}$/.test(phone)) {
      message.error("请输入正确的手机号");
      return;
    }
    setSendingCode(true);
    try {
      await sendCodeAction({
        url: "/api/auth/send-verify-code",
        method: "post",
        values: { phone },
      });
      message.success("验证码已发送");
      startCooldown();
    } catch {
      message.error("发送验证码失败，请稍后重试");
    } finally {
      setSendingCode(false);
    }
  };

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
              <Space.Compact style={{ width: "100%" }}>
                <Input prefix={<LockOutlined />} placeholder="验证码" style={{ flex: 1 }} />
                <Form.Item noStyle shouldUpdate={(prev, cur) => prev.username !== cur.username}>
                  {({ getFieldValue }) => (
                    <Button
                      onClick={() => handleSendCode(getFieldValue("username"))}
                      loading={sendingCode}
                      disabled={cooldown > 0}
                      style={{ minWidth: 120 }}
                    >
                      {cooldown > 0 ? `${cooldown}s` : "获取验证码"}
                    </Button>
                  )}
                </Form.Item>
              </Space.Compact>
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
