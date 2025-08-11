import { createApp } from "vue";
import { createNaiveUi } from "./naive";
import { router } from './router'

import App from "./App.vue";


const app = createApp(App);
app.use(router)
app.use(createNaiveUi());
app.mount("#app");