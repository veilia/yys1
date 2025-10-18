<template>
    <div v-if="stats && stats.cnt > 0" class="fixed-right-card">
        <el-card style="max-width: 480px">
            <div>
                <el-tag v-if="stats.cnt" class="text item">次数: {{ stats.cnt }}</el-tag>
                <el-tag v-if="stats.t_cost" class="text item">耗体: {{ stats.t_cost }}</el-tag>
            </div>
            <div v-for="rec in recs" :key="rec.name">
                <el-tag v-if="rec.name && rec.num > 0" class="text item">{{  rec.name  }}: {{  rec.num  }}</el-tag>
                <el-tag v-if="rec.name && rec.num > 0 && stats.t_cost > 0" class="text item">{{  (stats.t_cost / rec.num).toFixed(2)  }} 体/个</el-tag>
            </div>
        </el-card>
    </div>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core'
import { ElMessage } from 'element-plus'
import { ref, watch } from 'vue'
import { ResAct, ResStat } from '../stores/type'

const stats = ref<ResAct>()
const recs = ref<ResStat[]>()

const props = defineProps(["act"])

const get_act_recs = (act: string) => {
    invoke("get_act_recs_2", { act: act })
        .then((res) => {
            recs.value = res as ResStat[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage({
                message: msg,
                type: "error",
                showClose: true,
                placement: 'bottom-right',
            })
            recs.value = []
        })
}

const get_act_stats = (act: string) => {
    invoke("get_act_stats", { act: act })
        .then((res) => {
            stats.value = res as ResAct
        })
        .catch((err) => {
            const msg = err as string
            ElMessage({
                message: msg,
                type: "error",
                showClose: true,
                placement: 'bottom-right',
            })
        })
}

watch(
    () => props.act,
    (new_id: string | null) => {
        if (new_id != null) {
            get_act_stats(new_id)
            get_act_recs(new_id)
        }
    },
    { immediate: true } // 立即执行一次（相当于替代 onMounted）
)


defineExpose({
    get_act_stats,
    get_act_recs
})
</script>

<style scoped>
.fixed-right-card {
    position: fixed;
    right: 22px;
    /* 距离右边 20px，可按需调整 */
    top: 90px;
    /* 距离顶部 20px，避免贴边 */
    z-index: 1000;
    /* 确保在其他内容之上 */
}
</style>