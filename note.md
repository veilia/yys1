#### 使用表单验证
```html
<!-- 需要一个ref和rules -->
<el-form ref="form_ref" :model="form_data" :rules="rules">
    <!-- prop属性是必须的，且和input的model名称一致 -->
    <el-form-item label="名称" prop="name">
        <el-input v-model="form_data.name" />
    </el-form-item>
    <el-form-item>
        <el-button type="primary" @click="submit(form_ref, form_data)">submit</el-button>
    </el-form-item>
</el-form>
```

```ts
import { FormInstance, FormRules, type FormItemProps, type FormProps } from 'element-plus'
import { ref } from "vue"

interface DataType {
    id: string | null
    name: string
    cost: number
}

const form_data = ref<DataType>({
    id: null,
    name: "",
    cost: 0
})
const form_ref = ref<FormInstance>()
const rules = ref<FormRules<DataType>>({
    name: [
        { required: true, message: 'Please input name', trigger: 'blur' },
        { min: 1, message: 'Length least 1', trigger: 'blur' },
    ],
    cost: [
        {
            type: 'number', min: 0, message: 'cost >= 0', trigger: 'blur'
        }
    ],
})


const submit = async (form_el: FormInstance | undefined, data: DataType) => {
    if (!form_el) return
    await form_el.validate((valid, fields) => {
        if (valid) {
            // fn
        } else {
            console.log('error submit!', fields)
        }
    })

}
```