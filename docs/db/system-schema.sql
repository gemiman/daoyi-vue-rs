create schema system;

comment on schema system is 'system';

alter schema system owner to daoyivuers;


-- ----------------------------
-- Table structure for system.system_users
-- ----------------------------
DROP TABLE IF EXISTS system.system_users;
CREATE TABLE system.system_users
(
    id          varchar(32)  NOT NULL primary key,
    username    varchar(30)  NOT NULL,
    password    varchar(100) NOT NULL DEFAULT '',
    nickname    varchar(256) NOT NULL DEFAULT '',
    remark      varchar(500) NULL     DEFAULT NULL,
    dept_id     varchar(32)  NULL     DEFAULT NULL,
    post_ids    varchar(255)[] NULL     DEFAULT NULL,
    email       varchar(50)  NULL     DEFAULT '',
    mobile      varchar(11)  NULL     DEFAULT '',
    sex         varchar(1)   NULL     DEFAULT '0',
    avatar      varchar(512) NULL     DEFAULT '',
    status      varchar(1)   NOT NULL DEFAULT '0',
    login_ip    varchar(50)  NULL     DEFAULT '',
    login_date  timestamp    NULL     DEFAULT NULL,
    creator     varchar(64)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(64)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
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