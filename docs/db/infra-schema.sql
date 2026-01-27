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
    config_id varchar(32) NOT NULL,
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
    id          varchar(32) NOT NULL primary key,
    config_id   varchar(32) NULL     DEFAULT NULL,
    name        varchar(256)  NULL     DEFAULT NULL,
    path        varchar(512)  NOT NULL,
    url         varchar(1024) NOT NULL,
    type        varchar(128)  NULL     DEFAULT NULL,
    size        int4          NOT NULL,
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
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
-- ----------------------------
-- Table structure for infra.infra_codegen_table
-- ----------------------------
DROP TABLE IF EXISTS infra.infra_codegen_table;
CREATE TABLE infra.infra_codegen_table
(
    id                    varchar(32)  NOT NULL,
    data_source_config_id varchar(32)  NOT NULL,
    scene                 int2         NOT NULL DEFAULT 1,
    table_name            varchar(200) NOT NULL DEFAULT '',
    table_comment         varchar(500) NOT NULL DEFAULT '',
    remark                varchar(500)          DEFAULT NULL,
    module_name           varchar(100) NOT NULL DEFAULT '',
    business_name         varchar(100) NOT NULL DEFAULT '',
    class_name            varchar(100) NOT NULL DEFAULT '',
    class_comment         varchar(500) NOT NULL DEFAULT '',
    author                varchar(100) NOT NULL DEFAULT '',
    template_type         int2         NOT NULL DEFAULT 1,
    front_type            int2         NOT NULL DEFAULT 10,
    parent_menu_id        varchar(32)           DEFAULT NULL,
    master_table_id       varchar(32)           DEFAULT NULL,
    sub_join_column_id    varchar(32)           DEFAULT NULL,
    sub_join_many         bool                  DEFAULT NULL,
    tree_parent_column_id varchar(32)           DEFAULT NULL,
    tree_name_column_id   varchar(32)           DEFAULT NULL,
    creator               varchar(64)           DEFAULT '',
    create_time           timestamp             DEFAULT NULL,
    updater               varchar(64)           DEFAULT '',
    update_time           timestamp             DEFAULT NULL,
    deleted               bool         NOT NULL DEFAULT false,
    tenant_id             varchar(32)           DEFAULT NULL,
    PRIMARY KEY (id)
);

-- ----------------------------
-- Table structure for infra.infra_codegen_column
-- ----------------------------
DROP TABLE IF EXISTS infra.infra_codegen_column;
CREATE TABLE infra.infra_codegen_column
(
    id                       varchar(32)  NOT NULL,
    table_id                 varchar(32)  NOT NULL,
    column_name              varchar(200) NOT NULL,
    data_type                varchar(200) NOT NULL DEFAULT '',
    column_comment           varchar(500) NOT NULL DEFAULT '',
    nullable                 bool         NOT NULL DEFAULT false,
    primary_key              bool         NOT NULL DEFAULT false,
    ordinal_position         int4         NOT NULL DEFAULT 0,
    java_type                varchar(64)  NOT NULL DEFAULT '',
    java_field               varchar(64)  NOT NULL DEFAULT '',
    dict_type                varchar(200)          DEFAULT '',
    example                  varchar(500)          DEFAULT NULL,
    create_operation         bool         NOT NULL DEFAULT true,
    update_operation         bool         NOT NULL DEFAULT true,
    list_operation           bool         NOT NULL DEFAULT true,
    list_operation_condition varchar(32)  NOT NULL DEFAULT '=',
    list_operation_result    bool         NOT NULL DEFAULT true,
    html_type                varchar(32)  NOT NULL DEFAULT 'input',
    creator                  varchar(64)           DEFAULT '',
    create_time              timestamp             DEFAULT NULL,
    updater                  varchar(64)           DEFAULT '',
    update_time              timestamp             DEFAULT NULL,
    deleted                  bool         NOT NULL DEFAULT false,
    PRIMARY KEY (id)
);
create table infra.infra_data_source_config
(
    id                 character varying(32) primary key not null,                           -- 主键编号
    name               character varying(100)            not null default '',                -- 参数名称
    url                character varying(1024)           not null,                           -- 数据源连接
    schema_name        character varying(255)                     default NULL,              -- 数据库名
    username           character varying(255)                     default NULL,              -- 用户名
    password_plaintext character varying(255)                     default NULL,              -- 密码
    creator            character varying(32)                      default '',                -- 创建者
    create_time        timestamp without time zone       not null default CURRENT_TIMESTAMP, -- 创建时间
    updater            character varying(32)                      default '',                -- 更新者
    update_time        timestamp without time zone       not null default CURRENT_TIMESTAMP, -- 更新时间
    deleted            boolean                           not null default false,             -- 是否删除
    tenant_id          character varying(32)             not null default '0'                -- 租户编号
);
comment on table infra.infra_data_source_config is '数据源配置表';
comment on column infra.infra_data_source_config.id is '主键编号';
comment on column infra.infra_data_source_config.name is '参数名称';
comment on column infra.infra_data_source_config.url is '数据源连接';
comment on column infra.infra_data_source_config.schema_name is '数据库名';
comment on column infra.infra_data_source_config.username is '用户名';
comment on column infra.infra_data_source_config.password_plaintext is '密码';
comment on column infra.infra_data_source_config.creator is '创建者';
comment on column infra.infra_data_source_config.create_time is '创建时间';
comment on column infra.infra_data_source_config.updater is '更新者';
comment on column infra.infra_data_source_config.update_time is '更新时间';
comment on column infra.infra_data_source_config.deleted is '是否删除';
comment on column infra.infra_data_source_config.tenant_id is '租户编号';

