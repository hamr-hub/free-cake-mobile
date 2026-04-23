import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  Image,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
} from 'react-native';
import { useVote } from '../hooks/useVote';
import { VoteButton } from '../components/VoteButton';
import { RankBadge } from '../components/RankBadge';
import { RiskTag } from '../components/RiskTag';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatVoteCount, formatDate } from '../utils/formatters';
import { getRankList } from '../services/api';
import { RankedEntry } from '../types/entry';

interface DetailScreenProps {
  route?: { params?: { entryId: number } };
  navigation?: any;
}

export function DetailScreen({ route, navigation }: DetailScreenProps) {
  const entryId = route?.params?.entryId ?? 0;
  const { cast, voteRestriction, isLoading: voteLoading, error: voteError } = useVote();

  const handleVote = async () => {
    const result = await cast(entryId, 0);
    if (result) {
      // refresh rank
    }
  };

  const handleShare = () => {
    // Share poster
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.imageSection}>
        <View style={styles.imagePlaceholder}>
          <Text style={styles.imagePlaceholderText}>蛋糕设计大图</Text>
        </View>
      </View>

      <View style={styles.infoSection}>
        <View style={styles.titleRow}>
          <Text style={styles.title}>作品标题</Text>
          <RankBadge rank={1} />
        </View>
        <Text style={styles.voteCount}>{formatVoteCount(0)} 有效票</Text>
        <Text style={styles.time}>参赛时间: {formatDate(new Date().toISOString())}</Text>
      </View>

      {voteError && <Text style={styles.errorText}>{voteError}</Text>}

      <VoteButton
        remainingVotes={voteRestriction.remaining_votes}
        isLoading={voteLoading}
        isFrozen={voteRestriction.is_frozen}
        freezeReason={voteRestriction.freeze_reason}
        onPress={handleVote}
      />

      <TouchableOpacity style={styles.shareButton} onPress={handleShare}>
        <Text style={styles.shareButtonText}>分享拉票</Text>
      </TouchableOpacity>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  imageSection: {
    height: 300,
    overflow: 'hidden',
  },
  imagePlaceholder: {
    flex: 1,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
  },
  imagePlaceholderText: {
    ...typography.title,
    color: colors.textPrimary,
  },
  infoSection: {
    padding: spacing.xl,
    backgroundColor: colors.surface,
  },
  titleRow: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
  },
  title: {
    ...typography.heading,
    color: colors.textPrimary,
    flex: 1,
  },
  voteCount: {
    ...typography.body,
    color: colors.textSecondary,
    marginTop: spacing.sm,
  },
  time: {
    ...typography.caption,
    color: colors.textHint,
    marginTop: spacing.xs,
  },
  errorText: {
    color: colors.danger,
    fontSize: 13,
    textAlign: 'center',
    padding: spacing.md,
  },
  shareButton: {
    backgroundColor: colors.accent,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginHorizontal: spacing.xl,
    marginTop: spacing.md,
  },
  shareButtonText: {
    ...typography.button,
    color: colors.surface,
  },
});
