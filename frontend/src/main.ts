import { createApp } from 'vue'
import './styles/main.scss'
import App from './App.vue'
import { router } from './router'
import { createVuetify } from 'vuetify'
import * as components from 'vuetify/components'
import * as directives from 'vuetify/directives'



// Icons
import '@mdi/font/css/materialdesignicons.css'
const vuetify = createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'light',
        themes: {
            light: {
                colors: {
                    primary: '#1976d2',
                    secondary: '#03a9f4',
                    background: '#f5f5f5',
                },
            },
        },
    },
})

createApp(App)
    .use(vuetify)
    .use(router)
    .mount('#app')