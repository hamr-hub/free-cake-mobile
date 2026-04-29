import React, { useRef } from 'react';
import { View, Text, TouchableOpacity, StyleSheet, Image } from 'react-native';
import { captureAndShare } from '../services/share';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

interface SharePosterProps {
  entryId: number;
  title: string;
  imageUrl: string;
  shareCode: string;
}

export function SharePoster({ entryId, title, imageUrl, shareCode }: SharePosterProps) {
  const posterRef = useRef<View>(null);

  const handleShare = async () => {
    await captureAndShare(posterRef, `快来帮我投票！作品 #${entryId}`);
  };

  return (
    <View>
      <View ref={posterRef} style={styles.poster} collapsable={false}>
        <View style={styles.posterImage}>
          {imageUrl && !imageUrl.startsWith('placeholder://') ? (
            <Image source={{ uri: imageUrl }} style={styles.posterImageImg} resizeMode="cover" />
          ) : (
            <Text style={styles.posterImageText}>{title}</Text>
          )}
        </View>
        <View style={styles.posterInfo}>
          <Text style={styles.posterTitle}>{title}</Text>
          <Text style={styles.posterShareCode}>分享码: {shareCode}</Text>
          <Text style={styles.posterCall}>快来帮我投票吧！</Text>
        </View>
      </View>
      <TouchableOpacity style={styles.shareButton} onPress={handleShare}>
        <Text style={styles.shareButtonText}>分享给朋友</Text>
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  poster: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.lg,
    marginVertical: spacing.xl,
    overflow: 'hidden',
  },
  posterImage: {
    height: 200,
    backgroundColor: colors.primary,
    justifyContent: 'center',
    alignItems: 'center',
  },
  posterImageImg: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
  },
  posterImageText: {
    ...typography.title,
    color: colors.textPrimary,
  },
  posterInfo: {
    padding: spacing.xl,
  },
  posterTitle: {
    ...typography.title,
    color: colors.textPrimary,
  },
  posterShareCode: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.sm,
  },
  posterCall: {
    ...typography.body,
    color: colors.freeTag,
    marginTop: spacing.md,
  },
  shareButton: {
    backgroundColor: colors.accent,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
  },
  shareButtonText: {
    ...typography.button,
    color: colors.surface,
  },
});
