<template>
  <v-menu>
    <template v-slot:activator="{ props }">
      <v-btn v-bind="props" icon variant="text" class="scale-150">
        <v-avatar size="36" color="primary">
          <span class="text-sm font-medium">{{ userInitial }}</span>
        </v-avatar>
      </v-btn>
    </template>
    <v-list class="!bg-zinc-800">
      <v-list-item>
        <v-list-item-title class="text-gray-300">{{ userEmail }}</v-list-item-title>
      </v-list-item>
      <v-divider />
      <v-list-item to="/settings">
        <template v-slot:prepend>
          <v-icon icon="mdi-cog" class="mr-2" />
        </template>
        <v-list-item-title>Settings</v-list-item-title>
      </v-list-item>
      <v-list-item @click="logout">
        <template v-slot:prepend>
          <v-icon icon="mdi-logout" class="mr-2" />
        </template>
        <v-list-item-title>Sign Out</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useUserStore } from "@/stores/user";
import { authService } from "@/services/auth";

const userStore = useUserStore();

const userEmail = computed(() => userStore.email || "");

const userInitial = computed(() => {
  const email = userStore.email;
  if (!email) return "?";
  return email.charAt(0).toUpperCase();
});

const logout = () => {
  authService.logout();
  window.location.href = "/";
};
</script>
