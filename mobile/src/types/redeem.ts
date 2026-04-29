import { RedeemStatus } from '../utils/constants';

export interface RedeemCode {
  code: string;
  status: RedeemStatus;
  order_id: number;
  store_address: string;
  cake_size: string;
  cream_type: string;
  expires_at: string;
}

export interface RedeemVerifyRequest {
  redeem_code: string;
  phone: string;
  store_id: number;
}

export interface RedeemVerifyResponse {
  success: boolean;
  order_id: number;
  fail_reason: string | null;
}
