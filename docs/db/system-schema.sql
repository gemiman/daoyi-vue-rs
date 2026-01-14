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
        '系统管理员，默认初始化，密码：Aa123456'::varchar(500), '0'::varchar(32), '{0}', 'gemiman@vip.qq.com'::varchar(50),
        '17621038080'::varchar(11), '1'::varchar(1), DEFAULT, '0'::varchar(1), '0.0.0.0'::varchar(50),
        '2025-12-26 12:16:02.000000'::timestamp, '0'::varchar(64), '2025-12-26 12:16:12.000000'::timestamp,
        '0'::varchar(64), '2025-12-26 12:16:18.000000'::timestamp, false::boolean, '0'::varchar(32));
commit;

-- ----------------------------
-- Table structure for system.system_access_token
-- ----------------------------
DROP TABLE IF EXISTS system.system_access_token;
CREATE TABLE system.system_access_token
(
    id           varchar(32)  NOT NULL primary key,
    user_id      varchar(32)  NOT NULL,
    access_token varchar(255) NOT NULL,
    refresh_token varchar(255) NOT NULL,
    expires_time timestamp    NOT NULL,
    creator      varchar(32)  NULL     DEFAULT '',
    create_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater      varchar(32)  NULL     DEFAULT '',
    update_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted      boolean      NOT NULL DEFAULT false,
    tenant_id    varchar(32)  NOT NULL DEFAULT '0'
);

CREATE INDEX idx_system_access_token_01 ON system.system_access_token (access_token);
CREATE INDEX idx_system_refresh_token_01 ON system.system_access_token (refresh_token);

COMMENT ON COLUMN system.system_access_token.id IS '编号';
COMMENT ON COLUMN system.system_access_token.user_id IS '用户编号';
COMMENT ON COLUMN system.system_access_token.access_token IS '访问令牌';
COMMENT ON COLUMN system.system_access_token.refresh_token IS '刷新令牌';
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
    id              varchar(32)    NOT NULL primary key,
    name            varchar(128)   NOT NULL,
    contact_user_id varchar(32)    NULL     DEFAULT NULL,
    contact_name    varchar(128)   NOT NULL,
    contact_mobile  varchar(128)   NULL     DEFAULT NULL,
    status          varchar(1)     NOT NULL DEFAULT '0',
    websites        varchar(256)[] NULL     DEFAULT '{}',
    package_id      varchar(32)    NOT NULL,
    expire_time     timestamp      NOT NULL,
    account_count   int4           NOT NULL,
    creator         varchar(32)    NULL     DEFAULT '',
    create_time     timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater         varchar(32)    NULL     DEFAULT '',
    update_time     timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted         boolean        NOT NULL DEFAULT false,
    tenant_id       varchar(32)    NOT NULL DEFAULT '0'
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
INSERT INTO system.system_tenant (id, name, contact_user_id, contact_name, contact_mobile, status, websites, package_id,
                                  expire_time, account_count, creator, create_time, updater, update_time, deleted,
                                  tenant_id)
VALUES ('0', '系统租户', '0', '兰陵王', '17621038080', '0', '{localhost}', '0', '2035-12-26 16:15:46.000000', 3, '0',
        '2025-12-26 16:16:16.000000', '0', '2025-12-26 16:16:25.000000', false, '0');
commit;


