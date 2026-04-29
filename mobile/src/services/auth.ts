import { storage } from './storage';
import * as api from './api';
import { LoginResponse, WechatLoginResponse, BindPhoneResponse, SendCodeResponse } from '../types/user';

export async function loginWithPhone(phone: string, verifyCode: string): Promise<LoginResponse> {
  const data = await api.login(phone, verifyCode);
  if (data.token) {
    storage.setToken(data.token);
    storage.setUserId(data.user_id);
    storage.setRole(data.role);
    if (data.region_id) {
      storage.setRegionId(data.region_id);
    }
  }
  return data;
}

export async function loginWithWechat(code: string): Promise<WechatLoginResponse> {
  const data = await api.wechatLogin(code);
  if (data.token) {
    storage.setToken(data.token);
    storage.setUserId(data.user_id!);
    storage.setRole(data.role!);
    if (data.region_id) {
      storage.setRegionId(data.region_id);
    }
  }
  return data;
}

export async function bindPhoneAndLogin(openid: string, phone: string, verifyCode: string): Promise<BindPhoneResponse> {
  const data = await api.bindPhone({ openid, phone, verify_code: verifyCode });
  if (data.token) {
    storage.setToken(data.token);
    storage.setUserId(data.user_id);
    storage.setRole(data.role);
    if (data.region_id) {
      storage.setRegionId(data.region_id);
    }
  }
  return data;
}

export async function sendCode(phone: string): Promise<SendCodeResponse> {
  return await api.sendVerifyCode(phone);
}

export function isAuthenticated(): boolean {
  return !!storage.getToken();
}

export function getCurrentUserId(): number | null {
  return storage.getUserId();
}

export function getCurrentRegionId(): number | null {
  return storage.getRegionId();
}

export function logout(): void {
  storage.clearToken();
  storage.clearUserId();
  storage.clearRole();
  storage.clearRegionId();
}
