import { createRouter, createWebHistory } from "vue-router"
export const router_history = createWebHistory()

export const router = createRouter({
    history: router_history,
    strict: true,
    routes: [
        {
            path: "/",
            name: "home",
            component: () => import("../views/home.vue"),
            children: [
                {
                    path: "get-activity",
                    name: "get-activity",
                    component: () => import("../components/get-activity.vue"),
                },
                {
                    path: "get-rec",
                    name: "get-rec",
                    component: () => import("../components/get-rec.vue"),
                },
                {
                    path: "add-activity",
                    name: "add-activity",
                    component: () => import("../components/add-activity.vue"),
                },
                {
                    path: "add-runs",
                    name: "add-runs",
                    component: () => import("../components/add-runs.vue"),
                },
                {
                    path: "add-res-type",
                    name: "add-res-type",
                    component: () => import("../components/add-res-type.vue"),
                }
            ]
        }
    ]
})

router.beforeEach((to, from) => {
    console.log('to', to.name)
    console.log('from', from.name)
    if (from.name == null && to.name !== 'get-activity') {
        // 检测到刷新行为
        console.log('刷新行为')
        return { name: 'get-activity' }
    }
})


// router.beforeEach((to, from) => {
//     console.log('to', to.name)
//     console.log('from', from.name)
//     if (from.name == null && to.name != null && from.name != to.name) {
//         // 检测到刷新行为
//         console.log('刷新行为')
//         return { name: to.name }
//     }
// })