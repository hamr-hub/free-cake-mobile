import React, { useEffect, useState } from 'react';
import {
  View,
  Text,
  ScrollView,
  StyleSheet,
  ActivityIndicator,
} from 'react-native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { useTypedRoute } from '../hooks/useTypedRoute';
import { colors, spacing, borderRadius, typography } from '../theme';
import * as api from '../services/api';

const STATIC_RULES = [
  {
    title: '活动参与',
    items: [
      '活动仅限村镇周边10km内用户参与（GPS+手机号+IP三重校验）',
      '每两周一期活动，每期最多可提交1件参赛作品',
      'AI一键生成5款蛋糕效果图，选择1款发布参赛',
    ],
  },
  {
    title: '开奖与领奖',
    items: [
      '活动结束后自动结算Top100获奖名单',
      '获奖用户凭手机号+核销码到店领取免费蛋糕',
      '免费蛋糕标准：6寸动物奶油基础款',
      '领奖期限：开奖后7天内，逾期核销码失效',
    ],
  },
  {
    title: '付费下单',
    items: [
      '未获奖用户可付费下单自己设计的蛋糕',
      '价格按赛区、蛋糕尺寸、奶油类型配置',
      '支持微信支付，30分钟未支付订单自动关闭',
      '到店自提，凭核销码领取',
    ],
  },
  {
    title: '风控说明',
    items: [
      '系统监控手机号、OpenID、设备指纹、IP、GeoHash',
      '异常票数自动冻结并标记待复核',
      '存在风控标记的作品可能被扣减有效票数',
      '如有异议请联系客服申诉',
    ],
  },
];

export function RulesScreen() {
  const route = useTypedRoute<'Rules'>();
  const activityId = route.params.activityId;
  const [dynamicRule, setDynamicRule] = useState<any>(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!activityId) return;
    setLoading(true);
    api.getActivityRules(activityId)
      .then((data) => setDynamicRule(data?.data ?? data ?? null))
      .catch(() => setDynamicRule(null))
      .finally(() => setLoading(false));
  }, [activityId]);

  const votingItems = dynamicRule
    ? [
        `每人每天限投${dynamicRule.max_votes_per_day ?? 3}票`,
        '投票需在活动投票期内进行，逾期无法投票',
        '系统自动监控异常投票行为（刷票、代投等）',
      ]
    : [
        '每人每天限投3票，同一作品不可重复投票',
        '投票需在活动投票期内进行，逾期无法投票',
        '系统自动监控异常投票行为（刷票、代投等）',
      ];

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.mainTitle}>活动规则说明</Text>

      {loading && <ActivityIndicator color={colors.primary} style={{ marginBottom: spacing.md }} />}

      {dynamicRule && (
        <View style={styles.section}>
          <Text style={styles.sectionTitle}>本活动配置</Text>
          {dynamicRule.cake_size && <Text style={styles.itemText}>蛋糕尺寸：{dynamicRule.cake_size}</Text>}
          {dynamicRule.cream_type && <Text style={styles.itemText}>奶油类型：{dynamicRule.cream_type}</Text>}
          {dynamicRule.max_votes_per_day && <Text style={styles.itemText}>每日投票上限：{dynamicRule.max_votes_per_day}票</Text>}
          {dynamicRule.decoration_params && (() => {
            try {
              const params = typeof dynamicRule.decoration_params === 'string'
                ? JSON.parse(dynamicRule.decoration_params)
                : dynamicRule.decoration_params;
              return params.ai_generation_rate_limit
                ? <Text style={styles.itemText}>AI生成次数上限：{params.ai_generation_rate_limit}次</Text>
                : null;
            } catch { return null; }
          })()}
        </View>
      )}

      <View style={styles.section}>
        <Text style={styles.sectionTitle}>投票规则</Text>
        {votingItems.map((item, i) => (
          <View key={i} style={styles.itemRow}>
            <Text style={styles.bullet}>•</Text>
            <Text style={styles.itemText}>{item}</Text>
          </View>
        ))}
      </View>

      {STATIC_RULES.map((section, idx) => (
        <View key={idx} style={styles.section}>
          <Text style={styles.sectionTitle}>{section.title}</Text>
          {section.items.map((item, i) => (
            <View key={i} style={styles.itemRow}>
              <Text style={styles.bullet}>•</Text>
              <Text style={styles.itemText}>{item}</Text>
            </View>
          ))}
        </View>
      ))}

      <View style={styles.footer}>
        <Text style={styles.footerText}>如有疑问，请联系客服 400-123-4567</Text>
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
  },
  mainTitle: {
    ...typography.heading,
    color: colors.textPrimary,
    textAlign: 'center',
    marginBottom: spacing.xl,
  },
  section: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    marginBottom: spacing.lg,
  },
  sectionTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  itemRow: {
    flexDirection: 'row',
    marginBottom: spacing.sm,
    gap: spacing.sm,
  },
  bullet: {
    ...typography.body,
    color: colors.primary,
    fontWeight: '700',
  },
  itemText: {
    ...typography.body,
    color: colors.textSecondary,
    flex: 1,
  },
  footer: {
    alignItems: 'center',
    paddingVertical: spacing.xl,
    marginBottom: spacing.xxl,
  },
  footerText: {
    ...typography.caption,
    color: colors.textHint,
  },
});
