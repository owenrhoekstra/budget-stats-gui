import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        redirect: '/Landing-Page'
    },
    {
        path: '/Landing-Page',
        component: () => import('../views/Landing Pages/Landing-Page.vue')
    },
    {
        path: '/Settings',
        component: () => import('../views/Settings.vue')
    },
    {
        path: '/Script-Upload',
        component: () => import('../views/Script-Upload.vue')
    },
    {
        path: '/View-Data-Landing',
        component: () => import('../views/Landing Pages/View-Data-Landing.vue')
    },
    {path: '/Change-Data-Landing',
        component: () => import('../views/Landing Pages/Change-Data-Landing.vue')
    },
    {
        path: '/Data-View-Account-Selection',
        component: () => import('../views/Data-View-Account-Selection.vue')
    }
]

const router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router