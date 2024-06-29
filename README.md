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
## sql
````sql
-- 场地表 
-- auto-generated definition
create table jy_main_site
(
    id          int auto_increment comment 'ID'
        primary key,
    merchant_id int                                not null comment '商户ID',
    store_id    int                                not null comment '店铺ID',
    name        varchar(50)                        not null comment '场地名',
    images      varchar(4095)                      not null comment '场地图片',
    rc_config   json                               null comment '行列配置',
    status      tinyint  default 1                 not null comment '状态:1启用,0:禁用',
    is_delete   tinyint  default 0                 not null comment '是否删除:0=未删除,1=删除 ',
    created_at  datetime default CURRENT_TIMESTAMP not null comment '创建日期',
    updated_at  datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新日期'
)comment '场地表' charset = utf8;
    
INSERT INTO jy_main_site (id, merchant_id, store_id, name, images, rc_config, status, is_delete, created_at, updated_at) VALUES (1, 1, 23, 'asfa', '["http:\\/\\/0.0.0.0:8000\\/upload\\/admin\\/4f\\/cc4771e7e179c54de7fa5715e64fb1.jpg"]', '[{"col": "13", "row": "11"}, {"col": "14", "row": "11"}]', 1, 0, '2024-05-24 15:47:36', '2024-05-24 16:43:36');
INSERT INTO jy_main_site (id, merchant_id, store_id, name, images, rc_config, status, is_delete, created_at, updated_at) VALUES (2, 1, 24, '商户01-测试门店02-场地1', '["http:\\/\\/0.0.0.0:8000\\/upload\\/admin\\/42\\/629541ac04f01e4373c22f12934511.png"]', '[{"col": "13", "row": "12"}, {"col": "12", "row": "15"}]', 1, 0, '2024-05-25 01:04:10', '2024-05-25 01:04:10');
INSERT INTO jy_main_site (id, merchant_id, store_id, name, images, rc_config, status, is_delete, created_at, updated_at) VALUES (3, 4, 25, '今日测评门店场地A', '["http:\\/\\/192.168.50.13:8000\\/upload\\/admin\\/2c\\/3ff5d6ba19a1f83f20d8dfb09d79d2.png"]', '[{"col": "22", "row": "20"}, {"col": "13", "row": "11"}]', 1, 0, '2024-06-06 14:55:47', '2024-06-06 14:55:47');

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