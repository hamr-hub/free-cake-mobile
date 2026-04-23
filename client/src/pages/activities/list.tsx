import React from "react";
import { useTable, List, ShowButton } from "@refinedev/antd";
import { Table, Space, Tag, Button, Popconfirm } from "antd";
import { useNotification } from "@refinedev/core";
import { PlayCircleOutlined } from "@ant-design/icons";

const statusColorMap: Record<string, string> = {
  draft: "default",
  registration_open: "blue",
  voting_open: "green",
  voting_closed: "orange",
  settled: "gold",
  redeeming: "purple",
  finished: "red",
};

const statusLabelMap: Record<string, string> = {
  draft: "草稿",
  registration_open: "报名开放",
  voting_open: "投票开放",
  voting_closed: "投票截止",
  settled: "已结算",
  redeeming: "核销中",
  finished: "已结束",
};

const nextStatusMap: Record<string, { status: string; label: string }> = {
  draft: { status: "registration_open", label: "开放报名" },
  registration_open: { status: "voting_open", label: "开放投票" },
  voting_open: { status: "voting_closed", label: "截止投票" },
  voting_closed: { status: "settled", label: "结算" },
  settled: { status: "redeeming", label: "开启核销" },
  redeeming: { status: "finished", label: "结束活动" },
};

export const ActivityList: React.FC = () => {
  const { tableProps } = useTable({ resource: "activities" });
  const { open } = useNotification();

  const handleStatusTransition = async (activityId: number, newStatus: string) => {
    try {
      const res = await fetch(`/api/activities/${activityId}/status`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          Authorization: `Bearer ${localStorage.getItem("token")}`,
        },
        body: JSON.stringify({ new_status: newStatus }),
      });
      if (!res.ok) throw new Error("状态切换失败");
      open?.({
        type: "success",
        message: "状态切换成功",
        description: `活动已切换到 ${statusLabelMap[newStatus] || newStatus}`,
      });
      window.location.reload();
    } catch (e: any) {
      open?.({ type: "error", message: "状态切换失败", description: e.message });
    }
  };

  return (
    <List>
      <Table {...tableProps} rowKey="id">
        <Table.Column dataIndex="id" title="ID" width={60} />
        <Table.Column dataIndex="name" title="活动名称" ellipsis />
        <Table.Column dataIndex="status" title="状态" width={120} render={(v: string) => (
          <Tag color={statusColorMap[v] || "default"}>{statusLabelMap[v] || v}</Tag>
        )} />
        <Table.Column dataIndex="region_id" title="赛区ID" width={80} />
        <Table.Column dataIndex="max_winner_count" title="获奖上限" width={90} />
        <Table.Column dataIndex="registration_start_at" title="报名开始" width={120} />
        <Table.Column dataIndex="voting_start_at" title="投票开始" width={120} />
        <Table.Column dataIndex="voting_end_at" title="投票截止" width={120} />
        <Table.Column title="操作" width={220} render={(_, record: any) => {
          const next = nextStatusMap[record.status];
          return (
            <Space>
              <ShowButton hideText size="small" recordItemId={record.id} />
              {next && (
                <Popconfirm
                  title={`确认将活动切换到「${next.label}」？`}
                  onConfirm={() => handleStatusTransition(record.id, next.status)}
                >
                  <Button type="link" size="small" icon={<PlayCircleOutlined />}>
                    {next.label}
                  </Button>
                </Popconfirm>
              )}
            </Space>
          );
        }} />
      </Table>
    </List>
  );
};
