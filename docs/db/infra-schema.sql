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