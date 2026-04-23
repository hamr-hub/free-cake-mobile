import React from 'react';
import { View, Text, TouchableOpacity, StyleSheet } from 'react-native';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

interface VoteButtonProps {
  remainingVotes: number;
  isLoading: boolean;
  isFrozen: boolean;
  freezeReason: string | null;
  onPress: () => void;
}

export function VoteButton({ remainingVotes, isLoading, isFrozen, freezeReason, onPress }: VoteButtonProps) {
  const isDisabled = remainingVotes <= 0 || isLoading || isFrozen;

  return (
    <View style={styles.container}>
      <TouchableOpacity
        style={[styles.button, isDisabled && styles.buttonDisabled]}
        onPress={onPress}
        disabled={isDisabled}
        activeOpacity={0.7}
      >
        <Text style={styles.buttonText}>
          {isLoading ? '投票中...' : isFrozen ? '已冻结' : '投票'}
        </Text>
      </TouchableOpacity>
      <Text style={styles.remainingText}>
        今日剩余 {remainingVotes} 票
      </Text>
      {isFrozen && freezeReason && (
        <Text style={styles.freezeText}>{freezeReason}</Text>
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    alignItems: 'center',
  },
  button: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    paddingVertical: spacing.xl,
    paddingHorizontal: spacing.xxxl,
    elevation: 2,
  },
  buttonDisabled: {
    backgroundColor: colors.disabled,
    elevation: 0,
  },
  buttonText: {
    ...typography.button,
    color: colors.textPrimary,
  },
  remainingText: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.sm,
  },
  freezeText: {
    ...typography.caption,
    color: colors.danger,
    marginTop: spacing.xs,
  },
});