-- ----------------------------
-- Table structure for system.system_dict_data
-- ----------------------------
DROP TABLE IF EXISTS system.system_dict_data;
CREATE TABLE system.system_dict_data
(
    id          varchar(32)  NOT NULL primary key,
    sort        int4         NOT NULL DEFAULT 0,
    label       varchar(100) NOT NULL DEFAULT '',
    value       varchar(100) NOT NULL DEFAULT '',
    dict_type   varchar(100) NOT NULL DEFAULT '',
    status      varchar(1)   NOT NULL DEFAULT '0',
    color_type  varchar(100) NULL     DEFAULT '',
    css_class   varchar(100) NULL     DEFAULT '',
    remark      varchar(500) NULL     DEFAULT NULL,
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_dict_data.id IS '字典编码';
COMMENT ON COLUMN system.system_dict_data.sort IS '字典排序';
COMMENT ON COLUMN system.system_dict_data.label IS '字典标签';
COMMENT ON COLUMN system.system_dict_data.value IS '字典键值';
COMMENT ON COLUMN system.system_dict_data.dict_type IS '字典类型';
COMMENT ON COLUMN system.system_dict_data.status IS '状态（0正常 1停用）';
COMMENT ON COLUMN system.system_dict_data.color_type IS '颜色类型';
COMMENT ON COLUMN system.system_dict_data.css_class IS 'css 样式';
COMMENT ON COLUMN system.system_dict_data.remark IS '备注';
COMMENT ON COLUMN system.system_dict_data.creator IS '创建者';
COMMENT ON COLUMN system.system_dict_data.create_time IS '创建时间';
COMMENT ON COLUMN system.system_dict_data.updater IS '更新者';
COMMENT ON COLUMN system.system_dict_data.update_time IS '更新时间';
COMMENT ON COLUMN system.system_dict_data.deleted IS '是否删除';
COMMENT ON COLUMN system.system_dict_data.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_dict_data IS '字典数据表';


-- ----------------------------
-- Table structure for system.system_dict_type
-- ----------------------------
DROP TABLE IF EXISTS system.system_dict_type;
CREATE TABLE system.system_dict_type
(
    id           varchar(32)  NOT NULL primary key,
    name         varchar(100) NOT NULL DEFAULT '',
    type         varchar(100) NOT NULL DEFAULT '',
    status       varchar(1)   NOT NULL DEFAULT '0',
    remark       varchar(500) NULL     DEFAULT NULL,
    deleted_time timestamp    NULL     DEFAULT NULL,
    creator      varchar(32)  NULL     DEFAULT '',
    create_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater      varchar(32)  NULL     DEFAULT '',
    update_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted      boolean      NOT NULL DEFAULT false,
    tenant_id    varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_dict_type.id IS '字典主键';
COMMENT ON COLUMN system.system_dict_type.name IS '字典名称';
COMMENT ON COLUMN system.system_dict_type.type IS '字典类型';
COMMENT ON COLUMN system.system_dict_type.status IS '状态（0正常 1停用）';
COMMENT ON COLUMN system.system_dict_type.remark IS '备注';
COMMENT ON COLUMN system.system_dict_type.creator IS '创建者';
COMMENT ON COLUMN system.system_dict_type.create_time IS '创建时间';
COMMENT ON COLUMN system.system_dict_type.updater IS '更新者';
COMMENT ON COLUMN system.system_dict_type.update_time IS '更新时间';
COMMENT ON COLUMN system.system_dict_type.deleted IS '是否删除';
COMMENT ON COLUMN system.system_dict_type.deleted_time IS '删除时间';
COMMENT ON COLUMN system.system_dict_type.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_dict_type IS '字典类型表';


-- ----------------------------
-- Table structure for system.system_user_role
-- ----------------------------
DROP TABLE IF EXISTS system.system_user_role;
CREATE TABLE system.system_user_role
(
    id          varchar(32) NOT NULL primary key,
    user_id     varchar(32) NOT NULL,
    role_id     varchar(32) NOT NULL,
    creator     varchar(64) NULL     DEFAULT '',
    create_time timestamp   NULL     DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64) NULL     DEFAULT '',
    update_time timestamp   NULL     DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);


COMMENT ON COLUMN system.system_user_role.id IS '自增编号';
COMMENT ON COLUMN system.system_user_role.user_id IS '用户ID';
COMMENT ON COLUMN system.system_user_role.role_id IS '角色ID';
COMMENT ON COLUMN system.system_user_role.creator IS '创建者';
COMMENT ON COLUMN system.system_user_role.create_time IS '创建时间';
COMMENT ON COLUMN system.system_user_role.updater IS '更新者';
COMMENT ON COLUMN system.system_user_role.update_time IS '更新时间';
COMMENT ON COLUMN system.system_user_role.deleted IS '是否删除';
COMMENT ON COLUMN system.system_user_role.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_user_role IS '用户和角色关联表';


-- ----------------------------
-- Table structure for system.system_role
-- ----------------------------
DROP TABLE IF EXISTS system.system_role;
CREATE TABLE system.system_role
(
    id          varchar(32)  NOT NULL primary key,
    name        varchar(30)  NOT NULL,
    code        varchar(100) NOT NULL,
    sort        int4         NOT NULL,
    data_scope  varchar(1)   NOT NULL DEFAULT '1',
    data_scope_dept_ids varchar(32)[] NOT NULL DEFAULT '{}',
    status      varchar(1)   NOT NULL,
    type        varchar(1)   NOT NULL,
    remark      varchar(500) NULL     DEFAULT NULL,
    creator     varchar(64)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_role.id IS '角色ID';
COMMENT ON COLUMN system.system_role.name IS '角色名称';
COMMENT ON COLUMN system.system_role.code IS '角色权限字符串';
COMMENT ON COLUMN system.system_role.sort IS '显示顺序';
COMMENT ON COLUMN system.system_role.data_scope IS '数据范围（1：全部数据权限 2：自定数据权限 3：本部门数据权限 4：本部门及以下数据权限）';
COMMENT ON COLUMN system.system_role.data_scope_dept_ids IS '数据范围 ( 指定部门数组)';
COMMENT ON COLUMN system.system_role.status IS '角色状态（0正常 1停用）';
COMMENT ON COLUMN system.system_role.type IS '角色类型';
COMMENT ON COLUMN system.system_role.remark IS '备注';
COMMENT ON COLUMN system.system_role.creator IS '创建者';
COMMENT ON COLUMN system.system_role.create_time IS '创建时间';
COMMENT ON COLUMN system.system_role.updater IS '更新者';
COMMENT ON COLUMN system.system_role.update_time IS '更新时间';
COMMENT ON COLUMN system.system_role.deleted IS '是否删除';
COMMENT ON COLUMN system.system_role.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_role IS '角色信息表';


-- ----------------------------
-- Table structure for system.system_role_menu
-- ----------------------------
DROP TABLE IF EXISTS system.system_role_menu;
CREATE TABLE system.system_role_menu
(
    id          varchar(32) NOT NULL primary key,
    role_id     varchar(32) NOT NULL,
    menu_id     varchar(32) NOT NULL,
    creator     varchar(64) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_role_menu.id IS '自增编号';
COMMENT ON COLUMN system.system_role_menu.role_id IS '角色ID';
COMMENT ON COLUMN system.system_role_menu.menu_id IS '菜单ID';
COMMENT ON COLUMN system.system_role_menu.creator IS '创建者';
COMMENT ON COLUMN system.system_role_menu.create_time IS '创建时间';
COMMENT ON COLUMN system.system_role_menu.updater IS '更新者';
COMMENT ON COLUMN system.system_role_menu.update_time IS '更新时间';
COMMENT ON COLUMN system.system_role_menu.deleted IS '是否删除';
COMMENT ON COLUMN system.system_role_menu.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_role_menu IS '角色和菜单关联表';


-- ----------------------------
-- Table structure for system.system_menu
-- ----------------------------
DROP TABLE IF EXISTS system.system_menu;
CREATE TABLE system.system_menu
(
    id             varchar(32)  NOT NULL primary key,
    name           varchar(50)  NOT NULL,
    permission     varchar(100) NOT NULL DEFAULT '',
    type           varchar(1)   NOT NULL,
    sort           int4         NOT NULL DEFAULT 0,
    parent_id      varchar(32)  NOT NULL DEFAULT '0',
    path           varchar(200) NULL     DEFAULT '',
    icon           varchar(100) NULL     DEFAULT '#',
    component      varchar(255) NULL     DEFAULT NULL,
    component_name varchar(255) NULL     DEFAULT NULL,
    status         varchar(1)   NOT NULL DEFAULT '0',
    visible        bool         NOT NULL DEFAULT true,
    keep_alive     bool         NOT NULL DEFAULT true,
    always_show    bool         NOT NULL DEFAULT true,
    api            varchar(128) NULL     DEFAULT NULL,
    creator        varchar(64)  NULL     DEFAULT '',
    create_time    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(64)  NULL     DEFAULT '',
    update_time    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean      NOT NULL DEFAULT false,
    tenant_id      varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_menu.id IS '菜单ID';
COMMENT ON COLUMN system.system_menu.name IS '菜单名称';
COMMENT ON COLUMN system.system_menu.permission IS '权限标识';
COMMENT ON COLUMN system.system_menu.type IS '菜单类型';
COMMENT ON COLUMN system.system_menu.sort IS '显示顺序';
COMMENT ON COLUMN system.system_menu.parent_id IS '父菜单ID';
COMMENT ON COLUMN system.system_menu.path IS '路由地址';
COMMENT ON COLUMN system.system_menu.icon IS '菜单图标';
COMMENT ON COLUMN system.system_menu.component IS '组件路径';
COMMENT ON COLUMN system.system_menu.component_name IS '组件名';
COMMENT ON COLUMN system.system_menu.status IS '菜单状态';
COMMENT ON COLUMN system.system_menu.visible IS '是否可见';
COMMENT ON COLUMN system.system_menu.keep_alive IS '是否缓存';
COMMENT ON COLUMN system.system_menu.always_show IS '是否总是显示';
comment on column system.system_menu.api is '接口';
COMMENT ON COLUMN system.system_menu.creator IS '创建者';
COMMENT ON COLUMN system.system_menu.create_time IS '创建时间';
COMMENT ON COLUMN system.system_menu.updater IS '更新者';
COMMENT ON COLUMN system.system_menu.update_time IS '更新时间';
COMMENT ON COLUMN system.system_menu.deleted IS '是否删除';
COMMENT ON COLUMN system.system_menu.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_menu IS '菜单权限表';


-- ----------------------------
-- Table structure for system.system_tenant_package
-- ----------------------------
DROP TABLE IF EXISTS system.system_tenant_package;
CREATE TABLE system.system_tenant_package
(
    id          varchar(32)   NOT NULL primary key,
    name        varchar(30)   NOT NULL,
    status      varchar(1)    NOT NULL DEFAULT '0',
    remark      varchar(256)  NULL     DEFAULT '',
    menu_ids    varchar(32)[] NOT NULL DEFAULT '{}',
    creator     varchar(64)   NULL     DEFAULT '',
    create_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)   NULL     DEFAULT '',
    update_time timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean       NOT NULL DEFAULT false,
    tenant_id   varchar(32)   NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_tenant_package.id IS '套餐编号';
COMMENT ON COLUMN system.system_tenant_package.name IS '套餐名';
COMMENT ON COLUMN system.system_tenant_package.status IS '租户状态（0正常 1停用）';
COMMENT ON COLUMN system.system_tenant_package.remark IS '备注';
COMMENT ON COLUMN system.system_tenant_package.menu_ids IS '关联的菜单编号';
COMMENT ON COLUMN system.system_tenant_package.creator IS '创建者';
COMMENT ON COLUMN system.system_tenant_package.create_time IS '创建时间';
COMMENT ON COLUMN system.system_tenant_package.updater IS '更新者';
COMMENT ON COLUMN system.system_tenant_package.update_time IS '更新时间';
COMMENT ON COLUMN system.system_tenant_package.deleted IS '是否删除';
COMMENT ON COLUMN system.system_tenant_package.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_tenant_package IS '租户套餐表';


-- ----------------------------
-- Table structure for system.system_user_post
-- ----------------------------
DROP TABLE IF EXISTS system.system_user_post;
CREATE TABLE system.system_user_post
(
    id          varchar(32) NOT NULL primary key,
    user_id     varchar(32) NOT NULL DEFAULT '0',
    post_id     varchar(32) NOT NULL DEFAULT '0',
    creator     varchar(64) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_user_post.id IS 'id';
COMMENT ON COLUMN system.system_user_post.user_id IS '用户ID';
COMMENT ON COLUMN system.system_user_post.post_id IS '岗位ID';
COMMENT ON COLUMN system.system_user_post.creator IS '创建者';
COMMENT ON COLUMN system.system_user_post.create_time IS '创建时间';
COMMENT ON COLUMN system.system_user_post.updater IS '更新者';
COMMENT ON COLUMN system.system_user_post.update_time IS '更新时间';
COMMENT ON COLUMN system.system_user_post.deleted IS '是否删除';
COMMENT ON COLUMN system.system_user_post.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_user_post IS '用户岗位表';


-- ----------------------------
-- Table structure for system.system_dept
-- ----------------------------
DROP TABLE IF EXISTS system.system_dept;
CREATE TABLE system.system_dept
(
    id             varchar(32) NOT NULL primary key,
    name           varchar(30) NOT NULL DEFAULT '',
    parent_id      varchar(32) NOT NULL DEFAULT 0,
    sort           int4        NOT NULL DEFAULT 0,
    leader_user_id varchar(32) NULL     DEFAULT NULL,
    phone          varchar(32) NULL     DEFAULT NULL,
    email          varchar(50) NULL     DEFAULT NULL,
    status         varchar(1)  NOT NULL,
    creator        varchar(64) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(64) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_dept.id IS '部门id';
COMMENT ON COLUMN system.system_dept.name IS '部门名称';
COMMENT ON COLUMN system.system_dept.parent_id IS '父部门id';
COMMENT ON COLUMN system.system_dept.sort IS '显示顺序';
COMMENT ON COLUMN system.system_dept.leader_user_id IS '负责人';
COMMENT ON COLUMN system.system_dept.phone IS '联系电话';
COMMENT ON COLUMN system.system_dept.email IS '邮箱';
COMMENT ON COLUMN system.system_dept.status IS '部门状态（0正常 1停用）';
COMMENT ON COLUMN system.system_dept.creator IS '创建者';
COMMENT ON COLUMN system.system_dept.create_time IS '创建时间';
COMMENT ON COLUMN system.system_dept.updater IS '更新者';
COMMENT ON COLUMN system.system_dept.update_time IS '更新时间';
COMMENT ON COLUMN system.system_dept.deleted IS '是否删除';
COMMENT ON COLUMN system.system_dept.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_dept IS '部门表';


-- ----------------------------
-- Table structure for system.system_post
-- ----------------------------
DROP TABLE IF EXISTS system.system_post;
CREATE TABLE system.system_post
(
    id          varchar(32)  NOT NULL primary key,
    code        varchar(64)  NOT NULL,
    name        varchar(50)  NOT NULL,
    sort        int4         NOT NULL,
    status      varchar(1)   NOT NULL,
    remark      varchar(500) NULL     DEFAULT NULL,
    creator     varchar(64)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_post.id IS '岗位ID';
COMMENT ON COLUMN system.system_post.code IS '岗位编码';
COMMENT ON COLUMN system.system_post.name IS '岗位名称';
COMMENT ON COLUMN system.system_post.sort IS '显示顺序';
COMMENT ON COLUMN system.system_post.status IS '状态（0正常 1停用）';
COMMENT ON COLUMN system.system_post.remark IS '备注';
COMMENT ON COLUMN system.system_post.creator IS '创建者';
COMMENT ON COLUMN system.system_post.create_time IS '创建时间';
COMMENT ON COLUMN system.system_post.updater IS '更新者';
COMMENT ON COLUMN system.system_post.update_time IS '更新时间';
COMMENT ON COLUMN system.system_post.deleted IS '是否删除';
COMMENT ON COLUMN system.system_post.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_post IS '岗位信息表';


-- ----------------------------
-- Table structure for system.system_notice
-- ----------------------------
DROP TABLE IF EXISTS system.system_notice;
CREATE TABLE system.system_notice
(
    id          varchar(32) NOT NULL primary key,
    title       varchar(50) NOT NULL,
    content     text        NOT NULL,
    type        varchar(1)  NOT NULL,
    status      varchar(1)  NOT NULL DEFAULT '0',
    creator     varchar(64) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_notice.id IS '公告ID';
COMMENT ON COLUMN system.system_notice.title IS '公告标题';
COMMENT ON COLUMN system.system_notice.content IS '公告内容';
COMMENT ON COLUMN system.system_notice.type IS '公告类型（1通知 2公告）';
COMMENT ON COLUMN system.system_notice.status IS '公告状态（0正常 1关闭）';
COMMENT ON COLUMN system.system_notice.creator IS '创建者';
COMMENT ON COLUMN system.system_notice.create_time IS '创建时间';
COMMENT ON COLUMN system.system_notice.updater IS '更新者';
COMMENT ON COLUMN system.system_notice.update_time IS '更新时间';
COMMENT ON COLUMN system.system_notice.deleted IS '是否删除';
COMMENT ON COLUMN system.system_notice.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_notice IS '通知公告表';


-- ----------------------------
-- Table structure for system.system_notify_template
-- ----------------------------
DROP TABLE IF EXISTS system.system_notify_template;
CREATE TABLE system.system_notify_template
(
    id          varchar(32)    NOT NULL primary key,
    name        varchar(63)    NOT NULL,
    code        varchar(64)    NOT NULL,
    nickname    varchar(255)   NOT NULL,
    content     varchar(1024)  NOT NULL,
    type        varchar(1)     NOT NULL,
    params      varchar(255)[] NULL     DEFAULT '{}',
    status      varchar(1)     NOT NULL DEFAULT '0',
    remark      varchar(255)   NULL     DEFAULT NULL,
    creator     varchar(64)    NULL     DEFAULT '',
    create_time timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)    NULL     DEFAULT '',
    update_time timestamp      NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean        NOT NULL DEFAULT false,
    tenant_id   varchar(32)    NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_notify_template.id IS '主键';
COMMENT ON COLUMN system.system_notify_template.name IS '模板名称';
COMMENT ON COLUMN system.system_notify_template.code IS '模版编码';
COMMENT ON COLUMN system.system_notify_template.nickname IS '发送人名称';
COMMENT ON COLUMN system.system_notify_template.content IS '模版内容';
COMMENT ON COLUMN system.system_notify_template.type IS '类型';
COMMENT ON COLUMN system.system_notify_template.params IS '参数数组';
COMMENT ON COLUMN system.system_notify_template.status IS '状态';
COMMENT ON COLUMN system.system_notify_template.remark IS '备注';
COMMENT ON COLUMN system.system_notify_template.creator IS '创建者';
COMMENT ON COLUMN system.system_notify_template.create_time IS '创建时间';
COMMENT ON COLUMN system.system_notify_template.updater IS '更新者';
COMMENT ON COLUMN system.system_notify_template.update_time IS '更新时间';
COMMENT ON COLUMN system.system_notify_template.deleted IS '是否删除';
COMMENT ON COLUMN system.system_notify_template.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_notify_template IS '站内信模板表';


-- ----------------------------
-- Table structure for system.system_notify_message
-- ----------------------------
DROP TABLE IF EXISTS system.system_notify_message;
CREATE TABLE system.system_notify_message
(
    id                varchar(32)   NOT NULL primary key,
    user_id           varchar(32)   NOT NULL,
    user_type         varchar(1)    NOT NULL,
    template_id       varchar(32)   NOT NULL,
    template_code     varchar(64)   NOT NULL,
    template_nickname varchar(63)   NOT NULL,
    template_content  varchar(1024) NOT NULL,
    template_type     varchar(1)    NOT NULL,
    template_params   jsonb         NOT NULL,
    read_status       bool          NOT NULL,
    read_time         timestamp     NULL     DEFAULT NULL,
    creator           varchar(64)   NULL     DEFAULT '',
    create_time       timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(64)   NULL     DEFAULT '',
    update_time       timestamp     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean       NOT NULL DEFAULT false,
    tenant_id         varchar(32)   NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_notify_message.id IS '消息ID';
COMMENT ON COLUMN system.system_notify_message.user_id IS '用户id';
COMMENT ON COLUMN system.system_notify_message.user_type IS '用户类型';
COMMENT ON COLUMN system.system_notify_message.template_id IS '模版编号';
COMMENT ON COLUMN system.system_notify_message.template_code IS '模板编码';
COMMENT ON COLUMN system.system_notify_message.template_nickname IS '模版发送人名称';
COMMENT ON COLUMN system.system_notify_message.template_content IS '模版内容';
COMMENT ON COLUMN system.system_notify_message.template_type IS '模版类型';
COMMENT ON COLUMN system.system_notify_message.template_params IS '模版参数';
COMMENT ON COLUMN system.system_notify_message.read_status IS '是否已读';
COMMENT ON COLUMN system.system_notify_message.read_time IS '阅读时间';
COMMENT ON COLUMN system.system_notify_message.creator IS '创建者';
COMMENT ON COLUMN system.system_notify_message.create_time IS '创建时间';
COMMENT ON COLUMN system.system_notify_message.updater IS '更新者';
COMMENT ON COLUMN system.system_notify_message.update_time IS '更新时间';
COMMENT ON COLUMN system.system_notify_message.deleted IS '是否删除';
COMMENT ON COLUMN system.system_notify_message.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_notify_message IS '站内信消息表';


-- ----------------------------
-- Table structure for system.system_mail_account
-- ----------------------------
DROP TABLE IF EXISTS system.system_mail_account;
CREATE TABLE system.system_mail_account
(
    id                 varchar(32)  NOT NULL primary key,
    mail               varchar(255) NOT NULL,
    username           varchar(255) NOT NULL,
    password_plaintext varchar(255) NOT NULL,
    host               varchar(255) NOT NULL,
    port               int4         NOT NULL,
    ssl_enable         bool         NOT NULL DEFAULT false,
    starttls_enable    bool         NOT NULL DEFAULT false,
    creator            varchar(64)  NULL     DEFAULT '',
    create_time        timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater            varchar(64)  NULL     DEFAULT '',
    update_time        timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted            boolean      NOT NULL DEFAULT false,
    tenant_id          varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN system.system_mail_account.id IS '主键';
COMMENT ON COLUMN system.system_mail_account.mail IS '邮箱';
COMMENT ON COLUMN system.system_mail_account.username IS '用户名';
COMMENT ON COLUMN system.system_mail_account.password_plaintext IS '密码';
COMMENT ON COLUMN system.system_mail_account.host IS 'SMTP 服务器域名';
COMMENT ON COLUMN system.system_mail_account.port IS 'SMTP 服务器端口';
COMMENT ON COLUMN system.system_mail_account.ssl_enable IS '是否开启 SSL';
COMMENT ON COLUMN system.system_mail_account.starttls_enable IS '是否开启 STARTTLS';
COMMENT ON COLUMN system.system_mail_account.creator IS '创建者';
COMMENT ON COLUMN system.system_mail_account.create_time IS '创建时间';
COMMENT ON COLUMN system.system_mail_account.updater IS '更新者';
COMMENT ON COLUMN system.system_mail_account.update_time IS '更新时间';
COMMENT ON COLUMN system.system_mail_account.deleted IS '是否删除';
COMMENT ON COLUMN system.system_mail_account.tenant_id IS '租户编号';
COMMENT ON TABLE system.system_mail_account IS '邮箱账号表';