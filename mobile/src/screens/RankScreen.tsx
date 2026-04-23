import React, { useEffect, useState } from 'react';
import { View, Text, FlatList, TouchableOpacity, StyleSheet, RefreshControl } from 'react-native';
import { useAuth } from '../context/AuthContext';
import { getRankList } from '../services/api';
import { RankedEntry } from '../types/entry';
import { CakeCard } from '../components/CakeCard';
import { RegionGuard } from '../components/RegionGuard';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatRank } from '../utils/formatters';

export function RankScreen() {
  const { regionId } = useAuth();
  const [entries, setEntries] = useState<RankedEntry[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [page, setPage] = useState(1);
  const [total, setTotal] = useState(0);

  const fetchData = async (p: number = 1) => {
    setIsLoading(true);
    try {
      const data = await getRankList(0, p, 20);
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

  useEffect(() => {
    if (regionId) {
      fetchData(1);
    }
  }, [regionId]);

  const renderItem = ({ item }: { item: RankedEntry }) => (
    <CakeCard
      title={item.title}
      imageUrl={item.image_url}
      voteCount={item.valid_vote_count}
      rank={item.rank}
      onPress={() => {}}
    />
  );

  return (
    <RegionGuard>
      <View style={styles.container}>
        <View style={styles.header}>
          <Text style={styles.headerTitle}>排行榜</Text>
          <Text style={styles.headerInfo}>有效票排序 · 共 {total} 个作品</Text>
        </View>
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
});
