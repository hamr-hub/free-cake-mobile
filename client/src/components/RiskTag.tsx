import React from "react";
import { Tag } from "antd";

interface RiskTagProps {
  level?: "low" | "medium" | "high" | string;
  reason?: string;
  style?: React.CSSProperties;
}

const levelColorMap: Record<string, string> = {
  low: "green",
  medium: "orange",
  high: "red",
};

export const RiskTag: React.FC<RiskTagProps> = ({ level, reason, style }) => {
  if (!level && !reason) return null;

  const color = levelColorMap[level || ""] || "default";

  return (
    <Tag color={color} style={style}>
      {reason || level || "风险"}
    </Tag>
  );
};

export default RiskTag;
