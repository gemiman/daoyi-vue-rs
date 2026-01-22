-- 菜单 SQL
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ menu_id }}', '{{ table.classComment }}', '', '2', {{ table.genMenuSort | default(value=1) }}, '{{ table.parentMenuId }}',
    '{{ table.businessName }}', 'ep:menu', '{{ table.moduleName }}/{{ table.businessName }}/index', '{{ table.className }}', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 按钮 SQL
-- 1. 查询
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_query_id }}', '{{ table.classComment }}查询', '{{ table.moduleName }}:{{ table.businessName }}:query', '3', 1, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 2. 新增
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_create_id }}', '{{ table.classComment }}新增', '{{ table.moduleName }}:{{ table.businessName }}:create', '3', 2, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 3. 修改
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_update_id }}', '{{ table.classComment }}修改', '{{ table.moduleName }}:{{ table.businessName }}:update', '3', 3, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 4. 删除
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_delete_id }}', '{{ table.classComment }}删除', '{{ table.moduleName }}:{{ table.businessName }}:delete', '3', 4, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);

-- 5. 导出
INSERT INTO system.system_menu (
    id, name, permission, type, sort, parent_id,
    path, icon, component, component_name, status, visible, keep_alive, always_show,
    creator, create_time, updater, update_time, deleted, tenant_id
)
VALUES (
    '{{ button_export_id }}', '{{ table.classComment }}导出', '{{ table.moduleName }}:{{ table.businessName }}:export', '3', 5, '{{ menu_id }}',
    '', '', '', '', '0', true, true, true,
    'admin', CURRENT_TIMESTAMP, 'admin', CURRENT_TIMESTAMP, false, '1'
);