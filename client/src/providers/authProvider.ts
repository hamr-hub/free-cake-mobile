import { AuthProvider } from "@refinedev/core";

const API_URL = "/api";

function decodeJwtPayload(token: string): { exp?: number; [key: string]: any } | null {
  try {
    const parts = token.split(".");
    if (parts.length !== 3) return null;
    const payload = parts[1];
    const decoded = atob(payload.replace(/-/g, "+").replace(/_/g, "/"));
    return JSON.parse(decoded);
  } catch {
    return null;
  }
}

function isTokenExpiringSoon(token: string, minutesThreshold: number = 5): boolean {
  const payload = decodeJwtPayload(token);
  if (!payload?.exp) return true;
  const now = Math.floor(Date.now() / 1000);
  return payload.exp - now < minutesThreshold * 60;
}

async function refreshToken(): Promise<string | null> {
  const token = localStorage.getItem("token");
  if (!token) return null;
  try {
    const response = await fetch(`${API_URL}/auth/refresh`, {
      method: "POST",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });
    if (response.ok) {
      const data = await response.json();
      localStorage.setItem("token", data.token);
      return data.token;
    }
  } catch {}
  return null;
}

export const authProvider: AuthProvider = {
  login: async ({ username, password }) => {
    try {
      const response = await fetch(`${API_URL}/auth/login`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ phone: username, verify_code: password }),
      });

      if (response.ok) {
        const data = await response.json();
        localStorage.setItem("token", data.token);
        localStorage.setItem("user_id", data.user_id);
        localStorage.setItem("role", data.role);
        return { success: true };
      }
      return { success: false, error: { message: "Login failed", name: "AuthError" } };
    } catch {
      return { success: false, error: { message: "Network error", name: "AuthError" } };
    }
  },

  logout: async () => {
    localStorage.removeItem("token");
    localStorage.removeItem("user_id");
    localStorage.removeItem("role");
    return { success: true };
  },

  check: async () => {
    const token = localStorage.getItem("token");
    if (!token) {
      return { authenticated: false, logout: true, redirectTo: "/login" };
    }

    const payload = decodeJwtPayload(token);
    if (!payload?.exp) {
      return { authenticated: false, logout: true, redirectTo: "/login" };
    }

    const now = Math.floor(Date.now() / 1000);
    if (payload.exp < now - 300) {
      // Token expired more than 5 min ago, force re-login
      return { authenticated: false, logout: true, redirectTo: "/login" };
    }

    if (isTokenExpiringSoon(token)) {
      const newToken = await refreshToken();
      if (!newToken) {
        return { authenticated: false, logout: true, redirectTo: "/login" };
      }
    }

    return { authenticated: true };
  },

  getPermissions: async () => {
    const role = localStorage.getItem("role");
    return role;
  },

  getIdentity: async () => {
    const token = localStorage.getItem("token");
    if (!token) return null;
    return {
      id: localStorage.getItem("user_id"),
      role: localStorage.getItem("role"),
    };
  },

  onError: async (error) => {
    if (error?.status === 401) {
      const newToken = await refreshToken();
      if (newToken) {
        return { retry: true };
      }
      return { logout: true, redirectTo: "/login" };
    }
    return { error };
  },
};

export default authProvider;
