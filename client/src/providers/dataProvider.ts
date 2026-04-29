import { DataProvider, CustomResponse } from "@refinedev/core";

const API_URL = "/api";

async function request<T>(
  url: string,
  options: RequestInit = {}
): Promise<T> {
  const headers: Record<string, string> = {
    ...getAuthHeaders(),
    ...(options.headers as Record<string, string> || {}),
  };
  if (options.body && !headers["Content-Type"]) {
    headers["Content-Type"] = "application/json";
  }

  const response = await fetch(url, { ...options, headers });

  if (response.status === 401) {
    const error = new Error("Unauthorized");
    (error as any).statusCode = 401;
    throw error;
  }

  if (!response.ok) {
    const errorBody = await response.json().catch(() => null);
    const message = errorBody?.error || errorBody?.message || `Request failed: ${response.status}`;
    throw new Error(message);
  }

  return response.json();
}

function getAuthHeaders(): Record<string, string> {
  const token = localStorage.getItem("token");
  return token ? { Authorization: `Bearer ${token}` } : {};
}

function buildResourceUrl(resource: string): string {
  const specialResources: Record<string, string> = {
    "dashboard/stats": "dashboard/stats",
    "votes/risk": "votes/risk",
    "settlement": "settlement",
    "production": "production",
    "redeem": "redeem",
    "audit_log": "audit_log",
    "entries": "entries",
  };
  const path = specialResources[resource] || resource;
  return `${API_URL}/${path}`;
}

export const dataProvider: DataProvider = {
  getList: async ({ resource, pagination, filters, sorters }) => {
    const page = pagination?.currentPage ?? 1;
    const pageSize = pagination?.pageSize || 10;
    const params = new URLSearchParams();
    params.set("page", page.toString());
    params.set("page_size", pageSize.toString());

    if (sorters && sorters.length > 0) {
      params.set("sort", sorters[0].field as string);
      params.set("order", sorters[0].order as string);
    }

    if (filters) {
      for (const f of filters) {
        if (f.operator === "eq" && f.value !== undefined && f.value !== null) {
          params.set(f.field as string, String(f.value));
        }
      }
    }

    const url = `${buildResourceUrl(resource)}?${params.toString()}`;
    const data = await request<{ list: any[]; total: number }>(url);

    return {
      data: data.list || [],
      total: data.total || 0,
    };
  },

  getOne: async ({ resource, id }) => {
    const data = await request<any>(`${buildResourceUrl(resource)}/${id}`);
    return { data };
  },

  create: async ({ resource, variables }) => {
    const data = await request<any>(buildResourceUrl(resource), {
      method: "POST",
      body: JSON.stringify(variables),
    });
    return { data };
  },

  update: async ({ resource, id, variables }) => {
    const data = await request<any>(`${buildResourceUrl(resource)}/${id}`, {
      method: "PUT",
      body: JSON.stringify(variables),
    });
    return { data };
  },

  deleteOne: async ({ resource, id }) => {
    const data = await request<any>(`${buildResourceUrl(resource)}/${id}`, {
      method: "DELETE",
    });
    return { data };
  },

  custom: async ({ url, method, payload, query, headers }) => {
    let requestUrl = url || "";
    if (query) {
      const params = new URLSearchParams();
      for (const [k, v] of Object.entries(query)) {
        if (v !== undefined && v !== null) params.set(k, String(v));
      }
      requestUrl += `?${params.toString()}`;
    }

    const options: RequestInit = { method: method as string || "GET" };
    if (headers) options.headers = headers as Record<string, string>;
    if (payload) options.body = JSON.stringify(payload);

    const data = await request<any>(requestUrl.startsWith("/") ? `${API_URL}${requestUrl}` : `${API_URL}/${requestUrl}`, options);
    return { data } as CustomResponse<any>;
  },

  getApiUrl: () => API_URL,
};

export default dataProvider;
