import { createApp } from "vue"
import App from "./App.vue"
import { createPinia } from 'pinia'
import 'element-plus/dist/index.css'
import { router } from "./routes/router"

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

app.mount("#app")
