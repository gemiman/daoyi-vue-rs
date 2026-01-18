-- ========================================
-- 坐席表
-- ========================================
DROP TABLE IF EXISTS cti_agent;

CREATE TABLE cti_agent
(
    id             varchar(32) NOT NULL primary key,
    company_id     varchar(32) NOT NULL,
    agent_id       VARCHAR(32),
    agent_key      VARCHAR(32) NOT NULL,
    agent_name     VARCHAR(64),
    agent_code     VARCHAR(32),
    agent_type     VARCHAR(32)          DEFAULT '1' NOT NULL,
    passwd         VARCHAR(64) NOT NULL,
    sip_phone      VARCHAR(32),
    record         boolean              DEFAULT false,
    group_id       varchar(32),
    agent_online   INT                  DEFAULT 0,
    after_interval INT                  DEFAULT 0,
    display        VARCHAR(32),
    ring_time      INT                  DEFAULT 0,
    host           VARCHAR(64),
    ext1           VARCHAR(256),
    ext2           VARCHAR(256),
    ext3           VARCHAR(256),
    state          VARCHAR(32),
    status         varchar(32) NOT NULL default '0',
    creator        varchar(32) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_agent IS '坐席表,存储呼叫中心座席的基本信息';
COMMENT ON COLUMN cti_agent.id IS '主键ID';
COMMENT ON COLUMN cti_agent.company_id IS '企业ID';
COMMENT ON COLUMN cti_agent.agent_id IS '坐席工号';
COMMENT ON COLUMN cti_agent.agent_key IS '坐席账户(长度4-16位)';
COMMENT ON COLUMN cti_agent.agent_name IS '坐席名称';
COMMENT ON COLUMN cti_agent.agent_code IS '坐席分机号';
COMMENT ON COLUMN cti_agent.agent_type IS '坐席类型(1:普通坐席,2:班长)';
COMMENT ON COLUMN cti_agent.passwd IS '密码(SHA256加密后64位)';
COMMENT ON COLUMN cti_agent.sip_phone IS 'SIP号码(绑定电话)';
COMMENT ON COLUMN cti_agent.record IS '是否录音(0:否,1:是)';
COMMENT ON COLUMN cti_agent.group_id IS '主要技能组ID(必填项)';
COMMENT ON COLUMN cti_agent.agent_online IS '总机坐席';
COMMENT ON COLUMN cti_agent.after_interval IS '话后自动空闲间隔时长(秒)';
COMMENT ON COLUMN cti_agent.display IS '主叫显号';
COMMENT ON COLUMN cti_agent.ring_time IS '振铃时长(秒)';
COMMENT ON COLUMN cti_agent.host IS '登录服务地址';
COMMENT ON COLUMN cti_agent.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_agent.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_agent.ext3 IS '扩展字段3';
COMMENT ON COLUMN cti_agent.state IS '坐席登录状态';
COMMENT ON COLUMN cti_agent.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_agent.creator IS '创建者';
COMMENT ON COLUMN cti_agent.create_time IS '创建时间';
COMMENT ON COLUMN cti_agent.updater IS '更新者';
COMMENT ON COLUMN cti_agent.update_time IS '更新时间';
COMMENT ON COLUMN cti_agent.deleted IS '是否删除';
COMMENT ON COLUMN cti_agent.tenant_id IS '租户编号';

CREATE INDEX idx_agent_company ON cti_agent (company_id);
CREATE INDEX idx_agent_key ON cti_agent (agent_key);
CREATE INDEX idx_agent_status ON cti_agent (status);

-- ========================================
-- SIP账户表
-- ========================================
DROP TABLE IF EXISTS cti_agent_sip;

CREATE TABLE cti_agent_sip
(
    id            varchar(32) NOT NULL primary key,
    company_id    varchar(32) NOT NULL,
    sip           VARCHAR(32) NOT NULL,
    agent_id      varchar(32),
    sip_pwd       VARCHAR(32) NOT NULL,
    register_time timestamp,
    expire        INT                  DEFAULT 3600,
    status        varchar(32) NOT NULL default '0',
    creator       varchar(32) NULL     DEFAULT '',
    create_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(32) NULL     DEFAULT '',
    update_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       boolean     NOT NULL DEFAULT false,
    tenant_id     varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_agent_sip IS 'SIP账户表,存储座席SIP账户信息,用于座席SIP注册登录';
COMMENT ON COLUMN cti_agent_sip.id IS '主键ID';
COMMENT ON COLUMN cti_agent_sip.company_id IS '企业ID';
COMMENT ON COLUMN cti_agent_sip.sip IS 'SIP号码(5-16位)';
COMMENT ON COLUMN cti_agent_sip.agent_id IS '座席ID';
COMMENT ON COLUMN cti_agent_sip.sip_pwd IS 'SIP密码(8-16位)';
COMMENT ON COLUMN cti_agent_sip.register_time IS '注册时间(毫秒级时间戳)';
COMMENT ON COLUMN cti_agent_sip.expire IS '注册有效期(秒,默认3600)';
COMMENT ON COLUMN cti_agent_sip.status IS '状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_agent_sip.creator IS '创建者';
COMMENT ON COLUMN cti_agent_sip.create_time IS '创建时间';
COMMENT ON COLUMN cti_agent_sip.updater IS '更新者';
COMMENT ON COLUMN cti_agent_sip.update_time IS '更新时间';
COMMENT ON COLUMN cti_agent_sip.deleted IS '是否删除';
COMMENT ON COLUMN cti_agent_sip.tenant_id IS '租户编号';

-- ========================================
-- 坐席组表
-- ========================================
DROP TABLE IF EXISTS cti_agent_group;

CREATE TABLE cti_agent_group
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32) NOT NULL,
    agent_id    varchar(32) NOT NULL,
    agent_key   VARCHAR(32) NOT NULL,
    agent_type  varchar(32),
    group_id    varchar(32) NOT NULL,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_agent_group IS '坐席技能组表';
COMMENT ON COLUMN cti_agent_group.id IS '主键ID';
COMMENT ON COLUMN cti_agent_group.company_id IS '企业ID';
COMMENT ON COLUMN cti_agent_group.agent_id IS '坐席id';
COMMENT ON COLUMN cti_agent_group.agent_key IS '坐席key';
COMMENT ON COLUMN cti_agent_group.agent_type IS '坐席类型';
COMMENT ON COLUMN cti_agent_group.group_id IS '技能组id';
COMMENT ON COLUMN cti_agent_group.status IS '关系状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_agent_group.creator IS '创建者';
COMMENT ON COLUMN cti_agent_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_agent_group.updater IS '更新者';
COMMENT ON COLUMN cti_agent_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_agent_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_agent_group.tenant_id IS '租户编号';

CREATE INDEX idx_agent_group_agent ON cti_agent_group (agent_id);
CREATE INDEX idx_agent_group_group ON cti_agent_group (group_id);

-- ========================================
-- 坐席状态日志表
-- ========================================
DROP TABLE IF EXISTS cti_agent_state_log;

CREATE TABLE cti_agent_state_log
(
    id             varchar(32) NOT NULL primary key,
    company_id     varchar(32) NOT NULL,
    group_id       varchar(32),
    agent_id       varchar(32),
    agent_key      VARCHAR(32),
    agent_name     VARCHAR(64),
    call_id        varchar(32),
    login_type     varchar(32),
    work_type      varchar(32),
    host           VARCHAR(64),
    remote_address VARCHAR(64),
    before_state   VARCHAR(32),
    before_time    timestamp,
    state          VARCHAR(32),
    state_time     timestamp,
    duration       INT,
    busy_desc      VARCHAR(128),
    status         varchar(32) NOT NULL default '0',
    month          VARCHAR(32),
    ext1           VARCHAR(256),
    ext2           VARCHAR(256),
    ext3           VARCHAR(256),
    creator        varchar(32) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_agent_state_log IS '坐席状态历史表';
COMMENT ON COLUMN cti_agent_state_log.id IS '主键ID';
COMMENT ON COLUMN cti_agent_state_log.company_id IS '企业id';
COMMENT ON COLUMN cti_agent_state_log.group_id IS '主技能组id';
COMMENT ON COLUMN cti_agent_state_log.agent_id IS '坐席id';
COMMENT ON COLUMN cti_agent_state_log.agent_key IS '坐席编号';
COMMENT ON COLUMN cti_agent_state_log.agent_name IS '坐席名称';
COMMENT ON COLUMN cti_agent_state_log.call_id IS '通话唯一标识';
COMMENT ON COLUMN cti_agent_state_log.login_type IS '登录类型';
COMMENT ON COLUMN cti_agent_state_log.work_type IS '工作类型';
COMMENT ON COLUMN cti_agent_state_log.host IS '服务站点';
COMMENT ON COLUMN cti_agent_state_log.remote_address IS '远端地址';
COMMENT ON COLUMN cti_agent_state_log.before_state IS '变更之前状态';
COMMENT ON COLUMN cti_agent_state_log.before_time IS '更变之前时间';
COMMENT ON COLUMN cti_agent_state_log.state IS '变更之后状态';
COMMENT ON COLUMN cti_agent_state_log.state_time IS '当前时间';
COMMENT ON COLUMN cti_agent_state_log.duration IS '持续时间';
COMMENT ON COLUMN cti_agent_state_log.busy_desc IS '忙碌类型';
COMMENT ON COLUMN cti_agent_state_log.status IS '状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_agent_state_log.month IS '所属月份(yyyyMM)';
COMMENT ON COLUMN cti_agent_state_log.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_agent_state_log.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_agent_state_log.ext3 IS '扩展字段3';
COMMENT ON COLUMN cti_agent_state_log.creator IS '创建者';
COMMENT ON COLUMN cti_agent_state_log.create_time IS '创建时间';
COMMENT ON COLUMN cti_agent_state_log.updater IS '更新者';
COMMENT ON COLUMN cti_agent_state_log.update_time IS '更新时间';
COMMENT ON COLUMN cti_agent_state_log.deleted IS '是否删除';
COMMENT ON COLUMN cti_agent_state_log.tenant_id IS '租户编号';

-- ========================================
-- 通话详单表
-- ========================================
DROP TABLE IF EXISTS cti_call_detail;

CREATE TABLE cti_call_detail
(
    id            varchar(32) NOT NULL primary key,
    company_id    varchar(32) NOT NULL,
    call_id       varchar(32) NOT NULL,
    device_id     VARCHAR(128),
    detail_index  INT,
    cdr_type      varchar(32) NOT NULL,
    transfer_type varchar(32),
    transfer_id   varchar(32),
    caller        VARCHAR(32),
    called        VARCHAR(32),
    display       VARCHAR(32),
    agent_key     VARCHAR(32),
    agent_name    VARCHAR(64),
    group_id      varchar(32),
    start_time    timestamp,
    answer_time   timestamp,
    end_time      BIGINT,
    talk_time     INT                  DEFAULT 0,
    ring_time     INT                  DEFAULT 0,
    record_url    VARCHAR(256),
    reason        VARCHAR(128),
    ext1          VARCHAR(256),
    ext2          VARCHAR(256),
    month         INT         NOT NULL,
    status        varchar(32) NOT NULL default '0',
    creator       varchar(32) NULL     DEFAULT '',
    create_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(32) NULL     DEFAULT '',
    update_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       boolean     NOT NULL DEFAULT false,
    tenant_id     varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_call_detail IS '通话详单表,存储每通电话的详细通话记录';
COMMENT ON COLUMN cti_call_detail.id IS '主键ID';
COMMENT ON COLUMN cti_call_detail.company_id IS '企业ID';
COMMENT ON COLUMN cti_call_detail.call_id IS '通话ID';
COMMENT ON COLUMN cti_call_detail.device_id IS '设备ID';
COMMENT ON COLUMN cti_call_detail.detail_index IS '流程顺序';
COMMENT ON COLUMN cti_call_detail.cdr_type IS 'CDR类型';
COMMENT ON COLUMN cti_call_detail.transfer_type IS '转接类型(1:进VDN,2:进IVR,3:技能组,4:按键收号,5:外线)';
COMMENT ON COLUMN cti_call_detail.transfer_id IS '转接ID';
COMMENT ON COLUMN cti_call_detail.caller IS '主叫号码';
COMMENT ON COLUMN cti_call_detail.called IS '被叫号码';
COMMENT ON COLUMN cti_call_detail.display IS '显号';
COMMENT ON COLUMN cti_call_detail.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_call_detail.agent_name IS '坐席姓名';
COMMENT ON COLUMN cti_call_detail.group_id IS '技能组ID';
COMMENT ON COLUMN cti_call_detail.start_time IS '开始时间';
COMMENT ON COLUMN cti_call_detail.answer_time IS '应答时间';
COMMENT ON COLUMN cti_call_detail.end_time IS '结束时间';
COMMENT ON COLUMN cti_call_detail.talk_time IS '通话时长(秒)';
COMMENT ON COLUMN cti_call_detail.ring_time IS '振铃时长(秒)';
COMMENT ON COLUMN cti_call_detail.record_url IS '录音URL';
COMMENT ON COLUMN cti_call_detail.reason IS '出队列原因';
COMMENT ON COLUMN cti_call_detail.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_call_detail.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_call_detail.month IS '月份(yyyyMM)';
COMMENT ON COLUMN cti_call_detail.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_call_detail.creator IS '创建者';
COMMENT ON COLUMN cti_call_detail.create_time IS '创建时间';
COMMENT ON COLUMN cti_call_detail.updater IS '更新者';
COMMENT ON COLUMN cti_call_detail.update_time IS '更新时间';
COMMENT ON COLUMN cti_call_detail.deleted IS '是否删除';
COMMENT ON COLUMN cti_call_detail.tenant_id IS '租户编号';

CREATE INDEX idx_call_detail_company ON cti_call_detail (company_id);
CREATE INDEX idx_call_detail_call_id ON cti_call_detail (call_id);
CREATE INDEX idx_call_detail_agent ON cti_call_detail (agent_key);
CREATE INDEX idx_call_detail_month ON cti_call_detail (month);

-- ========================================
-- 通话设备表
-- ========================================
DROP TABLE IF EXISTS cti_call_device;

CREATE TABLE cti_call_device
(
    id                varchar(32)  NOT NULL primary key,
    company_id        varchar(32)  NOT NULL,
    call_id           varchar(32)  NOT NULL,
    device_id         VARCHAR(128) NOT NULL,
    agent_key         VARCHAR(32),
    agent_name        VARCHAR(64),
    device_type       varchar(32)  NOT NULL,
    cdr_type          varchar(32),
    from_agent        VARCHAR(32),
    caller            VARCHAR(32),
    called            VARCHAR(32),
    display           VARCHAR(32),
    called_location   VARCHAR(64),
    caller_location   VARCHAR(64),
    call_time         timestamp,
    ring_start_time   timestamp,
    ring_end_time     timestamp,
    answer_time       timestamp,
    bridge_time       timestamp,
    end_time          timestamp,
    talk_time         bigint                DEFAULT 0,
    record_start_time timestamp,
    record_time       bigint                DEFAULT 0,
    sip_protocol      VARCHAR(32),
    record            VARCHAR(256),
    record2           VARCHAR(256),
    record3           VARCHAR(256),
    channel_name      VARCHAR(256),
    hangup_cause      VARCHAR(64),
    ring_cause        VARCHAR(64),
    sip_status        VARCHAR(64),
    ext1              VARCHAR(256),
    ext2              VARCHAR(256),
    month             INT          NOT NULL,
    status            varchar(32)  NOT NULL default '0',
    creator           varchar(32)  NULL     DEFAULT '',
    create_time       timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32)  NULL     DEFAULT '',
    update_time       timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean      NOT NULL DEFAULT false,
    tenant_id         varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_call_device IS '通话设备表,记录每个设备的通话状态';
COMMENT ON COLUMN cti_call_device.id IS '主键ID';
COMMENT ON COLUMN cti_call_device.company_id IS '企业ID';
COMMENT ON COLUMN cti_call_device.call_id IS '通话ID';
COMMENT ON COLUMN cti_call_device.device_id IS '设备ID';
COMMENT ON COLUMN cti_call_device.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_call_device.agent_name IS '坐席姓名';
COMMENT ON COLUMN cti_call_device.device_type IS '设备类型(1:坐席,2:客户,3:外线)';
COMMENT ON COLUMN cti_call_device.cdr_type IS 'CDR类型(1:呼入,2:外呼,3:内呼,4:转接,5:咨询,6:监听,7:强插,8:耳语)';
COMMENT ON COLUMN cti_call_device.from_agent IS '咨询或转接来源坐席';
COMMENT ON COLUMN cti_call_device.caller IS '主叫号码';
COMMENT ON COLUMN cti_call_device.called IS '被叫号码';
COMMENT ON COLUMN cti_call_device.display IS '显号';
COMMENT ON COLUMN cti_call_device.called_location IS '被叫归属地';
COMMENT ON COLUMN cti_call_device.caller_location IS '主叫归属地';
COMMENT ON COLUMN cti_call_device.call_time IS '呼叫开始时间';
COMMENT ON COLUMN cti_call_device.ring_start_time IS '振铃开始时间';
COMMENT ON COLUMN cti_call_device.ring_end_time IS '振铃结束时间';
COMMENT ON COLUMN cti_call_device.answer_time IS '接通时间';
COMMENT ON COLUMN cti_call_device.bridge_time IS '桥接时间';
COMMENT ON COLUMN cti_call_device.end_time IS '结束时间';
COMMENT ON COLUMN cti_call_device.talk_time IS '通话时长(毫秒)';
COMMENT ON COLUMN cti_call_device.record_start_time IS '录音开始时间';
COMMENT ON COLUMN cti_call_device.record_time IS '录音时长(毫秒)';
COMMENT ON COLUMN cti_call_device.sip_protocol IS '信令协议(tcp/udp)';
COMMENT ON COLUMN cti_call_device.record IS '录音地址';
COMMENT ON COLUMN cti_call_device.record2 IS '备用录音地址2';
COMMENT ON COLUMN cti_call_device.record3 IS '备用录音地址3';
COMMENT ON COLUMN cti_call_device.channel_name IS '呼叫地址';
COMMENT ON COLUMN cti_call_device.hangup_cause IS '挂机原因';
COMMENT ON COLUMN cti_call_device.ring_cause IS '回铃音识别';
COMMENT ON COLUMN cti_call_device.sip_status IS 'SIP状态';
COMMENT ON COLUMN cti_call_device.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_call_device.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_call_device.month IS '月份(yyyyMM)';
COMMENT ON COLUMN cti_call_device.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_call_device.creator IS '创建者';
COMMENT ON COLUMN cti_call_device.create_time IS '创建时间';
COMMENT ON COLUMN cti_call_device.updater IS '更新者';
COMMENT ON COLUMN cti_call_device.update_time IS '更新时间';
COMMENT ON COLUMN cti_call_device.deleted IS '是否删除';
COMMENT ON COLUMN cti_call_device.tenant_id IS '租户编号';

CREATE INDEX idx_call_device_call_id ON cti_call_device (call_id);

-- ========================================
-- 通话DTMF按键表
-- ========================================
DROP TABLE IF EXISTS cti_call_dtmf;

CREATE TABLE cti_call_dtmf
(
    id          varchar(32) NOT NULL primary key,
    dtmf_key    VARCHAR(32),
    process_id  varchar(32),
    call_id     varchar(32),
    dtmf_time   timestamp,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_call_dtmf IS '呼叫按键表';
COMMENT ON COLUMN cti_call_dtmf.id IS '主键ID';
COMMENT ON COLUMN cti_call_dtmf.dtmf_key IS '按键号码';
COMMENT ON COLUMN cti_call_dtmf.process_id IS '业务流程id';
COMMENT ON COLUMN cti_call_dtmf.call_id IS '通话标识id';
COMMENT ON COLUMN cti_call_dtmf.dtmf_time IS '按键时间';
COMMENT ON COLUMN cti_call_dtmf.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_call_dtmf.creator IS '创建者';
COMMENT ON COLUMN cti_call_dtmf.create_time IS '创建时间';
COMMENT ON COLUMN cti_call_dtmf.updater IS '更新者';
COMMENT ON COLUMN cti_call_dtmf.update_time IS '更新时间';
COMMENT ON COLUMN cti_call_dtmf.deleted IS '是否删除';
COMMENT ON COLUMN cti_call_dtmf.tenant_id IS '租户编号';

-- ========================================
-- 通话日志表
-- ========================================
DROP TABLE IF EXISTS cti_call_log;

CREATE TABLE cti_call_log
(
    id                varchar(32) NOT NULL primary key,
    company_id        varchar(32) NOT NULL,
    call_id           varchar(32) NOT NULL,
    caller_display    VARCHAR(32),
    caller            VARCHAR(32),
    called_display    VARCHAR(32),
    called            VARCHAR(32),
    number_location   VARCHAR(128),
    agent_key         VARCHAR(32),
    agent_name        VARCHAR(64),
    group_id          varchar(32),
    login_type        varchar(32),
    task_id           varchar(32),
    ivr_id            varchar(32),
    bot_id            varchar(32),
    call_time         timestamp,
    answer_time       timestamp,
    end_time          timestamp,
    call_type         VARCHAR(32),
    direction         VARCHAR(32),
    answer_flag       INT,
    wait_time         BIGINT               DEFAULT 0,
    answer_count      INT                  DEFAULT 0,
    hangup_dir        INT,
    sdk_hangup        INT,
    hangup_code       INT,
    media_host        VARCHAR(64),
    cti_host          VARCHAR(64),
    client_host       VARCHAR(64),
    record            VARCHAR(256),
    record2           VARCHAR(256),
    record3           VARCHAR(256),
    record_type       varchar(32),
    record_start_time timestamp,
    record_time       bigint               DEFAULT 0,
    talk_time         bigint               DEFAULT 0,
    frist_queue_time  timestamp,
    queue_start_time  timestamp,
    queue_end_time    timestamp,
    month_time        varchar(32) NOT NULL,
    follow_data       TEXT,
    uuid1             VARCHAR(256),
    uuid2             VARCHAR(256),
    ext1              VARCHAR(256),
    ext2              VARCHAR(256),
    ext3              VARCHAR(256),
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_call_log IS '通话日志表,记录所有通话的详细日志信息';
COMMENT ON COLUMN cti_call_log.id IS '主键ID(话单ID)';
COMMENT ON COLUMN cti_call_log.company_id IS '企业ID';
COMMENT ON COLUMN cti_call_log.call_id IS '通话ID';
COMMENT ON COLUMN cti_call_log.caller_display IS '主叫显号';
COMMENT ON COLUMN cti_call_log.caller IS '主叫号码';
COMMENT ON COLUMN cti_call_log.called_display IS '被叫显号';
COMMENT ON COLUMN cti_call_log.called IS '被叫号码';
COMMENT ON COLUMN cti_call_log.number_location IS '号码归属地(省份-城市-运营商)';
COMMENT ON COLUMN cti_call_log.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_call_log.agent_name IS '坐席姓名';
COMMENT ON COLUMN cti_call_log.group_id IS '技能组ID';
COMMENT ON COLUMN cti_call_log.login_type IS '登录类型(1:SIP,2:WebRTC,3:手机)';
COMMENT ON COLUMN cti_call_log.task_id IS '任务ID';
COMMENT ON COLUMN cti_call_log.ivr_id IS 'IVR ID';
COMMENT ON COLUMN cti_call_log.bot_id IS '机器人ID';
COMMENT ON COLUMN cti_call_log.call_time IS '呼叫开始时间';
COMMENT ON COLUMN cti_call_log.answer_time IS '接听时间';
COMMENT ON COLUMN cti_call_log.end_time IS '结束时间';
COMMENT ON COLUMN cti_call_log.call_type IS '呼叫类型';
COMMENT ON COLUMN cti_call_log.direction IS '呼叫方向';
COMMENT ON COLUMN cti_call_log.answer_flag IS '通话标识(0:接通,1:坐席未接用户未接,2:坐席接通用户未接通,3:用户接通坐席未接通)';
COMMENT ON COLUMN cti_call_log.wait_time IS '累计等待时长(毫秒)';
COMMENT ON COLUMN cti_call_log.answer_count IS '应答设备数';
COMMENT ON COLUMN cti_call_log.hangup_dir IS '挂机方向(1:主叫挂机,2:被叫挂机,3:系统挂机)';
COMMENT ON COLUMN cti_call_log.sdk_hangup IS '是否SDK挂机(1:SDK挂机)';
COMMENT ON COLUMN cti_call_log.hangup_code IS '挂机原因码';
COMMENT ON COLUMN cti_call_log.media_host IS '媒体服务器地址';
COMMENT ON COLUMN cti_call_log.cti_host IS 'CTI服务器地址';
COMMENT ON COLUMN cti_call_log.client_host IS '客户端地址';
COMMENT ON COLUMN cti_call_log.record IS '录音地址';
COMMENT ON COLUMN cti_call_log.record2 IS '备用录音地址2';
COMMENT ON COLUMN cti_call_log.record3 IS '备用录音地址3';
COMMENT ON COLUMN cti_call_log.record_type IS '录音状态';
COMMENT ON COLUMN cti_call_log.record_start_time IS '录音开始时间';
COMMENT ON COLUMN cti_call_log.record_time IS '录音时长(毫秒)';
COMMENT ON COLUMN cti_call_log.talk_time IS '通话时长(毫秒)';
COMMENT ON COLUMN cti_call_log.frist_queue_time IS '第一次进队列时间';
COMMENT ON COLUMN cti_call_log.queue_start_time IS '进队列时间';
COMMENT ON COLUMN cti_call_log.queue_end_time IS '出队列时间';
COMMENT ON COLUMN cti_call_log.month_time IS '月份(yyyyMM)';
COMMENT ON COLUMN cti_call_log.follow_data IS '通话随路数据';
COMMENT ON COLUMN cti_call_log.uuid1 IS '扩展字段1';
COMMENT ON COLUMN cti_call_log.uuid2 IS '扩展字段2';
COMMENT ON COLUMN cti_call_log.ext1 IS '扩展字段3';
COMMENT ON COLUMN cti_call_log.ext2 IS '扩展字段4';
COMMENT ON COLUMN cti_call_log.ext3 IS '扩展字段5';
COMMENT ON COLUMN cti_call_log.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_call_log.creator IS '创建者';
COMMENT ON COLUMN cti_call_log.create_time IS '创建时间';
COMMENT ON COLUMN cti_call_log.updater IS '更新者';
COMMENT ON COLUMN cti_call_log.update_time IS '更新时间';
COMMENT ON COLUMN cti_call_log.deleted IS '是否删除';
COMMENT ON COLUMN cti_call_log.tenant_id IS '租户编号';

CREATE INDEX idx_call_log_company ON cti_call_log (company_id);
CREATE INDEX idx_call_log_month ON cti_call_log (month_time);

-- ========================================
-- 语音转文字记录表
-- ========================================
DROP TABLE IF EXISTS cti_call_speech_text;

CREATE TABLE cti_call_speech_text
(
    id             varchar(32) NOT NULL primary key,
    company_id     varchar(32),
    call_id        varchar(32),
    device_id      VARCHAR(64),
    device_type    varchar(32),
    speech_id      VARCHAR(128),
    speech_text    TEXT,
    asr_product    VARCHAR(64),
    intention      VARCHAR(256),
    quality_status INT,
    status         varchar(32) NOT NULL default '0',
    month          VARCHAR(32),
    creator        varchar(32) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_call_speech_text IS '通话语音转文字记录表,存储通话过程中的语音识别文字结果,用于质检和意图识别';
COMMENT ON COLUMN cti_call_speech_text.id IS '主键ID';
COMMENT ON COLUMN cti_call_speech_text.company_id IS '企业ID';
COMMENT ON COLUMN cti_call_speech_text.call_id IS '通话ID';
COMMENT ON COLUMN cti_call_speech_text.device_id IS '设备标识';
COMMENT ON COLUMN cti_call_speech_text.device_type IS '设备类型';
COMMENT ON COLUMN cti_call_speech_text.speech_id IS '语音识别唯一标识';
COMMENT ON COLUMN cti_call_speech_text.speech_text IS '语音识别文字内容';
COMMENT ON COLUMN cti_call_speech_text.asr_product IS 'ASR语音识别厂商';
COMMENT ON COLUMN cti_call_speech_text.intention IS '语义意图识别结果';
COMMENT ON COLUMN cti_call_speech_text.quality_status IS '质检结果状态';
COMMENT ON COLUMN cti_call_speech_text.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_call_speech_text.month IS '所属月份(yyyyMM)';
COMMENT ON COLUMN cti_call_speech_text.creator IS '创建者';
COMMENT ON COLUMN cti_call_speech_text.create_time IS '创建时间';
COMMENT ON COLUMN cti_call_speech_text.updater IS '更新者';
COMMENT ON COLUMN cti_call_speech_text.update_time IS '更新时间';
COMMENT ON COLUMN cti_call_speech_text.deleted IS '是否删除';
COMMENT ON COLUMN cti_call_speech_text.tenant_id IS '租户编号';

-- ========================================
-- 企业信息表
-- ========================================
DROP TABLE IF EXISTS cti_company;

CREATE TABLE cti_company
(
    id                varchar(32) NOT NULL primary key,
    name              VARCHAR(64) NOT NULL,
    id_path           VARCHAR(256),
    pid               varchar(32)          DEFAULT 0,
    company_code      VARCHAR(32) NOT NULL,
    gmt               INT                  DEFAULT 8,
    contact           VARCHAR(64),
    phone             VARCHAR(32),
    balance           BIGINT               DEFAULT 0,
    bill_type         varchar(32)          DEFAULT '0',
    pay_type          varchar(32)          DEFAULT '0',
    hidden_customer   varchar(32)          DEFAULT '0',
    ivr_limit         INT                  DEFAULT 0,
    agent_limit       INT                  DEFAULT 0,
    group_limit       INT                  DEFAULT 0,
    group_agent_limit INT                  DEFAULT 0,
    record_storage    INT                  DEFAULT 3,
    notify_url        VARCHAR(256),
    ext1              TEXT,
    ext2              TEXT,
    ext3              TEXT,
    ext4              TEXT,
    ext5              TEXT,
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_company IS '企业信息表,存储客户企业的基本信息和配置';
COMMENT ON COLUMN cti_company.name IS '企业名称(2-16位)';
COMMENT ON COLUMN cti_company.company_code IS '企业编码(2-8位,唯一)';
COMMENT ON COLUMN cti_company.notify_url IS '话单回调通知URL(最长255字符)';
COMMENT ON COLUMN cti_company.gmt IS '时区(默认GTM+8)';
COMMENT ON COLUMN cti_company.contact IS '联系人';
COMMENT ON COLUMN cti_company.phone IS '联系电话';
COMMENT ON COLUMN cti_company.balance IS '账户余额';
COMMENT ON COLUMN cti_company.bill_type IS '计费类型(1:呼出计费,2:呼入计费,3:双向计费,0:全免费)';
COMMENT ON COLUMN cti_company.pay_type IS '付费方式(0:预付费,1:后付费)';
COMMENT ON COLUMN cti_company.hidden_customer IS '隐藏客户号码(0:不隐藏,1:隐藏)';
COMMENT ON COLUMN cti_company.ivr_limit IS 'IVR通道数限制';
COMMENT ON COLUMN cti_company.agent_limit IS '开通坐席数限制';
COMMENT ON COLUMN cti_company.group_limit IS '开通技能组数限制';
COMMENT ON COLUMN cti_company.group_agent_limit IS '单技能组坐席数上限';
COMMENT ON COLUMN cti_company.record_storage IS '录音保留月数';
COMMENT ON COLUMN cti_company.notify_url IS '话单回调通知URL';
COMMENT ON COLUMN cti_company.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_company.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_company.ext3 IS '扩展字段3';
COMMENT ON COLUMN cti_company.ext4 IS '扩展字段4';
COMMENT ON COLUMN cti_company.ext5 IS '扩展字段5';
COMMENT ON COLUMN cti_company.status IS '状态(0:禁用,1:免费企业,2:试用企业,3:付费企业)';
COMMENT ON COLUMN cti_company.creator IS '创建者';
COMMENT ON COLUMN cti_company.create_time IS '创建时间';
COMMENT ON COLUMN cti_company.updater IS '更新者';
COMMENT ON COLUMN cti_company.update_time IS '更新时间';
COMMENT ON COLUMN cti_company.deleted IS '是否删除';
COMMENT ON COLUMN cti_company.tenant_id IS '租户编号';

CREATE INDEX idx_company_code ON cti_company (company_code);
CREATE INDEX idx_company_status ON cti_company (status);

-- ========================================
-- 企业显号配置表
-- ========================================
DROP TABLE IF EXISTS cti_company_display;

CREATE TABLE cti_company_display
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    name        VARCHAR(64),
    type        varchar(32),
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_company_display IS '号码池表';
COMMENT ON COLUMN cti_company_display.id IS '主键ID';
COMMENT ON COLUMN cti_company_display.company_id IS '企业id';
COMMENT ON COLUMN cti_company_display.name IS '号码池名称';
COMMENT ON COLUMN cti_company_display.type IS '号码池类型(1:呼入号码,2:主叫显号,3:被叫显号)';
COMMENT ON COLUMN cti_company_display.status IS '号码池状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_company_display.creator IS '创建者';
COMMENT ON COLUMN cti_company_display.create_time IS '创建时间';
COMMENT ON COLUMN cti_company_display.updater IS '更新者';
COMMENT ON COLUMN cti_company_display.update_time IS '更新时间';
COMMENT ON COLUMN cti_company_display.deleted IS '是否删除';
COMMENT ON COLUMN cti_company_display.tenant_id IS '租户编号';

-- ========================================
-- 企业号码池表
-- ========================================
DROP TABLE IF EXISTS cti_company_phone;

CREATE TABLE cti_company_phone
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32) NOT NULL,
    phone       VARCHAR(32) NOT NULL,
    type        varchar(32)          DEFAULT '1' NOT NULL,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_company_phone IS '企业号码池表,存储企业可使用的电话号码';
COMMENT ON COLUMN cti_company_phone.phone IS '号码(4-16位)';
COMMENT ON COLUMN cti_company_phone.type IS '号码类型(1:普通号码,2:400号码,3:95号码)';
COMMENT ON COLUMN cti_company_phone.creator IS '创建者';
COMMENT ON COLUMN cti_company_phone.create_time IS '创建时间';
COMMENT ON COLUMN cti_company_phone.updater IS '更新者';
COMMENT ON COLUMN cti_company_phone.update_time IS '更新时间';
COMMENT ON COLUMN cti_company_phone.deleted IS '是否删除';
COMMENT ON COLUMN cti_company_phone.tenant_id IS '租户编号';

-- ========================================
-- 企业号码组关系表
-- ========================================
DROP TABLE IF EXISTS cti_company_phone_group;

CREATE TABLE cti_company_phone_group
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    display_id  varchar(32),
    phone_id    varchar(32),
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_company_phone_group IS '企业号码与显号的关联关系表';
COMMENT ON COLUMN cti_company_phone_group.id IS '主键ID';
COMMENT ON COLUMN cti_company_phone_group.company_id IS '企业ID';
COMMENT ON COLUMN cti_company_phone_group.display_id IS '显号配置ID,关联company_display表';
COMMENT ON COLUMN cti_company_phone_group.phone_id IS '号码ID,关联company_phone表';
COMMENT ON COLUMN cti_company_phone_group.creator IS '创建者';
COMMENT ON COLUMN cti_company_phone_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_company_phone_group.updater IS '更新者';
COMMENT ON COLUMN cti_company_phone_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_company_phone_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_company_phone_group.tenant_id IS '租户编号';

-- ========================================
-- 企业统计配置表
-- ========================================
DROP TABLE IF EXISTS cti_company_stat;

CREATE TABLE cti_company_stat
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    type        varchar(32),
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_company_stat IS '企业统计配置表,配置企业的统计报表功能开关';
COMMENT ON COLUMN cti_company_stat.id IS '主键ID';
COMMENT ON COLUMN cti_company_stat.company_id IS '企业ID';
COMMENT ON COLUMN cti_company_stat.type IS '报表类型(1:座席报表)';
COMMENT ON COLUMN cti_company_stat.status IS '功能开关(0:启用,1:禁用)';
COMMENT ON COLUMN cti_company_stat.creator IS '创建者';
COMMENT ON COLUMN cti_company_stat.create_time IS '创建时间';
COMMENT ON COLUMN cti_company_stat.updater IS '更新者';
COMMENT ON COLUMN cti_company_stat.update_time IS '更新时间';
COMMENT ON COLUMN cti_company_stat.deleted IS '是否删除';
COMMENT ON COLUMN cti_company_stat.tenant_id IS '租户编号';

-- ========================================
-- 技能组表
-- ========================================
DROP TABLE IF EXISTS cti_group;

CREATE TABLE cti_group
(
    id                varchar(32) NOT NULL primary key,
    company_id        varchar(32) NOT NULL,
    name              VARCHAR(64) NOT NULL,
    after_interval    INT                  DEFAULT 0,
    caller_display_id varchar(32),
    called_display_id varchar(32),
    record_type       varchar(32)          DEFAULT '0',
    level_value       INT                  DEFAULT 1,
    tts_engine        BIGINT,
    play_content      VARCHAR(256),
    evaluate          boolean              DEFAULT false,
    queue_play        BIGINT,
    transfer_play     BIGINT,
    call_time_out     INT                  DEFAULT 60,
    group_type        varchar(32)          DEFAULT '1',
    notify_position   INT                  DEFAULT 0,
    notify_rate       INT                  DEFAULT 0,
    notify_content    VARCHAR(256),
    call_memory       INT                  DEFAULT 0,
    ext1              VARCHAR(256),
    ext2              VARCHAR(256),
    ext3              VARCHAR(256),
    ext4              VARCHAR(256),
    ext5              VARCHAR(256),
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group IS '技能组表,用于分组管理坐席';
COMMENT ON COLUMN cti_group.id IS '主键ID';
COMMENT ON COLUMN cti_group.company_id IS '企业ID';
COMMENT ON COLUMN cti_group.name IS '技能组名称';
COMMENT ON COLUMN cti_group.after_interval IS '话后自动空闲时长(秒)';
COMMENT ON COLUMN cti_group.caller_display_id IS '主叫显号号码池ID';
COMMENT ON COLUMN cti_group.called_display_id IS '被叫显号号码池ID';
COMMENT ON COLUMN cti_group.record_type IS '录音类型(0:不录音,1:振铃录音,2:接通录音)';
COMMENT ON COLUMN cti_group.level_value IS '技能组优先级';
COMMENT ON COLUMN cti_group.tts_engine IS '语音合成引擎ID';
COMMENT ON COLUMN cti_group.play_content IS '转坐席时播放内容';
COMMENT ON COLUMN cti_group.evaluate IS '转服务评价(0:否,1:是)';
COMMENT ON COLUMN cti_group.queue_play IS '排队音ID';
COMMENT ON COLUMN cti_group.transfer_play IS '转接提示音ID';
COMMENT ON COLUMN cti_group.call_time_out IS '外呼呼叫超时时间(秒)';
COMMENT ON COLUMN cti_group.group_type IS '技能组类型';
COMMENT ON COLUMN cti_group.notify_position IS '播放排队位置(0:不播放,1:播放)';
COMMENT ON COLUMN cti_group.notify_rate IS '播报频次';
COMMENT ON COLUMN cti_group.notify_content IS '播报内容模板';
COMMENT ON COLUMN cti_group.call_memory IS '主叫记忆(0:不开启,1:开启)';
COMMENT ON COLUMN cti_group.ext1 IS '扩展字段1';
COMMENT ON COLUMN cti_group.ext2 IS '扩展字段2';
COMMENT ON COLUMN cti_group.ext3 IS '扩展字段3';
COMMENT ON COLUMN cti_group.ext4 IS '扩展字段4';
COMMENT ON COLUMN cti_group.ext5 IS '扩展字段5';
COMMENT ON COLUMN cti_group.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group.creator IS '创建者';
COMMENT ON COLUMN cti_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_group.updater IS '更新者';
COMMENT ON COLUMN cti_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_group.tenant_id IS '租户编号';

CREATE INDEX idx_group_company ON cti_group (company_id);

-- ========================================
-- 技能组坐席分配策略表
-- ========================================
DROP TABLE IF EXISTS cti_group_agent_strategy;

CREATE TABLE cti_group_agent_strategy
(
    id                varchar(32) NOT NULL primary key,
    company_id        varchar(32),
    group_id          varchar(32),
    strategy_type     varchar(32),
    strategy_value    varchar(32),
    custom_expression TEXT,
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group_agent_strategy IS '技能组坐席分配策略表';
COMMENT ON COLUMN cti_group_agent_strategy.id IS '主键ID';
COMMENT ON COLUMN cti_group_agent_strategy.company_id IS '企业ID';
COMMENT ON COLUMN cti_group_agent_strategy.group_id IS '技能组ID';
COMMENT ON COLUMN cti_group_agent_strategy.strategy_type IS '策略类型(1:内置策略,2:自定义)';
COMMENT ON COLUMN cti_group_agent_strategy.strategy_value IS '内置策略值(1:最长空闲时间,2:最长平均空闲,3:最少应答次数,4:最少通话时长,5:最长话后时长,6:轮选,7:随机)';
COMMENT ON COLUMN cti_group_agent_strategy.custom_expression IS '自定义分配表达式';
COMMENT ON COLUMN cti_group_agent_strategy.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group_agent_strategy.creator IS '创建者';
COMMENT ON COLUMN cti_group_agent_strategy.create_time IS '创建时间';
COMMENT ON COLUMN cti_group_agent_strategy.updater IS '更新者';
COMMENT ON COLUMN cti_group_agent_strategy.update_time IS '更新时间';
COMMENT ON COLUMN cti_group_agent_strategy.deleted IS '是否删除';
COMMENT ON COLUMN cti_group_agent_strategy.tenant_id IS '租户编号';

-- ========================================
-- 座席记忆表
-- ========================================
DROP TABLE IF EXISTS cti_group_memory;

CREATE TABLE cti_group_memory
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    agent_key   VARCHAR(64),
    group_id    varchar(32),
    phone       VARCHAR(64),
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group_memory IS '坐席与客户记忆表,用于记录客户与坐席的服务关系';
COMMENT ON COLUMN cti_group_memory.id IS '主键ID';
COMMENT ON COLUMN cti_group_memory.company_id IS '企业ID';
COMMENT ON COLUMN cti_group_memory.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_group_memory.group_id IS '技能组ID';
COMMENT ON COLUMN cti_group_memory.phone IS '客户电话';
COMMENT ON COLUMN cti_group_memory.status IS '记忆状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group_memory.creator IS '创建者';
COMMENT ON COLUMN cti_group_memory.create_time IS '创建时间';
COMMENT ON COLUMN cti_group_memory.updater IS '更新者';
COMMENT ON COLUMN cti_group_memory.update_time IS '更新时间';
COMMENT ON COLUMN cti_group_memory.deleted IS '是否删除';
COMMENT ON COLUMN cti_group_memory.tenant_id IS '租户编号';

-- ========================================
-- 技能组记忆配置表
-- ========================================
DROP TABLE IF EXISTS cti_group_memory_config;

CREATE TABLE cti_group_memory_config
(
    id                     varchar(32) NOT NULL primary key,
    company_id             varchar(32),
    group_id               varchar(32),
    success_strategy       INT,
    success_strategy_value BIGINT,
    fail_strategy          INT,
    fail_strategy_value    BIGINT,
    memory_day             INT,
    inbound_cover          INT,
    outbound_cover         INT,
    status                 varchar(32) NOT NULL default '0',
    creator                varchar(32) NULL     DEFAULT '',
    create_time            timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater                varchar(32) NULL     DEFAULT '',
    update_time            timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted                boolean     NOT NULL DEFAULT false,
    tenant_id              varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group_memory_config IS '技能组记忆配置表';
COMMENT ON COLUMN cti_group_memory_config.id IS '主键ID';
COMMENT ON COLUMN cti_group_memory_config.company_id IS '企业ID';
COMMENT ON COLUMN cti_group_memory_config.group_id IS '技能组ID';
COMMENT ON COLUMN cti_group_memory_config.success_strategy IS '匹配成功策略(1:等待记忆坐席,2:超时转其他空闲坐席,3:忙碌转空闲坐席)';
COMMENT ON COLUMN cti_group_memory_config.success_strategy_value IS '匹配成功策略值';
COMMENT ON COLUMN cti_group_memory_config.fail_strategy IS '匹配失败策略(1:其他空闲坐席,2:其他技能组,3:vdn,4:ivr,5:挂机)';
COMMENT ON COLUMN cti_group_memory_config.fail_strategy_value IS '匹配失败策略值';
COMMENT ON COLUMN cti_group_memory_config.memory_day IS '记忆天数';
COMMENT ON COLUMN cti_group_memory_config.inbound_cover IS '呼入覆盖';
COMMENT ON COLUMN cti_group_memory_config.outbound_cover IS '外呼覆盖';
COMMENT ON COLUMN cti_group_memory_config.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group_memory_config.creator IS '创建者';
COMMENT ON COLUMN cti_group_memory_config.create_time IS '创建时间';
COMMENT ON COLUMN cti_group_memory_config.updater IS '更新者';
COMMENT ON COLUMN cti_group_memory_config.update_time IS '更新时间';
COMMENT ON COLUMN cti_group_memory_config.deleted IS '是否删除';
COMMENT ON COLUMN cti_group_memory_config.tenant_id IS '租户编号';

-- ========================================
-- 技能组溢出关系表
-- ========================================
DROP TABLE IF EXISTS cti_group_overflow;

CREATE TABLE cti_group_overflow
(
    id          varchar(32) NOT NULL primary key,
    group_id    varchar(32),
    overflow_id varchar(32),
    level_value INT,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group_overflow IS '技能组溢出关系表';
COMMENT ON COLUMN cti_group_overflow.id IS '主键ID';
COMMENT ON COLUMN cti_group_overflow.group_id IS '技能组ID';
COMMENT ON COLUMN cti_group_overflow.overflow_id IS '溢出策略ID';
COMMENT ON COLUMN cti_group_overflow.level_value IS '优先级';
COMMENT ON COLUMN cti_group_overflow.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group_overflow.creator IS '创建者';
COMMENT ON COLUMN cti_group_overflow.create_time IS '创建时间';
COMMENT ON COLUMN cti_group_overflow.updater IS '更新者';
COMMENT ON COLUMN cti_group_overflow.update_time IS '更新时间';
COMMENT ON COLUMN cti_group_overflow.deleted IS '是否删除';
COMMENT ON COLUMN cti_group_overflow.tenant_id IS '租户编号';

-- ========================================
-- 坐席自定义策略表
-- ========================================
DROP TABLE IF EXISTS cti_group_strategy_exp;

CREATE TABLE cti_group_strategy_exp
(
    id               varchar(32) NOT NULL primary key,
    company_id       varchar(32),
    group_id         varchar(32),
    strategy_key     VARCHAR(128),
    strategy_present INT,
    strategy_type    varchar(32),
    status           varchar(32) NOT NULL default '0',
    creator          varchar(32) NULL     DEFAULT '',
    create_time      timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater          varchar(32) NULL     DEFAULT '',
    update_time      timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted          boolean     NOT NULL DEFAULT false,
    tenant_id        varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_group_strategy_exp IS '坐席自定义策略表';
COMMENT ON COLUMN cti_group_strategy_exp.id IS '主键ID';
COMMENT ON COLUMN cti_group_strategy_exp.company_id IS '企业ID';
COMMENT ON COLUMN cti_group_strategy_exp.group_id IS '技能组ID';
COMMENT ON COLUMN cti_group_strategy_exp.strategy_key IS '自定义策略键';
COMMENT ON COLUMN cti_group_strategy_exp.strategy_present IS '策略百分比';
COMMENT ON COLUMN cti_group_strategy_exp.strategy_type IS '策略类型';
COMMENT ON COLUMN cti_group_strategy_exp.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_group_strategy_exp.creator IS '创建者';
COMMENT ON COLUMN cti_group_strategy_exp.create_time IS '创建时间';
COMMENT ON COLUMN cti_group_strategy_exp.updater IS '更新者';
COMMENT ON COLUMN cti_group_strategy_exp.update_time IS '更新时间';
COMMENT ON COLUMN cti_group_strategy_exp.deleted IS '是否删除';
COMMENT ON COLUMN cti_group_strategy_exp.tenant_id IS '租户编号';

-- ========================================
-- IVR流程表
-- ========================================
DROP TABLE IF EXISTS cti_ivr_workflow;

CREATE TABLE cti_ivr_workflow
(
    id          varchar(32)  NOT NULL primary key,
    company_id  varchar(32)  NOT NULL,
    name        VARCHAR(128) NOT NULL,
    oss_id      VARCHAR(128),
    create_user VARCHAR(64),
    verify_user VARCHAR(64),
    voice_item  TEXT,
    init_params TEXT,
    content     TEXT,
    type        varchar(32)           DEFAULT '1',
    status      varchar(32)  NOT NULL default '0',
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_ivr_workflow IS 'IVR流程表,存储IVR交互式语音流程配置';
COMMENT ON COLUMN cti_ivr_workflow.id IS '主键ID';
COMMENT ON COLUMN cti_ivr_workflow.company_id IS '企业ID';
COMMENT ON COLUMN cti_ivr_workflow.name IS '流程名称';
COMMENT ON COLUMN cti_ivr_workflow.oss_id IS '流程文件OSS标识';
COMMENT ON COLUMN cti_ivr_workflow.create_user IS '流程发布人';
COMMENT ON COLUMN cti_ivr_workflow.verify_user IS '流程审核人';
COMMENT ON COLUMN cti_ivr_workflow.voice_item IS '该流程用到的语音文件ID,以英文逗号分隔';
COMMENT ON COLUMN cti_ivr_workflow.init_params IS '流程启动所需参数描述';
COMMENT ON COLUMN cti_ivr_workflow.content IS '流程内容(JSON)';
COMMENT ON COLUMN cti_ivr_workflow.type IS '流程类型(1:转接,2:咨询)';
COMMENT ON COLUMN cti_ivr_workflow.status IS '流程状态(1:待发布,2:审核中,3:审核未通过,4:审核通过,5:已上线)';
COMMENT ON COLUMN cti_ivr_workflow.creator IS '创建者';
COMMENT ON COLUMN cti_ivr_workflow.create_time IS '创建时间';
COMMENT ON COLUMN cti_ivr_workflow.updater IS '更新者';
COMMENT ON COLUMN cti_ivr_workflow.update_time IS '更新时间';
COMMENT ON COLUMN cti_ivr_workflow.deleted IS '是否删除';
COMMENT ON COLUMN cti_ivr_workflow.tenant_id IS '租户编号';

-- ========================================
-- 溢出策略表
-- ========================================
DROP TABLE IF EXISTS cti_overflow_config;

CREATE TABLE cti_overflow_config
(
    id                varchar(32)  NOT NULL primary key,
    company_id        varchar(32)  NOT NULL,
    name              VARCHAR(128) NOT NULL,
    handle_type       varchar(32)           DEFAULT '1' NOT NULL,
    busy_type         varchar(32)           DEFAULT '1',
    queue_timeout     INT                   DEFAULT 60 NOT NULL,
    busy_timeout_type varchar(32)           DEFAULT '1' NOT NULL,
    overflow_type     INT,
    overflow_value    INT,
    lineup_expression TEXT,
    creator           varchar(32)  NULL     DEFAULT '',
    create_time       timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32)  NULL     DEFAULT '',
    update_time       timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean      NOT NULL DEFAULT false,
    tenant_id         varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_overflow_config IS '溢出策略配置表';
COMMENT ON COLUMN cti_overflow_config.id IS '主键ID';
COMMENT ON COLUMN cti_overflow_config.company_id IS '企业ID';
COMMENT ON COLUMN cti_overflow_config.name IS '溢出策略名称';
COMMENT ON COLUMN cti_overflow_config.handle_type IS '处理类型(1:排队,2:溢出,3:挂机)';
COMMENT ON COLUMN cti_overflow_config.busy_type IS '排队方式(1:先进先出,2:VIP,3:自定义)';
COMMENT ON COLUMN cti_overflow_config.queue_timeout IS '排队超时时间(秒)';
COMMENT ON COLUMN cti_overflow_config.busy_timeout_type IS '排队超时处理(1:溢出,2:挂机)';
COMMENT ON COLUMN cti_overflow_config.overflow_type IS '溢出目标类型(1:技能组,2:IVR,3:VDN)';
COMMENT ON COLUMN cti_overflow_config.overflow_value IS '溢出目标值';
COMMENT ON COLUMN cti_overflow_config.lineup_expression IS '自定义排队表达式';
COMMENT ON COLUMN cti_overflow_config.creator IS '创建者';
COMMENT ON COLUMN cti_overflow_config.create_time IS '创建时间';
COMMENT ON COLUMN cti_overflow_config.updater IS '更新者';
COMMENT ON COLUMN cti_overflow_config.update_time IS '更新时间';
COMMENT ON COLUMN cti_overflow_config.deleted IS '是否删除';
COMMENT ON COLUMN cti_overflow_config.tenant_id IS '租户编号';

-- ========================================
-- 自定义溢出策略优先级表
-- ========================================
DROP TABLE IF EXISTS cti_overflow_exp;

CREATE TABLE cti_overflow_exp
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    overflow_id varchar(32),
    exp_key     VARCHAR(128),
    rate        INT,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_overflow_exp IS '自定义溢出策略优先级表';
COMMENT ON COLUMN cti_overflow_exp.id IS '主键ID';
COMMENT ON COLUMN cti_overflow_exp.company_id IS '企业ID';
COMMENT ON COLUMN cti_overflow_exp.overflow_id IS '溢出策略ID';
COMMENT ON COLUMN cti_overflow_exp.exp_key IS '自定义表达式键';
COMMENT ON COLUMN cti_overflow_exp.rate IS '权重';
COMMENT ON COLUMN cti_overflow_exp.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_overflow_exp.creator IS '创建者';
COMMENT ON COLUMN cti_overflow_exp.create_time IS '创建时间';
COMMENT ON COLUMN cti_overflow_exp.updater IS '更新者';
COMMENT ON COLUMN cti_overflow_exp.update_time IS '更新时间';
COMMENT ON COLUMN cti_overflow_exp.deleted IS '是否删除';
COMMENT ON COLUMN cti_overflow_exp.tenant_id IS '租户编号';

-- ========================================
-- 溢出策略前置条件表
-- ========================================
DROP TABLE IF EXISTS cti_overflow_front;

CREATE TABLE cti_overflow_front
(
    id                varchar(32) NOT NULL primary key,
    company_id        varchar(32),
    overflow_id       varchar(32),
    front_type        varchar(32),
    compare_condition varchar(32),
    rank_value_start  INT,
    rank_value        INT,
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_overflow_front IS '溢出策略前置条件表';
COMMENT ON COLUMN cti_overflow_front.id IS '主键ID';
COMMENT ON COLUMN cti_overflow_front.company_id IS '企业ID';
COMMENT ON COLUMN cti_overflow_front.overflow_id IS '溢出策略ID';
COMMENT ON COLUMN cti_overflow_front.front_type IS '前置条件类型(1:队列长度,2:队列等待最大时长,3:呼损率)';
COMMENT ON COLUMN cti_overflow_front.compare_condition IS '比较条件(0:全部,1:小于或等于,2:等于,3:大于或等于,4:大于)';
COMMENT ON COLUMN cti_overflow_front.rank_value_start IS '条件起始值';
COMMENT ON COLUMN cti_overflow_front.rank_value IS '条件值';
COMMENT ON COLUMN cti_overflow_front.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_overflow_front.creator IS '创建者';
COMMENT ON COLUMN cti_overflow_front.create_time IS '创建时间';
COMMENT ON COLUMN cti_overflow_front.updater IS '更新者';
COMMENT ON COLUMN cti_overflow_front.update_time IS '更新时间';
COMMENT ON COLUMN cti_overflow_front.deleted IS '是否删除';
COMMENT ON COLUMN cti_overflow_front.tenant_id IS '租户编号';

-- ========================================
-- 号码归属地表
-- ========================================
DROP TABLE IF EXISTS cti_phone_location;

CREATE TABLE cti_phone_location
(
    id            SERIAL PRIMARY KEY,
    ruid          VARCHAR(64),
    username      VARCHAR(64),
    domain        VARCHAR(190),
    contact       VARCHAR(512),
    received      VARCHAR(128),
    path          VARCHAR(512),
    expires       TIMESTAMP,
    q             DECIMAL(10, 3),
    callid        VARCHAR(256),
    cseq          INT,
    last_modified TIMESTAMP            DEFAULT CURRENT_TIMESTAMP,
    flags         INT                  DEFAULT 0,
    cflags        INT                  DEFAULT 0,
    user_agent    VARCHAR(256),
    socket        VARCHAR(128),
    methods       INT,
    instance      VARCHAR(256),
    reg_id        varchar(32)          DEFAULT 0,
    creator       varchar(32) NULL     DEFAULT '',
    create_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(32) NULL     DEFAULT '',
    update_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       boolean     NOT NULL DEFAULT false,
    tenant_id     varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_phone_location IS 'SIP号码注册位置表';
COMMENT ON COLUMN cti_phone_location.id IS '主键ID';
COMMENT ON COLUMN cti_phone_location.ruid IS 'Record UID';
COMMENT ON COLUMN cti_phone_location.username IS '用户名';
COMMENT ON COLUMN cti_phone_location.domain IS '域名';
COMMENT ON COLUMN cti_phone_location.contact IS '联系方式';
COMMENT ON COLUMN cti_phone_location.received IS '接收地址';
COMMENT ON COLUMN cti_phone_location.path IS '路径';
COMMENT ON COLUMN cti_phone_location.expires IS '过期时间';
COMMENT ON COLUMN cti_phone_location.q IS '质量因子';
COMMENT ON COLUMN cti_phone_location.callid IS 'Call ID';
COMMENT ON COLUMN cti_phone_location.cseq IS 'CSeq序号';
COMMENT ON COLUMN cti_phone_location.last_modified IS '最后修改时间';
COMMENT ON COLUMN cti_phone_location.flags IS '标志位';
COMMENT ON COLUMN cti_phone_location.cflags IS '联系标志';
COMMENT ON COLUMN cti_phone_location.user_agent IS '用户代理';
COMMENT ON COLUMN cti_phone_location.socket IS 'Socket地址';
COMMENT ON COLUMN cti_phone_location.methods IS '支持的方法';
COMMENT ON COLUMN cti_phone_location.instance IS '实例标识';
COMMENT ON COLUMN cti_phone_location.reg_id IS '注册ID';
COMMENT ON COLUMN cti_phone_location.creator IS '创建者';
COMMENT ON COLUMN cti_phone_location.create_time IS '创建时间';
COMMENT ON COLUMN cti_phone_location.updater IS '更新者';
COMMENT ON COLUMN cti_phone_location.update_time IS '更新时间';
COMMENT ON COLUMN cti_phone_location.deleted IS '是否删除';
COMMENT ON COLUMN cti_phone_location.tenant_id IS '租户编号';

-- ========================================
-- 放音文件表
-- ========================================
DROP TABLE IF EXISTS cti_playback;

CREATE TABLE cti_playback
(
    id           varchar(32)  NOT NULL primary key,
    company_id   varchar(32)  NOT NULL,
    cti_playback VARCHAR(256) NOT NULL,
    status       varchar(32)  NOT NULL default '0',
    creator      varchar(32)  NULL     DEFAULT '',
    create_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater      varchar(32)  NULL     DEFAULT '',
    update_time  timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted      boolean      NOT NULL DEFAULT false,
    tenant_id    varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_playback IS '放音文件表,存储IVR和提示音文件';
COMMENT ON COLUMN cti_playback.id IS '主键ID';
COMMENT ON COLUMN cti_playback.company_id IS '企业ID';
COMMENT ON COLUMN cti_playback.cti_playback IS '放音文件路径或URL';
COMMENT ON COLUMN cti_playback.status IS '审核状态(1:待审核,2:审核通过)';
COMMENT ON COLUMN cti_playback.creator IS '创建者';
COMMENT ON COLUMN cti_playback.create_time IS '创建时间';
COMMENT ON COLUMN cti_playback.updater IS '更新者';
COMMENT ON COLUMN cti_playback.update_time IS '更新时间';
COMMENT ON COLUMN cti_playback.deleted IS '是否删除';
COMMENT ON COLUMN cti_playback.tenant_id IS '租户编号';

-- ========================================
-- 话单推送记录表
-- ========================================
DROP TABLE IF EXISTS cti_push_log;

CREATE TABLE cti_push_log
(
    id             varchar(32)  NOT NULL primary key,
    company_id     varchar(32)  NOT NULL,
    call_id        varchar(32)  NOT NULL,
    cdr_notify_url VARCHAR(256) NOT NULL,
    content        TEXT,
    push_times     INT                   DEFAULT 0,
    push_response  TEXT,
    status         varchar(32)  NOT NULL default '0',
    creator        varchar(32)  NULL     DEFAULT '',
    create_time    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32)  NULL     DEFAULT '',
    update_time    timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean      NOT NULL DEFAULT false,
    tenant_id      varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_push_log IS '话单推送记录表';
COMMENT ON COLUMN cti_push_log.id IS '主键ID';
COMMENT ON COLUMN cti_push_log.company_id IS '企业ID';
COMMENT ON COLUMN cti_push_log.call_id IS '通话ID';
COMMENT ON COLUMN cti_push_log.cdr_notify_url IS '推送地址';
COMMENT ON COLUMN cti_push_log.content IS '推送内容';
COMMENT ON COLUMN cti_push_log.push_times IS '推送次数';
COMMENT ON COLUMN cti_push_log.push_response IS '推送返回值';
COMMENT ON COLUMN cti_push_log.status IS '推送状态(1:推送失败,2:推送成功)';
COMMENT ON COLUMN cti_push_log.creator IS '创建者';
COMMENT ON COLUMN cti_push_log.create_time IS '创建时间';
COMMENT ON COLUMN cti_push_log.updater IS '更新者';
COMMENT ON COLUMN cti_push_log.update_time IS '更新时间';
COMMENT ON COLUMN cti_push_log.deleted IS '是否删除';
COMMENT ON COLUMN cti_push_log.tenant_id IS '租户编号';

-- ========================================
-- 字冠路由表
-- ========================================
DROP TABLE IF EXISTS cti_route_call;

CREATE TABLE cti_route_call
(
    id                 varchar(32) NOT NULL primary key,
    company_id         varchar(32) NOT NULL,
    cti_route_group_id varchar(32) NOT NULL,
    route_num          VARCHAR(32) NOT NULL,
    num_max            INT,
    num_min            INT,
    caller_change      INT                  DEFAULT 0,
    caller_change_num  VARCHAR(32),
    called_change      INT                  DEFAULT 0,
    called_change_num  VARCHAR(32),
    status             varchar(32) NOT NULL default '0',
    creator            varchar(32) NULL     DEFAULT '',
    create_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater            varchar(32) NULL     DEFAULT '',
    update_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted            boolean     NOT NULL DEFAULT false,
    tenant_id          varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_route_call IS '字冠路由表,根据号码前缀进行路由';
COMMENT ON COLUMN cti_route_call.id IS '主键ID';
COMMENT ON COLUMN cti_route_call.company_id IS '企业ID';
COMMENT ON COLUMN cti_route_call.cti_route_group_id IS '路由组ID';
COMMENT ON COLUMN cti_route_call.route_num IS '字冠号码';
COMMENT ON COLUMN cti_route_call.num_max IS '号码最大长度';
COMMENT ON COLUMN cti_route_call.num_min IS '号码最小长度';
COMMENT ON COLUMN cti_route_call.caller_change IS '主叫替换规则';
COMMENT ON COLUMN cti_route_call.caller_change_num IS '主叫替换号码';
COMMENT ON COLUMN cti_route_call.called_change IS '被叫替换规则';
COMMENT ON COLUMN cti_route_call.called_change_num IS '被叫替换号码';
COMMENT ON COLUMN cti_route_call.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_route_call.creator IS '创建者';
COMMENT ON COLUMN cti_route_call.create_time IS '创建时间';
COMMENT ON COLUMN cti_route_call.updater IS '更新者';
COMMENT ON COLUMN cti_route_call.update_time IS '更新时间';
COMMENT ON COLUMN cti_route_call.deleted IS '是否删除';
COMMENT ON COLUMN cti_route_call.tenant_id IS '租户编号';

-- ========================================
-- 媒体网关表
-- ========================================
DROP TABLE IF EXISTS cti_route_gateway;

CREATE TABLE cti_route_gateway
(
    id            varchar(32) NOT NULL primary key,
    name          VARCHAR(64) NOT NULL,
    media_host    VARCHAR(64),
    media_port    INT,
    caller_prefix VARCHAR(32),
    called_prefix VARCHAR(32),
    profile       VARCHAR(64),
    sip_header1   VARCHAR(256),
    sip_header2   VARCHAR(256),
    sip_header3   VARCHAR(256),
    status        varchar(32) NOT NULL default '0',
    creator       varchar(32) NULL     DEFAULT '',
    create_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(32) NULL     DEFAULT '',
    update_time   timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       boolean     NOT NULL DEFAULT false,
    tenant_id     varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_route_gateway IS '媒体网关表';
COMMENT ON COLUMN cti_route_gateway.id IS '主键ID';
COMMENT ON COLUMN cti_route_gateway.name IS '网关名称';
COMMENT ON COLUMN cti_route_gateway.media_host IS '媒体地址';
COMMENT ON COLUMN cti_route_gateway.media_port IS '媒体端口';
COMMENT ON COLUMN cti_route_gateway.caller_prefix IS '主叫号码前缀';
COMMENT ON COLUMN cti_route_gateway.called_prefix IS '被叫号码前缀';
COMMENT ON COLUMN cti_route_gateway.profile IS '媒体拨号计划文件';
COMMENT ON COLUMN cti_route_gateway.sip_header1 IS 'SIP头部1';
COMMENT ON COLUMN cti_route_gateway.sip_header2 IS 'SIP头部2';
COMMENT ON COLUMN cti_route_gateway.sip_header3 IS 'SIP头部3';
COMMENT ON COLUMN cti_route_gateway.status IS '网关状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_route_gateway.creator IS '创建者';
COMMENT ON COLUMN cti_route_gateway.create_time IS '创建时间';
COMMENT ON COLUMN cti_route_gateway.updater IS '更新者';
COMMENT ON COLUMN cti_route_gateway.update_time IS '更新时间';
COMMENT ON COLUMN cti_route_gateway.deleted IS '是否删除';
COMMENT ON COLUMN cti_route_gateway.tenant_id IS '租户编号';

-- ========================================
-- 路由网关关联表
-- ========================================
DROP TABLE IF EXISTS cti_route_gateway_group;

CREATE TABLE cti_route_gateway_group
(
    id             varchar(32) NOT NULL primary key,
    gateway_id     varchar(32),
    route_group_id varchar(32),
    creator        varchar(32) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_route_gateway_group IS '路由字冠关联组表';
COMMENT ON COLUMN cti_route_gateway_group.id IS '主键ID';
COMMENT ON COLUMN cti_route_gateway_group.gateway_id IS '网关ID';
COMMENT ON COLUMN cti_route_gateway_group.route_group_id IS '路由组ID';
COMMENT ON COLUMN cti_route_gateway_group.creator IS '创建者';
COMMENT ON COLUMN cti_route_gateway_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_route_gateway_group.updater IS '更新者';
COMMENT ON COLUMN cti_route_gateway_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_route_gateway_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_route_gateway_group.tenant_id IS '租户编号';

-- ========================================
-- 网关组表
-- ========================================
DROP TABLE IF EXISTS cti_route_group;

CREATE TABLE cti_route_group
(
    id          varchar(32) NOT NULL primary key,
    route_group VARCHAR(256),
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_route_group IS '网关组表';
COMMENT ON COLUMN cti_route_group.id IS '主键ID';
COMMENT ON COLUMN cti_route_group.route_group IS '网关组名称';
COMMENT ON COLUMN cti_route_group.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_route_group.creator IS '创建者';
COMMENT ON COLUMN cti_route_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_route_group.updater IS '更新者';
COMMENT ON COLUMN cti_route_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_route_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_route_group.tenant_id IS '租户编号';

-- ========================================
-- 网关注册账号表
-- ========================================
DROP TABLE IF EXISTS cti_sip_gateway;

CREATE TABLE cti_sip_gateway
(
    id            varchar(32)  NOT NULL primary key,
    company_id    varchar(32)  NOT NULL,
    company_code  VARCHAR(32),
    company_name  VARCHAR(64),
    username      VARCHAR(32)  NOT NULL,
    passwd        VARCHAR(32)  NOT NULL,
    internal      VARCHAR(64),
    external      VARCHAR(64),
    register_addr VARCHAR(128) NOT NULL,
    register_time timestamp,
    expire        INT                   DEFAULT 3600,
    status        varchar(32)  NOT NULL default '0',
    creator       varchar(32)  NULL     DEFAULT '',
    create_time   timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater       varchar(32)  NULL     DEFAULT '',
    update_time   timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted       boolean      NOT NULL DEFAULT false,
    tenant_id     varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_sip_gateway IS '网关注册账号表';
COMMENT ON COLUMN cti_sip_gateway.username IS '注册账号(2-16位)';
COMMENT ON COLUMN cti_sip_gateway.passwd IS '注册密码(2-16位)';
COMMENT ON COLUMN cti_sip_gateway.register_addr IS '注册地址(必填)';
COMMENT ON COLUMN cti_sip_gateway.internal IS '网关内网地址';
COMMENT ON COLUMN cti_sip_gateway.external IS '网关外网地址';
COMMENT ON COLUMN cti_sip_gateway.register_addr IS '注册地址';
COMMENT ON COLUMN cti_sip_gateway.register_time IS '注册时间';
COMMENT ON COLUMN cti_sip_gateway.expire IS '注册周期(秒)';
COMMENT ON COLUMN cti_sip_gateway.status IS '状态(0:删除,1:不在线,2:在线)';
COMMENT ON COLUMN cti_sip_gateway.creator IS '创建者';
COMMENT ON COLUMN cti_sip_gateway.create_time IS '创建时间';
COMMENT ON COLUMN cti_sip_gateway.updater IS '更新者';
COMMENT ON COLUMN cti_sip_gateway.update_time IS '更新时间';
COMMENT ON COLUMN cti_sip_gateway.deleted IS '是否删除';
COMMENT ON COLUMN cti_sip_gateway.tenant_id IS '租户编号';

-- ========================================
-- 技能表
-- ========================================
DROP TABLE IF EXISTS cti_skill;

CREATE TABLE cti_skill
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32) NOT NULL,
    name        VARCHAR(64) NOT NULL,
    remark      VARCHAR(256),
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_skill IS '技能表';
COMMENT ON COLUMN cti_skill.name IS '技能名称(2-16位)';
COMMENT ON COLUMN cti_skill.remark IS '技能描述(1-100位)';
COMMENT ON COLUMN cti_skill.creator IS '创建者';
COMMENT ON COLUMN cti_skill.create_time IS '创建时间';
COMMENT ON COLUMN cti_skill.updater IS '更新者';
COMMENT ON COLUMN cti_skill.update_time IS '更新时间';
COMMENT ON COLUMN cti_skill.deleted IS '是否删除';
COMMENT ON COLUMN cti_skill.tenant_id IS '租户编号';

-- ========================================
-- 坐席技能表
-- ========================================
DROP TABLE IF EXISTS cti_skill_agent;

CREATE TABLE cti_skill_agent
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    skill_id    varchar(32),
    agent_id    varchar(32),
    rank_value  INT,
    status      varchar(32) NOT NULL default '0',
    agent_key   VARCHAR(64),
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_skill_agent IS '坐席技能表,定义坐席拥有的技能';
COMMENT ON COLUMN cti_skill_agent.id IS '主键ID';
COMMENT ON COLUMN cti_skill_agent.company_id IS '企业ID';
COMMENT ON COLUMN cti_skill_agent.skill_id IS '技能ID';
COMMENT ON COLUMN cti_skill_agent.agent_id IS '坐席ID';
COMMENT ON COLUMN cti_skill_agent.rank_value IS '技能等级';
COMMENT ON COLUMN cti_skill_agent.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_skill_agent.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_skill_agent.creator IS '创建者';
COMMENT ON COLUMN cti_skill_agent.create_time IS '创建时间';
COMMENT ON COLUMN cti_skill_agent.updater IS '更新者';
COMMENT ON COLUMN cti_skill_agent.update_time IS '更新时间';
COMMENT ON COLUMN cti_skill_agent.deleted IS '是否删除';
COMMENT ON COLUMN cti_skill_agent.tenant_id IS '租户编号';

-- ========================================
-- 技能组技能表
-- ========================================
DROP TABLE IF EXISTS cti_skill_group;

CREATE TABLE cti_skill_group
(
    id               varchar(32) NOT NULL primary key,
    company_id       varchar(32),
    skill_id         varchar(32),
    group_id         varchar(32),
    level_value      INT,
    rank_type        varchar(32),
    rank_value_start INT,
    rank_value       INT,
    match_type       varchar(32),
    share_value      INT,
    status           varchar(32) NOT NULL default '0',
    creator          varchar(32) NULL     DEFAULT '',
    create_time      timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater          varchar(32) NULL     DEFAULT '',
    update_time      timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted          boolean     NOT NULL DEFAULT false,
    tenant_id        varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_skill_group IS '技能组技能表';
COMMENT ON COLUMN cti_skill_group.id IS '主键ID';
COMMENT ON COLUMN cti_skill_group.company_id IS '企业ID';
COMMENT ON COLUMN cti_skill_group.skill_id IS '技能ID';
COMMENT ON COLUMN cti_skill_group.group_id IS '技能组ID';
COMMENT ON COLUMN cti_skill_group.level_value IS '优先级';
COMMENT ON COLUMN cti_skill_group.rank_type IS '等级类型(1:全部,2:等于,3:大于,4:小于,5:介于)';
COMMENT ON COLUMN cti_skill_group.rank_value_start IS '等级起始值';
COMMENT ON COLUMN cti_skill_group.rank_value IS '等级值';
COMMENT ON COLUMN cti_skill_group.match_type IS '匹配规则(1:从低到高,2:从高到低,3:平均分配)';
COMMENT ON COLUMN cti_skill_group.share_value IS '分配权重';
COMMENT ON COLUMN cti_skill_group.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_skill_group.creator IS '创建者';
COMMENT ON COLUMN cti_skill_group.create_time IS '创建时间';
COMMENT ON COLUMN cti_skill_group.updater IS '更新者';
COMMENT ON COLUMN cti_skill_group.update_time IS '更新时间';
COMMENT ON COLUMN cti_skill_group.deleted IS '是否删除';
COMMENT ON COLUMN cti_skill_group.tenant_id IS '租户编号';

-- ========================================
-- 坐席日统计表
-- ========================================
DROP TABLE IF EXISTS cti_stat_day_agent;

CREATE TABLE cti_stat_day_agent
(
    id                 varchar(32) NOT NULL primary key,
    company_id         varchar(32) NOT NULL,
    agent_key          VARCHAR(32) NOT NULL,
    agent_name         VARCHAR(64),
    stat_time          timestamp   NOT NULL,
    callout_cnt        BIGINT               DEFAULT 0,
    callout_answer_cnt BIGINT               DEFAULT 0,
    callin_cnt         BIGINT               DEFAULT 0,
    callin_answer_cnt  BIGINT               DEFAULT 0,
    login_cnt          BIGINT               DEFAULT 0,
    ready_cnt          BIGINT               DEFAULT 0,
    not_ready_cnt      BIGINT               DEFAULT 0,
    after_cnt          BIGINT               DEFAULT 0,
    login_time         BIGINT               DEFAULT 0,
    ready_time         BIGINT               DEFAULT 0,
    not_ready_time     BIGINT               DEFAULT 0,
    busy_time          BIGINT               DEFAULT 0,
    after_time         BIGINT               DEFAULT 0,
    talk_time          BIGINT               DEFAULT 0,
    callin_talk_time   BIGINT               DEFAULT 0,
    callout_talk_time  BIGINT               DEFAULT 0,
    status             varchar(32) NOT NULL default '0',
    creator            varchar(32) NULL     DEFAULT '',
    create_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater            varchar(32) NULL     DEFAULT '',
    update_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted            boolean     NOT NULL DEFAULT false,
    tenant_id          varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_stat_day_agent IS '坐席日统计表';
COMMENT ON COLUMN cti_stat_day_agent.id IS '主键ID';
COMMENT ON COLUMN cti_stat_day_agent.company_id IS '企业ID';
COMMENT ON COLUMN cti_stat_day_agent.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_stat_day_agent.agent_name IS '坐席姓名';
COMMENT ON COLUMN cti_stat_day_agent.stat_time IS '统计日期';
COMMENT ON COLUMN cti_stat_day_agent.callout_cnt IS '外呼次数';
COMMENT ON COLUMN cti_stat_day_agent.callout_answer_cnt IS '外呼接通次数';
COMMENT ON COLUMN cti_stat_day_agent.callin_cnt IS '呼入次数';
COMMENT ON COLUMN cti_stat_day_agent.callin_answer_cnt IS '呼入应答次数';
COMMENT ON COLUMN cti_stat_day_agent.login_cnt IS '登录次数';
COMMENT ON COLUMN cti_stat_day_agent.ready_cnt IS '就绪次数';
COMMENT ON COLUMN cti_stat_day_agent.not_ready_cnt IS '未就绪次数';
COMMENT ON COLUMN cti_stat_day_agent.after_cnt IS '话后次数';
COMMENT ON COLUMN cti_stat_day_agent.login_time IS '登录时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.ready_time IS '就绪时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.not_ready_time IS '未就绪时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.busy_time IS '通话时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.after_time IS '话后时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.talk_time IS '总通话时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.callin_talk_time IS '呼入通话时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.callout_talk_time IS '外呼通话时长(秒)';
COMMENT ON COLUMN cti_stat_day_agent.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_stat_day_agent.creator IS '创建者';
COMMENT ON COLUMN cti_stat_day_agent.create_time IS '创建时间';
COMMENT ON COLUMN cti_stat_day_agent.updater IS '更新者';
COMMENT ON COLUMN cti_stat_day_agent.update_time IS '更新时间';
COMMENT ON COLUMN cti_stat_day_agent.deleted IS '是否删除';
COMMENT ON COLUMN cti_stat_day_agent.tenant_id IS '租户编号';

CREATE INDEX idx_stat_day_agent_company ON cti_stat_day_agent (company_id);
CREATE INDEX idx_stat_day_agent_time ON cti_stat_day_agent (stat_time);

-- ========================================
-- 坐席小时统计表
-- ========================================
DROP TABLE IF EXISTS cti_stat_hour_agent;

CREATE TABLE cti_stat_hour_agent
(
    id                 varchar(32) NOT NULL primary key,
    company_id         varchar(32) NOT NULL,
    agent_key          VARCHAR(32) NOT NULL,
    agent_name         VARCHAR(64),
    stat_time          timestamp   NOT NULL,
    callout_cnt        BIGINT               DEFAULT 0,
    callout_answer_cnt BIGINT               DEFAULT 0,
    callin_cnt         BIGINT               DEFAULT 0,
    callin_answer_cnt  BIGINT               DEFAULT 0,
    login_cnt          BIGINT               DEFAULT 0,
    ready_cnt          BIGINT               DEFAULT 0,
    not_ready_cnt      BIGINT               DEFAULT 0,
    after_cnt          BIGINT               DEFAULT 0,
    login_time         BIGINT               DEFAULT 0,
    ready_time         BIGINT               DEFAULT 0,
    not_ready_time     BIGINT               DEFAULT 0,
    busy_time          BIGINT               DEFAULT 0,
    after_time         BIGINT               DEFAULT 0,
    talk_time          BIGINT               DEFAULT 0,
    callin_talk_time   BIGINT               DEFAULT 0,
    callout_talk_time  BIGINT               DEFAULT 0,
    status             varchar(32) NOT NULL default '0',
    creator            varchar(32) NULL     DEFAULT '',
    create_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater            varchar(32) NULL     DEFAULT '',
    update_time        timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted            boolean     NOT NULL DEFAULT false,
    tenant_id          varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_stat_hour_agent IS '坐席小时统计表';
COMMENT ON COLUMN cti_stat_hour_agent.id IS '主键ID';
COMMENT ON COLUMN cti_stat_hour_agent.company_id IS '企业ID';
COMMENT ON COLUMN cti_stat_hour_agent.agent_key IS '坐席工号';
COMMENT ON COLUMN cti_stat_hour_agent.agent_name IS '坐席姓名';
COMMENT ON COLUMN cti_stat_hour_agent.stat_time IS '统计小时';
COMMENT ON COLUMN cti_stat_hour_agent.callout_cnt IS '外呼次数';
COMMENT ON COLUMN cti_stat_hour_agent.callout_answer_cnt IS '外呼接通次数';
COMMENT ON COLUMN cti_stat_hour_agent.callin_cnt IS '呼入次数';
COMMENT ON COLUMN cti_stat_hour_agent.callin_answer_cnt IS '呼入应答次数';
COMMENT ON COLUMN cti_stat_hour_agent.login_cnt IS '登录次数';
COMMENT ON COLUMN cti_stat_hour_agent.ready_cnt IS '就绪次数';
COMMENT ON COLUMN cti_stat_hour_agent.not_ready_cnt IS '未就绪次数';
COMMENT ON COLUMN cti_stat_hour_agent.after_cnt IS '话后次数';
COMMENT ON COLUMN cti_stat_hour_agent.login_time IS '登录时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.ready_time IS '就绪时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.not_ready_time IS '未就绪时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.busy_time IS '通话时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.after_time IS '话后时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.talk_time IS '总通话时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.callin_talk_time IS '呼入通话时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.callout_talk_time IS '外呼通话时长(秒)';
COMMENT ON COLUMN cti_stat_hour_agent.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_stat_hour_agent.creator IS '创建者';
COMMENT ON COLUMN cti_stat_hour_agent.create_time IS '创建时间';
COMMENT ON COLUMN cti_stat_hour_agent.updater IS '更新者';
COMMENT ON COLUMN cti_stat_hour_agent.update_time IS '更新时间';
COMMENT ON COLUMN cti_stat_hour_agent.deleted IS '是否删除';
COMMENT ON COLUMN cti_stat_hour_agent.tenant_id IS '租户编号';

CREATE INDEX idx_stat_hour_agent_company ON cti_stat_hour_agent (company_id);
CREATE INDEX idx_stat_hour_agent_time ON cti_stat_hour_agent (stat_time);

-- ========================================
-- 站点配置表
-- ========================================
DROP TABLE IF EXISTS cti_station;

CREATE TABLE cti_station
(
    id                varchar(32) NOT NULL primary key,
    application_id    varchar(32),
    application_type  varchar(32),
    application_group VARCHAR(64),
    application_host  VARCHAR(64) NOT NULL,
    application_port  INT         NOT NULL,
    username          VARCHAR(64) NOT NULL,
    pwd               VARCHAR(64) NOT NULL,
    status            varchar(32) NOT NULL default '0',
    creator           varchar(32) NULL     DEFAULT '',
    create_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater           varchar(32) NULL     DEFAULT '',
    update_time       timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted           boolean     NOT NULL DEFAULT false,
    tenant_id         varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_station IS '站点配置信息表,存储应用站点的配置信息,包括应用服务器地址、端口、认证信息等';
COMMENT ON COLUMN cti_station.id IS '主键ID';
COMMENT ON COLUMN cti_station.application_id IS '应用站点ID';
COMMENT ON COLUMN cti_station.application_type IS '应用类型';
COMMENT ON COLUMN cti_station.application_group IS '应用分组';
COMMENT ON COLUMN cti_station.application_host IS '应用服务器地址';
COMMENT ON COLUMN cti_station.application_port IS '应用服务器端口';
COMMENT ON COLUMN cti_station.username IS '登录用户名';
COMMENT ON COLUMN cti_station.pwd IS '登录密码';
COMMENT ON COLUMN cti_station.status IS '站点状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_station.creator IS '创建者';
COMMENT ON COLUMN cti_station.create_time IS '创建时间';
COMMENT ON COLUMN cti_station.updater IS '更新者';
COMMENT ON COLUMN cti_station.update_time IS '更新时间';
COMMENT ON COLUMN cti_station.deleted IS '是否删除';
COMMENT ON COLUMN cti_station.tenant_id IS '租户编号';

-- ========================================
-- 流媒体通话实体表
-- ========================================
DROP TABLE IF EXISTS cti_stream_entity;

CREATE TABLE cti_stream_entity
(
    id             varchar(32) NOT NULL primary key,
    call_id        varchar(32) NOT NULL,
    caller_address VARCHAR(128),
    called_address VARCHAR(128),
    device1        VARCHAR(128),
    device2        VARCHAR(128),
    creator        varchar(32) NULL     DEFAULT '',
    create_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater        varchar(32) NULL     DEFAULT '',
    update_time    timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted        boolean     NOT NULL DEFAULT false,
    tenant_id      varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_stream_entity IS '流媒体通话实体表,存储实时通话的流媒体信息';
COMMENT ON COLUMN cti_stream_entity.id IS '主键ID';
COMMENT ON COLUMN cti_stream_entity.call_id IS '通话的唯一标识';
COMMENT ON COLUMN cti_stream_entity.caller_address IS '主叫地址';
COMMENT ON COLUMN cti_stream_entity.called_address IS '被叫地址';
COMMENT ON COLUMN cti_stream_entity.device1 IS '设备1标识';
COMMENT ON COLUMN cti_stream_entity.device2 IS '设备2标识';
COMMENT ON COLUMN cti_stream_entity.creator IS '创建者';
COMMENT ON COLUMN cti_stream_entity.create_time IS '创建时间';
COMMENT ON COLUMN cti_stream_entity.updater IS '更新者';
COMMENT ON COLUMN cti_stream_entity.update_time IS '更新时间';
COMMENT ON COLUMN cti_stream_entity.deleted IS '是否删除';
COMMENT ON COLUMN cti_stream_entity.tenant_id IS '租户编号';

-- ========================================
-- 呼入路由VDN表
-- ========================================
DROP TABLE IF EXISTS cti_vdn_code;

CREATE TABLE cti_vdn_code
(
    id          varchar(32)  NOT NULL primary key,
    company_id  varchar(32)  NOT NULL,
    name        VARCHAR(128) NOT NULL,
    status      varchar(32)  NOT NULL default '0',
    creator     varchar(32)  NULL     DEFAULT '',
    create_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32)  NULL     DEFAULT '',
    update_time timestamp    NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean      NOT NULL DEFAULT false,
    tenant_id   varchar(32)  NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_vdn_code IS '呼入路由VDN表';
COMMENT ON COLUMN cti_vdn_code.id IS '主键ID';
COMMENT ON COLUMN cti_vdn_code.company_id IS '企业ID';
COMMENT ON COLUMN cti_vdn_code.name IS 'VDN名称';
COMMENT ON COLUMN cti_vdn_code.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_vdn_code.creator IS '创建者';
COMMENT ON COLUMN cti_vdn_code.create_time IS '创建时间';
COMMENT ON COLUMN cti_vdn_code.updater IS '更新者';
COMMENT ON COLUMN cti_vdn_code.update_time IS '更新时间';
COMMENT ON COLUMN cti_vdn_code.deleted IS '是否删除';
COMMENT ON COLUMN cti_vdn_code.tenant_id IS '租户编号';

CREATE INDEX idx_vdn_code_company ON cti_vdn_code (company_id);

-- ========================================
-- VDN配置表
-- ========================================
DROP TABLE IF EXISTS cti_vdn_config;

CREATE TABLE cti_vdn_config
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    name        VARCHAR(64),
    vdn_id      varchar(32),
    schedule_id varchar(32),
    route_type  varchar(32),
    route_value VARCHAR(64),
    play_type   varchar(32),
    play_value  BIGINT,
    dtmf_end    VARCHAR(32),
    retry       INT,
    dtmf_max    INT,
    dtmf_min    INT,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_vdn_config IS '路由字码表';
COMMENT ON COLUMN cti_vdn_config.id IS '主键ID';
COMMENT ON COLUMN cti_vdn_config.company_id IS '企业ID';
COMMENT ON COLUMN cti_vdn_config.name IS '子码日程';
COMMENT ON COLUMN cti_vdn_config.vdn_id IS 'vdn_id';
COMMENT ON COLUMN cti_vdn_config.schedule_id IS '日程id';
COMMENT ON COLUMN cti_vdn_config.route_type IS '路由类型(1:技能组,2:放音,3:ivr,4:坐席,5:外呼)';
COMMENT ON COLUMN cti_vdn_config.route_value IS '路由类型值';
COMMENT ON COLUMN cti_vdn_config.play_type IS '放音类型(1:按键导航,2:技能组,3:ivr,4:路由字码,5:挂机)';
COMMENT ON COLUMN cti_vdn_config.play_value IS '放音类型对应值';
COMMENT ON COLUMN cti_vdn_config.dtmf_end IS '结束按键符';
COMMENT ON COLUMN cti_vdn_config.retry IS '重复播放次数';
COMMENT ON COLUMN cti_vdn_config.dtmf_max IS '最大收键长度';
COMMENT ON COLUMN cti_vdn_config.dtmf_min IS '最小收键长度';
COMMENT ON COLUMN cti_vdn_config.status IS '配置状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_vdn_config.creator IS '创建者';
COMMENT ON COLUMN cti_vdn_config.create_time IS '创建时间';
COMMENT ON COLUMN cti_vdn_config.updater IS '更新者';
COMMENT ON COLUMN cti_vdn_config.update_time IS '更新时间';
COMMENT ON COLUMN cti_vdn_config.deleted IS '是否删除';
COMMENT ON COLUMN cti_vdn_config.tenant_id IS '租户编号';

-- ========================================
-- 按键导航表
-- ========================================
DROP TABLE IF EXISTS cti_vdn_dtmf;

CREATE TABLE cti_vdn_dtmf
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    navigate_id varchar(32),
    dtmf        VARCHAR(32),
    route_type  varchar(32),
    route_value BIGINT,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_vdn_dtmf IS '按键导航表,定义DTMF按键对应的路由规则';
COMMENT ON COLUMN cti_vdn_dtmf.id IS '主键ID';
COMMENT ON COLUMN cti_vdn_dtmf.company_id IS '企业ID';
COMMENT ON COLUMN cti_vdn_dtmf.navigate_id IS '导航ID';
COMMENT ON COLUMN cti_vdn_dtmf.dtmf IS 'DTMF按键值';
COMMENT ON COLUMN cti_vdn_dtmf.route_type IS '路由类型(1:技能组,2:IVR,3:VDN)';
COMMENT ON COLUMN cti_vdn_dtmf.route_value IS '路由值';
COMMENT ON COLUMN cti_vdn_dtmf.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_vdn_dtmf.creator IS '创建者';
COMMENT ON COLUMN cti_vdn_dtmf.create_time IS '创建时间';
COMMENT ON COLUMN cti_vdn_dtmf.updater IS '更新者';
COMMENT ON COLUMN cti_vdn_dtmf.update_time IS '更新时间';
COMMENT ON COLUMN cti_vdn_dtmf.deleted IS '是否删除';
COMMENT ON COLUMN cti_vdn_dtmf.tenant_id IS '租户编号';

-- ========================================
-- 路由号码表
-- ========================================
DROP TABLE IF EXISTS cti_vdn_phone;

CREATE TABLE cti_vdn_phone
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32) NOT NULL,
    vdn_id      varchar(32) NOT NULL,
    phone       VARCHAR(32) NOT NULL,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_vdn_phone IS '路由号码表';
COMMENT ON COLUMN cti_vdn_phone.id IS '主键ID';
COMMENT ON COLUMN cti_vdn_phone.company_id IS '企业ID';
COMMENT ON COLUMN cti_vdn_phone.vdn_id IS 'VDN ID';
COMMENT ON COLUMN cti_vdn_phone.phone IS '号码';
COMMENT ON COLUMN cti_vdn_phone.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_vdn_phone.creator IS '创建者';
COMMENT ON COLUMN cti_vdn_phone.create_time IS '创建时间';
COMMENT ON COLUMN cti_vdn_phone.updater IS '更新者';
COMMENT ON COLUMN cti_vdn_phone.update_time IS '更新时间';
COMMENT ON COLUMN cti_vdn_phone.deleted IS '是否删除';
COMMENT ON COLUMN cti_vdn_phone.tenant_id IS '租户编号';

CREATE INDEX idx_vdn_phone_company ON cti_vdn_phone (company_id);
CREATE INDEX idx_vdn_phone_vdn ON cti_vdn_phone (vdn_id);

-- ========================================
-- 日程表
-- ========================================
DROP TABLE IF EXISTS cti_vdn_schedule;

CREATE TABLE cti_vdn_schedule
(
    id          varchar(32) NOT NULL primary key,
    company_id  varchar(32),
    name        VARCHAR(256),
    level_value INT,
    type        varchar(32),
    start_day   VARCHAR(32),
    end_day     VARCHAR(32),
    start_time  VARCHAR(32),
    end_time    VARCHAR(32),
    mon         boolean              DEFAULT false,
    tue         boolean              DEFAULT false,
    wed         boolean              DEFAULT false,
    thu         boolean              DEFAULT false,
    fri         boolean              DEFAULT false,
    sat         boolean              DEFAULT false,
    sun         boolean              DEFAULT false,
    status      varchar(32) NOT NULL default '0',
    creator     varchar(32) NULL     DEFAULT '',
    create_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updater     varchar(32) NULL     DEFAULT '',
    update_time timestamp   NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted     boolean     NOT NULL DEFAULT false,
    tenant_id   varchar(32) NOT NULL DEFAULT '0'
);

COMMENT ON TABLE cti_vdn_schedule IS '日程表,用于配置呼叫中心的工作时间安排';
COMMENT ON COLUMN cti_vdn_schedule.id IS '主键ID';
COMMENT ON COLUMN cti_vdn_schedule.company_id IS '企业ID';
COMMENT ON COLUMN cti_vdn_schedule.name IS '日程名称';
COMMENT ON COLUMN cti_vdn_schedule.level_value IS '优先级';
COMMENT ON COLUMN cti_vdn_schedule.type IS '类型(1:每天,2:按星期,3:节假日)';
COMMENT ON COLUMN cti_vdn_schedule.start_day IS '开始日期';
COMMENT ON COLUMN cti_vdn_schedule.end_day IS '结束日期';
COMMENT ON COLUMN cti_vdn_schedule.start_time IS '开始时间';
COMMENT ON COLUMN cti_vdn_schedule.end_time IS '结束时间';
COMMENT ON COLUMN cti_vdn_schedule.mon IS '星期一(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.tue IS '星期二(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.wed IS '星期三(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.thu IS '星期四(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.fri IS '星期五(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.sat IS '星期六(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.sun IS '星期日(0:否,1:是)';
COMMENT ON COLUMN cti_vdn_schedule.status IS '数据状态(0:启用,1:禁用)';
COMMENT ON COLUMN cti_vdn_schedule.creator IS '创建者';
COMMENT ON COLUMN cti_vdn_schedule.create_time IS '创建时间';
COMMENT ON COLUMN cti_vdn_schedule.updater IS '更新者';
COMMENT ON COLUMN cti_vdn_schedule.update_time IS '更新时间';
COMMENT ON COLUMN cti_vdn_schedule.deleted IS '是否删除';
COMMENT ON COLUMN cti_vdn_schedule.tenant_id IS '租户编号';