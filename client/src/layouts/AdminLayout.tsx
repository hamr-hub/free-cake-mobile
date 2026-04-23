import React from "react";
import { useGetIdentity, useLogout } from "@refinedev/core";
import { Layout as AntdLayout, Menu, Avatar, Dropdown, Typography, theme } from "antd";
import {
  DashboardOutlined,
  TrophyOutlined,
  EnvironmentOutlined,
  PictureOutlined,
  SafetyCertificateOutlined,
  DollarOutlined,
  ScheduleOutlined,
  TagOutlined,
  AppstoreOutlined,
  ShopOutlined,
  TeamOutlined,
  LogoutOutlined,
  UserOutlined,
} from "@ant-design/icons";
import { useNavigate, useLocation, Outlet } from "react-router";

const { Sider, Header, Content } = AntdLayout;
const { Text } = Typography;

const menuItems = [
  { key: "/dashboard", icon: <DashboardOutlined />, label: "总览" },
  { key: "/activities", icon: <TrophyOutlined />, label: "活动管理" },
  { key: "/regions", icon: <EnvironmentOutlined />, label: "赛区管理" },
  { key: "/entries", icon: <PictureOutlined />, label: "作品审核" },
  { key: "/votes", icon: <SafetyCertificateOutlined />, label: "投票风控" },
  { key: "/settlement", icon: <DollarOutlined />, label: "开奖与订单" },
  { key: "/production", icon: <ScheduleOutlined />, label: "排产中心" },
  { key: "/redeem", icon: <TagOutlined />, label: "核销管理" },
  { key: "/inventory", icon: <AppstoreOutlined />, label: "库存中心" },
  { key: "/stores", icon: <ShopOutlined />, label: "门店管理" },
  { key: "/staff", icon: <TeamOutlined />, label: "人员考勤" },
];

export const AdminLayout: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { data: identity } = useGetIdentity();
  const { mutate: logout } = useLogout();
  const { token: themeToken } = theme.useToken();

  const selectedKey = "/" + location.pathname.split("/")[1];

  const userMenuItems = [
    {
      key: "logout",
      icon: <LogoutOutlined />,
      label: "退出登录",
      onClick: () => logout(),
    },
  ];

  return (
    <AntdLayout style={{ minHeight: "100vh" }}>
      <Sider
        width={220}
        style={{
          overflow: "auto",
          height: "100vh",
          position: "fixed",
          left: 0,
          top: 0,
          bottom: 0,
          background: themeToken.colorBgContainer,
          borderRight: `1px solid ${themeToken.colorBorderSecondary}`,
        }}
      >
        <div
          style={{
            height: 64,
            display: "flex",
            alignItems: "center",
            justifyContent: "center",
            borderBottom: `1px solid ${themeToken.colorBorderSecondary}`,
          }}
        >
          <Text strong style={{ fontSize: 18 }}>
            Free Cake
          </Text>
        </div>
        <Menu
          mode="inline"
          selectedKeys={[selectedKey]}
          items={menuItems}
          onClick={({ key }) => navigate(key)}
          style={{ borderRight: "none" }}
        />
      </Sider>
      <AntdLayout style={{ marginLeft: 220 }}>
        <Header
          style={{
            padding: "0 24px",
            background: themeToken.colorBgContainer,
            display: "flex",
            alignItems: "center",
            justifyContent: "flex-end",
            borderBottom: `1px solid ${themeToken.colorBorderSecondary}`,
            height: 64,
          }}
        >
          <Dropdown menu={{ items: userMenuItems }} placement="bottomRight">
            <div style={{ cursor: "pointer", display: "flex", alignItems: "center", gap: 8 }}>
              <Avatar icon={<UserOutlined />} size="small" />
              <Text>{(identity as any)?.role === "admin" ? "管理员" : "运营"}</Text>
            </div>
          </Dropdown>
        </Header>
        <Content style={{ margin: 24, minHeight: 280 }}>
          <Outlet />
        </Content>
      </AntdLayout>
    </AntdLayout>
  );
};

export default AdminLayout;
