import React from "react";
import { Drawer, Descriptions, Typography, Timeline } from "antd";
import { useCustom } from "@refinedev/core";

interface AuditDrawerProps {
  targetType: string;
  targetId: number;
  visible: boolean;
  onClose: () => void;
}

interface AuditLogEntry {
  id: number;
  operator_id: number;
  action: string;
  target_type: string;
  target_id: number;
  detail: string;
  created_at: string;
}

export const AuditDrawer: React.FC<AuditDrawerProps> = ({
  targetType,
  targetId,
  visible,
  onClose,
}) => {
  const result = useCustom({
    url: `/api/audit_log?target_type=${targetType}&target_id=${targetId}`,
    method: "get",
  });

  const isLoading = result.query.isLoading;
  const logs = (result.query.data?.data as any)?.list || [];

  return (
    <Drawer
      title="审计日志"
      placement="right"
      width={480}
      open={visible}
      onClose={onClose}
      loading={isLoading}
    >
      <Timeline
        items={logs.map((log: AuditLogEntry) => ({
          color: log.action.includes("frozen") || log.action.includes("deduct") ? "red" :
                 log.action.includes("redeem") ? "green" : "blue",
          children: (
            <Descriptions column={1} size="small">
              <Descriptions.Item label="操作">{log.action}</Descriptions.Item>
              <Descriptions.Item label="详情">{log.detail}</Descriptions.Item>
              <Descriptions.Item label="操作者ID">{log.operator_id}</Descriptions.Item>
              <Descriptions.Item label="时间">{log.created_at}</Descriptions.Item>
            </Descriptions>
          ),
        }))}
      />
      {logs.length === 0 && !isLoading && (
        <Typography.Text type="secondary">暂无审计记录</Typography.Text>
      )}
    </Drawer>
  );
};

export default AuditDrawer;
