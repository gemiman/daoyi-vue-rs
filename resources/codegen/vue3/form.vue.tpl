<template>
  <Dialog :title="dialogTitle" v-model="dialogVisible">
    <el-form ref="formRef" :model="formData" :rules="formRules" label-width="100px" v-loading="formLoading">
      {% for col in columns %}
      {% if col.createOperation or col.updateOperation %}
      <el-form-item label="{{ col.columnComment }}" prop="{{ col.javaField }}">
        {% if col.htmlType == 'input' %}
        <el-input v-model="formData.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" />
        {% elif col.htmlType == 'textarea' %}
        <el-input v-model="formData.{{ col.javaField }}" type="textarea" placeholder="请输入{{ col.columnComment }}" />
        {% elif col.htmlType == 'datetime' %}
        <el-date-picker v-model="formData.{{ col.javaField }}" type="datetime" value-format="x" placeholder="选择{{ col.columnComment }}" />
        {% else %}
        <el-input v-model="formData.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" />
        {% endif %}
      </el-form-item>
      {% endif %}
      {% endfor %}
    </el-form>
    
    {% if sub_tables | length > 0 %}
    <!-- Sub Tables -->
    <el-tabs v-model="activeTab">
      {% for sub in sub_tables %}
      <el-tab-pane label="{{ sub.tableComment }}" name="{{ sub.className | camel_case }}">
        <!-- Placeholder for sub-table component -->
        <div>Sub-table: {{ sub.tableComment }}</div>
      </el-tab-pane>
      {% endfor %}
    </el-tabs>
    {% endif %}

    <template #footer>
      <el-button @click="submitForm" type="primary" :disabled="formLoading">确 定</el-button>
      <el-button @click="dialogVisible = false">取 消</el-button>
    </template>
  </Dialog>
</template>
<script setup lang="ts">
import * as {{ table.className }}Api from '@/api/{{ table.moduleName }}/{{ table.businessName }}'

const { t } = useI18n()
const message = useMessage()

const dialogVisible = ref(false)
const dialogTitle = ref('')
const formLoading = ref(false)
const formType = ref('')
const formData = ref({
  {% for col in columns %}
  {% if col.createOperation or col.updateOperation %}
  {{ col.javaField }}: undefined,
  {% endif %}
  {% endfor %}
})
const formRules = reactive({
  {% for col in columns %}
  {% if not col.nullable and (col.createOperation or col.updateOperation) %}
  {{ col.javaField }}: [{ required: true, message: '{{ col.columnComment }}不能为空', trigger: 'blur' }],
  {% endif %}
  {% endfor %}
})
const formRef = ref()
const activeTab = ref('')

const open = async (type: string, id?: number) => {
  dialogVisible.value = true
  dialogTitle.value = t('action.' + type)
  formType.value = type
  resetForm()
  if (id) {
    formLoading.value = true
    try {
      formData.value = await {{ table.className }}Api.get{{ table.className }}(id)
    } finally {
      formLoading.value = false
    }
  }
}
defineExpose({ open })

const emit = defineEmits(['success'])
const submitForm = async () => {
  await formRef.value.validate()
  formLoading.value = true
  try {
    const data = formData.value
    if (formType.value === 'create') {
      await {{ table.className }}Api.create{{ table.className }}(data)
      message.success(t('common.createSuccess'))
    } else {
      await {{ table.className }}Api.update{{ table.className }}(data)
      message.success(t('common.updateSuccess'))
    }
    dialogVisible.value = false
    emit('success')
  } finally {
    formLoading.value = false
  }
}

const resetForm = () => {
  formData.value = {
    {% for col in columns %}
    {% if col.createOperation or col.updateOperation %}
    {{ col.javaField }}: undefined,
    {% endif %}
    {% endfor %}
  }
  formRef.value?.resetFields()
}
</script>