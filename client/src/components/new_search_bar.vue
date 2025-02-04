<template>
    <div class="search-container">
      <form @submit.prevent="handleSubmit" class="search-form">
        <div class="search-input-wrapper">
          <input
            type="text"
            v-model="searchQuery"
            :placeholder="`Search the web`"
            class="search-input"
            aria-label="Search query"
          />
        </div>
        <div class="search-controls">
          <div class="search-engine-select" @click="toggleDropdown" ref="dropdown">
            <img :src="getEngineIcon(selectedEngine)" :alt="selectedEngine" class="engine-icon" />
            <div class="dropdown" v-show="isDropdownOpen">
              <button
                v-for="engine in searchEngines"
                :key="engine"
                @click="selectEngine(engine)"
                class="dropdown-item"
                :class="{ 'selected': engine === selectedEngine }"
              >
                <img :src="getEngineIcon(engine)" :alt="engine" class="engine-icon" />
                <span>{{ engine }}</span>
              </button>
            </div>
          </div>
          <button type="submit" class="search-button" aria-label="Perform search">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="search-icon"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
          </button>
        </div>
      </form>
    </div>
  </template>
  
  <script setup>
  import { ref, onMounted, onUnmounted } from 'vue';
  
  const searchQuery = ref('');
  const selectedEngine = ref('Google');
  const isDropdownOpen = ref(false);
  const dropdown = ref(null);
  
  const searchEngines = ['Google', 'Perplexity', 'Bing'];
  
  const toggleDropdown = () => {
    isDropdownOpen.value = !isDropdownOpen.value;
  };
  
  const selectEngine = (engine) => {
    selectedEngine.value = engine;
    isDropdownOpen.value = false;
  };
  
  const getEngineIcon = (engine) => {
    switch (engine) {
      case 'Google':
        return 'https://www.google.com/favicon.ico';
      case 'Perplexity':
        return 'https://www.perplexity.ai/favicon.ico';
      case 'Bing':
        return 'https://www.bing.com/favicon.ico';
      default:
        return '';
    }
  };
  
  const handleSubmit = () => {
    let searchUrl;
    switch (selectedEngine.value) {
      case 'Google':
        searchUrl = `https://www.google.com/search?q=${encodeURIComponent(searchQuery.value)}`;
        break;
      case 'Perplexity':
        searchUrl = `https://www.perplexity.ai/?q=${encodeURIComponent(searchQuery.value)}`;
        break;
      case 'Bing':
        searchUrl = `https://www.bing.com/search?q=${encodeURIComponent(searchQuery.value)}`;
        break;
    }
    window.open(searchUrl, '_blank');
  };
  
  const handleClickOutside = (event) => {
    if (dropdown.value && !dropdown.value.contains(event.target)) {
      isDropdownOpen.value = false;
    }
  };
  
  onMounted(() => {
    document.addEventListener('click', handleClickOutside);
  });
  
  onUnmounted(() => {
    document.removeEventListener('click', handleClickOutside);
  });
  </script>
  
  <style scoped>
  :root {
    --primary-color: #4a90e2;
    --text-color: #e0e0e0;
    --background-color: #2a2a2a;
    --border-color: #404040;
    --hover-color: #353535;
  }
  
  .search-container {
    width: 100%;
    max-width: 600px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .search-form {
    display: flex;
    flex-direction: column;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    border-radius: 24px;
    overflow: hidden;
    transition: box-shadow 0.3s ease;
    background-color: #404040;
  }
  
  .search-form:focus-within {
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
  }
  
  .search-input-wrapper {
    display: flex;
    align-items: center;
    background-color: var(--background-color);
  }
  
  .search-input {
    flex-grow: 1;
    border: none;
    padding: 16px;
    font-size: 16px;
    color: var(--text-color);
    outline: none;
  }
  
  .search-controls {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px;
    background-color: var(--background-color);
    border-top: 1px solid var(--border-color);
  }
  
  .search-engine-select {
    position: relative;
    cursor: pointer;
    user-select: none;
    display: flex;
    align-items: center;
    color: var(--text-color);
  }
  
  .engine-icon {
    width: 24px;
    height: 24px;
    object-fit: contain;
  }
  
  .dropdown {
    position: absolute;
    top: 100%;
    left: 0;
    background-color: #2a2a2a;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
    z-index: 10;
    min-width: 150px;
  }
  
  .dropdown-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 8px 16px;
    text-align: left;
    background: none;
    border: none;
    cursor: pointer;
    color: var(--text-color);
    transition: background-color 0.2s ease;
  }
  
  .dropdown-item:hover,
  .dropdown-item:focus {
    background-color: var(--hover-color);
  }
  
  .dropdown-item.selected {
    font-weight: bold;
    color: var(--primary-color);
  }
  
  .dropdown-item .engine-icon {
    margin-right: 8px;
  }
  
  .search-button {
    background-color: #404040;
    border: none;
    padding: 8px 16px;
    cursor: pointer;
    transition: background-color 0.2s ease;
    border-radius: 20px;
  }
  
  .search-button:hover,
  .search-button:focus {
    background-color: #505050;
  }
  
  .search-icon {
    width: 20px;
    height: 20px;
    color: var(--background-color);
  }
  
  @media (max-width: 480px) {
    .search-form {
      border-radius: 12px;
    }
  
    .search-controls {
      flex-direction: column;
      align-items: stretch;
    }
  
    .search-engine-select {
      margin-bottom: 8px;
    }
  
    .search-button {
      width: 100%;
    }
  }
  </style>