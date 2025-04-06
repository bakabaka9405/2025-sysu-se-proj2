# 待办事项管理系统

本项目是一个基于 Tauri、Vue 3 和 TypeScript 的桌面待办事项管理应用，提供了添加、编辑、删除和搜索待办事项的功能。

## 功能特性

- **添加任务**：用户可以添加新的待办事项，包括标题和描述。
- **编辑任务**：支持编辑现有任务的标题、描述和完成状态。
- **删除任务**：用户可以删除不需要的任务。
- **搜索任务**：通过关键字快速搜索待办事项。
- **状态切换**：支持标记任务为已完成或未完成。

## 技术栈

- **前端**：Vue 3 + TypeScript + Vuetify
- **后端**：Tauri (Rust)
- **构建工具**：Farm
- **包管理**：Bun

## 安装与运行

### 环境要求

- Node.js (建议版本 >= 16)
- Rust (建议版本 >= 1.65)

### 安装步骤

1. 克隆项目代码：

   ```bash
   git clone <仓库地址>
   cd 2025-sysu-se-proj2
   ```

2. 安装依赖：

   ```bash
   bun i
   ```

3. 运行开发环境：

   ```bash
   bun run tauri dev
   ```

4. 构建生产版本：

   ```bash
   bun run tauri build
   ```

## 项目结构

```
2025-sysu-se-proj2/
├── src/                # 前端代码
├── src-tauri/          # 后端代码 (Rust)
├── public/             # 静态资源
├── package.json        # 项目依赖
├── tsconfig.json       # TypeScript 配置
└── README.md           # 项目说明
```

## 使用说明

1. 启动应用后，用户可以在主界面查看所有待办事项。
2. 点击“添加任务”按钮可以创建新的待办事项。
3. 点击任务右侧的编辑按钮可以修改任务内容。
4. 点击删除按钮可以移除任务。
5. 在搜索框中输入关键字可以快速筛选任务。

## 开发者指南

### 调用后端 API

- **获取所有任务**：`invoke("get_all_todos")`
- **创建任务**：`invoke("create_todo", { todo })`
- **更新任务**：`invoke("update_todo", { id, updates })`
- **删除任务**：`invoke("delete_todo", { id })`

### 代码风格

- 使用 ESLint 和 Prettier 进行代码格式化。
- 遵循 Vue 3 的最佳实践。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进本项目。

## 许可证

本项目基于 MIT 许可证开源。
