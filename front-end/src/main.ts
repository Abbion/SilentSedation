/*REFACTOR 4.0*/
import './assets/main.css'

import { createApp } from 'vue'
import App from './App.vue'

import { createRouter, createMemoryHistory } from 'vue-router'

import LoginPage from './components/LogInPage.vue'
import UserPage from './components/UserPage.vue'

const routes = [
    { path: '/', component: LoginPage },
    { path: '/userPage', component: UserPage }
];

const router = createRouter({
    history: createMemoryHistory(),
    routes,
});

createApp(App).use(router).mount('#app')