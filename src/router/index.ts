import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        redirect: '/Landing-Page'
    },
    {
        path: '/Landing-Page',
        component: () => import('../views/Landing-Page.vue')
    },
    {
        path: '/Settings',
        component: () => import('../views/Settings.vue')
    },
    {
        path: '/Script-Upload',
        component: () => import('../views/Script-Upload.vue')
    },
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router