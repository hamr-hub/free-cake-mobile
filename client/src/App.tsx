import React from "react";
import { Refine, Authenticated } from "@refinedev/core";
import routerProvider, { NavigateToResource } from "@refinedev/react-router";
import { ErrorComponent } from "@refinedev/antd";
import { BrowserRouter, Routes, Route } from "react-router";
import { ConfigProvider, App as AntdApp } from "antd";

import dataProvider from "./providers/dataProvider";
import authProvider from "./providers/authProvider";
import { AdminLayout } from "./layouts/AdminLayout";
import { ErrorBoundary } from "./components/ErrorBoundary";

import { DashboardPage } from "./pages/dashboard";
import { ActivityList, ActivityCreate, ActivityShow, ActivityEdit } from "./pages/activities";
import { RegionList, RegionCreate, RegionEdit, RegionShow } from "./pages/regions";
import { EntryList } from "./pages/entries";
import { EntryShow } from "./pages/entries/show";
import { RiskControlPage } from "./pages/votes";
import { VoteShow } from "./pages/votes/show";
import { SettlementList } from "./pages/settlement";
import { SettlementShow } from "./pages/settlement/show";
import { ProductionList } from "./pages/production";
import { ProductionShow } from "./pages/production/show";
import { RedeemList } from "./pages/redeem";
import { RedeemShow } from "./pages/redeem/show";
import { InventoryList } from "./pages/inventory";
import { InventoryShow } from "./pages/inventory/show";
import { StoreList, StoreCreate, StoreEdit, StoreShow } from "./pages/stores";
import { StaffList, StaffCreate, StaffEdit, StaffAttendance, StaffShow } from "./pages/staff";
import { LoginPage } from "./pages/login";
import { ReportsPage } from "./pages/reports";
import { PriceList } from "./pages/prices";
import { PriceShow } from "./pages/prices/show";
import { OrderList } from "./pages/orders";
import { OrderShow } from "./pages/orders/show";
import { TemplateList } from "./pages/templates";
import { TemplateShow } from "./pages/templates/show";
import { AuditLogList } from "./pages/audit-log";
import { AuditLogShow } from "./pages/audit-log/show";
import { RiskEventList } from "./pages/risk-events";
import { RiskEventShow } from "./pages/risk-events/show";
import { RoleGuard } from "./components/RoleGuard";

