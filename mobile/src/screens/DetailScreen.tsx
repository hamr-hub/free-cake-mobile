import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  Image,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
  Alert,
  Share,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { useVote } from '../hooks/useVote';
import { useTypedRoute } from '../hooks/useTypedRoute';
import { VoteButton } from '../components/VoteButton';
import { RankBadge } from '../components/RankBadge';
import { RiskTag } from '../components/RiskTag';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatVoteCount, formatDate } from '../utils/formatters';
import { buildEntryDeepLink } from '../utils/constants';
import * as api from '../services/api';

export function DetailScreen() {
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const route = useTypedRoute<'Detail'>();
  const entryId = route.params.entryId;
  const [entry, setEntry] = useState<any>(null);
  const [entryLoading, setEntryLoading] = useState(true);
  const [myRank, setMyRank] = useState<number | null>(null);
  const { cast, voteRestriction, isLoading: voteLoading, error: voteError } = useVote(entry?.activity_id);

  useEffect(() => {
    if (entryId) {
      setEntryLoading(true);
      api.getEntryDetail(entryId)
        .then((data) => {
          setEntry(data?.data ?? data ?? null);
        })
        .catch(() => setEntry(null))
        .finally(() => setEntryLoading(false));
    }
  }, [entryId]);

  const handleVote = async () => {
    const result = await cast(entryId, entry?.activity_id ?? 0);
    if (result) {
      setEntry((prev: any) => prev ? { ...prev, valid_vote_count: prev.valid_vote_count + 1 } : prev);
      const rankData = await api.getRankList(entry?.activity_id ?? 0, 1, 200);
      const myEntry = rankData.entries?.find((e: any) => e.id === entryId);
      if (myEntry) {
        setEntry((prev: any) => prev ? { ...prev, rank: myEntry.rank, valid_vote_count: myEntry.valid_vote_count } : prev);
      }
    }
  };

  const handleShare = async () => {
    if (!entry) return;
    const deepLink = buildEntryDeepLink(entry.id);
    try {
      await Share.share({
        title: entry.title || '我的蛋糕设计',
        message: `快来为我的蛋糕设计投票！作品：${entry.title || '我的蛋糕'}，当前排名 #${entry.rank || '-'}, 得票 ${entry.valid_vote_count || 0}\n${deepLink}`,
        url: deepLink,
      });
    } catch {}
  };

  if (entryLoading) {
    return (
      <View style={styles.loadingContainer}>
        <ActivityIndicator size="large" color={colors.primary} />
      </View>
    );
  }

  if (!entry) {
    return (
      <View style={styles.loadingContainer}>
        <Text style={styles.errorText}>作品不存在或已删除</Text>
        <TouchableOpacity onPress={() => navigation.goBack()}>
          <Text style={styles.backText}>返回</Text>
        </TouchableOpacity>
      </View>
    );
  }

  return (
    <ScrollView style={styles.container}>
      {entry.image_url ? (
        <Image source={{ uri: entry.image_url }} style={styles.entryImage} resizeMode="cover" />
      ) : (
        <View style={styles.imagePlaceholder}>
          <Text style={styles.imagePlaceholderText}>蛋糕设计大图</Text>
        </View>
      )}

      <View style={styles.infoSection}>
        <View style={styles.titleRow}>
          <Text style={styles.title}>{entry.title || `作品 #${entry.id}`}</Text>
          {entry.rank != null && <RankBadge rank={entry.rank} />}
        </View>
        <View style={styles.metaRow}>
          <Text style={styles.voteCount}>{formatVoteCount(entry.valid_vote_count ?? 0)} 有效票</Text>
          {entry.image_url && <RiskTag level="medium" label="AI生成" />}
        </View>
        <Text style={styles.time}>参赛时间: {formatDate(entry.created_at)}</Text>
        {entry.user_name && <Text style={styles.author}>作者: {entry.user_name}</Text>}
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

      {!entry.is_winner && (
        <TouchableOpacity style={styles.orderButton} onPress={() => navigation.navigate('Order', { entryId: entry.id })}>
          <Text style={styles.orderButtonText}>我要同款蛋糕</Text>
        </TouchableOpacity>
      )}

      {entry.is_winner && (
        <View style={styles.winnerBanner}>
          <Text style={styles.winnerText}>恭喜获奖！请前往领奖页面查看核销码</Text>
          <TouchableOpacity onPress={() => navigation.navigate('Redeem', { code: '' })}>
            <Text style={styles.winnerLink}>查看领奖码</Text>
          </TouchableOpacity>
        </View>
      )}
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  loadingContainer: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: colors.background,
  },
  entryImage: {
    width: '100%',
    height: 300,
    resizeMode: 'cover',
  },
  imagePlaceholder: {
    height: 300,
    backgroundColor: colors.primary + '20',
    justifyContent: 'center',
    alignItems: 'center',
  },
  imagePlaceholderText: {
    ...typography.title,
    color: colors.textHint,
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
  metaRow: {
    flexDirection: 'row',
    alignItems: 'center',
    marginTop: spacing.sm,
    gap: spacing.md,
  },
  voteCount: {
    ...typography.body,
    color: colors.textSecondary,
  },
  time: {
    ...typography.caption,
    color: colors.textHint,
    marginTop: spacing.xs,
  },
  author: {
    ...typography.caption,
    color: colors.textSecondary,
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
  winnerBanner: {
    backgroundColor: colors.freeTag + '20',
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginHorizontal: spacing.xl,
    marginTop: spacing.md,
    marginBottom: spacing.xxl,
    alignItems: 'center',
  },
  orderButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginHorizontal: spacing.xl,
    marginTop: spacing.md,
  },
  orderButtonText: {
    ...typography.button,
    color: colors.surface,
  },
  winnerText: {
    ...typography.body,
    color: colors.freeTag,
    textAlign: 'center',
  },
  winnerLink: {
    ...typography.button,
    color: colors.primary,
    marginTop: spacing.sm,
  },
  backText: {
    ...typography.body,
    color: colors.primary,
    marginTop: spacing.md,
  },
});
