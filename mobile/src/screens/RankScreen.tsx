import React, { useEffect, useState } from 'react';
import { View, Text, FlatList, TouchableOpacity, StyleSheet, RefreshControl, Modal, ScrollView } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { useAuth } from '../context/AuthContext';
import { useActivity } from '../hooks/useActivity';
import { getRankList } from '../services/api';
import { RankedEntry } from '../types/entry';
import { CakeCard } from '../components/CakeCard';
import { RegionGuard } from '../components/RegionGuard';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

export function RankScreen() {
  const { regionId, userId } = useAuth();
  const { currentActivity } = useActivity();
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const [entries, setEntries] = useState<RankedEntry[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);
  const [myRankInfo, setMyRankInfo] = useState<{ rank: number; voteCount: number } | null>(null);
  const [showMyRank, setShowMyRank] = useState(false);
  const [showVoteInfo, setShowVoteInfo] = useState(false);

  const activityId = currentActivity?.id ?? 0;

  const fetchData = async (p: number = 1) => {
    setIsLoading(true);
    try {
      const data = await getRankList(activityId, p, 20);
      if (p === 1) {
        setEntries(data.entries ?? []);
      } else {
        setEntries((prev) => [...prev, ...(data.entries ?? [])]);
      }
      setTotal(data.total ?? 0);
      setPage(p);
    } catch {
    } finally {
      setIsLoading(false);
    }
  };

  const fetchMyRank = async () => {
    if (!activityId || !userId) return;
    try {
      const allData = await getRankList(activityId, 1, 200);
      const myEntry = allData.entries?.find((e: any) => e.user_id === userId);
      if (myEntry) {
        setMyRankInfo({ rank: myEntry.rank, voteCount: myEntry.valid_vote_count });
        setShowMyRank(true);
      }
    } catch {}
  };

  useEffect(() => {
    if (regionId && activityId) {
      fetchData(1);
    }
  }, [regionId, activityId]);

  const handleEntryPress = (entryId: number) => {
    navigation.navigate('Detail', { entryId });
  };

  const renderItem = ({ item }: { item: RankedEntry }) => (
    <CakeCard
      title={item.title}
      imageUrl={item.image_url}
      voteCount={item.valid_vote_count}
      rank={item.rank}
      onPress={() => handleEntryPress(item.id)}
    />
  );

  return (
    <RegionGuard>
      <View style={styles.container}>
        <View style={styles.header}>
          <Text style={styles.headerTitle}>排行榜</Text>
          <TouchableOpacity onPress={() => setShowVoteInfo(true)}>
            <Text style={styles.headerInfo}>有效票排序 · 共 {total} 个作品 ⓘ</Text>
          </TouchableOpacity>
        </View>

        {myRankInfo && (
          <TouchableOpacity style={styles.myRankBanner} onPress={() => setShowMyRank(true)}>
            <Text style={styles.myRankText}>我的排名 #{myRankInfo.rank} · {myRankInfo.voteCount} 票</Text>
          </TouchableOpacity>
        )}

        <FlatList
          data={entries}
          renderItem={renderItem}
          keyExtractor={(item) => item.id.toString()}
          numColumns={2}
          refreshControl={
            <RefreshControl refreshing={isLoading} onRefresh={() => fetchData(1)} colors={[colors.primary]} />
          }
          onEndReached={() => fetchData(page + 1)}
          onEndReachedThreshold={0.5}
          contentContainerStyle={styles.list}
          ListEmptyComponent={
            <View style={styles.empty}>
              <Text style={styles.emptyText}>暂无作品</Text>
            </View>
          }
        />

        <TouchableOpacity style={styles.myRankButton} onPress={fetchMyRank}>
          <Text style={styles.myRankButtonText}>查看我的排名</Text>
        </TouchableOpacity>

        <Modal visible={showMyRank} transparent animationType="fade" onRequestClose={() => setShowMyRank(false)}>
          <TouchableOpacity style={styles.modalOverlay} activeOpacity={1} onPress={() => setShowMyRank(false)}>
            <View style={styles.modalContent}>
              <Text style={styles.modalTitle}>我的排名</Text>
              {myRankInfo ? (
                <>
                  <Text style={styles.modalRank}>#{myRankInfo.rank}</Text>
                  <Text style={styles.modalVotes}>{myRankInfo.voteCount} 有效票</Text>
                  {myRankInfo.rank <= 100 ? (
                    <Text style={styles.modalWinner}>恭喜！你在 Top100 内</Text>
                  ) : (
                    <Text style={styles.modalHint}>继续努力拉票，争取进入 Top100</Text>
                  )}
                </>
              ) : (
                <Text style={styles.modalHint}>暂无排名数据</Text>
              )}
            </View>
          </TouchableOpacity>
        </Modal>

        <Modal visible={showVoteInfo} transparent animationType="fade" onRequestClose={() => setShowVoteInfo(false)}>
          <TouchableOpacity style={styles.modalOverlay} activeOpacity={1} onPress={() => setShowVoteInfo(false)}>
            <View style={styles.modalContent}>
              <Text style={styles.modalTitle}>有效票说明</Text>
              <ScrollView>
                <Text style={styles.modalBody}>排行榜按"有效票数"排序。有效票是指：</Text>
                <Text style={styles.modalBody}>1. 经过风控系统验证的合法投票</Text>
                <Text style={styles.modalBody}>2. 非同一IP重复刷票</Text>
                <Text style={styles.modalBody}>3. 非同一设备重复投票</Text>
                <Text style={styles.modalBody}>4. 来自本赛区范围内的用户</Text>
                <Text style={styles.modalBody}>5. 数据每3-10秒刷新一次</Text>
                <Text style={styles.modalHint}>排名数据仅供参考，最终获奖名单以结算结果为准</Text>
              </ScrollView>
            </View>
          </TouchableOpacity>
        </Modal>
      </View>
    </RegionGuard>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  header: {
    padding: spacing.xl,
    backgroundColor: colors.surface,
  },
  headerTitle: {
    ...typography.heading,
    color: colors.textPrimary,
  },
  headerInfo: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  myRankBanner: {
    backgroundColor: colors.freeTag + '20',
    padding: spacing.md,
    alignItems: 'center',
  },
  myRankText: {
    ...typography.body,
    color: colors.freeTag,
    fontWeight: '600',
  },
  list: {
    padding: spacing.xl,
  },
  empty: {
    alignItems: 'center',
    paddingVertical: spacing.xxxl,
  },
  emptyText: {
    ...typography.body,
    color: colors.textHint,
  },
  myRankButton: {
    position: 'absolute',
    bottom: spacing.xl,
    right: spacing.xl,
    backgroundColor: colors.primary,
    borderRadius: borderRadius.xl,
    paddingVertical: spacing.md,
    paddingHorizontal: spacing.xl,
    elevation: 4,
  },
  myRankButtonText: {
    ...typography.button,
    color: colors.textPrimary,
    fontSize: 13,
  },
  modalOverlay: {
    flex: 1,
    backgroundColor: 'rgba(0,0,0,0.5)',
    justifyContent: 'center',
    alignItems: 'center',
  },
  modalContent: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.xl,
    padding: spacing.xxl,
    width: '80%',
    maxHeight: '70%',
  },
  modalTitle: {
    ...typography.heading,
    color: colors.textPrimary,
    marginBottom: spacing.xl,
  },
  modalRank: {
    fontSize: 36,
    fontWeight: '700',
    color: colors.freeTag,
    textAlign: 'center',
  },
  modalVotes: {
    ...typography.body,
    color: colors.textSecondary,
    textAlign: 'center',
    marginTop: spacing.md,
  },
  modalWinner: {
    ...typography.body,
    color: colors.freeTag,
    textAlign: 'center',
    marginTop: spacing.md,
    fontWeight: '600',
  },
  modalHint: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
    marginTop: spacing.md,
  },
  modalBody: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.sm,
    lineHeight: 22,
  },
});
