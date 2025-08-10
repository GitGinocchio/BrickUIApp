import { NNotificationProvider } from "naive-ui";
import App from "./App.vue";
import * as Vue from "vue";
import { expose } from "./apis";

expose();

const app = Vue.createApp({
  setup() {
    return () => Vue.h(NNotificationProvider, {
        placement : "top"
    }, { default: () => Vue.h(App) });
  }
});

app.mount("#app");
