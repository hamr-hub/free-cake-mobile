import { ContestEntry } from './entry';
import { VoteRecord } from './vote';
import { RedeemCode } from './redeem';

export interface User {
  id: number;
  phone: string;
  nickname: string;
  region_id: number;
  region_name: string;
  avatar_url: string;
}

export interface LoginRequest {
  phone: string;
  verify_code: string;
}

export interface LoginResponse {
  token: string;
  user_id: number;
  role: string;
  region_id: number | null;
}

export interface SendCodeRequest {
  phone: string;
}

export interface SendCodeResponse {
  success: boolean;
  expire_in: number;
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
