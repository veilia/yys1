<template>
    <el-container>
        <!-- <el-header>添加活动</el-header> -->
        <el-main v-if="formLabelAlign">
            <el-form ref="ruleFormRef" :label-position="labelPosition" label-width="auto" :model="formLabelAlign"
                style="max-width: 600px" :rules="rules">
                <!-- <el-form-item label="排布" :label-position="itemLabelPosition">
                    <el-radio-group v-model="itemLabelPosition" aria-label="item label position">
                        <el-radio-button value="">Empty</el-radio-button>
                        <el-radio-button value="left">Left</el-radio-button>
                        <el-radio-button value="right">Right</el-radio-button>
                        <el-radio-button value="top">Top</el-radio-button>
                    </el-radio-group>
                </el-form-item> -->
                <el-form-item label="名称" prop="name" :label-position="itemLabelPosition">
                    <el-input v-model="formLabelAlign.name" />
                </el-form-item>
                <el-form-item label="消耗" prop="cost" :label-position="itemLabelPosition">
                    <el-input-number :min="0" v-model="formLabelAlign.cost" />
                </el-form-item>
                <el-form-item :label-position="itemLabelPosition">
                    <el-radio-group v-model="formLabelAlign.tag">
                        <el-radio-button label="日常" value="日常" />
                        <el-radio-button label="活动" value="活动" />
                    </el-radio-group>
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="add_activity(ruleFormRef, formLabelAlign)">添加</el-button>
                </el-form-item>

            </el-form>
        </el-main>
        <el-main v-else></el-main>
    </el-container>
</template>


<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, FormInstance, FormRules, type FormItemProps, type FormProps } from 'element-plus'
import { Act } from "../stores/type"
import { ref } from "vue"

const add_activity = async (formEl: FormInstance | undefined, data: Act) => {
    if (!formEl) return
    await formEl.validate((valid, fields) => {
        if (valid) {
            invoke("add_act", { act: data })
                .then((res) => {
                    const msg = res as string
                    ElMessage.success({ message: msg })
                })
                .catch((err) => {
                    const msg = err as string
                    ElMessage.error({ message: msg })
                })
        } else {
            console.log('error submit!', fields)
        }
    })

}

const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')
const formLabelAlign = ref<Act>({
    id: null,
    name: "",
    cost: 0,
    tag: "日常"
})


const ruleFormRef = ref<FormInstance>()
const rules = ref<FormRules<Act>>({
    name: [
        { required: true, message: 'Please input activity name', trigger: 'blur' },
        { min: 1, message: 'Length least 1', trigger: 'blur' },
    ],
    cost: [
        {
            type: 'number', min: 0, message: 'cost >= 0', trigger: 'blur'
        }
    ],
})
</script>