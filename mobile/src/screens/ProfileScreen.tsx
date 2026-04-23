import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
} from 'react-native';
import { useAuth } from '../context/AuthContext';
import { getUserProfile } from '../services/api';
import { UserProfile, User } from '../types/user';
import { ContestEntry } from '../types/entry';
import { VoteRecord } from '../types/vote';
import { RedeemCode } from '../types/redeem';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatPhone, formatVoteCount, formatDate } from '../utils/formatters';

export function ProfileScreen() {
  const { logout, userId } = useAuth();
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    fetchProfile();
  }, []);

  const fetchProfile = async () => {
    setIsLoading(true);
    try {
      const data = await getUserProfile();
      setProfile(data);
    } catch {
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <ScrollView style={styles.container}>
      {isLoading && <ActivityIndicator color={colors.primary} />}

      {profile?.user && (
        <View style={styles.profileCard}>
          <View style={styles.avatarPlaceholder}>
            <Text style={styles.avatarText}>
              {profile.user.nickname?.[0] ?? 'U'}
            </Text>
          </View>
          <Text style={styles.nickname}>{profile.user.nickname ?? '用户'}</Text>
          <Text style={styles.phone}>{formatPhone(profile.user.phone)}</Text>
          <Text style={styles.region}>{profile.user.region_name ?? '未知赛区'}</Text>
        </View>
      )}

      {profile && (
        <View style={styles.statsRow}>
          <View style={styles.statItem}>
            <Text style={styles.statValue}>{profile.entries?.length ?? 0}</Text>
            <Text style={styles.statLabel}>我的作品</Text>
          </View>
          <View style={styles.statItem}>
            <Text style={styles.statValue}>{profile.votes?.length ?? 0}</Text>
            <Text style={styles.statLabel}>投票记录</Text>
          </View>
          <View style={styles.statItem}>
            <Text style={styles.statValue}>{profile.redeem_codes?.length ?? 0}</Text>
            <Text style={styles.statLabel}>领奖码</Text>
          </View>
        </View>
      )}

      {profile?.entries?.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>我的作品</Text>
          {profile.entries.map((entry) => (
            <View key={entry.id} style={styles.entryItem}>
              <Text style={styles.entryTitle}>{entry.title}</Text>
              <Text style={styles.entryVotes}>{formatVoteCount(entry.valid_vote_count)} 票 · 排名 #{entry.rank}</Text>
            </View>
          ))}
        </View>
      )}

      {profile?.redeem_codes?.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>领奖状态</Text>
          {profile.redeem_codes.map((code) => (
            <View key={code.code} style={styles.redeemItem}>
              <Text style={styles.redeemCode}>{code.code}</Text>
              <Text style={[styles.redeemStatus, code.status === 'used' && styles.redeemUsed]}>
                {code.status === 'unused' ? '待领取' : code.status === 'used' ? '已核销' : '已过期'}
              </Text>
            </View>
          ))}
        </View>
      )}

      <TouchableOpacity style={styles.logoutButton} onPress={logout}>
        <Text style={styles.logoutText}>退出登录</Text>
      </TouchableOpacity>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  profileCard: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.xl,
    padding: spacing.xl,
    alignItems: 'center',
    marginBottom: spacing.xl,
  },
  avatarPlaceholder: {
    width: 60,
    height: 60,
    borderRadius: 30,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
    marginBottom: spacing.md,
  },
  avatarText: {
    fontSize: 24,
    fontWeight: '700',
    color: colors.textPrimary,
  },
  nickname: {
    ...typography.title,
    color: colors.textPrimary,
  },
  phone: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  region: {
    ...typography.caption,
    color: colors.textHint,
    marginTop: spacing.xs,
  },
  statsRow: {
    flexDirection: 'row',
    justifyContent: 'space-around',
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginBottom: spacing.xl,
  },
  statItem: {
    alignItems: 'center',
  },
  statValue: {
    ...typography.number,
    fontSize: 20,
    color: colors.textPrimary,
  },
  statLabel: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  section: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginBottom: spacing.xl,
  },
  sectionTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  entryItem: {
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  entryTitle: {
    ...typography.body,
    color: colors.textPrimary,
  },
  entryVotes: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.xs,
  },
  redeemItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  redeemCode: {
    ...typography.body,
    color: colors.textPrimary,
  },
  redeemStatus: {
    ...typography.body,
    color: colors.warning,
  },
  redeemUsed: {
    color: colors.success,
  },
  logoutButton: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: colors.divider,
    marginTop: spacing.xxl,
  },
  logoutText: {
    ...typography.body,
    color: colors.danger,
  },
});
