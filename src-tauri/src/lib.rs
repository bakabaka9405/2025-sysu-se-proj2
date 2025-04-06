// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::fs;
use uuid::Uuid;
use std::sync::Mutex;
use once_cell::sync::Lazy;

// 定义全局数据库连接
static DB_CONNECTION: Lazy<Mutex<Option<Connection>>> = Lazy::new(|| Mutex::new(None));

// Todo项目模型
#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    id: String,
    title: String,
    description: String,
    completed: bool,
    #[serde(with = "chrono::serde::ts_seconds")]
    created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    completed_at: Option<DateTime<Utc>>,
}

// 新建Todo的请求模型
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    title: String,
    description: String,
}

// 更新Todo的请求模型
#[derive(Debug, Deserialize)]
pub struct UpdateTodoRequest {
    title: Option<String>,
    description: Option<String>,
    completed: Option<bool>,
}

// 数据库错误类型
#[derive(Debug, thiserror::Error)]
enum DatabaseError {
    #[error(transparent)]
    SqliteError(#[from] rusqlite::Error),
    
    #[error("Failed to get app directory: {0}")]
    AppDirError(String),
}

// 初始化数据库
fn initialize_database() -> Result<Connection, DatabaseError> {
    // 获取应用数据目录 - 使用标准库方法
    let app_data_dir = match dirs::data_dir() {
        Some(dir) => dir.join("todo-list-app"),
        None => return Err(DatabaseError::AppDirError("无法获取系统数据目录".into())),
    };
    
    // 确保数据目录存在
    fs::create_dir_all(&app_data_dir).map_err(|e| DatabaseError::AppDirError(e.to_string()))?;
    
    // 数据库文件路径
    let db_path = app_data_dir.join("todos.db");
    
    // 创建/打开数据库连接
    let conn = Connection::open(db_path)?;
    
    // 创建表格(如果不存在)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            completed_at INTEGER
        )",
        [],
    )?;
    
    Ok(conn)
}

// 确保数据库连接已初始化
fn get_db() -> Result<&'static Connection, DatabaseError> {
    let mut conn_guard = DB_CONNECTION.lock().unwrap();
    
    if conn_guard.is_none() {
        *conn_guard = Some(initialize_database()?);
    }
    
    // 这里使用一个不安全块来绕过借用检查
    // 实际上我们确保指针在函数调用期间有效
    unsafe {
        let conn_ptr = conn_guard.as_ref().unwrap() as *const Connection;
        Ok(&*conn_ptr)
    }
}

// Tauri命令：获取所有Todo项
#[tauri::command]
fn get_all_todos() -> Result<Vec<Todo>, String> {
    let conn = match get_db() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("数据库错误: {}", e)),
    };
    
    let mut stmt = conn.prepare("SELECT id, title, description, completed, created_at, completed_at FROM todos")
        .map_err(|e| format!("准备SQL语句失败: {}", e))?;
    
    let todo_iter = stmt.query_map([], |row| {
        Ok(Todo {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            completed: row.get::<_, i32>(3)? != 0,
            created_at: chrono::DateTime::from_timestamp(row.get::<_, i64>(4)?, 0)
                .unwrap_or(Utc::now()),
            completed_at: match row.get::<_, Option<i64>>(5)? {
                Some(ts) => chrono::DateTime::from_timestamp(ts, 0),
                None => None,
            },
        })
    }).map_err(|e| format!("执行查询失败: {}", e))?;
    
    let todos: Result<Vec<Todo>, _> = todo_iter.collect();
    todos.map_err(|e| format!("处理查询结果失败: {}", e))
}

// Tauri命令：创建Todo项
#[tauri::command]
fn create_todo(todo: CreateTodoRequest) -> Result<Todo, String> {
    let conn = match get_db() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("数据库错误: {}", e)),
    };
    
    let now = Utc::now();
    let todo_id = Uuid::new_v4().to_string();
    
    let new_todo = Todo {
        id: todo_id.clone(),
        title: todo.title,
        description: todo.description,
        completed: false,
        created_at: now,
        completed_at: None,
    };
    
    conn.execute(
        "INSERT INTO todos (id, title, description, completed, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            new_todo.id,
            new_todo.title,
            new_todo.description,
            new_todo.completed as i32,
            new_todo.created_at.timestamp(),
        ],
    ).map_err(|e| format!("插入数据失败: {}", e))?;
    
    Ok(new_todo)
}

// Tauri命令：更新Todo项
#[tauri::command]
fn update_todo(id: String, updates: UpdateTodoRequest) -> Result<Todo, String> {
    let conn = match get_db() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("数据库错误: {}", e)),
    };
    
    // 首先获取当前Todo
    let mut stmt = conn.prepare("SELECT id, title, description, completed, created_at, completed_at FROM todos WHERE id = ?")
        .map_err(|e| format!("准备SQL语句失败: {}", e))?;
    
    let mut todo: Option<Todo> = None;
    let mut rows = stmt.query([&id]).map_err(|e| format!("执行查询失败: {}", e))?;
    
    if let Some(row) = rows.next().map_err(|e| format!("获取行数据失败: {}", e))? {
        todo = Some(Todo {
            id: row.get(0).map_err(|e| format!("获取id失败: {}", e))?,
            title: row.get(1).map_err(|e| format!("获取title失败: {}", e))?,
            description: row.get(2).map_err(|e| format!("获取description失败: {}", e))?,
            completed: row.get::<_, i32>(3).map_err(|e| format!("获取completed失败: {}", e))? != 0,
            created_at: chrono::DateTime::from_timestamp(
                row.get::<_, i64>(4).map_err(|e| format!("获取created_at失败: {}", e))?, 
                0
            ).unwrap_or(Utc::now()),
            completed_at: match row.get::<_, Option<i64>>(5).map_err(|e| format!("获取completed_at失败: {}", e))? {
                Some(ts) => chrono::DateTime::from_timestamp(ts, 0),
                None => None,
            },
        });
    }
    
    let mut todo = todo.ok_or_else(|| format!("未找到ID为{}的Todo项", id))?;
    
    // 应用更新
    if let Some(title) = updates.title {
        todo.title = title;
    }
    
    if let Some(description) = updates.description {
        todo.description = description;
    }
    
    if let Some(completed) = updates.completed {
        todo.completed = completed;
        if completed {
            todo.completed_at = Some(Utc::now());
        } else {
            todo.completed_at = None;
        }
    }
    
    // 更新数据库
    conn.execute(
        "UPDATE todos SET title = ?1, description = ?2, completed = ?3, completed_at = ?4 WHERE id = ?5",
        params![
            todo.title,
            todo.description,
            todo.completed as i32,
            todo.completed_at.map(|dt| dt.timestamp()),
            todo.id,
        ],
    ).map_err(|e| format!("更新数据失败: {}", e))?;
    
    Ok(todo)
}

// Tauri命令：删除Todo项
#[tauri::command]
fn delete_todo(id: String) -> Result<(), String> {
    let conn = match get_db() {
        Ok(conn) => conn,
        Err(e) => return Err(format!("数据库错误: {}", e)),
    };
    
    conn.execute("DELETE FROM todos WHERE id = ?", [id])
        .map_err(|e| format!("删除数据失败: {}", e))?;
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_all_todos, 
            create_todo, 
            update_todo, 
            delete_todo
        ])
        .setup(|_| {
            // 应用启动时初始化数据库连接
            let mut conn_guard = DB_CONNECTION.lock().unwrap();
            *conn_guard = Some(initialize_database().expect("Failed to initialize database"));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
