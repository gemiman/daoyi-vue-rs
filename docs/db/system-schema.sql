create schema system;

comment on schema system is 'system';

alter schema system owner to daoyivuers;


-- ----------------------------
-- Table structure for system.system_users
-- ----------------------------
DROP TABLE IF EXISTS system.system_users;
CREATE TABLE system.system_users
(
    id          varchar(32)    NOT NULL primary key,
    username    varchar(30)    NOT NULL,
    password    varchar(100)   NOT NULL DEFAULT '',
    nickname    varchar(256)   NOT NULL DEFAULT '',
    remark      varchar(500)   NULL     DEFAULT NULL,
    dept_id     varchar(32)    NULL     DEFAULT NULL,
    post_ids    varchar(255)[] NULL     DEFAULT NULL,
    email       varchar(128)   NULL     DEFAULT '',
    mobile      varchar(128)   NULL     DEFAULT '',
    sex         varchar(1)     NULL     DEFAULT '0',
    avatar      varchar(512)   NULL     DEFAULT '',
    status      varchar(1)     NOT NULL DEFAULT '0',
    login_ip    varchar(128)   NULL     DEFAULT '',
    login_date  timestamp      NULL     DEFAULT NULL,
    creator     varchar(32)    NULL     DEFAULT '',
    create_time timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)    NULL     DEFAULT '',
    update_time timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean        NOT NULL DEFAULT false,
    tenant_id   varchar(32)    NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_users.id IS '用户ID';
COMMENT ON COLUMN system.system_users.username IS '用户账号';
COMMENT ON COLUMN system.system_users.password IS '密码';
COMMENT ON COLUMN system.system_users.nickname IS '用户昵称';
COMMENT ON COLUMN system.system_users.remark IS '备注';
COMMENT ON COLUMN system.system_users.dept_id IS '部门ID';
COMMENT ON COLUMN system.system_users.post_ids IS '岗位编号数组';
COMMENT ON COLUMN system.system_users.email IS '用户邮箱';
COMMENT ON COLUMN system.system_users.mobile IS '手机号码';
COMMENT ON COLUMN system.system_users.sex IS '用户性别';
COMMENT ON COLUMN system.system_users.avatar IS '头像地址';
COMMENT ON COLUMN system.system_users.status IS '帐号状态（0正常 1停用）';
COMMENT ON COLUMN system.system_users.login_ip IS '最后登录IP';
COMMENT ON COLUMN system.system_users.login_date IS '最后登录时间';
COMMENT ON COLUMN system.system_users.creator IS '创建者';
COMMENT ON COLUMN system.system_users.create_time IS '创建时间';
COMMENT ON COLUMN system.system_users.updater IS '更新者';
COMMENT ON COLUMN system.system_users.update_time IS '更新时间';
COMMENT ON COLUMN system.system_users.deleted IS '是否删除';
COMMENT ON COLUMN system.system_users.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_users IS '用户信息表';
INSERT INTO system.system_users (id, username, password, nickname, remark, dept_id, post_ids, email, mobile, sex,
                                 avatar, status, login_ip, login_date, creator, create_time, updater, update_time,
                                 deleted, tenant_id)
VALUES ('0'::varchar(32), 'admin'::varchar(30),
        '$2b$04$oVX9LhAfLryctEw7L5iAk.R1XFXnW8Pq1KLi9MBvOA47nXisTnKKu'::varchar(100), '系统管理员'::varchar(256),
        '系统管理员，默认初始化'::varchar(500), '0'::varchar(32), '{0}', 'gemiman@vip.qq.com'::varchar(50),
        '17621038080'::varchar(11), '1'::varchar(1), DEFAULT, '0'::varchar(1), '0.0.0.0'::varchar(50),
        '2025-12-26 12:16:02.000000'::timestamp, '0'::varchar(64), '2025-12-26 12:16:12.000000'::timestamp,
        '0'::varchar(64), '2025-12-26 12:16:18.000000'::timestamp, false::boolean, '0'::varchar(32));


