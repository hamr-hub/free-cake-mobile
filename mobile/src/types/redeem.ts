import { RedeemStatus } from '../utils/constants';

export interface RedeemCode {
  code: string;
  order_id: number;
  status: RedeemStatus;
  expire_at: string;
  store_address: string;
  store_distance: number;
  cake_name: string;
  cake_image_url: string;
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
