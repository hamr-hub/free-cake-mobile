import React, { ReactNode, useEffect, useState } from 'react';
import { View, Text, StyleSheet, FlatList, TouchableOpacity } from 'react-native';
import { useAuth } from '../context/AuthContext';
import * as api from '../services/api';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

interface RegionGuardProps {
  children: ReactNode;
  fallback?: ReactNode;
}

interface AvailableRegion {
  id: number;
  name: string;
  province: string;
  city: string;
  status: string;
}

export function RegionGuard({ children, fallback }: RegionGuardProps) {
  const { regionId } = useAuth();
  const [availableRegions, setAvailableRegions] = useState<AvailableRegion[]>([]);
  const [regionsLoading, setRegionsLoading] = useState(false);

  useEffect(() => {
    if (!regionId) {
      setRegionsLoading(true);
      api.getActivities(1, 50, 'voting_open')
        .then((data) => {
          const acts = Array.isArray(data?.data) ? data.data : Array.isArray(data?.list) ? data.list : [];
          const regions = acts.map((a: any) => ({
            id: a.region_id,
            name: a.region_name || `赛区 #${a.region_id}`,
            province: '',
            city: '',
            status: 'active',
          }));
          const unique = regions.filter((r: any, i: number) => regions.findIndex((x: any) => x.id === r.id) === i);
          setAvailableRegions(unique);
        })
        .catch(() => setAvailableRegions([]))
        .finally(() => setRegionsLoading(false));
    }
  }, [regionId]);

  if (!regionId) {
    return (
      fallback ?? (
        <View style={styles.container}>
          <Text style={styles.title}>暂未开放</Text>
          <Text style={styles.description}>
            您所在区域暂未开通免费蛋糕服务
          </Text>
          {availableRegions.length > 0 && (
            <View style={styles.regionsSection}>
              <Text style={styles.regionsTitle}>可参与赛区：</Text>
              <FlatList
                data={availableRegions}
                keyExtractor={(item) => item.id.toString()}
                renderItem={({ item }) => (
                  <View style={styles.regionItem}>
                    <Text style={styles.regionName}>{item.name}</Text>
                    <Text style={styles.regionStatus}>活动进行中</Text>
                  </View>
                )}
                scrollEnabled={false}
              />
            </View>
          )}
          <Text style={styles.hint}>敬请期待更多赛区开放！</Text>
        </View>
      )
    );
  }

  return <>{children}</>;
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    justifyContent: 'center',
    alignItems: 'center',
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  title: {
    fontSize: 20,
    fontWeight: '700',
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  description: {
    fontSize: 14,
    color: colors.textSecondary,
    textAlign: 'center',
    lineHeight: 20,
    marginBottom: spacing.xl,
  },
  regionsSection: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    width: '100%',
    marginBottom: spacing.xl,
  },
  regionsTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  regionItem: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    paddingVertical: spacing.md,
    borderBottomWidth: 1,
    borderBottomColor: colors.divider,
  },
  regionName: {
    ...typography.body,
    color: colors.textPrimary,
  },
  regionStatus: {
    ...typography.caption,
    color: colors.success,
  },
  hint: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
  },
});
