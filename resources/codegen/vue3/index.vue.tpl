<template>
  <ContentWrap>
    <!-- Search Form -->
    <el-form :model="queryParams" ref="queryFormRef" :inline="true" label-width="68px">
      {% for col in columns %}
      {% if col.listOperation %}
      <el-form-item label="{{ col.columnComment }}" prop="{{ col.javaField }}">
        <el-input v-model="queryParams.{{ col.javaField }}" placeholder="请输入{{ col.columnComment }}" clearable class="!w-240px" />
      </el-form-item>
      {% endif %}
      {% endfor %}
      <el-form-item>
        <el-button @click="handleQuery"><Icon icon="ep:search" class="mr-5px" /> 搜索</el-button>
        <el-button @click="resetQuery"><Icon icon="ep:refresh" class="mr-5px" /> 重置</el-button>
        <el-button type="primary" plain @click="openForm('create')">
          <Icon icon="ep:plus" class="mr-5px" /> 新增
        </el-button>
      </el-form-item>
    </el-form>
  </ContentWrap>

  <ContentWrap>
    <el-table v-loading="loading" :data="list">
      {% for col in columns %}
      {% if col.listOperationResult %}
      <el-table-column label="{{ col.columnComment }}" align="center" prop="{{ col.javaField }}" />
      {% endif %}
      {% endfor %}
      <el-table-column label="操作" align="center" fixed="right" width="180">
        <template #default="scope">
          <el-button link type="primary" @click="openForm('update', scope.row.id)">编辑</el-button>
          <el-button link type="danger" @click="handleDelete(scope.row.id)">删除</el-button>
        </template>
      </el-table-column>
    </el-table>
    <Pagination
      :total="total"
      v-model:page="queryParams.pageNo"
      v-model:limit="queryParams.pageSize"
      @pagination="getList"
    />
  </ContentWrap>

  <!-- Form Dialog -->
  <{{ table.className }}Form ref="formRef" @success="getList" />
</template>
<script setup lang="ts">
import { dateFormatter } from '@/utils/formatTime'
import * as {{ table.className }}Api from '@/api/{{ table.moduleName }}/{{ table.businessName }}'
import {{ table.className }}Form from './{{ table.className }}Form.vue'

const message = useMessage()
const { t } = useI18n()

const loading = ref(true)
const list = ref([])
const total = ref(0)
const queryParams = reactive({
  pageNo: 1,
  pageSize: 10,
  {% for col in columns %}
  {% if col.listOperation %}
  {{ col.javaField }}: undefined,
  {% endif %}
  {% endfor %}
})
const queryFormRef = ref()

const getList = async () => {
  loading.value = true
  try {
    const data = await {{ table.className }}Api.get{{ table.className }}Page(queryParams)
    list.value = data.list
    total.value = data.total
  } finally {
    loading.value = false
  }
}

const handleQuery = () => {
  queryParams.pageNo = 1
  getList()
}

const resetQuery = () => {
  queryFormRef.value.resetFields()
  handleQuery()
}

const formRef = ref()
const openForm = (type: string, id?: number) => {
  formRef.value.open(type, id)
}

const handleDelete = async (id: number) => {
  try {
    await message.delConfirm()
    await {{ table.className }}Api.delete{{ table.className }}(id)
    message.success(t('common.delSuccess'))
    await getList()
  } catch {}
}

onMounted(() => {
  getList()
})
</script>