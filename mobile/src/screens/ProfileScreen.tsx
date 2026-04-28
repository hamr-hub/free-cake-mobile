import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
  Linking,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import { useAuth } from '../context/AuthContext';
import { getUserProfile, getRedeemDetail } from '../services/api';
import { UserProfile } from '../types/user';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { formatPhone, formatVoteCount, formatDate } from '../utils/formatters';

export function ProfileScreen() {
  const { logout, userId } = useAuth();
  const navigation = useNavigation<any>();
  const [profile, setProfile] = useState<UserProfile | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [showVoteRecords, setShowVoteRecords] = useState(false);
  const [redeemDetails, setRedeemDetails] = useState<Record<string, any>>({});

  useEffect(() => {
    fetchProfile();
  }, []);

  const fetchProfile = async () => {
    setIsLoading(true);
    try {
      const data = await getUserProfile();
      setProfile(data);
      if (data?.redeem_codes?.length > 0) {
        for (const code of data.redeem_codes) {
          if (code.status === 'unused') {
            try {
              const detail = await getRedeemDetail(code.code);
              setRedeemDetails((prev) => ({ ...prev, [code.code]: detail }));
            } catch {}
          }
        }
      }
    } catch {
    } finally {
      setIsLoading(false);
    }
  };

  const handleEntryPress = (entryId: number) => {
    navigation.navigate('Detail', { entryId });
  };

  const handleRedeemPress = () => {
    navigation.navigate('Redeem');
  };

  const handleCustomerService = () => {
    Linking.openURL('tel:4001234567');
  };

  const handleAppeal = () => {
    Linking.openURL('https://free-cake.example.com/appeal');
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
          <TouchableOpacity style={styles.statItem} onPress={() => setShowVoteRecords(!showVoteRecords)}>
            <Text style={styles.statValue}>{profile.entries?.length ?? 0}</Text>
            <Text style={styles.statLabel}>我的作品</Text>
          </TouchableOpacity>
          <View style={styles.statItem}>
            <Text style={styles.statValue}>{profile.votes?.length ?? 0}</Text>
            <Text style={styles.statLabel}>投票记录</Text>
          </View>
          <TouchableOpacity style={styles.statItem} onPress={handleRedeemPress}>
            <Text style={styles.statValue}>{profile.redeem_codes?.length ?? 0}</Text>
            <Text style={styles.statLabel}>领奖码</Text>
          </TouchableOpacity>
        </View>
      )}

      {profile !== null && profile.entries && profile.entries.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>我的作品</Text>
          {profile.entries.map((entry: any) => (
            <TouchableOpacity key={entry.id} style={styles.entryItem} onPress={() => handleEntryPress(entry.id)}>
              <Text style={styles.entryTitle}>{entry.title || `作品 #${entry.id}`}</Text>
              <Text style={styles.entryVotes}>{formatVoteCount(entry.valid_vote_count)} 票 · 排名 #{entry.rank}</Text>
              {entry.is_winner && <Text style={styles.entryWinner}>已获奖</Text>}
            </TouchableOpacity>
          ))}
        </View>
      )}

      {showVoteRecords && profile !== null && profile.votes && profile.votes.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>投票记录</Text>
          {profile.votes.map((vote: any, index: number) => (
            <View key={vote.id || index} style={styles.voteItem}>
              <Text style={styles.voteEntry}>作品 #{vote.entry_id}</Text>
              <Text style={styles.voteTime}>{formatDate(vote.created_at)}</Text>
              <Text style={[styles.voteStatus, vote.vote_status === 'valid' && styles.voteValid]}>
                {vote.vote_status === 'valid' ? '有效' : vote.vote_status === 'frozen' ? '冻结' : '无效'}
              </Text>
            </View>
          ))}
        </View>
      )}

      {profile !== null && profile.redeem_codes && profile.redeem_codes.length > 0 && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>领奖状态</Text>
          {profile.redeem_codes.map((code: any) => (
            <TouchableOpacity key={code.code} style={styles.redeemItem} onPress={handleRedeemPress}>
              <View>
                <Text style={styles.redeemCode}>{code.code}</Text>
                {redeemDetails[code.code]?.store_address && (
                  <Text style={styles.redeemAddress}>{redeemDetails[code.code].store_address}</Text>
                )}
              </View>
              <Text style={[styles.redeemStatus, code.status === 'used' && styles.redeemUsed]}>
                {code.status === 'unused' ? '待领取' : code.status === 'used' ? '已核销' : '已过期'}
              </Text>
            </TouchableOpacity>
          ))}
        </View>
      )}

      <View style={styles.supportSection}>
        <Text style={styles.sectionTitle}>客服与申诉</Text>
        <TouchableOpacity style={styles.supportButton} onPress={handleCustomerService}>
          <Text style={styles.supportButtonText}>联系客服 (400-123-4567)</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.supportButton} onPress={handleAppeal}>
          <Text style={styles.supportButtonText}>投票异常申诉</Text>
        </TouchableOpacity>
        <TouchableOpacity style={styles.supportButton} onPress={() => Linking.openURL('https://free-cake.example.com/help')}>
          <Text style={styles.supportButtonText}>常见问题</Text>
        </TouchableOpacity>
      </View>

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
  entryWinner: {
    ...typography.caption,
    color: colors.freeTag,
    marginTop: spacing.xs,
    fontWeight: '600',
  },
  voteItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  voteEntry: {
    ...typography.body,
    color: colors.textPrimary,
  },
  voteTime: {
    ...typography.caption,
    color: colors.textHint,
  },
  voteStatus: {
    ...typography.caption,
    color: colors.warning,
    fontWeight: '600',
  },
  voteValid: {
    color: colors.success,
  },
  redeemItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  redeemCode: {
    ...typography.body,
    color: colors.textPrimary,
  },
  redeemAddress: {
    ...typography.caption,
    color: colors.textHint,
    marginTop: spacing.xs,
  },
  redeemStatus: {
    ...typography.body,
    color: colors.warning,
  },
  redeemUsed: {
    color: colors.success,
  },
  supportSection: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginBottom: spacing.xl,
  },
  supportButton: {
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  supportButtonText: {
    ...typography.body,
    color: colors.primary,
  },
  logoutButton: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    borderWidth: 1,
    borderColor: colors.divider,
    marginTop: spacing.xxl,
    marginBottom: spacing.xxl,
  },
  logoutText: {
    ...typography.body,
    color: colors.danger,
  },
});
