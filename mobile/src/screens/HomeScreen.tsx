import React, { useEffect } from 'react';
import { View, Text, ScrollView, TouchableOpacity, StyleSheet, RefreshControl, Image } from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../context/AuthContext';
import { useActivity } from '../hooks/useActivity';
import { useNetwork } from '../hooks/useNetwork';
import { RegionGuard } from '../components/RegionGuard';
import { CountdownBanner } from '../components/CountdownBanner';
import { CakeCard } from '../components/CakeCard';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import * as api from '../services/api';
import { RankedEntry } from '../types/entry';

export function HomeScreen() {
  const { regionId } = useAuth();
  const { currentActivity, isLoading, fetchCurrentActivity, fetchActivities, activities } = useActivity();
  const { isOffline } = useNetwork();
  const navigation = useNavigation<any>();
  const [hotEntries, setHotEntries] = React.useState<RankedEntry[]>([]);
  const [hotLoading, setHotLoading] = React.useState(false);

  useEffect(() => {
    if (regionId) {
      fetchCurrentActivity();
      fetchActivities();
    }
  }, [regionId]);

  useEffect(() => {
    if (currentActivity?.id) {
      setHotLoading(true);
      api.getRankList(currentActivity.id, 1, 6)
        .then((data) => {
          setHotEntries(data.entries ?? []);
        })
        .catch(() => setHotEntries([]))
        .finally(() => setHotLoading(false));
    }
  }, [currentActivity?.id]);

  const handleGenerate = () => {
    if (currentActivity) {
      navigation.navigate('Generate', { activityId: currentActivity.id });
    }
  };

  const handleEntryPress = (entryId: number) => {
    navigation.navigate('Detail', { entryId });
  };

  return (
    <RegionGuard>
      <ScrollView
        style={styles.container}
        refreshControl={
          <RefreshControl refreshing={isLoading} onRefresh={fetchCurrentActivity} colors={[colors.primary]} />
        }
      >
        {isOffline && (
          <View style={styles.offlineBanner}>
            <Text style={styles.offlineText}>网络不可用，部分功能可能受限</Text>
          </View>
        )}

        {currentActivity?.banner_url && (
          <Image source={{ uri: currentActivity.banner_url }} style={styles.bannerImage} resizeMode="cover" />
        )}

        {currentActivity && (
          <CountdownBanner
            title={currentActivity.name}
            endTime={currentActivity.voting_end_at}
            regionName={currentActivity.region_name}
          />
        )}

        <View style={styles.ctaSection}>
          <TouchableOpacity style={styles.ctaButton} onPress={handleGenerate}>
            <Text style={styles.ctaText}>立即生成我的蛋糕</Text>
          </TouchableOpacity>
        </View>

        <View style={styles.infoSection}>
          <Text style={styles.infoTitle}>Top100 免费领 6寸蛋糕</Text>
          <Text style={styles.infoDescription}>
            AI 设计 → 参赛投票 → Top100 获奖 → 到店核销领取
          </Text>
          <Text style={styles.infoDetail}>
            投票仅限本赛区用户，每日可投{currentActivity?.rules?.max_votes_per_day ?? 3}票
          </Text>
        </View>

        {hotEntries.length > 0 && (
          <View style={styles.hotSection}>
            <Text style={styles.sectionTitle}>热门作品</Text>
            <View style={styles.hotList}>
              {hotEntries.map((entry) => (
                <CakeCard
                  key={entry.id}
                  title={entry.title}
                  imageUrl={entry.image_url}
                  voteCount={entry.valid_vote_count}
                  rank={entry.rank}
                  onPress={() => handleEntryPress(entry.id)}
                />
              ))}
            </View>
          </View>
        )}

        <View style={styles.rulesSection}>
          <Text style={styles.sectionTitle}>活动规则</Text>
          <Text style={styles.ruleText}>参与范围：活动赛区 {currentActivity?.rules?.region_radius_km ?? 10}km 内</Text>
          <Text style={styles.ruleText}>投票限制：每人每天 {currentActivity?.rules?.max_votes_per_day ?? 3} 票</Text>
          <Text style={styles.ruleText}>奖品：{currentActivity?.rules?.free_cake_size ?? '6寸'} {currentActivity?.rules?.cream_type ?? '动物奶油'}免费蛋糕</Text>
          <Text style={styles.ruleText}>领取：到店扫码核销自提</Text>
          <Text style={styles.ruleText}>AI生成：每小时限 {currentActivity?.rules?.ai_generation_rate_limit ?? 5} 次</Text>
        </View>
      </ScrollView>
    </RegionGuard>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
  },
  offlineBanner: {
    backgroundColor: colors.warning,
    padding: spacing.md,
    alignItems: 'center',
  },
  offlineText: {
    color: colors.textPrimary,
    fontSize: 13,
  },
  bannerImage: {
    width: '100%',
    height: 160,
    resizeMode: 'cover',
  },
  ctaSection: {
    padding: spacing.xl,
    alignItems: 'center',
  },
  ctaButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    paddingVertical: spacing.xl,
    paddingHorizontal: spacing.xxxl,
    elevation: 3,
  },
  ctaText: {
    ...typography.button,
    color: colors.textPrimary,
  },
  infoSection: {
    padding: spacing.xl,
    alignItems: 'center',
  },
  infoTitle: {
    ...typography.title,
    color: colors.freeTag,
    marginBottom: spacing.sm,
  },
  infoDescription: {
    ...typography.caption,
    color: colors.textSecondary,
    textAlign: 'center',
  },
  infoDetail: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
    marginTop: spacing.sm,
  },
  hotSection: {
    padding: spacing.xl,
  },
  sectionTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  hotList: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
  },
  rulesSection: {
    padding: spacing.xl,
    backgroundColor: colors.surface,
    marginHorizontal: spacing.xl,
    borderRadius: borderRadius.lg,
    marginBottom: spacing.xxl,
  },
  ruleText: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.sm,
  },
});
