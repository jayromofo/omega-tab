<template>
  <div class="link-card-grid">
    <div>
      <h2 class="text-xl">Tools</h2>
      <LinkCard v-for="(tool, index) in tools" :key="tool.title" :icon="tool.icon" :title="tool.title"
        :description="tool.description" :link="tool.link" :index="index" :shortcut="ctrl" class="mb-2" />
    </div>
    <div>
      <h2 class="text-xl">Docs</h2>
      <LinkCard v-for="(doc, index) in docs" :key="doc.title" :icon="doc.icon" :title="doc.title"
        :description="doc.description" :link="doc.link" :index="index" :shortcut="alt" class="mb-2" />
    </div>
  </div>
</template>

<script setup lang="ts">
  import { defineProps, onMounted, onUnmounted } from 'vue';
  import LinkCard from './LinkCard.vue';
  import type { iLinkCard } from '../types/LinkCard';

  const ctrl = "ctrl";
  const alt = "alt";
  const props = defineProps<{
    tools: iLinkCard[];
    docs: iLinkCard[];
  }>();

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.ctrlKey) {
      const index = Number.parseInt(event.key) - 1;
      if (index >= 0 && index < props.tools.length) {
        window.open(props.tools[index].link, '_blank');
      }
    } else if (event.altKey) {
      const index = Number.parseInt(event.key) - 1;
      if (index >= 0 && index < props.docs.length) {
        window.open(props.docs[index].link, '_blank');
      }
    }
  };

  onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
  });

  onUnmounted(() => {
    window.removeEventListener('keydown', handleKeydown);
  });
</script>

<style scoped>
  .link-card-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    margin-top: 3rem;
    gap: 2rem;
  }
</style>