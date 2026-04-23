import { MMKV } from 'react-native-mmkv';

const mmkv = new MMKV();

const KEYS = {
  TOKEN: 'auth_token',
  USER_ID: 'user_id',
  ROLE: 'user_role',
  REGION_ID: 'region_id',
  REDEEM_CODES: 'redeem_codes_cache',
  LAST_LOCATION: 'last_location',
};

export const storage = {
  setToken(token: string): void {
    mmkv.set(KEYS.TOKEN, token);
  },
  getToken(): string | null {
    return mmkv.getString(KEYS.TOKEN) ?? null;
  },
  clearToken(): void {
    mmkv.delete(KEYS.TOKEN);
  },

  setUserId(id: number): void {
    mmkv.set(KEYS.USER_ID, id);
  },
  getUserId(): number | null {
    return mmkv.getNumber(KEYS.USER_ID) ?? null;
  },
  clearUserId(): void {
    mmkv.delete(KEYS.USER_ID);
  },

  setRole(role: string): void {
    mmkv.set(KEYS.ROLE, role);
  },
  getRole(): string | null {
    return mmkv.getString(KEYS.ROLE) ?? null;
  },
  clearRole(): void {
    mmkv.delete(KEYS.ROLE);
  },

  setRegionId(id: number): void {
    mmkv.set(KEYS.REGION_ID, id);
  },
  getRegionId(): number | null {
    return mmkv.getNumber(KEYS.REGION_ID) ?? null;
  },
  clearRegionId(): void {
    mmkv.delete(KEYS.REGION_ID);
  },

  cacheRedeemCodes(data: string): void {
    mmkv.set(KEYS.REDEEM_CODES, data);
  },
  getCachedRedeemCodes(): string | null {
    return mmkv.getString(KEYS.REDEEM_CODES) ?? null;
  },

  cacheLocation(data: string): void {
    mmkv.set(KEYS.LAST_LOCATION, data);
  },
  getCachedLocation(): string | null {
    return mmkv.getString(KEYS.LAST_LOCATION) ?? null;
  },

  clearAll(): void {
    mmkv.clearAll();
  },
};
