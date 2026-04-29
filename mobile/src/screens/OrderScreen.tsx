import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
  Alert,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { colors, spacing, borderRadius, typography } from '../theme';
import * as api from '../services/api';
import { useAuth } from '../context/AuthContext';
import { useTypedRoute } from '../hooks/useTypedRoute';

const SIZE_OPTIONS = [
  { value: '6inch', label: '6 寸' },
  { value: '8inch', label: '8 寸' },
  { value: '10inch', label: '10 寸' },
];

const CREAM_OPTIONS = [
  { value: 'animal', label: '动物奶油' },
  { value: 'vegetable', label: '植物奶油' },
  { value: 'mixed', label: '混合奶油' },
];

export function OrderScreen() {
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const route = useTypedRoute<'Order'>();
  const { regionId: userRegionId } = useAuth();
  const entryId = route.params.entryId;
  const regionId = userRegionId ?? 0;

  const [cakeSize, setCakeSize] = useState('6inch');
  const [creamType, setCreamType] = useState('animal');
  const [prices, setPrices] = useState<any[]>([]);
  const [stores, setStores] = useState<any[]>([]);
  const [storeId, setStoreId] = useState<number>(0);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    api.getPrices({ region_id: regionId }).then((res) => {
      setPrices(res.data?.list ?? res.list ?? []);
    }).catch(() => {});
  }, [regionId]);

  useEffect(() => {
    api.getStores({ region_id: regionId }).then((res) => {
      const list: any[] = res.data?.list ?? res.list ?? [];
      setStores(list);
      if (list.length > 0) {
        setStoreId(list[0].id);
      }
    }).catch(() => {});
  }, [regionId]);

  const currentPrice = prices.find(
    (p: any) => p.cake_size === cakeSize && p.cream_type === creamType && p.status === 'active'
  );
  const amount = currentPrice?.price ?? 0;

  const handleOrder = async () => {
    if (!storeId) {
      Alert.alert('提示', '附近暂无可下单门店');
      return;
    }
    if (amount <= 0) {
      Alert.alert('提示', '当前规格暂无有效价格');
      return;
    }
    setSubmitting(true);
    try {
      const result = await api.createPaidOrder({
        entry_id: entryId,
        cake_size: cakeSize,
        cream_type: creamType,
        store_id: storeId,
      });

      // If JSAPI prepay params returned, attempt to invoke WeChat Pay
      if (result.prepay_params) {
        const p = result.prepay_params;
        Alert.alert(
          '订单已创建',
          `订单号: ${result.order_id}\n金额: ¥${Number(result.amount).toFixed(2)}\n\n即将唤起微信支付`,
          [
            {
              text: '去支付',
              onPress: () => {
                // TODO: Call WeChat Pay SDK when react-native-wechat-lib is integrated
                // WeChatPaySDK.pay(p.partnerid, p.prepayid, p.noncestr, p.timestamp, p.sign_type, p.pay_sign)
                console.log('WeChat Pay params:', JSON.stringify(p));
                Alert.alert('提示', '微信支付需集成 react-native-wechat-lib 原生模块后调用');
                navigation.navigate('OrderDetail', { orderId: result.order_id });
              },
            },
            { text: '稍后支付', onPress: () => navigation.navigate('OrderDetail', { orderId: result.order_id }) },
          ],
        );
      } else if (result.prepay_id) {
        // Dev mode stub prepay_id
        Alert.alert('下单成功(开发模式)', `订单号: ${result.order_id}\n金额: ¥${Number(result.amount).toFixed(2)}`, [
          { text: '查看订单', onPress: () => navigation.navigate('OrderDetail', { orderId: result.order_id }) },
          { text: '返回', onPress: () => navigation.goBack() },
        ]);
      } else {
        Alert.alert('下单成功', `订单号: ${result.order_id}\n金额: ¥${Number(result.amount).toFixed(2)}`, [
          { text: '查看订单', onPress: () => navigation.navigate('OrderDetail', { orderId: result.order_id }) },
          { text: '返回', onPress: () => navigation.goBack() },
        ]);
      }
    } catch (e: any) {
      Alert.alert('下单失败', e?.response?.data?.message ?? e.message);
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.sectionTitle}>蛋糕尺寸</Text>
      <View style={styles.optionRow}>
        {SIZE_OPTIONS.map((opt) => (
          <TouchableOpacity
            key={opt.value}
            style={[styles.optionChip, cakeSize === opt.value && styles.optionChipActive]}
            onPress={() => setCakeSize(opt.value)}
          >
            <Text style={[styles.optionText, cakeSize === opt.value && styles.optionTextActive]}>
              {opt.label}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <Text style={styles.sectionTitle}>奶油类型</Text>
      <View style={styles.optionRow}>
        {CREAM_OPTIONS.map((opt) => (
          <TouchableOpacity
            key={opt.value}
            style={[styles.optionChip, creamType === opt.value && styles.optionChipActive]}
            onPress={() => setCreamType(opt.value)}
          >
            <Text style={[styles.optionText, creamType === opt.value && styles.optionTextActive]}>
              {opt.label}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <Text style={styles.sectionTitle}>选择门店</Text>
      {stores.length === 0 ? (
        <Text style={styles.emptyHint}>附近暂无可下单门店</Text>
      ) : (
        <View style={styles.optionRow}>
          {stores.map((s: any) => (
            <TouchableOpacity
              key={s.id}
              style={[styles.optionChip, storeId === s.id && styles.optionChipActive]}
              onPress={() => setStoreId(s.id)}
            >
              <Text style={[styles.optionText, storeId === s.id && styles.optionTextActive]}>
                {s.name}
              </Text>
            </TouchableOpacity>
          ))}
        </View>
      )}

      <View style={styles.priceCard}>
        <Text style={styles.priceLabel}>价格</Text>
        <Text style={styles.priceValue}>{amount > 0 ? `¥${Number(amount).toFixed(2)}` : '暂无报价'}</Text>
      </View>

      <TouchableOpacity
        style={[styles.orderButton, (submitting || amount <= 0 || !storeId) && styles.orderButtonDisabled]}
        onPress={handleOrder}
        disabled={submitting || amount <= 0 || !storeId}
      >
        {submitting ? (
          <ActivityIndicator color={colors.surface} />
        ) : (
          <Text style={styles.orderButtonText}>确认下单</Text>
        )}
      </TouchableOpacity>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  sectionTitle: {
    ...typography.heading,
    color: colors.textPrimary,
    marginTop: spacing.lg,
    marginBottom: spacing.md,
  },
  optionRow: {
    flexDirection: 'row',
    gap: spacing.md,
  },
  optionChip: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    borderWidth: 1,
    borderColor: colors.divider,
  },
  optionChipActive: {
    borderColor: colors.primary,
    backgroundColor: colors.primary + '15',
  },
  optionText: {
    ...typography.body,
    color: colors.textSecondary,
  },
  optionTextActive: {
    color: colors.primary,
    fontWeight: '600',
  },
  priceCard: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginTop: spacing.xl,
  },
  priceLabel: {
    ...typography.body,
    color: colors.textSecondary,
  },
  priceValue: {
    ...typography.heading,
    color: colors.primary,
  },
  orderButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginTop: spacing.xl,
    marginBottom: spacing.xxl,
  },
  orderButtonDisabled: {
    opacity: 0.5,
  },
  orderButtonText: {
    ...typography.button,
    color: colors.surface,
  },
  emptyHint: {
    ...typography.body,
    color: colors.textSecondary,
    marginTop: spacing.sm,
  },
});
