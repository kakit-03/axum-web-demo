create table main_user_store_relationship
(
    id         int auto_increment comment 'ID'
        primary key,
    store_id   int                                not null comment '门店ID',
    user_id    int                                not null comment '用户ID',
    status     tinyint  default 1                 not null comment '状态:1启用,0:禁用',
    is_deleted tinyint  default 0                 not null comment '是否删除:0=未删除,1=删除	',
    created_at datetime default CURRENT_TIMESTAMP not null comment '创建日期',
    updated_at datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新日期'
) comment '门店用户关联表' charset = utf8;
create table main_site
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
)
    comment '场地表' charset = utf8;
create table main_store
(
    id          int auto_increment comment 'ID'
        primary key,
    merchant_id int                                    not null comment '商户ID',
    user_id     int                                    not null comment '用户ID',
    name        varchar(50)                            not null comment '门店名称',
    logo        varchar(255)                           not null comment '门店照片',
    images      varchar(4095)                          not null comment '门店相册',
    status      tinyint      default 1                 not null comment '状态:1启用,0:禁用',
    latitude    decimal(10, 6)                         not null comment '纬度',
    longitude   decimal(10, 6)                         not null comment '经度',
    start_time  varchar(20)                            not null comment '开启时间',
    end_time    varchar(20)                            not null comment '关闭时间',
    province    varchar(50)  default ''                not null comment '门店所在省份',
    city        varchar(50)  default ''                not null comment '门店所在城市',
    district    varchar(100) default ''                not null comment '门店所在地区',
    address     varchar(255) default ''                not null comment '门店地址',
    is_deleted  tinyint      default 0                 not null comment '是否删除:0=未删除,1=删除	',
    created_at  datetime     default CURRENT_TIMESTAMP not null comment '创建日期',
    updated_at  datetime     default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新日期'
)
    comment '门店表' charset = utf8;
create table main_user
(
    id            int auto_increment comment 'ID'
        primary key,
    f_user_id     int      default 0                 null comment '上级用户',
    merchant_id   int                                not null comment '商户ID',
    membership_id int      default 0                 not null comment '会籍ID',
    coach_id      int      default 0                 not null comment '教练ID',
    coach_plus_id int      default 0                 not null comment '教练管家ID',
    account       varchar(50)                        not null comment '账号名',
    nickname      varchar(50)                        not null comment '用户昵称',
    phone         varchar(20)                        not null comment '手机号',
    open_id       varchar(255)                       not null comment '微信OPENID',
    avatar        varchar(255)                       not null comment '头像',
    password      varchar(32)                        not null comment '密码',
    point         int      default 0                 not null comment '魅力值',
    gt_tdee       float    default 0                 null comment '团课总热量',
    invite_code   char(6)                            null comment '邀请码',
    sign_num      int      default 0                 null comment '签到总数',
    gender        tinyint  default 0                 not null comment '性别 0：未知、1：男、2：女',
    status        tinyint  default 1                 not null comment '状态:1启用,0:禁用',
    is_delete     tinyint  default 0                 not null comment '是否删除:0=未删除,1=删除	',
    created_at    datetime default CURRENT_TIMESTAMP not null comment '创建日期',
    updated_at    datetime default CURRENT_TIMESTAMP not null on update CURRENT_TIMESTAMP comment '更新日期'
)
    comment '用户表' charset = utf8;