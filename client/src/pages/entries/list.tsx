import React, { useState } from "react";
import { useTable, List } from "@refinedev/antd";
import { useNotification } from "@refinedev/core";
import { Table, Space, Tag, Image, Button, Popconfirm, Modal, Descriptions } from "antd";
import { CheckCircleOutlined, CloseCircleOutlined, EyeOutlined } from "@ant-design/icons";
import { RiskTag } from "../../components/RiskTag";

const statusColorMap: Record<string, string> = {
  pending: "orange",
  approved: "green",
  rejected: "red",
  active: "green",
  frozen: "blue",
};

const statusLabelMap: Record<string, string> = {
  pending: "待审核",
  approved: "已通过",
  rejected: "已驳回",
  active: "正常",
  frozen: "已冻结",
};

export const EntryList: React.FC = () => {
  const { tableProps, tableQuery } = useTable({ resource: "entries" });
  const { open } = useNotification();
  const [previewVisible, setPreviewVisible] = useState(false);
  const [previewEntry, setPreviewEntry] = useState<any>(null);

  const handleStatusChange = async (entryId: number, newStatus: string) => {
    try {
      const res = await fetch(`/api/entries/${entryId}/status`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ status: newStatus }),
      });
      if (!res.ok) throw new Error("操作失败");
      open?.({ type: "success", message: `作品已${newStatus === "approved" ? "通过" : "驳回"}` });
      tableQuery?.refetch();
    } catch (e: any) {
      open?.({ type: "error", message: "操作失败", description: e.message });
    }
  };

  const showPreview = (record: any) => {
    setPreviewEntry(record);
    setPreviewVisible(true);
  };

  return (
    <>
      <List>
        <Table {...tableProps} rowKey="id">
          <Table.Column dataIndex="id" title="ID" width={60} />
          <Table.Column dataIndex="user_name" title="参赛者" width={100} />
          <Table.Column dataIndex="region_name" title="赛区" width={100} />
          <Table.Column
            dataIndex="image_url"
            title="蛋糕图"
            width={100}
            render={(value: string) => value ? <Image src={value} width={60} height={60} style={{ objectFit: "cover", borderRadius: 4 }} preview={false} /> : "-"}
          />
          <Table.Column dataIndex="ai_generated" title="AI生成" width={80} render={(v: boolean) => <Tag color={v ? "blue" : "default"}>{v ? "是" : "否"}</Tag>} />
          <Table.Column dataIndex="status" title="审核状态" width={100} render={(v: string) => (
            <Tag color={statusColorMap[v] || "default"}>{statusLabelMap[v] || v}</Tag>
          )} />
          <Table.Column dataIndex="vote_count" title="得票数" width={80} />
          <Table.Column dataIndex="risk_tags" title="风险" width={120} render={(v: any) => {
            if (!v) return "-";
            const tags = typeof v === "string" ? JSON.parse(v) : v;
            return Array.isArray(tags) ? tags.map((t: string, i: number) => <RiskTag key={i} level={t.includes("high") ? "high" : t.includes("cluster") ? "medium" : "low"} reason={t} />) : "-";
          }} />
          <Table.Column dataIndex="created_at" title="提交时间" width={140} />
          <Table.Column title="操作" width={200} render={(_, record: any) => (
            <Space>
              <Button type="link" size="small" icon={<EyeOutlined />} onClick={() => showPreview(record)}>预览</Button>
              {record.status === "pending" && (
                <>
                  <Popconfirm title="确认通过？" onConfirm={() => handleStatusChange(record.id, "approved")}>
                    <Button type="link" size="small" icon={<CheckCircleOutlined />} style={{ color: "#52c41a" }}>通过</Button>
                  </Popconfirm>
                  <Popconfirm title="确认驳回？" onConfirm={() => handleStatusChange(record.id, "rejected")}>
                    <Button type="link" size="small" icon={<CloseCircleOutlined />} danger>驳回</Button>
                  </Popconfirm>
                </>
              )}
            </Space>
          )} />
        </Table>
      </List>

      <Modal
        title={`作品预览 #${previewEntry?.id || ""}`}
        open={previewVisible}
        onCancel={() => setPreviewVisible(false)}
        footer={null}
        width={600}
      >
        {previewEntry && (
          <Descriptions column={1} bordered>
            <Descriptions.Item label="作品图片">
              {previewEntry.image_url && <Image src={previewEntry.image_url} style={{ maxWidth: "100%" }} />}
            </Descriptions.Item>
            <Descriptions.Item label="参赛者">{previewEntry.user_name}</Descriptions.Item>
            <Descriptions.Item label="标题">{previewEntry.title}</Descriptions.Item>
            <Descriptions.Item label="状态">
              <Tag color={statusColorMap[previewEntry.status]}>{statusLabelMap[previewEntry.status] || previewEntry.status}</Tag>
            </Descriptions.Item>
            <Descriptions.Item label="得票数">{previewEntry.vote_count}</Descriptions.Item>
            <Descriptions.Item label="分享码">{previewEntry.share_code}</Descriptions.Item>
          </Descriptions>
        )}
      </Modal>
    </>
  );
};
