import React, { useState } from "react";
import { useCustom } from "@refinedev/core";
import { Card, Row, Col, DatePicker, Select, Statistic, Spin, Tabs, Table, Tag, Alert, Empty } from "antd";
import ReactEChartsCore from "echarts-for-react/lib/core";
import * as echarts from "echarts/core";
import { BarChart, LineChart, PieChart } from "echarts/charts";
import { GridComponent, TooltipComponent, LegendComponent } from "echarts/components";
import { CanvasRenderer } from "echarts/renderers";

echarts.use([BarChart, LineChart, PieChart, GridComponent, TooltipComponent, LegendComponent, CanvasRenderer]);

const { RangePicker } = DatePicker;

interface PeriodData {
  period: string;
  total_orders: number;
  total_paid_orders: number;
  total_revenue: number;
  total_refunded: number;
  total_entries: number;
  total_votes: number;
  top_stores: { store_id: number; store_name: string; order_count: number; revenue: number }[];
}

const PeriodReportTab: React.FC<{ type: "daily" | "weekly" | "monthly" }> = ({ type }) => {
  const [date, setDate] = useState<string | undefined>(undefined);

  const params = new URLSearchParams();
  if (date) params.set("date", date);
  const url = `/api/reports/${type}?${params.toString()}`;

  const { query } = useCustom({ url, method: "get" });
  const data = (query.data?.data as PeriodData) || null;

  return (
    <Spin spinning={query.isLoading}>
      <Row gutter={[16, 16]} style={{ marginBottom: 16 }}>
        <Col>
          <DatePicker
            placeholder="选择日期"
            onChange={(_, ds) => setDate(ds as string | undefined)}
          />
        </Col>
      </Row>

      {data && (
        <>
          <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
            <Col span={4}><Card><Statistic title="总订单" value={data.total_orders} /></Card></Col>
            <Col span={4}><Card><Statistic title="已付订单" value={data.total_paid_orders} /></Card></Col>
            <Col span={4}><Card><Statistic title="总收入" value={data.total_revenue} prefix="¥" precision={2} /></Card></Col>
            <Col span={4}><Card><Statistic title="退款总额" value={data.total_refunded} prefix="¥" precision={2} /></Card></Col>
            <Col span={4}><Card><Statistic title="参赛" value={data.total_entries} /></Card></Col>
            <Col span={4}><Card><Statistic title="投票" value={data.total_votes} /></Card></Col>
          </Row>

          {data.top_stores.length > 0 && (
            <Card title="门店收入排行 (Top 10)">
              <Table
                rowKey="store_id"
                dataSource={data.top_stores}
                pagination={false}
                size="small"
              >
                <Table.Column dataIndex="store_id" title="门店ID" width={80} />
                <Table.Column dataIndex="store_name" title="门店名称" />
                <Table.Column dataIndex="order_count" title="订单数" width={100} />
                <Table.Column dataIndex="revenue" title="收入" width={120} render={(v: number) => `¥${v.toFixed(2)}`} />
              </Table>
            </Card>
          )}
        </>
      )}

      {!query.isLoading && !data && <Empty description="请选择日期查看报表" />}
    </Spin>
  );
};

