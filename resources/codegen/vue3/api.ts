import request from '@/config/axios'

export interface {

{
    table.className
}
}
VO
{
    number
    {%
        for col in columns %
    }
    {%
        if col.columnName != "id" %
    }
    {
        {
            col.javaField
        }
    }
:
    {
        {
            col.javaType | replace(from = "i32", to = "number") | replace(from = "i64", to = "number") | replace(from = "f64", to = "number") | replace(from = "String", to = "string") | replace(from = "bool", to = "boolean") | replace(from = "DateTime", to = "number") | replace(from = "Date", to = "number") | replace(from = "Decimal", to = "number")
        }
    }
    {%
        endif %
    }
    {%
        endfor %
    }
}

export interface {

{
    table.className
}
}
PageReqVO
extends
PageParam
{
    {%
        for col in columns %
    }
    {%
        if col.listOperation %
    }
    {
        {
            col.javaField
        }
    }
        ? : {
    {
        col.javaType | replace(from = "i32", to = "number") | replace(from = "i64", to = "number") | replace(from = "f64", to = "number") | replace(from = "String", to = "string") | replace(from = "bool", to = "boolean") | replace(from = "DateTime", to = "number") | replace(from = "Date", to = "number") | replace(from = "Decimal", to = "number")
    }
}
    {%
        endif %
    }
    {%
        endfor %
    }
}

// 查询列表
export const get
{
    {
        table.className
    }
}
Page = (params: {
{
    table.className
}
}
PageReqVO
) =>
{
    return request.get({url: '/{{ table.moduleName }}/{{ table.businessName }}/page', params})
}

// 查询详情
export const get
{
    {
        table.className
    }
}
= (id: number) => {
    return request.get({url: '/{{ table.moduleName }}/{{ table.businessName }}/get?id=' + id})
}

// 新增
export const create
{
    {
        table.className
    }
}
= (data: {
{
    table.className
}
}
VO
) =>
{
    return request.post({url: '/{{ table.moduleName }}/{{ table.businessName }}/create', data})
}

// 修改
export const update
{
    {
        table.className
    }
}
= (data: {
{
    table.className
}
}
VO
) =>
{
    return request.put({url: '/{{ table.moduleName }}/{{ table.businessName }}/update', data})
}

// 删除
export const delete
{
    {
        table.className
    }
}
= (id: number) => {
    return request.delete({url: '/{{ table.moduleName }}/{{ table.businessName }}/delete?id=' + id})
}

// 导出
export const export
{
    {
        table.className
    }
}
= (params: {
{
    table.className
}
}
PageReqVO
) =>
{
    return request.download({url: '/{{ table.moduleName }}/{{ table.businessName }}/export-excel', params})
}
