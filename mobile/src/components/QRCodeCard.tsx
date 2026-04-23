import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import QRCode from 'react-native-qrcode-svg';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatDate, formatDistance } from '../utils/formatters';
import { RedeemCode } from '../types/redeem';

interface QRCodeCardProps {
  redeemCode: RedeemCode;
}

export function QRCodeCard({ redeemCode }: QRCodeCardProps) {
  const isExpired = redeemCode.status === 'expired';
  const isUsed = redeemCode.status === 'used';

  return (
    <View style={[styles.card, isExpired && styles.cardExpired, isUsed && styles.cardUsed]}>
      <View style={styles.qrContainer}>
        <QRCode
          value={redeemCode.code}
          size={180}
          color={isExpired || isUsed ? colors.disabled : colors.brownDark}
          backgroundColor={colors.surface}
        />
      </View>
      <View style={styles.info}>
        <Text style={styles.code}>{redeemCode.code}</Text>
        <Text style={styles.cakeName}>{redeemCode.cake_name}</Text>
        <Text style={styles.storeAddress}>
          领取地点: {redeemCode.store_address}
        </Text>
        <Text style={styles.distance}>
          距您 {formatDistance(redeemCode.store_distance)}
        </Text>
        <Text style={styles.expire}>
          领取截止: {formatDate(redeemCode.expire_at)}
        </Text>
        {isExpired && <Text style={styles.statusExpired}>已过期</Text>}
        {isUsed && <Text style={styles.statusUsed}>已核销</Text>}
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  card: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.xl,
    padding: spacing.xl,
    marginVertical: spacing.xl,
  },
  cardExpired: {
    opacity: 0.6,
  },
  cardUsed: {
    opacity: 0.6,
  },
  qrContainer: {
    alignItems: 'center',
    paddingVertical: spacing.xl,
  },
  info: {
    marginTop: spacing.md,
  },
  code: {
    ...typography.body,
    color: colors.textPrimary,
    textAlign: 'center',
    fontWeight: '600',
  },
  cakeName: {
    ...typography.title,
    color: colors.textPrimary,
    textAlign: 'center',
    marginTop: spacing.sm,
  },
  storeAddress: {
    ...typography.body,
    color: colors.textSecondary,
    marginTop: spacing.md,
  },
  distance: {
    ...typography.caption,
    color: colors.textHint,
  },
  expire: {
    ...typography.caption,
    color: colors.warning,
    marginTop: spacing.sm,
  },
  statusExpired: {
    ...typography.body,
    color: colors.danger,
    fontWeight: '700',
    marginTop: spacing.md,
    textAlign: 'center',
  },
  statusUsed: {
    ...typography.body,
    color: colors.success,
    fontWeight: '700',
    marginTop: spacing.md,
    textAlign: 'center',
  },
});