-- ----------------------------
-- Table structure for system.system_access_token
-- ----------------------------
DROP TABLE IF EXISTS system.system_access_token;
CREATE TABLE system.system_access_token
(
    id           varchar(32)  NOT NULL primary key,
    user_id      varchar(32)  NOT NULL,
    access_token varchar(255) NOT NULL,
    expires_time timestamp    NOT NULL,
    creator      varchar(32)  NULL     DEFAULT '',
    create_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater      varchar(32)  NULL     DEFAULT '',
    update_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted      boolean      NOT NULL DEFAULT false,
    tenant_id    varchar(32)  NOT NULL DEFAULT '0'
);

CREATE INDEX idx_system_access_token_01 ON system.system_access_token (access_token);

COMMENT ON COLUMN system.system_access_token.id IS '编号';
COMMENT ON COLUMN system.system_access_token.user_id IS '用户编号';
COMMENT ON COLUMN system.system_access_token.access_token IS '访问令牌';
COMMENT ON COLUMN system.system_access_token.expires_time IS '过期时间';
COMMENT ON COLUMN system.system_access_token.creator IS '创建者';
COMMENT ON COLUMN system.system_access_token.create_time IS '创建时间';
COMMENT ON COLUMN system.system_access_token.updater IS '更新者';
COMMENT ON COLUMN system.system_access_token.update_time IS '更新时间';
COMMENT ON COLUMN system.system_access_token.deleted IS '是否删除';
COMMENT ON COLUMN system.system_access_token.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_access_token IS '访问令牌';


-- ----------------------------
-- Table structure for system.system_tenant
-- ----------------------------
DROP TABLE IF EXISTS system.system_tenant;
CREATE TABLE system.system_tenant
(
    id              varchar(32)  NOT NULL primary key,
    name            varchar(128) NOT NULL,
    contact_user_id varchar(32)  NULL     DEFAULT NULL,
    contact_name    varchar(128) NOT NULL,
    contact_mobile  varchar(128) NULL     DEFAULT NULL,
    status          varchar(1)   NOT NULL DEFAULT '0',
    websites        varchar(256) NULL     DEFAULT '',
    package_id      varchar(32)         NOT NULL,
    expire_time     timestamp    NOT NULL,
    account_count   int4         NOT NULL,
    creator         varchar(32)  NULL     DEFAULT '',
    create_time     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater         varchar(32)  NULL     DEFAULT '',
    update_time     timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted         boolean      NOT NULL DEFAULT false,
    tenant_id       varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_tenant.id IS '租户编号';
COMMENT ON COLUMN system.system_tenant.name IS '租户名';
COMMENT ON COLUMN system.system_tenant.contact_user_id IS '联系人的用户编号';
COMMENT ON COLUMN system.system_tenant.contact_name IS '联系人';
COMMENT ON COLUMN system.system_tenant.contact_mobile IS '联系手机';
COMMENT ON COLUMN system.system_tenant.status IS '租户状态（0正常 1停用）';
COMMENT ON COLUMN system.system_tenant.websites IS '绑定域名数组';
COMMENT ON COLUMN system.system_tenant.package_id IS '租户套餐编号';
COMMENT ON COLUMN system.system_tenant.expire_time IS '过期时间';
COMMENT ON COLUMN system.system_tenant.account_count IS '账号数量';
COMMENT ON COLUMN system.system_tenant.creator IS '创建者';
COMMENT ON COLUMN system.system_tenant.create_time IS '创建时间';
COMMENT ON COLUMN system.system_tenant.updater IS '更新者';
COMMENT ON COLUMN system.system_tenant.update_time IS '更新时间';
COMMENT ON COLUMN system.system_tenant.deleted IS '是否删除';
COMMENT ON COLUMN system.system_tenant.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_tenant IS '租户表';