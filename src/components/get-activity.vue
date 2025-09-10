<template>
    <el-container>
        <!-- <el-header>活动</el-header> -->
        <el-main v-if="act">
            <el-tag type="primary">活动</el-tag>
            <el-switch v-model="tag" style="--el-switch-on-color: #13ce66; --el-switch-off-color: #ff4949"
                active-value="日常" inactive-value="活动" @click="switch_mode" />
            <el-tag type="success">日常</el-tag>
            <el-table :data="act" style="width: 100%">
                <el-table-column fixed prop="name" label="Name" width="150" />
                <el-table-column prop="cost" label="Cost" width="120" />
                <el-table-column prop="tag" label="Tag" width="120" />
                <el-table-column fixed="right" label="Operations" min-width="120">
                    <template #default="{ row }">
                        <el-button type="primary" size="small" @click="">
                            Detail
                        </el-button>
                        <el-button type="danger" size="small" @click="rm_act(row.id)">Delete</el-button>
                    </template>
                </el-table-column>
            </el-table>
        </el-main>
        <el-main v-else></el-main>
    </el-container>
</template>


<script setup lang="ts">
import { onMounted, ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage } from 'element-plus'
import { type Act } from "../stores/type"

const act = ref<Act[]>([])
const tag = ref<string>('日常')
const get_act = () => {
    console.log("invoke -> get act")
    invoke("get_acts", { tag: tag.value })
        .then((res) => {
            act.value = res as Act[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            act.value = []
        })
}

const rm_act = (id: string) => {
    console.log("invoke -> rm act")
    invoke("rm_act", { id: id })
        .then((res) => {
            act.value = act.value.filter((item) => item.id !== id)
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
}

const switch_mode = () => {
    console.log("invoke -> switch_mode")
    invoke("get_acts", { tag: tag.value })
        .then((res) => {
            act.value = res as Act[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            act.value = []
        })
}

onMounted(() => {
    get_act()
    console.log("mount -> get act")
})
</script>