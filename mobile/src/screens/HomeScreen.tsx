import React, { useEffect } from 'react';
import { View, Text, ScrollView, TouchableOpacity, StyleSheet, RefreshControl } from 'react-native';
import { useAuth } from '../context/AuthContext';
import { useActivity } from '../hooks/useActivity';
import { useNetwork } from '../hooks/useNetwork';
import { RegionGuard } from '../components/RegionGuard';
import { CountdownBanner } from '../components/CountdownBanner';
import { CakeCard } from '../components/CakeCard';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatCountdown } from '../utils/formatters';

export function HomeScreen() {
  const { regionId } = useAuth();
  const { currentActivity, isLoading, fetchCurrentActivity, fetchActivities, activities } = useActivity();
  const { isOffline } = useNetwork();

  useEffect(() => {
    if (regionId) {
      fetchCurrentActivity();
      fetchActivities();
    }
  }, [regionId]);

  const handleGenerate = () => {
    if (currentActivity) {
      // @ts-ignore - navigation will be injected by React Navigation
      const navigation = globalThis.__navigation;
      navigation?.navigate('Generate', { activityId: currentActivity.id });
    }
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
        </View>

        {currentActivity && (
          <View style={styles.hotSection}>
            <Text style={styles.sectionTitle}>热门作品</Text>
            <View style={styles.hotList}>
              {activities.slice(0, 6).map((_, index) => (
                <CakeCard
                  key={index}
                  title={`作品 #${index + 1}`}
                  imageUrl=""
                  voteCount={0}
                  rank={index + 1}
                  onPress={() => {}}
                />
              ))}
            </View>
          </View>
        )}

        <View style={styles.rulesSection}>
          <Text style={styles.sectionTitle}>活动规则</Text>
          <Text style={styles.ruleText}>参与范围：活动赛区 10km 内</Text>
          <Text style={styles.ruleText}>投票限制：每人每天 3 票</Text>
          <Text style={styles.ruleText}>奖品：6寸动物奶油免费蛋糕</Text>
          <Text style={styles.ruleText}>领取：到店扫码核销自提</Text>
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
