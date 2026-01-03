create schema infra;

comment on schema infra is '基础设施';

alter schema infra owner to daoyivuers;


-- ----------------------------
-- Table structure for infra.infra_file_config
-- ----------------------------
DROP TABLE IF EXISTS infra.infra_file_config;
CREATE TABLE infra.infra_file_config
(
    id          varchar(32)  NOT NULL primary key,
    name        varchar(63)  NOT NULL,
    storage     varchar(2)   NOT NULL,
    remark      varchar(255) NULL     DEFAULT NULL,
    master      bool         NOT NULL,
    config      jsonb        NOT NULL,
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN infra.infra_file_config.id IS '编号';
COMMENT ON COLUMN infra.infra_file_config.name IS '配置名';
COMMENT ON COLUMN infra.infra_file_config.storage IS '存储器';
COMMENT ON COLUMN infra.infra_file_config.remark IS '备注';
COMMENT ON COLUMN infra.infra_file_config.master IS '是否为主配置';
COMMENT ON COLUMN infra.infra_file_config.config IS '存储配置';
COMMENT ON COLUMN infra.infra_file_config.creator IS '创建者';
COMMENT ON COLUMN infra.infra_file_config.create_time IS '创建时间';
COMMENT ON COLUMN infra.infra_file_config.updater IS '更新者';
COMMENT ON COLUMN infra.infra_file_config.update_time IS '更新时间';
COMMENT ON COLUMN infra.infra_file_config.deleted IS '是否删除';
COMMENT ON COLUMN infra.infra_file_config.tenant_id IS '租户编号';
COMMENT ON TABLE infra.infra_file_config IS '文件配置表';


-- ----------------------------
-- Table structure for infra.infra_file_content
-- ----------------------------
DROP TABLE IF EXISTS infra.infra_file_content;
CREATE TABLE infra.infra_file_content
(
    id          varchar(32)  NOT NULL primary key,
    config_id   varchar(32)         NOT NULL,
    path        varchar(512) NOT NULL,
    content     bytea        NOT NULL,
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON COLUMN infra.infra_file_content.id IS '编号';
COMMENT ON COLUMN infra.infra_file_content.config_id IS '配置编号';
COMMENT ON COLUMN infra.infra_file_content.path IS '文件路径';
COMMENT ON COLUMN infra.infra_file_content.content IS '文件内容';
COMMENT ON COLUMN infra.infra_file_content.creator IS '创建者';
COMMENT ON COLUMN infra.infra_file_content.create_time IS '创建时间';
COMMENT ON COLUMN infra.infra_file_content.updater IS '更新者';
COMMENT ON COLUMN infra.infra_file_content.update_time IS '更新时间';
COMMENT ON COLUMN infra.infra_file_content.deleted IS '是否删除';
COMMENT ON COLUMN infra.infra_file_content.tenant_id IS '租户编号';
COMMENT ON TABLE infra.infra_file_content IS '文件存储表';


-- ----------------------------
-- Table structure for infra.infra_file
-- ----------------------------
DROP TABLE IF EXISTS infra.infra_file;
CREATE TABLE infra.infra_file
(
    id          varchar(32)  NOT NULL primary key,
    config_id   varchar(32)          NULL     DEFAULT NULL,
    name        varchar(256)  NULL     DEFAULT NULL,
    path        varchar(512)  NOT NULL,
    url         varchar(1024) NOT NULL,
    type        varchar(128)  NULL     DEFAULT NULL,
    size        int4          NOT NULL,
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);


COMMENT ON COLUMN infra.infra_file.id IS '文件编号';
COMMENT ON COLUMN infra.infra_file.config_id IS '配置编号';
COMMENT ON COLUMN infra.infra_file.name IS '文件名';
COMMENT ON COLUMN infra.infra_file.path IS '文件路径';
COMMENT ON COLUMN infra.infra_file.url IS '文件 URL';
COMMENT ON COLUMN infra.infra_file.type IS '文件类型';
COMMENT ON COLUMN infra.infra_file.size IS '文件大小';
COMMENT ON COLUMN infra.infra_file.creator IS '创建者';
COMMENT ON COLUMN infra.infra_file.create_time IS '创建时间';
COMMENT ON COLUMN infra.infra_file.updater IS '更新者';
COMMENT ON COLUMN infra.infra_file.update_time IS '更新时间';
COMMENT ON COLUMN infra.infra_file.deleted IS '是否删除';
COMMENT ON COLUMN infra.infra_file.tenant_id IS '租户编号';
COMMENT ON TABLE infra.infra_file IS '文件表';