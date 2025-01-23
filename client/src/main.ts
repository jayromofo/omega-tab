import "./assets/css/tailwind.css";

// Vuetify
import "vuetify/styles";
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";

import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "./App.vue";
// import router from "./router";
import "@mdi/font/css/materialdesignicons.css";

const app = createApp(App);

const vuetify = createVuetify({
	components,
	directives,
	theme: {
		defaultTheme: "dark",
		themes: {
			dark: {
				dark: true,
				colors: {
					primary: "#1867C0",
					secondary: "#5CBBF6",
				},
			},
		},
	},
	defaults: {
		global: {
			borderRadius: "12px", // Set your desired border radius here
		},
	},
});

app.use(createPinia());
app.use(vuetify);
// app.use(router);

app.mount("#app");
