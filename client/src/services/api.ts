import { useUserStore } from "@/stores/user";
// src/services/api.ts
import axios, { type AxiosInstance } from "axios";

const api: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || "http://localhost:3000",
  timeout: 10000, // Optional
});

// Request interceptor to add custom headers
api.interceptors.request.use(
  (config) => {
    const userStore = useUserStore();
    // Add your custom headers here
    config.headers.set("X-User-Id", userStore.userId);
    config.headers.set("X-User-Email", userStore.email);
    config.headers.set(
      "Authorization",
      `Bearer ${localStorage.getItem("token") || ""}`,
    );
    config.headers.set("Content-Type", "application/json");
    return config;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// Response interceptor for token refresh and error handling
api.interceptors.response.use(
  (response) => {
    // Check for new token in response headers (auto-refresh from backend)
    const newToken = response.headers["x-new-auth-token"];
    if (newToken) {
      localStorage.setItem("token", newToken);
    }
    return response;
  },
  (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid - clear and redirect
      localStorage.removeItem("token");
      const userStore = useUserStore();
      userStore.clearUser();
      window.location.href = "/";
    }
    return Promise.reject(error);
  },
);

export default api;
