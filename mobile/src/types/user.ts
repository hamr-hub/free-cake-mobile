import { ContestEntry } from './entry';
import { VoteRecord } from './vote';
import { RedeemCode } from './redeem';

export interface User {
  id: number;
  phone: string;
  nickname: string;
  region_name: string;
}

export interface LoginRequest {
  phone: string;
  verify_code: string;
}

export interface LoginResponse {
  token: string;
  user_id: number;
  role: string;
}

export interface WechatLoginResponse {
  token?: string;
  user_id?: number;
  role?: string;
  openid?: string;
  need_bind_phone?: boolean;
}

export interface BindPhoneResponse {
  token: string;
  user_id: number;
  role: string;
}

export interface SendCodeRequest {
  phone: string;
}

export interface SendCodeResponse {
  success: boolean;
  expires_in: number;
}

export interface UserProfile {
  user: User;
  entries: ContestEntry[];
  votes: VoteRecord[];
  redeem_codes: RedeemCode[];
}

export interface LocationData {
  latitude: number;
  longitude: number;
  accuracy: number;
  region_id: number | null;
  region_name: string | null;
  is_in_range: boolean;
}
