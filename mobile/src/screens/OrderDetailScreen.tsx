import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  ActivityIndicator,
  TouchableOpacity,
  Alert,
} from 'react-native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { colors, spacing, borderRadius, typography } from '../theme';
import * as api from '../services/api';
import { useTypedRoute } from '../hooks/useTypedRoute';
import { OrderDetail } from '../types/order';
import { formatDate } from '../utils/formatters';

const payStatusLabel: Record<string, string> = {
  free: '免费',
  pending: '待支付',
  paid: '已支付',
  closed: '已关闭',
  refunded: '已退款',
};

export function OrderDetailScreen() {
  const route = useTypedRoute<'OrderDetail'>();
  const orderId = route.params.orderId;
  const [order, setOrder] = useState<OrderDetail | null>(null);
  const [loading, setLoading] = useState(true);
  const [paying, setPaying] = useState(false);

  const handleInitPay = async () => {
    if (!order) return;
    setPaying(true);
    try {
      const res = await api.initPay(order.id);
      const prepayParams = res.data ?? res;
      Alert.alert('支付参数已获取', '请在微信中完成支付', [
        { text: '取消', style: 'cancel' },
        {
          text: '确认',
          onPress: () => {
            api.getOrderDetail(order.id)
              .then((r) => setOrder(r.data ?? r))
              .catch(() => {});
          },
        },
      ]);
    } catch (e: any) {
      Alert.alert('发起支付失败', e?.message ?? '请稍后重试');
    } finally {
      setPaying(false);
    }
  };

  useEffect(() => {
    if (orderId) {
      api.getOrderDetail(orderId)
        .then((res) => setOrder(res.data ?? res))
        .catch(() => setOrder(null))
        .finally(() => setLoading(false));
    }
  }, [orderId]);

  if (loading) {
    return (
      <View style={styles.center}>
        <ActivityIndicator size="large" color={colors.primary} />
      </View>
    );
  }

  if (!order) {
    return (
      <View style={styles.center}>
        <Text style={styles.errorText}>订单不存在</Text>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container}>
      <View style={styles.card}>
        <Text style={styles.cardTitle}>订单信息</Text>
        <View style={styles.row}>
          <Text style={styles.label}>订单号</Text>
          <Text style={styles.value}>#{order.id}</Text>
        </View>
        <View style={styles.row}>
          <Text style={styles.label}>类型</Text>
          <Text style={styles.value}>{order.order_type === 'paid' ? '付费' : '免费'}</Text>
        </View>
        {order.order_type === 'paid' && (
          <>
            <View style={styles.row}>
              <Text style={styles.label}>金额</Text>
              <Text style={styles.value}>¥{Number(order.amount ?? 0).toFixed(2)}</Text>
            </View>
            <View style={styles.row}>
              <Text style={styles.label}>支付状态</Text>
              <Text style={styles.value}>{payStatusLabel[order.pay_status] ?? order.pay_status ?? '-'}</Text>
            </View>
          </>
        )}
        <View style={styles.row}>
          <Text style={styles.label}>生产状态</Text>
          <Text style={styles.value}>{order.production_status ?? '-'}</Text>
        </View>
        <View style={styles.row}>
          <Text style={styles.label}>核销状态</Text>
          <Text style={styles.value}>{order.redeem_status ?? '-'}</Text>
        </View>
        <View style={styles.row}>
          <Text style={styles.label}>门店</Text>
          <Text style={styles.value}>#{order.store_id}</Text>
        </View>
        <View style={styles.row}>
          <Text style={styles.label}>创建时间</Text>
          <Text style={styles.value}>{formatDate(order.created_at)}</Text>
        </View>
        {order.paid_at && (
          <View style={styles.row}>
            <Text style={styles.label}>支付时间</Text>
            <Text style={styles.value}>{formatDate(order.paid_at)}</Text>
          </View>
        )}
      </View>

      {order.order_type === 'paid' && order.pay_status === 'pending' && (
        <View style={styles.card}>
          <TouchableOpacity
            style={[styles.payButton, paying && styles.payButtonDisabled]}
            onPress={handleInitPay}
            disabled={paying}
          >
            {paying ? (
              <ActivityIndicator size="small" color={colors.surface} />
            ) : (
              <Text style={styles.payButtonText}>发起支付</Text>
            )}
          </TouchableOpacity>
          <Text style={styles.payHint}>点击后将跳转微信完成支付</Text>
        </View>
      )}

      {order.redeem_code && (
        <View style={styles.card}>
          <Text style={styles.cardTitle}>核销码</Text>
          <Text style={styles.redeemCode}>{order.redeem_code}</Text>
          <Text style={styles.hint}>请到门店出示此码领取蛋糕</Text>
        </View>
      )}

      {order.refund_status && (
        <View style={styles.card}>
          <Text style={styles.cardTitle}>退款信息</Text>
          <View style={styles.row}>
            <Text style={styles.label}>退款状态</Text>
            <Text style={[styles.value, styles.refundText]}>{order.refund_status}</Text>
          </View>
          {order.refund_reason && (
            <View style={styles.row}>
              <Text style={styles.label}>退款原因</Text>
              <Text style={styles.value}>{order.refund_reason}</Text>
            </View>
          )}
        </View>
      )}
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  center: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: colors.background,
  },
  card: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginBottom: spacing.lg,
  },
  cardTitle: {
    ...typography.heading,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  row: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: spacing.sm,
  },
  label: {
    ...typography.body,
    color: colors.textSecondary,
  },
  value: {
    ...typography.body,
    color: colors.textPrimary,
  },
  redeemCode: {
    ...typography.heading,
    color: colors.primary,
    textAlign: 'center',
    fontSize: 28,
    letterSpacing: 4,
    marginVertical: spacing.md,
  },
  hint: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
  },
  refundText: {
    color: colors.danger,
  },
  errorText: {
    ...typography.body,
    color: colors.danger,
  },
  payButton: {
    backgroundColor: colors.primaryDark,
    borderRadius: borderRadius.lg,
    paddingVertical: spacing.md,
    alignItems: 'center',
    marginBottom: spacing.sm,
  },
  payButtonDisabled: {
    backgroundColor: colors.disabled,
  },
  payButtonText: {
    ...typography.heading,
    color: colors.surface,
    fontSize: 18,
  },
  payHint: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
  },
});
