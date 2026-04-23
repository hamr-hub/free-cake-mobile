import React, { ReactNode } from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { useAuth } from '../context/AuthContext';
import { colors } from '../theme';
import { spacing, borderRadius } from '../theme';

interface RegionGuardProps {
  children: ReactNode;
  fallback?: ReactNode;
}

export function RegionGuard({ children, fallback }: RegionGuardProps) {
  const { regionId } = useAuth();

  if (!regionId) {
    return (
      fallback ?? (
        <View style={styles.container}>
          <Text style={styles.title}>暂未开放</Text>
          <Text style={styles.description}>
            您所在区域暂未开通免费蛋糕服务，敬请期待！
          </Text>
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
  },
});
