import {createApp} from "vue";
import {createPinia} from 'pinia'
import { createWebHistory, createRouter } from 'vue-router'
import {routes} from "vue-router/auto-routes";
import App from "./App.vue";
import './style.css'
export const router = createRouter({
    history: createWebHistory(),
    routes,
})

const pinia = createPinia()
const app = createApp(App)
app.use(router)
app.use(pinia)
app.mount('#app')
