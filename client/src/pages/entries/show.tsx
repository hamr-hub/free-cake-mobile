import React from "react";
import { useParams } from "react-router";
import { useCustom } from "@refinedev/core";
import { Card, Descriptions, Tag, Button, Image } from "antd";
import { ArrowLeftOutlined } from "@ant-design/icons";

export const EntryShow: React.FC = () => {
  const { id } = useParams<{ id: string }>();
  const entryId = Number(id) || 0;

  const { query } = useCustom({
    url: `/api/entries/${entryId}`,
    method: "get",
  });

  const record = (query.data as any)?.data ?? null;

  if (query.isLoading) return <Card loading />;

  return (
    <Card
      title={`作品 #${record?.id ?? "-"}`}
      extra={<Button icon={<ArrowLeftOutlined />} onClick={() => window.history.back()}>返回</Button>}
    >
      {record?.image_url && (
        <div style={{ textAlign: "center", marginBottom: 16 }}>
          <Image src={record.image_url} style={{ maxHeight: 300 }} />
        </div>
      )}
      <Descriptions column={2} bordered>
        <Descriptions.Item label="ID">{record?.id}</Descriptions.Item>
        <Descriptions.Item label="活动ID">{record?.activity_id}</Descriptions.Item>
        <Descriptions.Item label="用户ID">{record?.user_id}</Descriptions.Item>
        <Descriptions.Item label="标题">{record?.title || "-"}</Descriptions.Item>
        <Descriptions.Item label="分享码">{record?.share_code || "-"}</Descriptions.Item>
        <Descriptions.Item label="有效票数">{record?.valid_vote_count ?? 0}</Descriptions.Item>
        <Descriptions.Item label="生成记录ID">{record?.selected_generation_id}</Descriptions.Item>
        <Descriptions.Item label="模板ID">{record?.selected_template_id}</Descriptions.Item>
        <Descriptions.Item label="状态">
          <Tag color={record?.status === "active" ? "green" : record?.status === "frozen" ? "orange" : "red"}>
            {record?.status ?? "-"}
          </Tag>
        </Descriptions.Item>
        <Descriptions.Item label="创建时间" span={2}>
          {record?.created_at ? new Date(record.created_at).toLocaleString("zh-CN") : "-"}
        </Descriptions.Item>
      </Descriptions>
    </Card>
  );
};
