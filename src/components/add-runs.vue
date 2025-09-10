<template>
    <el-container>
        <!-- <el-header>添加掉落物类型</el-header> -->
        <el-main v-if="formLabelAlign">
            <el-form :label-position="labelPosition" label-width="auto" :model="formLabelAlign"
                style="max-width: 600px">
                <!-- <el-form-item label="排布" :label-position="itemLabelPosition">
                    <el-radio-group v-model="itemLabelPosition" aria-label="item label position">
                        <el-radio-button value="">Empty</el-radio-button>
                        <el-radio-button value="left">Left</el-radio-button>
                        <el-radio-button value="right">Right</el-radio-button>
                        <el-radio-button value="top">Top</el-radio-button>
                    </el-radio-group>
                </el-form-item> -->
                <el-form-item label="活动" :label-position="itemLabelPosition">
                    <el-select v-model="formLabelAlign.act" placeholder="活动" style="width: 240px">
                        <el-option v-for="(item, index) in act" :key="index" :label="item.name" :value="item.id" />
                    </el-select>
                </el-form-item>
                <el-form-item label="消耗" :label-position="itemLabelPosition">
                    <el-input-number v-model="formLabelAlign.cost" :min="0" size="small"
                        style="width: 120px; margin-left: 8px" />
                </el-form-item>



                <!-- 动态资源列表 -->
                <el-form-item v-for="(item, index) in formLabelAlign.recs" :key="index" :label="index === 0 ? '资源' : ''"
                    :label-position="itemLabelPosition" class="resource-item">
                    <el-select v-model="item.id" placeholder="请选择资源" style="width: 200px" size="small">
                        <el-option v-for="(r, idx) in rec" :key="idx" :label="r.name" :value="r.id" />
                    </el-select>
                    <el-input-number v-model="item.num" :min="0" size="small" style="width: 120px; margin-left: 8px" />
                    <el-button v-if="formLabelAlign.recs.length > 1" link type="danger" size="small"
                        @click="rm_recs(index)" style="margin-left: 8px">
                        删除
                    </el-button>
                </el-form-item>

                <!-- 添加资源按钮 -->
                <el-form-item :label-position="itemLabelPosition">
                    <el-button type="primary" plain size="small" @click="add_recs">
                        + 添加资源
                    </el-button>
                </el-form-item>

                <el-form-item :label-position="itemLabelPosition">
                    <el-button type="primary" @click="add_runs">提交</el-button>
                    <el-button type="success" @click="reset">重置</el-button>
                    <el-button @click="flush">刷新</el-button>
                </el-form-item>
            </el-form>
        </el-main>
        <el-main v-else></el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref, watch } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, type FormItemProps, type FormProps } from 'element-plus'
import { Rec, type Act } from "../stores/type"
import { onMounted } from "vue"

const act = ref<Act[]>([])
const rec = ref<Rec[]>([])

const actCostMap = ref<Record<string, number>>({})

const get_act = () => {
    invoke("get_all_act",)
        .then((res) => {
            act.value = res as Act[]
            actCostMap.value = {}
            act.value.forEach(a => {
                if (a.id && a.cost !== undefined) {
                    actCostMap.value[a.id] = a.cost
                }
            })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            act.value = []
            actCostMap.value = {}
        })
}
const get_rec = () => {
    invoke("get_rec")
        .then((res) => {
            rec.value = res as Rec[]
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
            rec.value = []
        })
}

const add_runs = () => {
    invoke("add_run", {
        act: formLabelAlign.value.act,
        cost: formLabelAlign.value.cost,
        recs: formLabelAlign.value.recs,
    })
        .then((res) => {
            const msg = res as string
            ElMessage.success({ message: msg })
        })
        .catch((err) => {
            const msg = err as string
            ElMessage.error({ message: msg })
        })
    reset()
}

const reset = () => {
    formLabelAlign.value.recs.forEach((item) => item.num = 0)
}



const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')
const formLabelAlign = ref<Form>({
    act: "",
    cost: 0,
    recs: [{ id: "", num: 0 }],
})

interface Form {
    act: string
    cost: number
    recs: RunRec[]
}

interface RunRec {
    id: string
    num: number
}

const add_recs = () => {
    formLabelAlign.value.recs.push({
        id: "",
        num: 0
    })
}
const rm_recs = (index: number) => {
    if (formLabelAlign.value.recs.length > 1) {
        formLabelAlign.value.recs.splice(index, 1)
    } else {
        ElMessage.warning("至少保留一个资源项")
    }
}

const flush = () => {
    get_act()
    get_rec()
}


onMounted(() => {
    get_act()
    get_rec()
})

watch(
    () => formLabelAlign.value.act,
    (newActId, oldActId) => {
        // 仅当之前没有选择过，或切换了活动时才设置
        if (!oldActId || (newActId && newActId !== oldActId)) {
            if (newActId && actCostMap.value[newActId] !== undefined) {
                formLabelAlign.value.cost = actCostMap.value[newActId]
            } else {
                formLabelAlign.value.cost = 0
            }
        }
    }
)
</script>

<style lang="scss"></style>