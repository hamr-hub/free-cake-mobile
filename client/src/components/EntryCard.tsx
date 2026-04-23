import React from "react";
import { Card, Image, Tag, Typography, Space } from "antd";
import { TrophyOutlined } from "@ant-design/icons";

interface EntryCardProps {
  id: number;
  imageUrl: string;
  title?: string;
  userName?: string;
  voteCount?: number;
  rank?: number;
  status?: string;
  aiGenerated?: boolean;
  onClick?: (id: number) => void;
}

const statusColorMap: Record<string, string> = {
  pending: "orange",
  approved: "green",
  rejected: "red",
  active: "green",
  frozen: "blue",
};

export const EntryCard: React.FC<EntryCardProps> = ({
  id,
  imageUrl,
  title,
  userName,
  voteCount,
  rank,
  status,
  aiGenerated,
  onClick,
}) => {
  return (
    <Card
      hoverable
      style={{ width: 280 }}
      cover={<Image src={imageUrl} alt={title || "参赛作品"} style={{ height: 180, objectFit: "cover" }} preview={false} />}
      onClick={() => onClick?.(id)}
    >
      <Card.Meta
        title={
          <Space>
            {rank && rank <= 100 && (
              <Typography.Text style={{ color: "#faad14", fontSize: 12 }}>
                <TrophyOutlined /> #{rank}
              </Typography.Text>
            )}
            <Typography.Text ellipsis style={{ maxWidth: 160 }}>{title || `作品 #${id}`}</Typography.Text>
          </Space>
        }
        description={
          <Space direction="vertical" size={4} style={{ width: "100%" }}>
            <Space size={8}>
              {userName && <Typography.Text type="secondary">{userName}</Typography.Text>}
              {aiGenerated && <Tag color="blue" style={{ fontSize: 11 }}>AI生成</Tag>}
              {status && <Tag color={statusColorMap[status] || "default"}>{status}</Tag>}
            </Space>
            {voteCount !== undefined && (
              <Typography.Text type="secondary">得票 {voteCount}</Typography.Text>
            )}
          </Space>
        }
      />
    </Card>
  );
};

export default EntryCard;
