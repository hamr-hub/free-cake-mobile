import React from "react";
import { Refine, Authenticated } from "@refinedev/core";
import routerProvider, { NavigateToResource } from "@refinedev/react-router";
import { ErrorComponent } from "@refinedev/antd";
import { BrowserRouter, Routes, Route } from "react-router";
import { ConfigProvider, App as AntdApp } from "antd";

import dataProvider from "./providers/dataProvider";
import authProvider from "./providers/authProvider";
import { AdminLayout } from "./layouts/AdminLayout";

import { DashboardPage } from "./pages/dashboard";
import { ActivityList, ActivityCreate, ActivityShow } from "./pages/activities";
import { RegionList, RegionCreate } from "./pages/regions";
import { EntryList } from "./pages/entries";
import { RiskControlPage } from "./pages/votes";
import { SettlementList } from "./pages/settlement";
import { ProductionList } from "./pages/production";
import { RedeemList } from "./pages/redeem";
import { InventoryList } from "./pages/inventory";
import { StoreList, StoreCreate } from "./pages/stores";
import { StaffList, StaffCreate } from "./pages/staff";
import { LoginPage } from "./pages/login";

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
              { name: "activities", list: "/activities", create: "/activities/create", show: "/activities/show/:id", meta: { label: "活动管理" } },
              { name: "regions", list: "/regions", create: "/regions/create", meta: { label: "赛区管理" } },
              { name: "entries", list: "/entries", meta: { label: "作品审核" } },
              { name: "votes", list: "/votes", meta: { label: "投票风控" } },
              { name: "settlement", list: "/settlement", meta: { label: "开奖与订单" } },
              { name: "production", list: "/production", meta: { label: "排产中心" } },
              { name: "redeem", list: "/redeem", meta: { label: "核销管理" } },
              { name: "inventory", list: "/inventory", meta: { label: "库存中心" } },
              { name: "stores", list: "/stores", create: "/stores/create", meta: { label: "门店管理" } },
              { name: "staff", list: "/staff", create: "/staff/create", meta: { label: "人员考勤" } },
            ]}
          >
            <Routes>
              <Route path="/login" element={<LoginPage />} />
              <Route
                element={
                  <Authenticated key="authenticated-routes" redirectOnFail="/login">
                    <AdminLayout />
                  </Authenticated>
                }
              >
                <Route index element={<NavigateToResource resource="dashboard" />} />
                <Route path="/dashboard" element={<DashboardPage />} />
                <Route path="/activities">
                  <Route index element={<ActivityList />} />
                  <Route path="create" element={<ActivityCreate />} />
                  <Route path="show/:id" element={<ActivityShow />} />
                </Route>
                <Route path="/regions">
                  <Route index element={<RegionList />} />
                  <Route path="create" element={<RegionCreate />} />
                </Route>
                <Route path="/entries" element={<EntryList />} />
                <Route path="/votes" element={<RiskControlPage />} />
                <Route path="/settlement" element={<SettlementList />} />
                <Route path="/production" element={<ProductionList />} />
                <Route path="/redeem" element={<RedeemList />} />
                <Route path="/inventory" element={<InventoryList />} />
                <Route path="/stores">
                  <Route index element={<StoreList />} />
                  <Route path="create" element={<StoreCreate />} />
                </Route>
                <Route path="/staff">
                  <Route index element={<StaffList />} />
                  <Route path="create" element={<StaffCreate />} />
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
