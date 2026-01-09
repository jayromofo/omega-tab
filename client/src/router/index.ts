import { API } from "@/constants/api";
import api from "@/services/api";
import { CacheKeys, cache } from "@/utils/cache";
// src/router/index.ts
import { createRouter, createWebHistory } from "vue-router";
import { useUserSettingsStore } from "../stores/settings";
import { useUserStore } from "../stores/user";

const isStaging = import.meta.env.VITE_STAGING === "true";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/Home.vue"),
      beforeEnter: (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
        } else {
          next();
        }
      },
    },
    {
      path: "/staging-login",
      name: "stagingLogin",
      component: () => import("../views/staging_login.vue"),
    },
    {
      path: "/contact",
      name: "contact",
      component: () => import("../views/Contact.vue"),
    },
    {
      path: "/privacy-policy",
      name: "privacyPolicy",
      component: () => import("../views/PrivacyPolicy.vue"),
    },
    {
      path: "/terms-of-service",
      name: "termsOfService",
      component: () => import("../views/TermsOfService.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/Settings.vue"),
      beforeEnter: async (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
          return;
        }

        const userStore = useUserStore();
        const userSettingsStore = useUserSettingsStore();

        // Check if user has a token
        const token = localStorage.getItem("token");
        if (!token) {
          next("/");
          return;
        }

        // if not logged in already in userStore (user probably refreshed page)
        if (!userStore.userId) {
          try {
            // Fetch user data from server
            const response = await api.get<{
              user: { id: string; email: string };
            }>(API.GET_USER_DATA);

            if (!response.data.user) {
              throw new Error("User not logged in");
            }

            const authUser = {
              id: response.data.user.id,
              email: response.data.user.email,
            };

            // Fetch user data asynchronously without blocking the UI
            userStore
              .fetchUserData(authUser)
              .then((success) => {
                if (!success) {
                  console.error("Failed to fetch user data");
                }
              })
              .catch((err) => {
                console.error("Error fetching user data:", err);
              });

            // always fetch settings with User
            await userSettingsStore.fetchSettings();

            next();
          } catch (err) {
            console.error(err);
            localStorage.removeItem("token");
            next("/");
          }
        } else {
          next();
        }
      },
    },
  ],
});

export default router;
