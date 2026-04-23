import React, { useState, useEffect } from "react";
import { Space, Typography } from "antd";
import { ClockCircleOutlined } from "@ant-design/icons";

interface CountdownBannerProps {
  targetTime: string;
  label?: string;
  onExpired?: () => void;
}

export const CountdownBanner: React.FC<CountdownBannerProps> = ({
  targetTime,
  label = "",
  onExpired,
}) => {
  const [remaining, setRemaining] = useState<number>(0);

  useEffect(() => {
    const target = new Date(targetTime).getTime();
    const timer = setInterval(() => {
      const diff = Math.max(0, target - Date.now());
      setRemaining(diff);
      if (diff <= 0 && onExpired) {
        onExpired();
        clearInterval(timer);
      }
    }, 1000);
    return () => clearInterval(timer);
  }, [targetTime, onExpired]);

  if (remaining <= 0) {
    return (
      <Space style={{ padding: "8px 16px", background: "#fff2f0", borderRadius: 6 }}>
        <ClockCircleOutlined style={{ color: "#ff4d4f" }} />
        <Typography.Text type="danger">{label}已结束</Typography.Text>
      </Space>
    );
  }

  const days = Math.floor(remaining / 86400000);
  const hours = Math.floor((remaining % 86400000) / 3600000);
  const minutes = Math.floor((remaining % 3600000) / 60000);
  const seconds = Math.floor((remaining % 60000) / 1000);

  const parts: string[] = [];
  if (days > 0) parts.push(`${days}天`);
  parts.push(`${hours}时`);
  parts.push(`${minutes}分`);
  parts.push(`${seconds}秒`);

  return (
    <Space style={{ padding: "8px 16px", background: "#e6f7ff", borderRadius: 6, display: "flex" }}>
      <ClockCircleOutlined style={{ color: "#1677ff" }} />
      <Typography.Text>
        {label}剩余 <Typography.Text strong style={{ color: "#1677ff" }}>{parts.join("")}</Typography.Text>
      </Typography.Text>
    </Space>
  );
};

export default CountdownBanner;
