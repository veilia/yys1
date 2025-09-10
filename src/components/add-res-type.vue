<template>
    <el-container>
        <!-- <el-header>添加物品类型</el-header> -->
        <el-main>
            <el-form ref="form_ref" :model="form_data" :rules="rules"  :label-position="labelPosition" label-width="auto"
                style="max-width: 600px">
                <!-- <el-form-item label="排布" :label-position="itemLabelPosition">
                    <el-radio-group v-model="itemLabelPosition" aria-label="item label position">
                        <el-radio-button value="">Empty</el-radio-button>
                        <el-radio-button value="left">Left</el-radio-button>
                        <el-radio-button value="right">Right</el-radio-button>
                        <el-radio-button value="top">Top</el-radio-button>
                    </el-radio-group>
                </el-form-item> -->
                <el-form-item label="名称" prop="name" :label-position="itemLabelPosition">
                    <el-input v-model="form_data.name" />
                </el-form-item>
                <el-form-item>
                    <el-button type="primary" @click="add_res_type(form_ref, form_data.name)">添加</el-button>
                </el-form-item>
            </el-form>
        </el-main>
    </el-container>
</template>


<script setup lang="ts">
import { ref } from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage, FormInstance, FormRules, type FormItemProps, type FormProps } from 'element-plus'


const labelPosition = ref<FormProps['labelPosition']>('right')
const itemLabelPosition = ref<FormItemProps['labelPosition']>('right')

interface DataType {
    name: string
}

const form_data = ref<DataType>({
    name: ""
})


const form_ref = ref<FormInstance>()
const rules = ref<FormRules<DataType>>({
    name: [
        { required: true, message: 'Please input name', trigger: 'blur' },
    ]
})

const add_res_type = async (form_el: FormInstance | undefined, name: string) => {
    if (!form_el) return
    await form_el.validate((valid, fields) => {
        if (valid) {
            // fn
            invoke("add_rec", { name: name })
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

</script>