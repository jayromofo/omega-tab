<template>
  <div class="plans-page min-h-screen bg-gray-900 py-12 px-4">
    <div class="container mx-auto">
      <div class="mb-8">
        <v-btn @click="handleBack" color="primary" variant="text" prepend-icon="mdi-arrow-left">
          Back
        </v-btn>
      </div>

      <h1 class="text-center text-4xl font-bold mb-12">Choose Your Plan</h1>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
        <!-- Free Plan -->
        <v-card class="plan-card" variant="outlined">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Free</v-card-title>
            <div class="text-h3 mb-1">$0</div>
            <div class="text-caption mb-4">forever</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Up to 6 pinned items</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Basic search functionality</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Single personal launch page</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="primary" variant="flat">Get Started</v-btn>
          </v-card-actions>
        </v-card>

        <!-- Plus Plan -->
        <v-card class="plan-card" variant="outlined">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Plus</v-card-title>
            <div class="text-h3 mb-1">$8</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Up to 20 pinned items</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Advanced search with history</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Multiple launch pages</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Custom themes</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="primary" variant="flat">Upgrade to Plus</v-btn>
          </v-card-actions>
        </v-card>

        <!-- Team Plan -->
        <v-card class="plan-card" color="primary">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Team</v-card-title>
            <div class="text-h3 mb-1">$8</div>
            <div class="text-caption mb-4">per user/month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list bg-color="primary">
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Up to 50 pinned items per page</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Team-wide search history</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Shared launch pages</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Team analytics</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Admin controls</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="white" variant="flat">Start Team Trial</v-btn>
          </v-card-actions>
        </v-card>

        <!-- Enterprise Plan -->
        <v-card class="plan-card" variant="outlined">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Enterprise</v-card-title>
            <div class="text-h3 mb-1">Custom</div>
            <div class="text-caption mb-4">contact sales</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Unlimited pinned items</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Custom integrations</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Advanced security features</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Dedicated support</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Custom deployment options</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="primary" variant="flat">Contact Sales</v-btn>
          </v-card-actions>
        </v-card>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
const router = useRouter();
const handleBack = () => {
  router.go(-1);
};
</script>

<style scoped>
.plan-card {
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: transform 0.2s;
}

.plan-card:hover {
  transform: translateY(-5px);
}
</style>