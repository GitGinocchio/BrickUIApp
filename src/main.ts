import { createApp } from "vue";
import App from "./App.vue";
import { createNaiveUi } from "./naive";


const app = createApp(App);
app.mount("#app");
app.use(createNaiveUi())