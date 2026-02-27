import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        redirect: '/fileupload'
    },
    {
        path: '/fileupload',
        component: () => import('../views/FileUpload.vue')
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router