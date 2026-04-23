import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { storage } from '../services/storage';
import * as authService from '../services/auth';
import { LoginResponse } from '../types/user';

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
  logout: () => {},
  sendCode: async () => {},
});

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
      setState({
        isAuthenticated: true,
        token: data.token,
        userId: data.user_id,
        regionId: data.region_id ?? null,
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
    <AuthContext.Provider value={{ ...state, login, logout, sendCode }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth(): AuthContextType {
  return useContext(AuthContext);
}
