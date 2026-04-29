import React, { useState } from 'react';
import {
  View,
  TextInput,
  TouchableOpacity,
  Text,
  StyleSheet,
  KeyboardAvoidingView,
  Platform,
  Alert,
} from 'react-native';
import { useNavigation } from '@react-navigation/native';
import type { NativeStackNavigationProp } from '@react-navigation/native-stack';
import type { RootStackParamList } from '../navigation/AppNavigator';
import { useAuth } from '../context/AuthContext';
import { validatePhone, validateVerifyCode } from '../utils/validators';
import { colors } from '../theme';
import { spacing, borderRadius, typography } from '../theme';

export function LoginScreen() {
  const { login, wechatLogin, sendCode, isLoading, error } = useAuth();
  const navigation = useNavigation<NativeStackNavigationProp<RootStackParamList>>();
  const [phone, setPhone] = useState('');
  const [verifyCode, setVerifyCode] = useState('');
  const [codeSent, setCodeSent] = useState(false);
  const [codeCooldown, setCodeCooldown] = useState(0);
  const [localError, setLocalError] = useState('');

  const handleSendCode = async () => {
    if (!validatePhone(phone)) {
      setLocalError('请输入正确的手机号');
      return;
    }
    try {
      await sendCode(phone);
      setCodeSent(true);
      setCodeCooldown(60);
      setLocalError('');
      const interval = setInterval(() => {
        setCodeCooldown((prev) => {
          if (prev <= 1) {
            clearInterval(interval);
            return 0;
          }
          return prev - 1;
        });
      }, 1000);
    } catch {
      setLocalError('发送验证码失败，请重试');
    }
  };

  const handleLogin = async () => {
    if (!validatePhone(phone)) {
      setLocalError('请输入正确的手机号');
      return;
    }
    if (!validateVerifyCode(verifyCode)) {
      setLocalError('请输入验证码');
      return;
    }
    try {
      setLocalError('');
      await login(phone, verifyCode);
    } catch {
      setLocalError(error ?? '登录失败');
    }
  };

  const handleWechatLogin = async () => {
    try {
      setLocalError('');
      // TODO: Replace with native WeChat SDK call: wx.login() → code
      // For now, this placeholder shows the intended flow:
      // const code = await WeChatModule.login();
      // const result = await wechatLogin(code);
      //
      // If the user already has a phone bound, result.token is set
      // and AuthContext will set isAuthenticated = true automatically.
      //
      // If the user is new, result.need_bind_phone = true and
      // result.openid is set — navigate to BindPhone screen:
      // if (result.need_bind_phone) {
      //   navigation.navigate('BindPhone', { openid: result.openid });
      // }
      Alert.alert(
        '微信登录',
        '微信登录需要集成微信 SDK 原生模块。请配置 react-native-wechat-lib 后启用此功能。',
      );
    } catch {
      setLocalError(error ?? '微信登录失败');
    }
  };

  const displayError = localError || error;

  return (
    <KeyboardAvoidingView
      style={styles.container}
      behavior={Platform.OS === 'ios' ? 'padding' : 'height'}
    >
      <View style={styles.logoContainer}>
        <Text style={styles.logo}>Free Cake</Text>
        <Text style={styles.subtitle}>村镇免费蛋糕 · AI设计投票领奖</Text>
      </View>

      {displayError && <Text style={styles.errorText}>{displayError}</Text>}

      <TextInput
        style={styles.input}
        placeholder="手机号"
        keyboardType="phone-pad"
        maxLength={11}
        value={phone}
        onChangeText={setPhone}
        editable={!isLoading}
      />

      <View style={styles.codeRow}>
        <TextInput
          style={[styles.input, styles.codeInput]}
          placeholder="验证码"
          keyboardType="number-pad"
          maxLength={6}
          value={verifyCode}
          onChangeText={setVerifyCode}
          editable={!isLoading}
        />
        <TouchableOpacity
          style={[styles.codeButton, codeCooldown > 0 && styles.codeButtonDisabled]}
          onPress={handleSendCode}
          disabled={codeCooldown > 0 || isLoading}
        >
          <Text style={styles.codeButtonText}>
            {codeCooldown > 0 ? `${codeCooldown}s` : '获取验证码'}
          </Text>
        </TouchableOpacity>
      </View>

      <TouchableOpacity
        style={[styles.loginButton, isLoading && styles.loginButtonDisabled]}
        onPress={handleLogin}
        disabled={isLoading}
      >
        <Text style={styles.loginButtonText}>
          {isLoading ? '登录中...' : '登录'}
        </Text>
      </TouchableOpacity>

      <View style={styles.dividerContainer}>
        <View style={styles.dividerLine} />
        <Text style={styles.dividerText}>或</Text>
        <View style={styles.dividerLine} />
      </View>

      <TouchableOpacity
        style={styles.wechatButton}
        onPress={handleWechatLogin}
        disabled={isLoading}
      >
        <Text style={styles.wechatButtonText}>微信登录</Text>
      </TouchableOpacity>

      <Text style={styles.hint}>
        仅限活动赛区10km范围内用户参与
      </Text>
    </KeyboardAvoidingView>
  );
}

const styles = StyleSheet.create({
  container: {
    flex: 1,
    backgroundColor: colors.background,
    padding: spacing.xl,
    justifyContent: 'center',
  },
  logoContainer: {
    alignItems: 'center',
    marginBottom: spacing.xxxl,
  },
  logo: {
    ...typography.heading,
    fontSize: 32,
    fontWeight: '800',
    color: colors.primary,
  },
  subtitle: {
    ...typography.caption,
    color: colors.textSecondary,
    marginTop: spacing.sm,
  },
  errorText: {
    color: colors.danger,
    fontSize: 13,
    textAlign: 'center',
    marginBottom: spacing.md,
  },
  input: {
    backgroundColor: colors.surface,
    borderRadius: borderRadius.md,
    padding: spacing.lg,
    fontSize: 16,
    marginBottom: spacing.md,
    borderWidth: 1,
    borderColor: colors.divider,
  },
  codeRow: {
    flexDirection: 'row',
    marginBottom: spacing.md,
  },
  codeInput: {
    flex: 1,
    marginRight: spacing.md,
  },
  codeButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.md,
    padding: spacing.lg,
    justifyContent: 'center',
    minWidth: 120,
  },
  codeButtonDisabled: {
    backgroundColor: colors.disabled,
  },
  codeButtonText: {
    ...typography.button,
    color: colors.textPrimary,
    textAlign: 'center',
    fontSize: 14,
  },
  loginButton: {
    backgroundColor: colors.primary,
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
    marginTop: spacing.md,
  },
  loginButtonDisabled: {
    backgroundColor: colors.disabled,
  },
  loginButtonText: {
    ...typography.button,
    color: colors.textPrimary,
  },
  dividerContainer: {
    flexDirection: 'row',
    alignItems: 'center',
    marginVertical: spacing.lg,
  },
  dividerLine: {
    flex: 1,
    height: 1,
    backgroundColor: colors.divider,
  },
  dividerText: {
    ...typography.caption,
    color: colors.textHint,
    marginHorizontal: spacing.md,
  },
  wechatButton: {
    backgroundColor: '#07C160',
    borderRadius: borderRadius.lg,
    padding: spacing.xl,
    alignItems: 'center',
  },
  wechatButtonText: {
    ...typography.button,
    color: '#FFFFFF',
  },
  hint: {
    ...typography.caption,
    color: colors.textHint,
    textAlign: 'center',
    marginTop: spacing.lg,
  },
});
