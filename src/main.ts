import { createApp } from 'vue'
import { createPinia } from 'pinia'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import App from './App.vue'
import router from './router'
import { vuetify } from './plugins/vuetify'
import VueMasonry from 'vue3-masonry-css'
import { reportFrontendError } from '@/telemetry/errors'

import './component-hooks'
import './plugins/masonry-css'
import '@mdi/font/css/materialdesignicons.css'

// 🧠 Global utility functions
import {
  applicationToString,
  stringToUTCString,
  conjugateStatusToString,
  lotStatusToString,
  validationStatusToString,
  tagStatusToString,
} from './utils/converters'

const isDev = import.meta.env.NODE_ENV === 'development'

const app = createApp(App)
const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

// 💡 Global property configuration
app.config.globalProperties.$productionTip = false
app.config.globalProperties.$performance = isDev
app.config.globalProperties.$applicationToString = applicationToString
app.config.globalProperties.$stringToUTCString = stringToUTCString
app.config.globalProperties.$validationStatusToString = validationStatusToString
app.config.globalProperties.$conjugateStatusToString = conjugateStatusToString
app.config.globalProperties.$lotStatusToString = lotStatusToString
app.config.globalProperties.$tagStatusToString = tagStatusToString
app.config.errorHandler = (err, instance, info) => {
  reportFrontendError(err, info as any)
}


  // only for dev
  ; (app.config as any).devtools = true

// 🔌 Plugins
app.use(pinia)
app.use(router)
app.use(vuetify)
app.use(VueMasonry)

// 🚀 Mount
app.mount('#app')

