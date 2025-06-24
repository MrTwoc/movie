# Movie App

## 简介
这是一个跟随课程学习的练习Demo的第二章</br>
["Rust 实战：电影信息维护（命令行版）"](https://www.bilibili.com/video/BV1i1jJz3Eqf?spm_id_from=333.788.videopod.sections&vd_source=511b084e4bf87d71d725c5db0fb20b7f)</br>
Movie App 是一个简单的电影应用程序，用于管理电影收藏。</br>
该应用程序提供了用户登录和退出登录的功能，支持管理员和普通用户两种角色。</br>

## 项目结构
```plaintext
movie/
├── src/
│   ├── handler.rs      # 处理用户登录和退出登录的逻辑
│   ├── main.rs         # 程序入口，处理命令行参数
│   ├── models.rs       # 定义用户和角色的数据结构
│   ├── services.rs     # 提供用户服务，如获取用户列表、登录、退出登录等
│   └── lib.rs          # 模块导出文件
├── Cargo.toml          # Rust 项目配置文件
└── .gitignore          # Git 忽略文件配置
```
## 依赖
clap 4.5.40：用于解析命令行参数。</br>
rpassword 7.4.0：用于安全地读取用户输入的密码。</br>

## 使用方法
```
登录：
cargo run -- login --username <your_username>
例如
cargo run -- login --username dave
登出
cargo run -- logout
```
可用用户
| 用户名 | 密码 | 角色 |
 |--------|-------|--------| 
 | admin | admin | Admin | 
 | Dave | Dave | User | 
 | user | user | User |