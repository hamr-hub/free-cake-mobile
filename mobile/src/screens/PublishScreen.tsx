import React, { useState } from 'react';
import { View, Text, TextInput, TouchableOpacity, StyleSheet, ScrollView } from 'react-native';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { submitEntry } from '../services/api';
import { SharePoster } from '../components/SharePoster';

interface PublishScreenProps {
  route?: { params?: { activityId: number; imageUrl: string; imageIndex: number } };
  navigation?: any;
}

export function PublishScreen({ route, navigation }: PublishScreenProps) {
  const { activityId, imageUrl, imageIndex } = route?.params ?? {};
  const [title, setTitle] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [isPublished, setIsPublished] = useState(false);
  const [entryId, setEntryId] = useState<number | null>(null);
  const [shareCode, setShareCode] = useState('');
  const [error, setError] = useState('');

  const handlePublish = async () => {
    if (!title.trim()) {
      setError('请输入作品标题');
      return;
    }
    setIsLoading(true);
    setError('');
    try {
      const data = await submitEntry(activityId, {
        selected_generation_id: imageIndex + 1,
        selected_template_id: imageIndex + 1,
        title: title.trim(),
      });
      setEntryId(data.entry_id);
      setShareCode(data.share_code);
      setIsPublished(true);
    } catch (err: any) {
      setError(err.response?.data?.message ?? '发布失败');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <ScrollView style={styles.container}>
      <View style={styles.selectedImage}>
        <View style={styles.imagePlaceholder}>
          <Text style={styles.imagePlaceholderText}>已选择的蛋糕设计</Text>
        </View>
      </View>

      {!isPublished ? (
        <>
          <Text style={styles.label}>作品标题</Text>
          <TextInput
            style={styles.input}
            placeholder="给你的蛋糕设计起个名字"
            value={title}
            onChangeText={setTitle}
            maxLength={50}
          />

          {error && <Text style={styles.errorText}>{error}</Text>}

          <TouchableOpacity
            style={[styles.publishButton, isLoading && styles.publishButtonDisabled]}
            onPress={handlePublish}
            disabled={isLoading}
          >
            <Text style={styles.publishButtonText}>
              {isLoading ? '发布中...' : '发布参赛'}
            </Text>
          </TouchableOpacity>

          <Text style={styles.disclaimer}>
            发布后将进入投票池，其他人可以为你的作品投票
          </Text>
        </>
      ) : (
        <View style={styles.successSection}>
          <Text style={styles.successTitle}>发布成功！</Text>
          <Text style={styles.successInfo}>作品编号: #{entryId}</Text>
          <Text style={styles.successInfo}>分享码: {shareCode}</Text>

          <SharePoster
            entryId={entryId ?? 0}
            title={title}
            imageUrl={imageUrl}
            shareCode={shareCode}
          />

          <TouchableOpacity
            style={styles.viewDetailButton}
            onPress={() => navigation?.navigate('Detail', { entryId })}
          >
            <Text style={styles.viewDetailButtonText}>查看我的作品</Text>
          </TouchableOpacity>

          <TouchableOpacity
            style={styles.homeButton}
            onPress={() => navigation?.navigate('Main')}
          >
            <Text style={styles.homeButtonText}>返回首页</Text>
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
    padding: spacing.xl,
  },
  selectedImage: {
    height: 200,
    borderRadius: borderRadius.lg,
    marginBottom: spacing.xxl,
    overflow: 'hidden',
  },
  imagePlaceholder: {
    flex: 1,
    backgroundColor: colors.divider,
    justifyContent: 'center',
    alignItems: 'center',
  },
  imagePlaceholderText: {
    ...typography.caption,
    color: colors.textHint,
  },
  label: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
  },
  input: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    padding: spacing.lg,
    fontSize: 16,
    borderWidth: 1,
    borderColor: colors.divider,
    marginBottom: spacing.md,
  },
  errorText: {
    color: colors.danger,
    fontSize: 13,
    textAlign: 'center',
    marginBottom: spacing.md,
  },
  publishButton: {
    backgroundColor: colors.freeTag,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
  },
  publishButtonDisabled: {
    backgroundColor: colors.disabled,
  },
  publishButtonText: {
    ...typography.button,
    color: colors.surface,
  },
  disclaimer: {
    ...typography.caption,
    color: colors.textSecondary,
    textAlign: 'center',
    marginTop: spacing.md,
  },
  successSection: {
    alignItems: 'center',
  },
  successTitle: {
    ...typography.heading,
    color: colors.success,
    marginBottom: spacing.lg,
  },
  successInfo: {
    ...typography.body,
    color: colors.textSecondary,
    marginBottom: spacing.sm,
  },
  viewDetailButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    paddingHorizontal: spacing.xxxl,
    marginTop: spacing.lg,
  },
  viewDetailButtonText: {
    ...typography.button,
    color: colors.textPrimary,
  },
  homeButton: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    paddingHorizontal: spacing.xxxl,
    marginTop: spacing.md,
    borderWidth: 1,
    borderColor: colors.divider,
  },
  homeButtonText: {
    ...typography.button,
    color: colors.textSecondary,
  },
});