const App: React.FC = () => {
  return (
    <BrowserRouter>
      <ConfigProvider
        theme={{
          token: {
            colorPrimary: "#1677ff",
            borderRadius: 6,
          },
        }}
      >
        <AntdApp>
          <Refine
            dataProvider={dataProvider}
            authProvider={authProvider}
            routerProvider={routerProvider}
            resources={[
              { name: "dashboard", list: "/dashboard", meta: { label: "总览" } },
              { name: "activities", list: "/activities", create: "/activities/create", show: "/activities/show/:id", edit: "/activities/edit/:id", meta: { label: "活动管理" } },
              { name: "regions", list: "/regions", create: "/regions/create", show: "/regions/show/:id", edit: "/regions/edit/:id", meta: { label: "赛区管理" } },
              { name: "entries", list: "/entries", show: "/entries/show/:id", meta: { label: "作品审核" } },
              { name: "votes", list: "/votes", show: "/votes/show/:id", meta: { label: "投票风控" } },
              { name: "settlement", list: "/settlement", show: "/settlement/show/:id", meta: { label: "开奖与订单" } },
              { name: "production", list: "/production", show: "/production/show/:id", meta: { label: "排产中心" } },
              { name: "redeem", list: "/redeem", show: "/redeem/show/:id", meta: { label: "核销管理" } },
              { name: "inventory", list: "/inventory", show: "/inventory/show/:id", meta: { label: "库存中心" } },
              { name: "reports", list: "/reports", meta: { label: "运营报表" } },
              { name: "prices", list: "/prices", show: "/prices/show/:id", meta: { label: "价格配置" } },
              { name: "orders", list: "/orders", show: "/orders/show/:id", meta: { label: "订单管理" } },
              { name: "templates", list: "/templates", show: "/templates/show/:id", meta: { label: "设计模板" } },
              { name: "audit-log", list: "/audit-log", show: "/audit-log/show/:id", meta: { label: "审计日志" } },
              { name: "risk-events", list: "/risk-events", show: "/risk-events/show/:id", meta: { label: "风控事件" } },
              { name: "stores", list: "/stores", create: "/stores/create", show: "/stores/show/:id", edit: "/stores/edit/:id", meta: { label: "门店管理" } },
              { name: "staff", list: "/staff", create: "/staff/create", show: "/staff/show/:id", edit: "/staff/edit/:id", meta: { label: "人员考勤" } },
            ]}
          >
            <Routes>
              <Route path="/login" element={<LoginPage />} />
              <Route
                element={
                  <Authenticated key="authenticated-routes" redirectOnFail="/login">
                    <ErrorBoundary>
                      <AdminLayout />
                    </ErrorBoundary>
                  </Authenticated>
                }
              >
                <Route index element={<NavigateToResource resource="dashboard" />} />
                <Route path="/dashboard" element={<RoleGuard><DashboardPage /></RoleGuard>} />
                <Route path="/activities">
                  <Route index element={<RoleGuard><ActivityList /></RoleGuard>} />
                  <Route path="create" element={<RoleGuard><ActivityCreate /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><ActivityShow /></RoleGuard>} />
                  <Route path="edit/:id" element={<RoleGuard><ActivityEdit /></RoleGuard>} />
                </Route>
                <Route path="/regions">
                  <Route index element={<RoleGuard><RegionList /></RoleGuard>} />
                  <Route path="create" element={<RoleGuard><RegionCreate /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><RegionShow /></RoleGuard>} />
                  <Route path="edit/:id" element={<RoleGuard><RegionEdit /></RoleGuard>} />
                </Route>
                <Route path="/entries">
                  <Route index element={<RoleGuard><EntryList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><EntryShow /></RoleGuard>} />
                </Route>
                <Route path="/votes">
                  <Route index element={<RoleGuard><RiskControlPage /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><VoteShow /></RoleGuard>} />
                </Route>
                <Route path="/settlement">
                  <Route index element={<RoleGuard><SettlementList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><SettlementShow /></RoleGuard>} />
                </Route>
                <Route path="/production">
                  <Route index element={<RoleGuard><ProductionList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><ProductionShow /></RoleGuard>} />
                </Route>
                <Route path="/redeem">
                  <Route index element={<RoleGuard><RedeemList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><RedeemShow /></RoleGuard>} />
                </Route>
                <Route path="/inventory">
                  <Route index element={<RoleGuard><InventoryList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><InventoryShow /></RoleGuard>} />
                </Route>
                <Route path="/reports" element={<RoleGuard><ReportsPage /></RoleGuard>} />
                <Route path="/prices">
                  <Route index element={<RoleGuard><PriceList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><PriceShow /></RoleGuard>} />
                </Route>
                <Route path="/orders">
                  <Route index element={<RoleGuard><OrderList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><OrderShow /></RoleGuard>} />
                </Route>
                <Route path="/templates">
                  <Route index element={<RoleGuard><TemplateList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><TemplateShow /></RoleGuard>} />
                </Route>
                <Route path="/audit-log">
                  <Route index element={<RoleGuard><AuditLogList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><AuditLogShow /></RoleGuard>} />
                </Route>
                <Route path="/risk-events">
                  <Route index element={<RoleGuard><RiskEventList /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><RiskEventShow /></RoleGuard>} />
                </Route>
                <Route path="/stores">
                  <Route index element={<RoleGuard><StoreList /></RoleGuard>} />
                  <Route path="create" element={<RoleGuard><StoreCreate /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><StoreShow /></RoleGuard>} />
                  <Route path="edit/:id" element={<RoleGuard><StoreEdit /></RoleGuard>} />
                </Route>
                <Route path="/staff">
                  <Route index element={<RoleGuard><StaffList /></RoleGuard>} />
                  <Route path="create" element={<RoleGuard><StaffCreate /></RoleGuard>} />
                  <Route path="show/:id" element={<RoleGuard><StaffShow /></RoleGuard>} />
                  <Route path="edit/:id" element={<RoleGuard><StaffEdit /></RoleGuard>} />
                  <Route path="attendance" element={<RoleGuard><StaffAttendance /></RoleGuard>} />
                </Route>
                <Route path="*" element={<ErrorComponent />} />
              </Route>
            </Routes>
          </Refine>
        </AntdApp>
      </ConfigProvider>
    </BrowserRouter>
  );
};

export default App;
