import { createI18n } from "vue-i18n";
import en from "./locales/en.json";
import it from "./locales/it.json";

const i18n = createI18n({
  legacy: false,
  locale: "en",
  fallbackLocale: "en",
  messages: {
    en,it
  },
});

export default i18n;