export const ReportsPage: React.FC = () => {
  const [dateRange, setDateRange] = useState<[string, string] | null>(null);
  const [regionId, setRegionId] = useState<number | undefined>(undefined);

  const params = new URLSearchParams();
  if (dateRange) {
    params.set("start", dateRange[0]);
    params.set("end", dateRange[1]);
  }
  if (regionId) params.set("region_id", String(regionId));
  const reportUrl = `/api/reports/summary?${params.toString()}`;

  const { query: reportQuery } = useCustom({
    url: reportUrl,
    method: "get",
  });
  const reportData = (reportQuery.data?.data as any) || null;
  const loading = reportQuery.isLoading;

  const { query: reconQuery } = useCustom({
    url: "/api/reports/reconciliation",
    method: "get",
  });
  const reconData = (reconQuery.data?.data as any) || null;

  const entries = reportData?.entries ?? [];
  const votes = reportData?.votes ?? [];
  const regions = reportData?.regions ?? [];

  const reconIssues = (reconData?.payment_mismatches?.length ?? 0) + (reconData?.vote_count_drifts?.length ?? 0) + (reconData?.inventory_drifts?.length ?? 0);

  return (
    <Spin spinning={loading}>
      <div style={{ padding: 24 }}>
        <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
          <Col>
            <RangePicker onChange={(_, ds) => setDateRange(ds as [string, string] | null)} />
          </Col>
          <Col>
            <Select
              placeholder="选择赛区"
              allowClear
              style={{ width: 200 }}
              onChange={(v) => setRegionId(v)}
              options={regions.map((r: any) => ({ value: r.id, label: r.name }))}
            />
          </Col>
        </Row>

        <Tabs defaultActiveKey="overview" items={[
          {
            key: "overview",
            label: "运营概览",
            children: (
              <>
                <Row gutter={[16, 16]} style={{ marginBottom: 24 }}>
                  <Col span={6}>
                    <Card><Statistic title="总参赛" value={reportData?.total_entries ?? 0} /></Card>
                  </Col>
                  <Col span={6}>
                    <Card><Statistic title="总投票" value={reportData?.total_votes ?? 0} /></Card>
                  </Col>
                  <Col span={6}>
                    <Card><Statistic title="核销率" value={reportData?.redeem_rate ?? 0} suffix="%" /></Card>
                  </Col>
                  <Col span={6}>
                    <Card><Statistic title="付费转化率" value={reportData?.conversion_rate ?? 0} suffix="%" /></Card>
                  </Col>
                </Row>

                {entries.length > 0 && (
                  <Card title="参赛趋势" style={{ marginBottom: 24 }}>
                    <ReactEChartsCore
                      echarts={echarts}
                      option={{
                        xAxis: { type: "category", data: entries.map((e: any) => e.date) },
                        yAxis: { type: "value" },
                        series: [{ data: entries.map((e: any) => e.count), type: "line", smooth: true }],
                        tooltip: { trigger: "axis" },
                      }}
                      style={{ height: 300 }}
                    />
                  </Card>
                )}

                {votes.length > 0 && (
                  <Card title="投票趋势">
                    <ReactEChartsCore
                      echarts={echarts}
                      option={{
                        xAxis: { type: "category", data: votes.map((v: any) => v.date) },
                        yAxis: { type: "value" },
                        series: [{ data: votes.map((v: any) => v.count), type: "bar" }],
                        tooltip: { trigger: "axis" },
                      }}
                      style={{ height: 300 }}
                    />
                  </Card>
                )}
              </>
            ),
          },
          {
            key: "daily",
            label: "日报",
            children: <PeriodReportTab type="daily" />,
          },
          {
            key: "weekly",
            label: "周报",
            children: <PeriodReportTab type="weekly" />,
          },
          {
            key: "monthly",
            label: "月报",
            children: <PeriodReportTab type="monthly" />,
          },
          {
            key: "reconciliation",
            label: reconIssues > 0 ? `数据对账 (${reconIssues})` : "数据对账",
            children: (
              <Spin spinning={reconQuery.isLoading}>
                {reconIssues === 0 && !reconQuery.isLoading && (
                  <Alert type="success" message="所有数据校验通过，未发现不一致项" style={{ marginBottom: 16 }} />
                )}

                {(reconData?.payment_mismatches?.length > 0) && (
                  <Card title="支付-订单不一致" style={{ marginBottom: 16 }}>
                    <Table
                      rowKey="order_id"
                      dataSource={reconData.payment_mismatches}
                      pagination={false}
                      size="small"
                    >
                      <Table.Column dataIndex="order_id" title="订单ID" width={80} />
                      <Table.Column dataIndex="order_pay_status" title="订单支付状态" width={120} render={(v: string) => <Tag color="blue">{v}</Tag>} />
                      <Table.Column dataIndex="paid_records" title="成功支付记录数" width={140} />
                      <Table.Column dataIndex="total_paid_amount" title="已付金额" width={120} render={(v: number) => v != null ? `¥${v.toFixed(2)}` : "-"} />
                    </Table>
                  </Card>
                )}

                {(reconData?.vote_count_drifts?.length > 0) && (
                  <Card title="投票计数漂移" style={{ marginBottom: 16 }}>
                    <Table
                      rowKey="entry_id"
                      dataSource={reconData.vote_count_drifts}
                      pagination={false}
                      size="small"
                    >
                      <Table.Column dataIndex="entry_id" title="作品ID" width={80} />
                      <Table.Column dataIndex="stored_count" title="存储计数" width={100} />
                      <Table.Column dataIndex="actual_count" title="实际计数" width={100} />
                      <Table.Column dataIndex="diff" title="差异" width={80} render={(v: number) => <Tag color={v > 0 ? "orange" : "red"}>{v > 0 ? `+${v}` : v}</Tag>} />
                    </Table>
                  </Card>
                )}

                {(reconData?.inventory_drifts?.length > 0) && (
                  <Card title="库存余额漂移" style={{ marginBottom: 16 }}>
                    <Table
                      rowKey="item_id"
                      dataSource={reconData.inventory_drifts}
                      pagination={false}
                      size="small"
                    >
                      <Table.Column dataIndex="item_id" title="库存ID" width={80} />
                      <Table.Column dataIndex="stored_quantity" title="存储数量" width={100} />
                      <Table.Column dataIndex="txn_sum" title="流水合计" width={100} />
                      <Table.Column dataIndex="diff" title="差异" width={80} render={(v: number) => <Tag color={v > 0 ? "orange" : "red"}>{v > 0 ? `+${v}` : v}</Tag>} />
                    </Table>
                  </Card>
                )}

                {!reconQuery.isLoading && reconIssues === 0 && (
                  <Empty description="暂无对账数据" />
                )}
              </Spin>
            ),
          },
        ]} />
      </div>
    </Spin>
  );
};
