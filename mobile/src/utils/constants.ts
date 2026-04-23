export const API_BASE_URL = 'http://localhost:3000/api';
export const AI_GENERATE_TIMEOUT = 30000;
export const DEFAULT_TIMEOUT = 10000;
export const MAX_VOTES_PER_DAY = 3;
export const AI_GENERATE_RATE_LIMIT = 5;
export const MAX_RETRY_COUNT = 3;
export const RETRY_DELAY_MS = 1000;
export const CACHE_TTL_MS = 10000;
export const REGION_RADIUS_KM = 10;
export const REDEEM_CODE_CACHE_TTL_MS = 3600000;
export const SUPPORTED_PLATFORMS = ['ios', 'android'] as const;

export type SupportedPlatform = typeof SUPPORTED_PLATFORMS[number];

export const SCENES = ['birthday', 'children', 'festival', 'wedding', 'other'] as const;
export type Scene = typeof SCENES[number];

export const STYLES = ['cartoon', 'realistic', 'minimal', 'romantic', 'traditional'] as const;
export type Style = typeof STYLES[number];

export const COLOR_PREFERENCES = ['warm', 'cool', 'pastel', 'vivid', 'natural'] as const;
export type ColorPreference = typeof COLOR_PREFERENCES[number];

export const ACTIVITY_STATUS = [
  'draft',
  'pending_publish',
  'registration_open',
  'voting_open',
  'voting_closed',
  'settled',
  'redeeming',
  'finished',
] as const;
export type ActivityStatus = typeof ACTIVITY_STATUS[number];

export const VOTE_STATUS = ['valid', 'pending_review', 'invalid', 'frozen'] as const;
export type VoteStatus = typeof VOTE_STATUS[number];

export const REDEEM_STATUS = ['unused', 'used', 'expired'] as const;
export type RedeemStatus = typeof REDEEM_STATUS[number];
