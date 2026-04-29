import React from "react";
import { Navigate } from "react-router";

const adminOnlyRoutes = ["/regions", "/reports", "/stores", "/inventory", "/settlement", "/prices", "/orders", "/templates", "/audit-log", "/risk-events"];

export const RoleGuard: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const role = localStorage.getItem("role") || "operator";

  const currentPath = window.location.pathname;
  const basePath = "/" + currentPath.split("/")[1];

  if (role !== "admin" && adminOnlyRoutes.includes(basePath)) {
    return <Navigate to="/dashboard" replace />;
  }

  return <>{children}</>;
};
