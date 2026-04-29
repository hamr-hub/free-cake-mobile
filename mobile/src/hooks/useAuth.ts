import { useAuth } from '../context/AuthContext';

export function useAuthHook() {
  const auth = useAuth();

  return {
    isAuthenticated: auth.isAuthenticated,
    userId: auth.userId,
    regionId: auth.regionId,
    isLoading: auth.isLoading,
    error: auth.error,
    login: auth.login,
    wechatLogin: auth.wechatLogin,
    bindPhone: auth.bindPhone,
    logout: auth.logout,
    sendCode: auth.sendCode,
  };
}
