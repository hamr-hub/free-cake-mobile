import axios, { AxiosInstance, AxiosError, InternalAxiosRequestConfig } from 'axios';
import { API_BASE_URL, DEFAULT_TIMEOUT, AI_GENERATE_TIMEOUT, MAX_RETRY_COUNT, RETRY_DELAY_MS } from '../utils/constants';
import { storage } from './storage';

let refreshPromise: Promise<string | null> | null = null;

async function getOrRefreshToken(): Promise<string | null> {
  if (refreshPromise) return refreshPromise;

  refreshPromise = (async () => {
    const token = storage.getToken();
    if (!token) return null;
    try {
      const res = await axios.post(`${API_BASE_URL}/auth/refresh`, {}, {
        headers: { Authorization: `Bearer ${token}` },
      });
      if (res.data?.token) {
        storage.setToken(res.data.token);
        return res.data.token;
      }
      return null;
    } catch {
      storage.clearToken();
      return null;
    } finally {
      refreshPromise = null;
    }
  })();

  return refreshPromise;
}

const apiClient: AxiosInstance = axios.create({
  baseURL: API_BASE_URL,
  timeout: DEFAULT_TIMEOUT,
  headers: {
    'Content-Type': 'application/json',
  },
});

apiClient.interceptors.request.use((config: InternalAxiosRequestConfig) => {
  const token = storage.getToken();
  if (token && config.headers) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

apiClient.interceptors.response.use(
  (response) => response,
  async (error: AxiosError) => {
    const config = error.config as InternalAxiosRequestConfig & { _retry?: number };

    if (error.response?.status === 401 && config._retry !== 1) {
      config._retry = 1;
      const newToken = await getOrRefreshToken();
      if (newToken && config.headers) {
        config.headers.Authorization = `Bearer ${newToken}`;
        return apiClient(config);
      }
      storage.clearToken();
      return Promise.reject(error);
    }

    if (config._retry === undefined) {
      config._retry = 0;
    }

    if (config._retry < MAX_RETRY_COUNT && (!error.response || error.response.status >= 500)) {
      config._retry += 1;
      const delay = RETRY_DELAY_MS * Math.pow(2, config._retry - 1);
      await new Promise((resolve) => setTimeout(resolve, delay));
      return apiClient(config);
    }

    return Promise.reject(error);
  }
);

export async function login(phone: string, verifyCode: string) {
  const response = await apiClient.post('/auth/login', { phone, verify_code: verifyCode });
  return response.data;
}

export async function sendVerifyCode(phone: string) {
  const response = await apiClient.post('/auth/send-verify-code', { phone });
  return response.data;
}

export async function getActivities(page: number = 1, pageSize: number = 10, status?: string, regionId?: number) {
  const params: Record<string, unknown> = { page, page_size: pageSize };
  if (status) params.status = status;
  if (regionId) params.region_id = regionId;
  const response = await apiClient.get('/activities', { params });
  return response.data;
}

export async function getActivityDetail(id: number) {
  const response = await apiClient.get(`/activities/${id}`);
  return response.data;
}

export async function generateEntries(activityId: number, data: { scene: string; theme: string; blessing: string; color_preference: string; style: string }) {
  const response = await apiClient.post(`/activities/${activityId}/entries/generate`, data, {
    timeout: AI_GENERATE_TIMEOUT,
  });
  return response.data;
}

export async function submitEntry(activityId: number, data: { selected_generation_id: number; selected_template_id: number; title: string }) {
  const response = await apiClient.post(`/activities/${activityId}/entries`, data);
  return response.data;
}

export async function castVote(entryId: number, activityId: number) {
  const response = await apiClient.post(`/entries/${entryId}/vote`, { activity_id: activityId });
  return response.data;
}

export async function getRankList(activityId: number, page: number = 1, pageSize: number = 20) {
  const response = await apiClient.get(`/activities/${activityId}/rank`, { params: { page, page_size: pageSize } });
  return response.data;
}

export async function getUserProfile() {
  const response = await apiClient.get('/users/me');
  return response.data;
}

export async function getRedeemDetail(code: string) {
  const response = await apiClient.get(`/redeem/${code}`);
  return response.data;
}

export async function getEntryDetail(entryId: number) {
  const response = await apiClient.get(`/entries/${entryId}`);
  return response.data;
}

export async function resolveRegion(lat: number, lng: number) {
  const response = await apiClient.get('/users/resolve-region', { params: { lat, lng } });
  return response.data;
}

export async function createPaidOrder(data: { entry_id: number; cake_size: string; cream_type: string; store_id: number }) {
  const response = await apiClient.post('/orders', data);
  return response.data;
}

export async function getOrderDetail(orderId: number) {
  const response = await apiClient.get(`/orders/${orderId}`);
  return response.data;
}

export async function initPay(orderId: number) {
  const response = await apiClient.post(`/orders/${orderId}/init-pay`);
  return response.data;
}

export async function wechatLogin(code: string) {
  const response = await apiClient.post('/auth/wechat-login', { code });
  return response.data;
}

export async function bindPhone(data: { openid: string; phone: string; verify_code: string }) {
  const response = await apiClient.post('/auth/bind-phone', data);
  return response.data;
}

export async function getActivityRules(activityId: number) {
  const response = await apiClient.get(`/activities/${activityId}/rules`);
  return response.data;
}

export async function getPrices(params: { region_id?: number }) {
  const response = await apiClient.get('/prices', { params });
  return response.data;
}

export async function getStores(params: { region_id?: number }) {
  const response = await apiClient.get('/stores', { params });
  return response.data;
}

export { apiClient };
