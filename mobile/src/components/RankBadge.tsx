import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { colors } from '../theme';
import { spacing, borderRadius } from '../theme';
import { formatRank } from '../utils/formatters';

interface RankBadgeProps {
  rank: number;
}

export function RankBadge({ rank }: RankBadgeProps) {
  const isTop3 = rank <= 3;
  const backgroundColor = isTop3 ? colors.topTag : colors.divider;
  const textColor = isTop3 ? colors.textPrimary : colors.textSecondary;

  return (
    <View style={[styles.badge, { backgroundColor }]}>
      <Text style={[styles.text, { color: textColor }]}>
        {formatRank(rank)}
      </Text>
    </View>
  );
}

const styles = StyleSheet.create({
  badge: {
    paddingHorizontal: spacing.sm,
    paddingVertical: spacing.xs,
    borderRadius: borderRadius.sm,
    minWidth: 32,
    alignItems: 'center',
  },
  text: {
    fontSize: 12,
    fontWeight: '700',
  },
});
