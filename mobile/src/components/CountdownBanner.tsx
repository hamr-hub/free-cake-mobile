import React, { useState, useEffect } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { formatCountdown } from '../utils/formatters';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

interface CountdownBannerProps {
  title: string;
  endTime: string;
  regionName: string;
}

export function CountdownBanner({ title, endTime, regionName }: CountdownBannerProps) {
  const [remaining, setRemaining] = useState(0);

  useEffect(() => {
    const endMs = new Date(endTime).getTime();
    const update = () => setRemaining(Math.max(0, endMs - Date.now()));
    update();
    const timer = setInterval(update, 1000);
    return () => clearInterval(timer);
  }, [endTime]);

  return (
    <View style={styles.container}>
      <View style={styles.headerRow}>
        <Text style={styles.title}>{title}</Text>
        <Text style={styles.region}>{regionName}</Text>
      </View>
      <View style={styles.countdownRow}>
        <Text style={styles.countdownLabel}>距投票结束：</Text>
        <Text style={styles.countdownValue}>{formatCountdown(remaining)}</Text>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    backgroundColor: colors.primary,
    padding: spacing.xl,
    borderBottomLeftRadius: borderRadius.xl,
    borderBottomRightRadius: borderRadius.xl,
  },
  headerRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  title: {
    ...typography.title,
    color: colors.textPrimary,
  },
  region: {
    ...typography.caption,
    color: colors.brownDark,
    backgroundColor: colors.surface,
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.xs,
    borderRadius: borderRadius.sm,
  },
  countdownRow: {
    flexDirection: 'row',
    alignItems: 'center',
  },
  countdownLabel: {
    ...typography.caption,
    color: colors.textSecondary,
  },
  countdownValue: {
    ...typography.number,
    fontSize: 18,
    color: colors.brownDark,
  },
});
