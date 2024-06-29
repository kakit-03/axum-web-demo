# axum-web-demo
a web api appcation with axum

## dependencies
````toml
# web框架基础依赖
axum = {version = "0.7.5" ,features = ["macros"]}
tokio = { version = "1.38", features = ["full"] }
tower-http = { version = "0.5.0", features = ["trace", "fs"] }
tower = { version = "0.4.13", features = ["timeout"] }
futures = "0.3.30"
futures-util = "0.3.30"
hyper = "1.3.1"
# 参数校验
validator = { version = "0.18.1", features = ["derive"] }
axum-valid = "0.18.0"
# 序列化
serde = { version = "1.0.203", features = ["derive"] }
serde_derive = "1.0.203"
serde_json = "1.0"
# 数据库orm
sea-orm = { version = "0.12.15", features = ["runtime-tokio-native-tls", "sqlx-mysql","macros", "debug-print"] }
# 为 Rust 应用程序组织分层或分层配置
config="0.13"
# 此库从 .env 文件加载环境变量。
dotenv="0.15"
# 日志
tracing = "0.1"
# 日志订阅者
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
# 日期和时间
chrono = { version = "0.4.24", features = ["serde"] }
http = "1.1.0"
#jwt
jsonwebtoken = "9.3.0"
# BCrypt 密码哈希函数
bcrypt = "0.15.1"

````


## UNFINISHED
1. log data persistence
2. login valida by database data ` login function valida account & password is writed at middleware/auth.rs`
> to impl jwt, let's read this blog https://blog.logrocket.com/using-rust-axum-build-jwt-authentication-api/
3. full err handler
> no idea 
4. seaORM CUD 
5. db relationship CRUD
6. dto,service&route generator cli

## FINISHED
1. normal select