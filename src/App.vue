<script setup lang="ts">
import { ref, reactive, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import '@mdi/font/css/materialdesignicons.css'
// Todo类型定义
interface Todo {
  id: string;
  title: string;
  description: string;
  completed: boolean;
  created_at: number;
  completed_at: number | null;
}

// 用于添加新Todo的表单数据
const newTodo = reactive({
  title: "",
  description: "",
});

// 用于编辑Todo的表单数据
const editForm = reactive({
  id: "",
  title: "",
  description: "",
  completed: false,
});

// 待办事项列表
const todos = ref<Todo[]>([]);

// 显示控制
const loading = ref(false);
const errorMessage = ref("");
const titleError = ref(""); // 新增：标题错误提示
const successMessage = ref("");
const addDialog = ref(false);
const editDialog = ref(false);
const deleteDialog = ref(false);
const todoToDelete = ref<Todo | null>(null);
const searchText = ref("");

// Snackbar配置
const snackbar = ref(false);
const snackbarText = ref("");
const snackbarColor = ref("success");
const snackbarTimeout = ref(1500);

// 搜索过滤后的Todo列表
const filteredTodos = computed(() => {
  if (!searchText.value) return todos.value;
  const searchLower = searchText.value.toLowerCase();

  return todos.value.filter(todo =>
    todo.title.toLowerCase().includes(searchLower) ||
    todo.description.toLowerCase().includes(searchLower)
  );
});

// 显示消息通知
function showMessage(text: string, isError = false) {
  snackbarText.value = text;
  snackbarColor.value = isError ? "error" : "success";
  snackbar.value = true;
}

// 获取所有待办事项
async function loadTodos() {
  loading.value = true;
  errorMessage.value = "";

  try {
    todos.value = await invoke("get_all_todos");
  } catch (error) {
    showMessage(`加载待办事项失败: ${error}`, true);
  } finally {
    loading.value = false;
  }
}

// 添加新待办事项
async function addTodo() {
  // 重置错误信息
  titleError.value = "";
  errorMessage.value = "";

  if (!newTodo.title) {
    titleError.value = "标题不能为空";
    return;
  }

  loading.value = true;

  try {
    await invoke("create_todo", { todo: newTodo });
    showMessage("添加成功!");
    newTodo.title = "";
    newTodo.description = "";
    addDialog.value = false;
    await loadTodos();
  } catch (error) {
    showMessage(`添加待办事项失败: ${error}`, true);
  } finally {
    loading.value = false;
  }
}

// 打开编辑对话框
function openEditDialog(todo: Todo) {
  editForm.id = todo.id;
  editForm.title = todo.title;
  editForm.description = todo.description;
  editForm.completed = todo.completed;
  editDialog.value = true;
}

// 更新待办事项
async function updateTodo() {
  // 重置错误信息
  titleError.value = "";
  errorMessage.value = "";

  if (!editForm.title) {
    titleError.value = "标题不能为空";
    return;
  }

  loading.value = true;

  try {
    await invoke("update_todo", {
      id: editForm.id,
      updates: {
        title: editForm.title,
        description: editForm.description,
        completed: editForm.completed
      }
    });
    showMessage("更新成功!");
    editDialog.value = false;
    await loadTodos();
  } catch (error) {
    showMessage(`更新待办事项失败: ${error}`, true);
  } finally {
    loading.value = false;
  }
}

// 切换待办事项完成状态
async function toggleTodoStatus(todo: Todo) {
  // 不再设置全局 loading 状态
  try {
    // 在本地更新状态
    const todoIndex = todos.value.findIndex(t => t.id === todo.id);
    if (todoIndex !== -1) {
      // 后台调用 API 更新状态
      await invoke("update_todo", {
        id: todo.id,
        updates: {
          completed: todo.completed
        }
      });

      // 添加成功提示
      showMessage(`已${todo.completed ? '完成' : '取消完成'}任务: ${todo.title}`);

      // 不再重新加载所有数据
      // 如果 API 调用成功，本地状态已经更新，无需其他操作
    }
  } catch (error) {
    // 发生错误时恢复原始状态
    const todoIndex = todos.value.findIndex(t => t.id === todo.id);
    if (todoIndex !== -1) {
      todos.value[todoIndex].completed = !todos.value[todoIndex].completed;
    }
    showMessage(`更新状态失败: ${error}`, true);
  }
}

// 打开删除确认对话框
function openDeleteDialog(todo: Todo) {
  todoToDelete.value = todo;
  deleteDialog.value = true;
}

// 删除待办事项
async function deleteTodo() {
  if (!todoToDelete.value) return;

  loading.value = true;

  try {
    await invoke("delete_todo", { id: todoToDelete.value.id });
    showMessage("删除成功!");
    deleteDialog.value = false;
    todoToDelete.value = null;
    await loadTodos();
  } catch (error) {
    showMessage(`删除待办事项失败: ${error}`, true);
  } finally {
    loading.value = false;
  }
}

// 清除消息
function clearMessages() {
  errorMessage.value = "";
  successMessage.value = "";
  snackbar.value = false;
}

// 页面加载时获取所有待办事项
onMounted(() => {
  loadTodos();
  window.addEventListener('contextmenu', (e) => e.preventDefault(), false);
  document.addEventListener('keydown', function (event) {
    // Prevent F5 or Ctrl+R (Windows/Linux) and Command+R (Mac) from refreshing the page
    if (
      event.key === 'F5' ||
      (event.ctrlKey && event.key === 'r') ||
      (event.metaKey && event.key === 'r')
    ) {
      event.preventDefault();
    }
  });
});
</script>

<template>
  <v-app>
    <v-app-bar color="primary" density="default">
      <v-app-bar-title>待办事项管理系统</v-app-bar-title>

      <v-spacer></v-spacer>

      <v-text-field v-model="searchText" density="compact" hide-details variant="outlined" label="搜索"
        prepend-inner-icon="mdi-magnify" class="mx-4" style="max-width: 250px;"></v-text-field>

      <v-btn color="success" @click="addDialog = true">
        <v-icon>mdi-plus</v-icon>
        添加任务
      </v-btn>
    </v-app-bar>

    <v-main>
      <v-container>
        <v-alert v-if="errorMessage" type="error" closable @click:close="clearMessages" class="mb-4">
          {{ errorMessage }}
        </v-alert>

        <v-alert v-if="successMessage" type="success" closable @click:close="clearMessages" class="mb-4">
          {{ successMessage }}
        </v-alert>

        <v-card>
          <v-card-title class="text-h5 d-flex align-center">
            我的待办事项
            <v-spacer></v-spacer>
            <v-chip>共 {{ filteredTodos.length }} 项</v-chip>
          </v-card-title>

          <v-divider></v-divider>

          <v-card-text v-if="loading" class="text-center py-4">
            <v-progress-circular indeterminate color="primary"></v-progress-circular>
            <div class="mt-2">加载中...</div>
          </v-card-text>

          <div v-else-if="filteredTodos.length === 0" class="text-center py-8">
            <v-icon size="64" color="grey">mdi-clipboard-text-outline</v-icon>
            <div class="text-h6 text-grey mt-2">暂无待办事项</div>
            <v-btn color="primary" class="mt-4" @click="addDialog = true">
              <v-icon start>mdi-plus</v-icon>
              添加第一个任务
            </v-btn>
          </div>

          <v-list v-else lines="two">
            <v-list-item v-for="todo in filteredTodos" :key="todo.id"
              :title-class="{ 'text-decoration-line-through': todo.completed }"
              :subtitle-class="{ 'text-decoration-line-through': todo.completed }">
              <template v-slot:prepend>
                <v-checkbox v-model="todo.completed" @change="() => toggleTodoStatus(todo)" hide-details
                  color="success"></v-checkbox>
              </template>

              <!-- 添加自定义标题插槽以应用删除线效果 -->
              <template v-slot:title>
                <span :class="{ 'text-decoration-line-through': todo.completed }">{{ todo.title }}</span>
              </template>

              <!-- 添加自定义描述插槽以应用删除线效果 -->
              <template v-slot:subtitle>
                <span :class="{ 'text-decoration-line-through': todo.completed }">{{ todo.description }}</span>
              </template>

              <template v-slot:append>
                <div class="d-flex align-center">
                  <v-chip size="small" :color="todo.completed ? 'success' : 'warning'" class="mr-2">
                    {{ todo.completed ? '已完成' : '进行中' }}
                  </v-chip>

                  <v-btn icon size="small" color="primary" class="mr-1" @click="openEditDialog(todo)">
                    <v-icon size="small">mdi-pencil</v-icon>
                  </v-btn>

                  <v-btn icon size="small" color="error" @click="openDeleteDialog(todo)">
                    <v-icon size="small">mdi-delete</v-icon>
                  </v-btn>
                </div>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-container>
    </v-main>

    <!-- 添加待办事项对话框 -->
    <v-dialog v-model="addDialog" max-width="500px">
      <v-card>
        <v-card-title>添加新待办事项</v-card-title>
        <v-card-text>
          <v-form @submit.prevent="addTodo">
            <v-text-field v-model="newTodo.title" label="标题*" required autofocus
              :error-messages="titleError"></v-text-field>

            <v-textarea v-model="newTodo.description" label="描述" rows="3"></v-textarea>
          </v-form>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="grey-darken-1" variant="text" @click="addDialog = false">
            取消
          </v-btn>
          <v-btn color="primary" variant="text" @click="addTodo" :loading="loading">
            保存
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- 编辑待办事项对话框 -->
    <v-dialog v-model="editDialog" max-width="500px">
      <v-card>
        <v-card-title>编辑待办事项</v-card-title>
        <v-card-text>
          <v-form @submit.prevent="updateTodo">
            <v-text-field v-model="editForm.title" label="标题*" required :error-messages="titleError"></v-text-field>

            <v-textarea v-model="editForm.description" label="描述" rows="3"></v-textarea>

            <v-switch v-model="editForm.completed" color="success" label="标记为已完成"></v-switch>
          </v-form>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="grey-darken-1" variant="text" @click="editDialog = false">
            取消
          </v-btn>
          <v-btn color="primary" variant="text" @click="updateTodo" :loading="loading">
            更新
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- 删除确认对话框 -->
    <v-dialog v-model="deleteDialog" max-width="400px">
      <v-card>
        <v-card-title class="text-h5">确认删除</v-card-title>
        <v-card-text>
          确定要删除"{{ todoToDelete?.title }}"吗？此操作无法撤销。
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="grey-darken-1" variant="text" @click="deleteDialog = false">
            取消
          </v-btn>
          <v-btn color="error" variant="text" @click="deleteTodo" :loading="loading">
            删除
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Snackbar通知 -->
    <v-snackbar v-model="snackbar" :timeout="snackbarTimeout" :color="snackbarColor" top right>
      {{ snackbarText }}
    </v-snackbar>
  </v-app>
</template>

<style scoped>
.completed {
  text-decoration: line-through;
  opacity: 0.7;
}
</style>

<style>
/* 移除滚动栏 */
::-webkit-scrollbar {
  display: none;
}

body {
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE and Edge */

  /* 禁止文本选中 */
  user-select: none;
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
}

/* 禁止浏览器右键菜单 */
html {
  -webkit-touch-callout: none;
}
</style>