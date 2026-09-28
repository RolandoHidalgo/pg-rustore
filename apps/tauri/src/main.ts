import {createApp} from "vue";
import {createPinia} from 'pinia'
import {createWebHistory, createRouter} from 'vue-router'
import {routes} from "vue-router/auto-routes";
import App from "./App.vue";
import './style.css'
import {useAppStore} from "@/stores/appStore.ts";

export const router = createRouter({
    history: createWebHistory(),
    routes,
})

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)
router.beforeEach(async (to, _from, next) => {

    const store = useAppStore()
    const isAuthenticated = store.isAuth



    if (to.path !== '/login' && !isAuthenticated) {

        next('/login') // redirige si no está autenticado
    } else if (to.path === '/login' && isAuthenticated) {
        next("/")
    } else {

        next() // permite la navegación
    }
})


app.use(router)

app.mount('#app')
