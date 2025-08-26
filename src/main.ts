import { createApp } from "vue";
import { createNaiveUi } from "./naive";
import { router } from './router'

import App from "./App.vue";
import i18n from "./i18n";


const app = createApp(App);
app.use(router)
app.use(createNaiveUi());
app.use(i18n);
app.mount("#app");