import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { storage } from '../services/storage';
import * as authService from '../services/auth';
import * as api from '../services/api';
import { LoginResponse, WechatLoginResponse, BindPhoneResponse } from '../types/user';

interface AuthState {
  isAuthenticated: boolean;
  token: string | null;
  userId: number | null;
  regionId: number | null;
  isLoading: boolean;
  error: string | null;
}

interface AuthContextType extends AuthState {
  login: (phone: string, verifyCode: string) => Promise<LoginResponse>;
  wechatLogin: (code: string) => Promise<WechatLoginResponse>;
  bindPhone: (openid: string, phone: string, verifyCode: string) => Promise<BindPhoneResponse>;
  logout: () => void;
  sendCode: (phone: string) => Promise<void>;
}

const initialState: AuthState = {
  isAuthenticated: false,
  token: null,
  userId: null,
  regionId: null,
  isLoading: false,
  error: null,
};

export const AuthContext = createContext<AuthContextType>({
  ...initialState,
  login: async () => ({}) as LoginResponse,
  wechatLogin: async () => ({}) as WechatLoginResponse,
  bindPhone: async () => ({}) as BindPhoneResponse,
  logout: () => {},
  sendCode: async () => {},
});

async function fetchRegionId(userId: number): Promise<number | null> {
  try {
    const profile = await api.getUserProfile();
    const regionId = (profile as any)?.user?.region_id ?? null;
    if (regionId) storage.setRegionId(regionId);
    return regionId;
  } catch {
    return null;
  }
}

export function AuthProvider({ children }: { children: ReactNode }) {
  const [state, setState] = useState<AuthState>(() => ({
    isAuthenticated: !!storage.getToken(),
    token: storage.getToken(),
    userId: storage.getUserId(),
    regionId: storage.getRegionId(),
    isLoading: false,
    error: null,
  }));

  useEffect(() => {
    if (state.token) {
      storage.setToken(state.token);
    }
  }, [state.token]);

  const login = async (phone: string, verifyCode: string): Promise<LoginResponse> => {
    setState((prev) => ({ ...prev, isLoading: true, error: null }));
    try {
      const data = await authService.loginWithPhone(phone, verifyCode);
      const regionId = data.user_id ? await fetchRegionId(data.user_id) : null;
      setState({
        isAuthenticated: true,
        token: data.token,
        userId: data.user_id,
        regionId,
        isLoading: false,
        error: null,
      });
      return data;
    } catch (error: any) {
      setState((prev) => ({
        ...prev,
        isLoading: false,
        error: error.response?.data?.message ?? '登录失败，请重试',
      }));
      throw error;
    }
  };

  const wechatLogin = async (code: string): Promise<WechatLoginResponse> => {
    setState((prev) => ({ ...prev, isLoading: true, error: null }));
    try {
      const data = await authService.loginWithWechat(code);
      if (data.token) {
        const regionId = data.user_id ? await fetchRegionId(data.user_id) : null;
        setState({
          isAuthenticated: true,
          token: data.token,
          userId: data.user_id ?? null,
          regionId,
          isLoading: false,
          error: null,
        });
      } else {
        setState((prev) => ({ ...prev, isLoading: false }));
      }
      return data;
    } catch (error: any) {
      setState((prev) => ({
        ...prev,
        isLoading: false,
        error: error.response?.data?.message ?? '微信登录失败',
      }));
      throw error;
    }
  };

  const bindPhone = async (openid: string, phone: string, verifyCode: string): Promise<BindPhoneResponse> => {
    setState((prev) => ({ ...prev, isLoading: true, error: null }));
    try {
      const data = await authService.bindPhoneAndLogin(openid, phone, verifyCode);
      const regionId = data.user_id ? await fetchRegionId(data.user_id) : null;
      setState({
        isAuthenticated: true,
        token: data.token,
        userId: data.user_id,
        regionId,
        isLoading: false,
        error: null,
      });
      return data;
    } catch (error: any) {
      setState((prev) => ({
        ...prev,
        isLoading: false,
        error: error.response?.data?.message ?? '绑定失败',
      }));
      throw error;
    }
  };

  const logout = () => {
    authService.logout();
    setState(initialState);
  };

  const sendCode = async (phone: string) => {
    try {
      await authService.sendCode(phone);
    } catch (error: any) {
      setState((prev) => ({
        ...prev,
        error: error.response?.data?.message ?? '发送验证码失败',
      }));
      throw error;
    }
  };

  return (
    <AuthContext.Provider value={{ ...state, login, wechatLogin, bindPhone, logout, sendCode }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}
