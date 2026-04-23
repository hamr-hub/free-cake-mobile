import React, { useEffect, useState } from 'react';
import { View, Text, ScrollView, StyleSheet, ActivityIndicator } from 'react-native';
import { QRCodeCard } from '../components/QRCodeCard';
import { RegionGuard } from '../components/RegionGuard';
import { useAuth } from '../context/AuthContext';
import { getUserProfile } from '../services/api';
import { RedeemCode } from '../types/redeem';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { storage } from '../services/storage';
import { formatPhone } from '../utils/formatters';

interface RedeemScreenProps {
  route?: { params?: { code?: string } };
  navigation?: any;
}

export function RedeemScreen({ route, navigation }: RedeemScreenProps) {
  const { userId } = useAuth();
  const [redeemCodes, setRedeemCodes] = useState<RedeemCode[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    fetchRedeemCodes();
  }, []);

  const fetchRedeemCodes = async () => {
    setIsLoading(true);
    try {
      const data = await getUserProfile();
      const codes = data.redeem_codes ?? [];
      setRedeemCodes(codes);
      storage.cacheRedeemCodes(JSON.stringify(codes));
    } catch {
      const cached = storage.getCachedRedeemCodes();
      if (cached) {
        setRedeemCodes(JSON.parse(cached));
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <RegionGuard>
      <ScrollView style={styles.container}>
        <Text style={styles.title}>我的领奖码</Text>

        {isLoading && <ActivityIndicator color={colors.primary} style={styles.loader} />}

        {redeemCodes.length === 0 && !isLoading && (
          <View style={styles.empty}>
            <Text style={styles.emptyText}>暂无领奖码</Text>
            <Text style={styles.emptyHint}>参与投票活动，进入Top100即可获得免费蛋糕！</Text>
          </View>
        )}

        {redeemCodes.map((code) => (
          <QRCodeCard key={code.code} redeemCode={code} />
        ))}

        <View style={styles.notice}>
          <Text style={styles.noticeTitle}>领取须知</Text>
          <Text style={styles.noticeItem}>1. 仅限获奖本人到店核销</Text>
          <Text style={styles.noticeItem}>2. 凭手机号 + 核销码领取</Text>
          <Text style={styles.noticeItem}>3. 请在有效期内领取，过期失效</Text>
          <Text style={styles.noticeItem}>4. 免费 6寸动物奶油蛋糕，不可定制修改</Text>
        </View>
      </ScrollView>
    </RegionGuard>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  title: {
    ...typography.heading,
    color: colors.textPrimary,
    marginBottom: spacing.xl,
  },
  loader: {
    marginVertical: spacing.xxxl,
  },
  empty: {
    alignItems: 'center',
    paddingVertical: spacing.xxxl,
  },
  emptyText: {
    ...typography.title,
    color: colors.textHint,
  },
  emptyHint: {
    ...typography.caption,
    color: colors.textSecondary,
    textAlign: 'center',
    marginTop: spacing.md,
  },
  notice: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginTop: spacing.xxl,
  },
  noticeTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  noticeItem: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.sm,
  },
});
