import i18next from 'i18next';
import { initReactI18next } from 'react-i18next';
import LanguageDetector from 'i18next-browser-languagedetector';

import en from './locales/en/common.json';
import ptBR from './locales/pt-BR/common.json';
import es from './locales/es/common.json';
import nl from './locales/nl/common.json';
import fr from './locales/fr/common.json';

const resources = {
  en: {
    common: en,
  },
  'pt-BR': {
    common: ptBR,
  },
  es: {
    common: es,
  },
  nl: {
    common: nl,
  },
  fr: {
    common: fr,
  },
};

i18next
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'en',
    defaultNS: 'common',
    interpolation: {
      escapeValue: false,
    },
  });

export default i18next;

