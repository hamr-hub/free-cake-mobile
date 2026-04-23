import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

interface RiskTagProps {
  level: 'low' | 'medium' | 'high';
  label: string;
}

const levelConfig = {
  low: { bg: colors.success, text: colors.surface },
  medium: { bg: colors.warning, text: colors.textPrimary },
  high: { bg: colors.danger, text: colors.surface },
};

export function RiskTag({ level, label }: RiskTagProps) {
  const config = levelConfig[level];
  return (
    <View style={[styles.tag, { backgroundColor: config.bg }]}>
      <Text style={[styles.text, { color: config.text }]}>{label}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  tag: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.xs,
    borderRadius: borderRadius.sm,
    alignSelf: 'flex-start',
  },
  text: {
    ...typography.caption,
    fontWeight: '600',
  },
});
