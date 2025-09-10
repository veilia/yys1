<template>

    <!-- <div class="current-route">
        当前路由: {{ route.fullPath }} | 路由名称: {{ route.name }}
    </div> -->

    <el-menu :default-active="active" class="el-menu-demo" mode="horizontal" :ellipsis="false" router>
        <el-menu-item index="get-activity">
            活动
        </el-menu-item>
        <el-menu-item index="get-rec">
            资源
        </el-menu-item>
        <el-menu-item index="add-activity">
            添加活动
        </el-menu-item>
        <el-menu-item index="add-res-type">
            添加物品
        </el-menu-item>
        <el-menu-item index="add-runs">
            添加掉落
        </el-menu-item>

        <el-menu-item>
            <el-popconfirm title="该操作很危险，会重置所有数据，是否确认?" @confirm="init_db">
                <template #reference>
                    <el-button type="danger">初始化数据</el-button>
                </template>
            </el-popconfirm>
        </el-menu-item>
    </el-menu>
    <router-view v-slot="{ Component }">
        <keep-alive :exclude="exclude">
            <component :is="Component" />
        </keep-alive>
    </router-view>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { invoke } from "@tauri-apps/api/core"
import { ElMessage } from 'element-plus'

const active = ref("get-activity")
const exclude = ref(["get-activity"])
console.log("setup -> active: ", active.value)

const route = useRoute()


const init_db = () => {
    invoke("init_db")
        .then((res) => {
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
}

watch(route, (new_route) => {
    console.log("home.vue watch -> route: ", new_route.name)
    active.value = new_route.name as string
})
</script>


<style scoped>
.demo-tabs>.el-tabs__content {
    padding: 32px;
    color: #6b778c;
    font-size: 32px;
    font-weight: 600;
}

.demo-tabs .custom-tabs-label .el-icon {
    vertical-align: middle;
}

.demo-tabs .custom-tabs-label span {
    vertical-align: middle;
    margin-left: 4px;
}

.el-menu--horizontal>.el-menu-item:nth-child(5) {
    margin-right: auto;
}
</style>
