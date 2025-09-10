<template>
    <el-container>
        <!-- <el-header>活动</el-header> -->
        <el-main v-if="act">
            <el-table :data="act" style="width: 100%">
                <el-table-column fixed prop="name" label="Name" width="150" />
                <el-table-column fixed="right" label="Operations" min-width="120">
                    <template #default="{ row }">
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
const get_act = () => {
    console.log("invoke -> get rec")
    invoke("get_rec", {tag: '日常'})
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
    console.log("invoke -> rm rec")
    invoke("rm_rec", { id: id })
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

onMounted(() => {
    get_act()
    console.log("mount -> get rec")
})
</script>