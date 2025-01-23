<script setup lang="ts">

  import { ref } from 'vue';
  import SearchBar from './components/SearchBar.vue';
  import LinkColumns from './components/LinkColumns.vue';
  import LinkCard from './components/LinkCard.vue';
  import WeatherAndTime from './components/WeatherAndTime.vue';

  import type { iLinkCard } from './types/LinkCard';

  const tools: iLinkCard[] = [
    { icon: 'mdi-gmail', title: 'Gmail', description: 'Email service', link: 'https://mail.google.com' },
    { icon: 'mdi-calendar', title: 'Google Calendar', description: 'Calendar service', link: 'https://calendar.google.com' },
  ];

  const docs: iLinkCard[] = [
    { icon: 'mdi-file-document', title: 'Google Docs', description: 'Document creation', link: 'https://docs.google.com' },
    { icon: 'mdi-google-spreadsheet', title: 'Google Sheets', description: 'Spreadsheet creation', link: 'https://sheets.google.com' },
    { icon: 'mdi-google-drive', title: 'Google Drive', description: 'File storage', link: 'https://drive.google.com' },
  ];

  const news: iLinkCard[] = [
    { icon: 'mdi-information', title: '"Company News"', description: 'Relevant news for all employees will display here periodically once integrated.', link: 'https://google.com' },
  ];

  const showHelpDialog = ref(false);

  // const keyboardShortcuts = computed(() => {
  const toolShortcuts = tools.map((tool, index) => ({
    shortcut: `Ctrl+${index + 1}`,
    description: `Open ${tool.title}`
  }));

  const docShortcuts = docs.map((doc, index) => ({
    shortcut: `Alt+${index + 1}`,
    description: `Open ${doc.title}`
  }));

  const news_shortcut = "";
</script>

<template>
  <v-theme-provider theme="dark">
    <div class="header">
      <h1 class="mt-16 text-3xl">
        <v-icon icon="mdi-rocket" size="24" />
        Better New Tab
      </h1>
      <div class="WeatherAndTime">
        <WeatherAndTime location="Las Vegas" />
        <WeatherAndTime location="North Bay" />
      </div>
      <v-btn icon="mdi-help" @click="showHelpDialog = true">
      </v-btn>

    </div>
    <SearchBar :tools="tools" :docs="docs" />
    <LinkColumns :tools="tools" :docs="docs" />
    <h2 class="mt-16">Company News</h2>
    <LinkCard v-for="(news, index) in news" :key="news.title" :icon="news.icon" :title="news.title"
      :description="news.description" :link="news.link" :index="index" :shortcut="news_shortcut" class="mb-2" disabled />

    <v-dialog v-model="showHelpDialog" max-width="900px">
      <v-card>
        <v-card-title class="headline">Help</v-card-title>
        <v-card-text>
          <h3 class="text-xl">Search Bar Controls</h3>
          <p>While in the search bar, type in a Jira Ticket number for relevant links, then use arrow keys or your mouse
            to navigate</p>
          <img src="/src/assets/screenshots/jira_ticket_example.webp" />
          <br />
          <h3 class="text-xl">Keyboard Shortcuts</h3>
          <br />
          <h4 class="text-lg"><v-icon icon="mdi-chevron-right" />Tools and Docs</h4>
          <v-row>
            <v-col>
              <ul>
                <li v-for="shortcut in toolShortcuts" :key="shortcut.shortcut">
                  <strong>{{ shortcut.shortcut }}</strong>:
                  {{ shortcut.description }}
                </li>
              </ul>
            </v-col>
            <v-col>
              <ul>
                <li v-for="shortcut in docShortcuts" :key="shortcut.shortcut">
                  <strong>{{ shortcut.shortcut }}</strong>:
                  {{ shortcut.description }}
                </li>
              </ul>
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn variant="tonal" @click="showHelpDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-theme-provider>

</template>

<style scoped>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .logo {
    display: block;
    margin: 0 auto 2rem;
  }

  img {
    width: 100%;
    height: auto;
    border: 1px solid transparent;
    border-radius: 12px;
  }

  .WeatherAndTime {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
  }

  ul {
    list-style-type: none;
    padding: 0;
  }

  li {
    margin-bottom: 0.5rem;
  }

  nav {
    width: 100%;
    font-size: 12px;
    text-align: center;
    margin-top: 2rem;
  }

  nav a.router-link-exact-active {
    color: var(--color-text);
  }

  nav a.router-link-exact-active:hover {
    background-color: transparent;
  }

  nav a {
    display: inline-block;
    padding: 0 1rem;
    border-left: 1px solid var(--color-border);
  }

  nav a:first-of-type {
    border: 0;
  }

  @media (min-width: 1024px) {
    header {
      display: flex;
      place-items: center;
      padding-right: calc(var(--section-gap) / 2);
    }

    .logo {
      margin: 0 2rem 0 0;
    }

    nav {
      text-align: left;
      margin-left: -1rem;
      font-size: 1rem;

      padding: 1rem 0;
      margin-top: 1rem;
    }
  }
</style>
