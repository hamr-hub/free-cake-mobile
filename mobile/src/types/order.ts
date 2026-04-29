export interface Order {
  id: number;
  order_type: string | null;
  amount: number | null;
  pay_status: string | null;
  refund_status: string | null;
  created_at: string;
}

export interface OrderDetail {
  id: number;
  winner_id: number;
  store_id: number;
  template_id: number;
  order_type: string;
  amount: number;
  pay_status: string;
  production_status: string;
  redeem_status: string;
  refund_status: string | null;
  refund_reason: string | null;
  redeem_code: string | null;
  created_at: string;
  paid_at: string | null;
}

export interface CreateOrderRequest {
  entry_id: number;
  cake_size: string;
  cream_type: string;
  store_id: number;
}

export interface CreateOrderResponse {
  order_id: number;
  amount: number;
  pay_status: string;
  prepay_id?: string;
  prepay_params?: {
    appid: string;
    partnerid: string;
    prepayid: string;
    package: string;
    noncestr: string;
    timestamp: string;
    sign_type: string;
    pay_sign: string;
  };
}

export interface OrderListResponse {
  list: Order[];
  total: number;
}
