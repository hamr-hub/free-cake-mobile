import { AuthProvider } from "@refinedev/core";

const API_URL = "/api";

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
    if (token) {
      return { authenticated: true };
    }
    return { authenticated: false, logout: true, redirectTo: "/login" };
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
      return { logout: true, redirectTo: "/login" };
    }
    return { error };
  },
};

export default authProvider;
