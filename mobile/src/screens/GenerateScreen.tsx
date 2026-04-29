import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  ScrollView,
  StyleSheet,
  ActivityIndicator,
  Image,
} from 'react-native';
import { SCENES, STYLES, COLOR_PREFERENCES, AI_GENERATE_RATE_LIMIT } from '../utils/constants';
import { Scene, Style, ColorPreference } from '../utils/constants';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';
import { generateEntries } from '../services/api';

interface GenerateScreenProps {
  route?: { params?: { activityId: number } };
  navigation?: any;
}

export function GenerateScreen({ route, navigation }: GenerateScreenProps) {
  const activityId = route?.params?.activityId ?? 0;
  const [scene, setScene] = useState<Scene>('birthday');
  const [theme, setTheme] = useState('');
  const [blessing, setBlessing] = useState('');
  const [colorPreference, setColorPreference] = useState<ColorPreference>('warm');
  const [style, setStyle] = useState<Style>('cartoon');
  const [isLoading, setIsLoading] = useState(false);
  const [generatedImages, setGeneratedImages] = useState<string[]>([]);
  const [templateIds, setTemplateIds] = useState<number[]>([]);
  const [generationId, setGenerationId] = useState<number>(0);
  const [selectedImageIndex, setSelectedImageIndex] = useState<number | null>(null);
  const [error, setError] = useState('');

  const sceneLabels: Record<Scene, string> = {
    birthday: '生日',
    children: '儿童',
    festival: '节庆',
    wedding: '婚庆',
    other: '其他',
  };

  const handleGenerate = async () => {
    if (!theme.trim()) {
      setError('请输入主题关键词');
      return;
    }
    setIsLoading(true);
    setError('');
    try {
      const data = await generateEntries(activityId, {
        scene,
        theme: theme.trim(),
        blessing: blessing.trim(),
        color_preference: colorPreference,
        style,
      });
      setGeneratedImages(data.images ?? []);
      setTemplateIds(data.template_ids ?? []);
      setGenerationId(data.generation_id ?? 0);
    } catch (err: any) {
      setError(err.response?.data?.message ?? '生成失败，请重试');
    } finally {
      setIsLoading(false);
    }
  };

  const handleSelect = (index: number) => {
    setSelectedImageIndex(index);
  };

  const handlePublish = () => {
    if (selectedImageIndex !== null) {
      navigation?.navigate('Publish', {
        activityId,
        imageUrl: generatedImages[selectedImageIndex],
        imageIndex: selectedImageIndex,
        generationId,
        templateId: templateIds[selectedImageIndex] ?? 0,
      });
    }
  };

  return (
    <ScrollView style={styles.container}>
      <Text style={styles.sectionTitle}>选择场景</Text>
      <View style={styles.optionRow}>
        {SCENES.map((s) => (
          <TouchableOpacity
            key={s}
            style={[styles.optionItem, scene === s && styles.optionItemSelected]}
            onPress={() => setScene(s)}
          >
            <Text style={[styles.optionText, scene === s && styles.optionTextSelected]}>
              {sceneLabels[s]}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <Text style={styles.sectionTitle}>主题关键词</Text>
      <TextInput
        style={styles.input}
        placeholder="如：星空、彩虹、小熊..."
        value={theme}
        onChangeText={setTheme}
        maxLength={20}
      />

      <Text style={styles.sectionTitle}>祝福语</Text>
      <TextInput
        style={styles.input}
        placeholder="如：生日快乐！"
        value={blessing}
        onChangeText={setBlessing}
        maxLength={30}
      />

      <Text style={styles.sectionTitle}>偏好色系</Text>
      <View style={styles.optionRow}>
        {COLOR_PREFERENCES.map((c) => (
          <TouchableOpacity
            key={c}
            style={[styles.optionItem, colorPreference === c && styles.optionItemSelected]}
            onPress={() => setColorPreference(c)}
          >
            <Text style={[styles.optionText, colorPreference === c && styles.optionTextSelected]}>
              {c}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      <Text style={styles.sectionTitle}>风格偏好</Text>
      <View style={styles.optionRow}>
        {STYLES.map((s) => (
          <TouchableOpacity
            key={s}
            style={[styles.optionItem, style === s && styles.optionItemSelected]}
            onPress={() => setStyle(s)}
          >
            <Text style={[styles.optionText, style === s && styles.optionTextSelected]}>
              {s}
            </Text>
          </TouchableOpacity>
        ))}
      </View>

      {error && <Text style={styles.errorText}>{error}</Text>}

      <TouchableOpacity
        style={[styles.generateButton, isLoading && styles.generateButtonDisabled]}
        onPress={handleGenerate}
        disabled={isLoading}
      >
        {isLoading ? (
          <ActivityIndicator color={colors.textPrimary} />
        ) : (
          <Text style={styles.generateButtonText}>生成 5 款设计</Text>
        )}
      </TouchableOpacity>

      {generatedImages.length > 0 && (
        <View style={styles.resultSection}>
          <Text style={styles.sectionTitle}>选择你喜欢的设计</Text>
          <View style={styles.imageGrid}>
            {generatedImages.map((url, index) => (
              <TouchableOpacity
                key={index}
                style={[styles.imageItem, selectedImageIndex === index && styles.imageItemSelected]}
                onPress={() => handleSelect(index)}
              >
                {url.startsWith('placeholder://') ? (
                  <View style={styles.imagePlaceholder}>
                    <Text style={styles.imagePlaceholderText}>设计 #{index + 1}</Text>
                  </View>
                ) : (
                  <Image source={{ uri: url }} style={styles.imageThumb} resizeMode="cover" />
                )}
                {selectedImageIndex === index && (
                  <View style={styles.selectedOverlay}>
                    <Text style={styles.selectedText}>已选择</Text>
                  </View>
                )}
              </TouchableOpacity>
            ))}
          </View>

          {selectedImageIndex !== null && (
            <TouchableOpacity style={styles.publishButton} onPress={handlePublish}>
              <Text style={styles.publishButtonText}>确认并发布</Text>
            </TouchableOpacity>
          )}
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
  sectionTitle: {
    ...typography.title,
    color: colors.textPrimary,
    marginBottom: spacing.md,
    marginTop: spacing.lg,
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
  optionRow: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    marginBottom: spacing.md,
  },
  optionItem: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.sm,
    marginRight: spacing.sm,
    marginBottom: spacing.sm,
    borderWidth: 1,
    borderColor: colors.divider,
  },
  optionItemSelected: {
    backgroundColor: colors.primary,
    borderColor: colors.primaryDark,
  },
  optionText: {
    fontSize: 14,
    color: colors.textSecondary,
  },
  optionTextSelected: {
    color: colors.textPrimary,
    fontWeight: '600',
  },
  errorText: {
    color: colors.danger,
    fontSize: 13,
    textAlign: 'center',
    marginBottom: spacing.md,
  },
  generateButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  generateButtonDisabled: {
    backgroundColor: colors.disabled,
  },
  generateButtonText: {
    ...typography.button,
    color: colors.textPrimary,
  },
  resultSection: {
    marginTop: spacing.xxxl,
  },
  imageGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
  },
  imageItem: {
    width: '48%',
    height: 150,
    borderRadius: borderRadius.md,
    marginBottom: spacing.md,
    borderWidth: 2,
    borderColor: colors.divider,
    overflow: 'hidden',
  },
  imageItemSelected: {
    borderColor: colors.primary,
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
  imageThumb: {
    flex: 1,
    width: '100%',
  },
  selectedOverlay: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    backgroundColor: colors.primary,
    padding: spacing.sm,
    alignItems: 'center',
  },
  selectedText: {
    fontSize: 12,
    fontWeight: '600',
    color: colors.textPrimary,
  },
  publishButton: {
    backgroundColor: colors.freeTag,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginTop: spacing.lg,
  },
  publishButtonText: {
    ...typography.button,
    color: colors.surface,
  },
});
