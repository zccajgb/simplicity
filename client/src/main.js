import './assets/main.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'

import App from './App.vue'
import router from './router'

import vue3GoogleLogin from 'vue3-google-login';
import store from './store'

const app = createApp(App)

app.use(vue3GoogleLogin, {
  clientId: '921147596634-f7ecgkskgigfps0snjbmb31ur4qrd5cd.apps.googleusercontent.com'
})

app.use(store);
app.use(createPinia())
app.use(router)

app.mount('#app')
