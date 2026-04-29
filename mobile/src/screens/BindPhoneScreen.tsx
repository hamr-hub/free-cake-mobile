import React, { useState } from 'react';
import {
  View,
  Text,
  TextInput,
  TouchableOpacity,
  StyleSheet,
  ActivityIndicator,
  Alert,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { useAuth } from '../context/AuthContext';
import { colors, spacing, borderRadius, typography } from '../theme';
import { useTypedRoute } from '../hooks/useTypedRoute';

export function BindPhoneScreen() {
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const route = useTypedRoute<'BindPhone'>();
  const openid = route.params.openid ?? '';
  const { bindPhone, sendCode, isLoading, error } = useAuth();

  const [phone, setPhone] = useState('');
  const [verifyCode, setVerifyCode] = useState('');
  const [cooldown, setCooldown] = useState(0);
  const [localError, setLocalError] = useState('');

  const handleSendCode = async () => {
    if (!phone || phone.length !== 11) {
      Alert.alert('提示', '请输入11位手机号');
      return;
    }
    try {
      await sendCode(phone);
      setCooldown(60);
      const timer = setInterval(() => {
        setCooldown((prev) => {
          if (prev <= 1) {
            clearInterval(timer);
            return 0;
          }
          return prev - 1;
        });
      }, 1000);
    } catch {
      setLocalError('发送验证码失败，请稍后重试');
    }
  };

  const handleBind = async () => {
    if (!phone || !verifyCode) {
      setLocalError('请填写手机号和验证码');
      return;
    }
    try {
      setLocalError('');
      await bindPhone(openid, phone, verifyCode);
    } catch {
      setLocalError(error ?? '绑定失败');
    }
  };

  const displayError = localError || error;

  return (
    <View style={styles.container}>
      <Text style={styles.title}>绑定手机号</Text>
      <Text style={styles.subtitle}>首次使用微信登录，需绑定手机号</Text>

      {displayError && <Text style={styles.errorText}>{displayError}</Text>}

      <View style={styles.inputRow}>
        <TextInput
          style={styles.phoneInput}
          placeholder="手机号"
          keyboardType="phone-pad"
          maxLength={11}
          value={phone}
          onChangeText={setPhone}
          editable={!isLoading}
        />
        <TouchableOpacity
          style={[styles.codeButton, cooldown > 0 && styles.codeButtonDisabled]}
          onPress={handleSendCode}
          disabled={cooldown > 0 || isLoading}
        >
          <Text style={styles.codeButtonText}>
            {cooldown > 0 ? `${cooldown}s` : '获取验证码'}
          </Text>
        </TouchableOpacity>
      </View>

      <TextInput
        style={styles.input}
        placeholder="验证码"
        keyboardType="number-pad"
        maxLength={6}
        value={verifyCode}
        onChangeText={setVerifyCode}
        editable={!isLoading}
      />

      <TouchableOpacity
        style={[styles.bindButton, isLoading && styles.bindButtonDisabled]}
        onPress={handleBind}
        disabled={isLoading}
      >
        {isLoading ? (
          <ActivityIndicator color={colors.surface} />
        ) : (
          <Text style={styles.bindButtonText}>确认绑定</Text>
        )}
      </TouchableOpacity>
    </View>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
    justifyContent: 'center',
  },
  title: {
    ...typography.heading,
    color: colors.textPrimary,
    textAlign: 'center',
    marginBottom: spacing.sm,
  },
  subtitle: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
    marginBottom: spacing.xxl,
  },
  errorText: {
    color: colors.danger,
    fontSize: 13,
    textAlign: 'center',
    marginBottom: spacing.md,
  },
  inputRow: {
    flexDirection: 'row',
    gap: spacing.md,
    marginBottom: spacing.md,
  },
  phoneInput: {
    flex: 1,
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    ...typography.body,
    color: colors.textPrimary,
  },
  codeButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.lg,
    justifyContent: 'center',
  },
  codeButtonDisabled: {
    opacity: 0.5,
  },
  codeButtonText: {
    ...typography.caption,
    color: colors.textPrimary,
    fontWeight: '600',
  },
  input: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    ...typography.body,
    color: colors.textPrimary,
    marginBottom: spacing.xl,
  },
  bindButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
  },
  bindButtonDisabled: {
    opacity: 0.5,
  },
  bindButtonText: {
    ...typography.button,
    color: colors.textPrimary,
  },
});
