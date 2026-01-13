# 座席SDK API接口文档

座席SDK API接口文档

### 文档修订摘要

| 日期         | 版本号     | 描述                                        |
|:-----------|:--------|:------------------------------------------|
| 2023-07-18 | V1.0.0  | 新增文档说明接口定义以及事件定义                          |
| 2023-07-28 | V1.0.0  | 新增班长拦截以及代接，静音等接口以及事件定义说明                  |
| 2023-08-07 | V1.0.0  | 优化接口以及事件的说明内容                             |
| 2023-08-16 | V1.0.1  | 修改开发集成方式，更改登录传参                           |
| 2023-08-23 | V1.0.2  | 新增通话语音识别翻译功能                              |
| 2023-09-04 | V1.0.2  | 优化文案描述                                    |
| 2023-09-07 | V1.0.2  | 新增主动请求音视频切换，媒体文件推送，工号临时加入/移除队列，本地视频推流(验证) |
| 2023-09-18 | V1.0.3  | 新增客服变音功能                                  |
| 2023-12-11 | V1.0.4  | 修改变音相关接口和事件说明                             |
| 2023-12-29 | V1.0.5  | 新增队列信息查询接口以及两个信息回调事件                      |
| 2024-01-30 | V1.0.6  | 新增许可报错，新增使用接听工具拦截SIP接听电话方式                |
| 2024-02-19 | V1.0.7  | 去除操作告警拦截逻辑                                |
| 2024-05-28 | V1.0.8  | 兼容支持webSocketUrl的完整地址传输                   |
| 2024-08-14 | V1.0.9  | 新增验证接听工具是否运行接口                            |
| 2024-12-25 | V1.0.10 | 新增座席加入队列和座席退出队列反馈事件                       |
| 2025-02-12 | V1.0.11 | 新增外呼通过分机注册网关的中转方式呼叫到手机                    |
| 2025-03-06 | V1.1.0  | 新增webrtc支持                                |

### 目录

1. 概述
2. 使用说明
    - 2.1. 软件安装和配置
    - 2.2. 开发集成与配置
        - 2.2.1. 集成方式
        - 2.2.2. 纯JS开发集成
        - 2.2.3. Vue开发集成
        - 2.2.4. Webrtc开发集成
    - 2.3. 运行和调试
        - 2.3.1. 开发所需外部端口
        - 2.3.2. 外线拨打规则
        - 2.3.3. 运行纯JS
    - 2.4. 问题报告
3. API接口定义
    - 3.1. AgentControl API接口
        - 3.1.1. 登录login()
        - 3.1.2. 登出 logout()
        - 3.1.3. 设置座席状态 setStatus()
        - 3.1.4. 班长席按钮状态控制
        - 3.1.5. 强制置忙 forceBusy()
        - 3.1.6. 强制置闲 forceFree()
        - 3.1.7. 强制退出 forceLogout()
        - 3.1.8. 监听 forceListen()
        - 3.1.9. 强插 forceConference()
        - 3.1.10. 耳语 forceWhisper()
        - 3.1.11. 强制挂断 forceHangup()
        - 3.1.12. 强制拦截 forceIntercept()
        - 3.1.13. 座席工号临时加入某队列 tierOn()
        - 3.1.14. 座席工号临时移除某队列 tierOff()
        - 3.1.15. 获取队列信息 queueDetail()
    - 3.2. CallControl API接口
        - 3.2.1. 语音呼出 makeCall()
        - 3.2.2. 挂机 hangup()
        - 3.2.3. 呼叫保持 hold()
        - 3.2.4. 呼叫取消保持 unHold()
        - 3.2.5. 发起磋商 consult()
        - 3.2.6. 取消磋商 retrieve()
        - 3.2.7. 呼叫转移completeTransfer()
        - 3.2.8. 三方会话completeThreeParties()
        - 3.2.9. 单步呼叫转移(座席) singleStepTransfer()
        - 3.2.10. 单步呼叫转移(外线) singleStepTransferPhone()
        - 3.2.11. 单步转技能组 transferAgentGroup()
        - 3.2.12. 满意度评分 agentEvaluate()
        - 3.2.13. 静音mute()
        - 3.2.14. 呼叫取消静音 unMute()
        - 3.2.15. 代接 proxyAnswer()
        - 3.2.16. 外呼时发送DTMF
        - 3.2.17. 显示视频 showCustomerViewer()
        - 3.2.18. 关闭视频 closeCustomerViewer()
        - 3.2.19. 预开启桌面推流 publishLocalStream()
        - 3.2.20. 开始桌面推流 startPushVideoStream()
        - 3.2.21. 停止桌面推流 closeLocalStream()
        - 3.2.22. 主动请求视频模式 requestVideoMode()
        - 3.2.23. 主动请求音频模式 requestAudioMode()
        - 3.2.24. 开启媒体文件推送 startPushMediaFile()
        - 3.2.25. 关闭推送 stopPushMedia()
        - 3.2.26. 变音:粗 voiceChangeThick()
        - 3.2.27. 变音:细 voiceChangeThin()
        - 3.2.28. 接听 answer()
        - 3.2.29. 验证接听工具是否启动 checkAnswerTool()
        - 3.2.30. 分机注册网关单步语音外呼singleStepTransferErgtoc()
        - 3.2.31. 分机注册网关语音外呼 makeCallRegGWExtNo()
4. API事件定义
    - 4.1. AgentControl事件定义
        - 4.1.1. 登录成功回调 setAgentLoginSuccessEventListener()
        - 4.1.2. 登录失败回调 setAgentLoginFailedEventListener()
        - 4.1.3. 登出回调 setAgentLogoutEventListener()
        - 4.1.4. 座席状态回调 setAgentStatusChangeEventListener()
        - 4.1.5. 队列等待回调 setMembersCountEventListener()
        - 4.1.6. 班长电话切入回调setMonitorForceActionAnsweredEvent()
        - 4.1.7. 错误消息回调 setAgentErrorEventListener()
        - 4.1.8. 获取队列列表回调 setAllQueueListEventListener
        - 4.1.9. 获取队列座席对照回调 setAllQueueAgentMapListener
        - 4.1.10. 座席加入队列回调事件setTierOnResultEventListener
        - 4.1.11. 座席退出队列回调事件setTierOffResultEventListener
    - 4.2. CallControl事件定义
        - 4.2.1. 呼入振铃回调 setChannelRingingEventListener()
        - 4.2.2. 呼出振铃回调 setChannelRingBackEventListener()
        - 4.2.3. 被呼叫接通后回调 setChannelThisPartyAnsweredEventListener()
        - 4.2.4. 呼叫接通后回调 setChannelOtherPartyAnsweredEventListener()
        - 4.2.5. 呼叫挂断回调 setChannelDestroyEventListener()
        - 4.2.6. 呼叫保持回调 setChannelHoldEventListener()
        - 4.2.7. 呼叫恢复回调 setChannelUnHoldEventListener()
        - 4.2.8. 磋商事件回调 setConsultEventListener()
        - 4.2.9. 磋商外线挂机取回回调 setOtherPartyCanceledEventListener()
        - 4.2.10. 进入三方会话回调 setRecvDtmfEventListener()
        - 4.2.11. 提示被呼客服正通话中回调setAgentOnCallEventListener()
        - 4.2.12. 呼叫静音回调 setChannelMuteEventListener()
        - 4.2.13. 呼叫恢复静音回调 setChannelUnMuteEventListener()
        - 4.2.14. 视频流播放错误回调 setVideoStreamErrorListener()
        - 4.2.15. 通话语音识别翻译回调 setDetectSpeechEventListener()
        - 4.2.16. 推流准备反馈 setPublishStreamPrepareEventListener()
        - 4.2.17. 通知开始视频推流 setStartVideoPushEventListener()
        - 4.2.18. 通知结束视频推流 setStopVideoPushEventListener()
        - 4.2.19. 变音反馈事件setVoiceChangeEventListener()

## 1. 概述

本Agent SDK是座席端开发包，使用TypeScript或JavaScript集成座席控制和呼叫控制功能。接口功能分类如下：

| 接口分类           | 接口说明                                                                         |
|:---------------|:-----------------------------------------------------------------------------|
| 全局函数接口         | setMode()设置开发或生产模式(production/development) createXXXXControl()为创建下述两处接口的工厂方法 |
| AgentControl接口 | 座席登录、登出、状态控制、班长席操作及回调                                                        |
| CallControl接口  | 呼叫控制功能及回调                                                                    |

Agent SDK和服务端采用WebSocket长连接，由Agent SDK内部的AccessWebSocketClient类实现。注意，F5键或调用浏览器接口刷新整个页面会导致WebSocket断连，因此Agent
SDK支持的是Vue/React等框架的Single Page Application架构。

Agent SDK包含以下内容：
a. doc: API接口文档
b. demo.zip: 使用Agent SDK包的纯JS演示项目
c. MicroSIP-3.21.2.exe: SIP软电话安装文件
SIP官方下载地址: https://www.microsip.org/downloads

## 2. 使用说明

### 2.1 软电话安装和配置

安装测试SIP软电话

1. 安装MicroSIP，如图配置MicroSIP，具体账号和密码可联系管理员
   (此处有图片，显示MicroSIP配置界面)

### 2.2 开发集成与配置

#### 2.2.1 集成方式

座席用户界面开发项目如果使用`<script>`标签导入Agent SDK，请使用2.2.2节的方式，`vc_agent_api.js`
的格式是UMD，符合ES2015规范，请参见ES2015浏览器支持。

#### 2.2.2 纯JS开发集成

Agent SDK的Javascript包在demo.zip内的js子目录中，请使用`<script>`标签在HTML中引入。

以JS Demo为例，说明集成步骤如下：

1. 在index.html中导入SDK包
   `<script src="./js/vc_agent_api.js"></script>`

2. 在callback.js的初始化函数initialize()调用全局设置
   `VoiceCommAPI.setMode("production");`
   `VoiceCommAPI.setConfigureServer(config);`

3. 配置(config)相关描述

```javascript
{
    websocketHost: "127.0.0.1", //socket host
        websocketPort
:
    9058, //socket port
        websocketSSL
:
    false, //socket ssl
        webSocketUrl
:
    "ws://127.0.0.1:9058",//完整地址的传输方式，建议使用这种方式，webSocketHost、webSocketPort、webSocketSSL某些特殊地址的拼接不兼容
        domain
:
    "172.16.20.181", //domain地址
        trunkNumber
:
    "+862160568356", //外显号码
        gateway
:
    "dfdea3e4-ba6c-4398-8b9d-781cc71dbb98", //外显号码网关
        outno
:
    "98", //外显号码拨号前缀
        isAfterCall
:
    false, //是否话后处理 true/false (该状态最好需要班长和成员是配置一致，否则在做强制置闲会出现问题)
        traceMsgDebug
:
    true, //API内部收发消息打印
        isDetectSpeech
:
    true, //是否使用语音识别翻译 true/false
        videoPublishUrl
:
    "http://172.17.23.206:8080/webrtc/push/live_206/", //桌面推流地址(需monibuca服务)
        pushMediaUrl
:
    "http://172.17.23.205:9898/upload", //媒体文件上传地址(需http_file服务)
        answerToolUrl
:
    "http://127.0.0.1:9999/answer" //使用接听工具服务时需配置该配置
}
```

4. 在callback.js中使用工厂方法创建接口
   `var agent = VoiceCommAPI.createAgentControl();`
   `var call = VoiceCommAPI.createCallControl();`

5. 在callback.js的setListners()中设置所需的回调函数，请参见”4. API事件定义”章节
   `agent.setAgentLoginSuccessEventListener(...)`
   等等，演示程序将收到的事件在浏览器console输出并限制当前状态下的操作

6. UI使用agent或call调用接口方法，请参见”3. API接口定义”章节。
   `agent.login(...);`

7. 3.2节说明的操作控制体现在enableOperations函数中，复用该段代码逻辑的前提是对应操作按钮在HTML中的id属性必须和本演示程序保持一致。

#### 2.2.3 Vue开发集成

以Vue为例说明安装和配置过程如下：

1. 在Vue项目使用npm安装Agent SDK:
   `npm i vcswitch-agent-api -S`

以Vue演示项目为例，说明集成步骤如下：

1. 在main.js中导入SDK包
   `import * as VoiceCommAPI from "vcswitch-agent-api";`

2. 在main.js调用全局设置
   `VoiceCommAPI.setConfigureServer(config);`
   `VoiceCommAPI.setMode("production");`

3. 在main.js中使用工厂方法创建接口
   `const agent = VoiceCommAPI.createAgentControl();`
   `const call = VoiceCommAPI.createCallControl();`

4. 在main.js中设置所需的回调函数，请参见”4. API事件定义”章节
   `agent.setAgentLoginSuccessEventListener(...)`
   等等，演示程序将收到的事件在浏览器console输出

5. 将接口对象绑定到Vue全局属性，供UI调用接口方法，请参见”3. API接口定义”章节
   `app.config.globalProperties.$agent = agent;`
   `app.config.globalProperties.$call = call;`

#### 2.2.4 Webrtc开发集成

版本V1.1.0开始支持webrtc，无需再使用MicroSIP软件（不可共用）。除沿用2.2.2的配置之外，使用webrtc还需要新增对应的配置项，如下：

```javascript
{
    deviceType: "webphone", //选择模式，分为webphone（webrtc方式）和softphone（SIP软件+web方式）两种模式，默认是softphone
        isVideo
:
    false, //是否使用视频通话，默认false，建议使用false，视频通话未能充分验证可行性。
        isAutoAnswer
:
    false, //是否自动接听，默认false
        sipHost
:
    "172.20.12.205", //同domain
        sipPort
:
    "7443", //sip端口号
        webrtcWssUrl
:
    "wss://volte.voicecomm.cn:7443", //webrtc连接需使用wss域名:7443
}
```

其他注意事项：
a. webrtc需要安全的网络环境，本地测试使用localhost方式开发测试，正式访问需要https域名方式。
b. 若本机安装有虚拟网卡/vpn，请卸载/禁用/关闭，否则会存在呼叫延迟等情况，导致呼叫失败！

### 2.3 运行和调试

#### 2.3.1 开发所需外部端口

如果在公司内网开发，需访问以下外网端口：

| 端口    | 协议      | 用途                 |
|:------|:--------|:-------------------|
| 15060 | udp、tcp | SIP电话，端口可能不同请咨询    |
| 9058  | tcp     | websocket连接访问服务器端口 |

#### 2.3.2 外线拨打规则

通过Agent SDK拨打外线，本地号码请直接拨打，外地号码可能需加前导”0”。
使用microSIP软电话直接拨打需加前导”9”。按配置需要进行修改

#### 2.3.3 运行纯JS

1. 打开SIP软电话并注册成功
2. 打开index.html
3. 点击”登录”按钮，输入用户名，用户名为MicroSIP账户名，密码请使用缺省值
4. 注意不勾选”是否为班长”
5. F12打开浏览器console。

### 2.4 问题报告

Agent SDK在集成开发时遇到问题需诊断时，请提供以下内容：
a. 发生错误的具体时间
b. 错误情况简述
c. 从浏览器console导出的console.txt
d. 导出CTI服务日志,访问服务器安装目录/log子目录下的日志文件，如app.log。

## 3. API接口定义

### 3.1 AgentControl API接口

#### 3.1.1 登录login()

登录接口指的是座席登录SIP分机，一般地在接入本系统的接入方在登录自己的业务系统成功后再调用本接口。

| 参数名称          | 字段类型        | 说明                                                | 示例值                               |
|:--------------|:------------|:--------------------------------------------------|:----------------------------------|
| agentID       | 字符串         | 座席工号                                              | 8001                              |
| device        | 字符串         | SIP分机(注意本系统支持工号和分机的分离登录，适应三班倒坐席安排需求)              | 8002                              |
| password      | 字符串         | SIP分机密码，一般是”AbC@321”,本接口不验证密码,但在接入的时候需要象征性的填写非空字符 | AbC@321                           |
| belong_queues | 数组          | 所属技能组的数组，每个对象类型定义如下：                              | [{queue_id:” ”,is_monitor:false}] |
| queue_id      | 字符串         | 登录座席所属的技能组号,非必填 传空串①                              | “”                                |
| is_monitor    | 布尔          | 登录座席是否该技能组的班长                                     | false                             |
| peerAudio     | HTMLElement | 获取另一端音视频元素                                        | document.getElementById(id)       |
| localAudio    | HTMLElement | 获取本端音视频元素                                         | document.getElementById(id)       |

**触发事件**

| 事件   | 设置回调                   | 说明                                |
|:-----|:-----------------------|:----------------------------------|
| 登录成功 | AgentLoginSuccessEvent | setAgentLoginSuccessEventListener | 详见4.1.1 |
| 登录失败 | AgentLoginFailedEvent  | setAgentLoginFailedEventListener  | 详见4.1.2 |

①技能组暂时不需要在登录时传递，可传递空串。若客服是班长则需要将is_monitor赋true。

注意：V1.1.0新增peerAudio、localAudio两video元素传递（非必填），目的为了适配webrtc，html中新增`<video>`标签示例如下：
`<audio id="local" style="display: none;" controls></audio>`
`<audio id="peer" style="display: none;" controls></audio>`

#### 3.1.2 登出 logout()

座席登出指断开WebSocket连接并在系统中将座席状态设置成`logged out`。

#### 3.1.3 设置座席状态 setStatus()

| 字段名称   | 字段类型 | 说明                                                                                                                                                                                                                                                                                                                                                                                                                                           | 示例值 |
|:-------|:-----|:---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|:----|
| status | 常量枚举 | STATUS_READY 就绪(Available(On Demand)), 即”0” <br> STATUS_NOT_READY 未就绪, 即”1” <br> STATUS_AFTER_ACW 呼入话后处理, 即”2” <br> STATUS_REST 休息, 即”3” <br> STATUS_DINNER 就餐, 即”4” <br> STATUS_TRAINING 培训, 即”5” <br> STATUS_AFTER_WORK 呼出话后处理, 即”9” <br> STATUS_LOGOUT 登出, 即”10” <br> STATUS_LOGIN 登录, 即”11” <br> STATUS_TALKING 通话中, 即”12” <br> STATUS_DIALING 呼出振铃, 即”13” <br> STATUS_RINGING 呼入振铃, 即”14” <br> STATUS_INCALL_READY 就绪(Available), 即”16” | 1   |

注意，STATUS_DIALING、STATUS_RINGING、STATUS_TALKING、STATUS_AFTER_ACW、STATUS_AFTER_WORK页面需根据事件动态的变更状态显示，详见demo

| 事件   | 设置回调                   | 说明                                |
|:-----|:-----------------------|:----------------------------------|
| 状态变化 | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

#### 3.1.4 班长席按钮状态控制

班长权限各状态下允许的操作集合定义如下：

| 状态         | 操作名称                                                                                                                                                                                  |
|:-----------|:--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 班长登录空闲/忙碌时 | force_busy 强制置忙, <br> force_free 强制置闲, <br> force_logout 强制登出, <br> force_listen 监听, <br> force_whisper 耳语, <br> force_conference 强插, <br> force_intercept 拦截, <br> force_hangup 强制挂机 |
| 班长振铃中      | /                                                                                                                                                                                     |
| 班长通话中      | force_hangup 强制挂机                                                                                                                                                                     |

注：客服功能的操作按钮权限集合详见3.2 CallControl API接口

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.5 强制置忙 forceBusy()

班长强制将组内某个座席的状态设置为忙碌，即未就绪。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调                   | 说明                                |
|:-----|:-----------------------|:----------------------------------|
| 状态变化 | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**本文档的班长功能是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.6 强制置闲 forceFree()

班长强制将组内某个座席的状态设置为空闲，即就绪。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调                   | 说明                                |
|:-----|:-----------------------|:----------------------------------|
| 状态变化 | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.7 强制退出 forceLogout()

班长强制将组内某个座席的状态设置为登出。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调                   | 说明                                |
|:-----|:-----------------------|:----------------------------------|
| 状态变化 | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |
| 座席登出 | AgentLogoutEvent       | setAgentLogoutEventListener       | 详见4.1.3 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.8 监听 forceListen()

班长席发起对组内座席的监听。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调         | 说明                             |
|:-----|:-------------|:-------------------------------|
| 被呼振铃 | RingingEvent | setChannelRingingEventListener | 详见4.2.1 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.9 强插 forceConference()

班长席发起对组内座席的强插，即班长席加入和客户的三方通话。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调         | 说明                             |
|:-----|:-------------|:-------------------------------|
| 被呼振铃 | RingingEvent | setChannelRingingEventListener | 详见4.2.1 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.10 耳语 forceWhisper()

班长席发起对组内座席的耳语，即班长的话语仅座席能听见、客户是听不见的。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件   | 设置回调         | 说明                             |
|:-----|:-------------|:-------------------------------|
| 被呼振铃 | RingingEvent | setChannelRingingEventListener | 详见4.2.1 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.11 强制挂断 forceHangup()

班长席发起对组内座席的强制挂机，注意：该操作将引起座席和客户间通话的立即中断。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件               | 设置回调                   | 说明                                |
|:-----------------|:-----------------------|:----------------------------------|
| AgentDevice 通话挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化             | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.12 强制拦截 forceIntercept()

班长席发现对组内座席通话异常就将座席对端的电话拦截过来，注意：该操作将引起座席和客户间通话的立即中断，班长席将与客户进行通话。

| 字段名称        | 字段类型 | 说明     | 示例值  |
|:------------|:-----|:-------|:-----|
| AgentDevice | 字符串  | 组内座席分机 | 8002 |

**触发事件**

| 事件               | 设置回调                   | 说明                                |
|:-----------------|:-----------------------|:----------------------------------|
| AgentDevice 通话挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化             | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**本文档的班长功能只是验证性质的，座席SDK API不包含成员列表的状态机制，故要真正实现班长功能需要接入监控SDK API接口文档。**

#### 3.1.13 座席工号临时加入某队列 tierOn()

在业务繁忙时，可灵活分配座席所属队列。

| 字段名称        | 字段类型 | 说明   | 示例值    |
|:------------|:-----|:-----|:-------|
| peerAgentID | 字符串  | 座席工号 | 8001   |
| queue       | 字符串  | 队列号  | 888888 |

**触发事件**

| 事件   | 设置回调              | 说明                           |
|:-----|:------------------|:-----------------------------|
| 队列列表 | TierOnResultEvent | setTierOnResultEventListener | 详见座席加入队列回调事件setTierOnResultEventListener |

#### 3.1.14 座席工号临时移除某队列 tierOff()

在业务繁忙时，可灵活分配座席所属队列。

| 字段名称        | 字段类型 | 说明   | 示例值    |
|:------------|:-----|:-----|:-------|
| peerAgentID | 字符串  | 座席工号 | 8001   |
| queue       | 字符串  | 队列号  | 888888 |

**触发事件**

| 事件   | 设置回调               | 说明                            |
|:-----|:-------------------|:------------------------------|
| 队列列表 | TierOffResultEvent | setTierOffResultEventListener | 详见座席退出队列回调事件setTierOffResultEventListener |

#### 3.1.15 获取队列信息 queueDetail()

查询队列座席分配信息

| 字段名称        | 字段类型 | 说明   | 示例值    |
|:------------|:-----|:-----|:-------|
| peerAgentID | 字符串  | 座席工号 | 8001   |
| queue       | 字符串  | 队列号  | 888888 |

**触发事件(以下两事件都会触发)**

| 事件     | 设置回调                  | 说明                               |
|:-------|:----------------------|:---------------------------------|
| 队列列表   | AllQueueListEvent     | setAllQueueListEventListener     | 详见4.1.8 |
| 队列座席对应 | AllQueueAgentMapEvent | setAllQueueAgentMapEventListener | 详见4.1.9 |

### 3.2 CallControl API接口

CallControl接口用于呼叫控制，注意呼叫操作仅能在特定的状态下发起，呼叫控制事件的allow字段说明在当前状态下允许的操作集合，操作名称定义如下：

| 操作名称                       | 调用函数名                     | 说明         |
|:---------------------------|:--------------------------|:-----------|
| hangup                     | hangup()                  | 挂断         |
| make                       | makeCall()                | 外呼         |
| proxyAnswer                | proxyAnswer()             | 代接         |
| hold                       | hold()                    | 保持③        |
| unhold                     | unHold()                  | 取消保持       |
| mute                       | mute()                    | 静音④        |
| unmute                     | unMute()                  | 取消静音       |
| consult                    | consult()                 | 发起磋商       |
| cancel_consult             | retrieve()                | 取消磋商       |
| complete_transfer          | completeTransfer()        | 呼叫转移       |
| complete_three_parties     | completeThreeParties()    | 三方会话       |
| single_step_transfer       | singleStepTransfer()      | 单步呼叫转移(座席) |
| single_step_transfer_phone | singleStepTransferPhone() | 单步呼叫转移(外呼) |
| transfer_agent_group       | transferAgentGroup()      | 转技能组       |
| agent_evaluate             | agentEvaluate()           | 客户评分       |
| /                          | sendDTMF()                | 外呼时发送DTMF  |

③保持: 客户会播放等待音乐，座席和客服互相不能听见对方声音
④静音: 客户听不到座席声音，座席能收到客户说话声音

各状态下允许的操作集合定义如下：

| 状态   | 操作名称                                                                                                              |
|:-----|:------------------------------------------------------------------------------------------------------------------|
| 振铃中  | hangup                                                                                                            |
| 通话中  | hold, mute, consult, single_step_transfer, single_step_transfer_phone,transfer_agent_group,agent_evaluate, hangup |
| 磋商呼入 | hangup                                                                                                            |
| 保持   | unhold                                                                                                            |
| 静音   | unmute                                                                                                            |
| 发起磋商 | cancel_consult                                                                                                    |
| 磋商中  | cancel_consult, complete_transfer, complete_three_parties                                                         |
| 三方会话 | hangup                                                                                                            |
| 挂机后  | make,proxyAnswer,status,logout                                                                                    |

请使用上述允许的操作集合限制UI界面的操作按钮，错误使用按钮调用的方法会报相应限制错误提示。注：班长功能的操作集合详见3.1.4
班长席按钮状态控制

**注：以上操作功能的罗列可能会有疏漏，详见各事件真实返回**

#### 3.2.1 语音呼出 makeCall()

呼出电话，若无其他需要，默认以配置项中的外显号码、网关、拨号前缀进行呼出

| 字段名称        | 字段类型 | 说明                                        | 示例值                                  |
|:------------|:-----|:------------------------------------------|:-------------------------------------|
| target      | 字符串  | 呼出电话号码，可以是外线或内线分机号码                       | 18122223333                          |
| trunkNumber | 字符串  | 可为空，若不满足配置里设置的外显号码，需要客服选择输入则可自行输入其他外显号码呼叫 | +862160568356                        |
| gateway     | 字符串  | 可为空，当前外显号码的网关                             | dfdea3e4-ba6c-4398-8b9d-781cc71dbb98 |
| outno       | 字符串  | 可为空，当前外显号码的拨号前缀                           | 98                                   |

**触发事件**

| 事件    | 设置回调                   | 说明                              |
|:------|:-----------------------|:--------------------------------|
| 对方振铃时 | AgentCallRingBackEvent | setChannelRingBackEventListener | 详见4.2.2 |

#### 3.2.2 挂机 hangup()

技术上座席端使用API可以在任何情况下调用hangup()
挂机，但从座席操作规范一般地由客户主动挂机或者由座席发起对服务进行评分，座席一般不先挂机。无论是客户还是座席挂机后，座席端都会收到AgentCallHangupEvent。

**触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

#### 3.2.3 呼叫保持 hold()

座席仅能在通话时发起呼叫保持，注意三方通话时无法发起呼叫保持操作。发起呼叫保持后会收到AgentCallHoldEvent。

**触发事件**

| 事件    | 设置回调               | 说明                          |
|:------|:-------------------|:----------------------------|
| 呼叫已保持 | AgentCallHoldEvent | setChannelHoldEventListener | 详见4.2.6 |

#### 3.2.4 呼叫取消保持 unHold()

座席仅能在呼叫保持时发起呼叫取消保持，呼叫取消保持后会收到AgentCallUnHoldEvent。

**触发事件**

| 事件    | 设置回调                 | 说明                            |
|:------|:---------------------|:------------------------------|
| 呼叫已恢复 | AgentCallUnHoldEvent | setChannelUnHoldEventListener | 详见4.2.7 |

#### 3.2.5 发起磋商 consult()

| 字段名称        | 字段类型 | 说明                                        | 示例值                                  |
|:------------|:-----|:------------------------------------------|:-------------------------------------|
| target      | 字符串  | 磋商电话号码，支持座席工号和外线号码                        | 18122223333                          |
| callData    | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换成字符串传递         | { “a” :1,” b” :2}                    |
| trunkNumber | 字符串  | 可为空，若不满足配置里设置的外显号码，需要客服选择输入则可自行输入外显号码进行呼叫 | +862160568356                        |
| gateway     | 字符串  | 可为空，外显号码的网关                               | dfdea3e4-ba6c-4398-8b9d-781cc71dbb98 |
| outno       | 字符串  | 可为空，外显号码的拨号前缀                             | 98                                   |

目前返回值总是true，可忽略。
座席仅能在通话时发起磋商，注意三方通话时无法发起磋商操作。磋商会收到AgentCallConsultEvent。
磋商有三种情况反馈，事件返回字段为Reason：
Hungup磋商挂断、Success磋商成功、Fail磋商失败
磋商成功后座席可以进行取消磋商retrieve()、呼叫转移compleete_transfer()或三方会话complete_conference()操作。

**触发事件**

| 事件   | 设置回调                  | 说明                      |
|:-----|:----------------------|:------------------------|
| 磋商成功 | AgentCallConsultEvent | setConsultEventListener | 详见4.2.8 |

#### 3.2.6 取消磋商 retrieve()

目前返回值总是true，可忽略。
座席仅能在通话时发起磋商，取消磋商后会收到AgentCallConsultEvent回到通话状态。

**触发事件**

| 事件   | 设置回调                  | 说明                      |
|:-----|:----------------------|:------------------------|
| 磋商挂断 | AgentCallConsultEvent | setConsultEventListener | 详见4.2.8 |

若磋商外线，外线拒接等超时会收到OtherPartyCanceledEvent事件

| 事件   | 设置回调                    | 说明                                 |
|:-----|:------------------------|:-----------------------------------|
| 磋商拒接 | OtherPartyCanceledEvent | setOtherPartyCanceledEventListener | 详见4.2.9 |

#### 3.2.7 呼叫转移completeTransfer()

目前返回值总是true，可忽略。
座席仅能在磋商成功后发起呼叫转移，呼叫转移后电话被转移给磋商方，座席会收到AgentCallHangupEvent。

**触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

#### 3.2.8 三方会话completeThreeParties()

目前返回值总是true，可忽略。
座席仅能在磋商成功后发起三方会话，座席会收到RecvDtmfEvent。注意三方会话状态不同于普通通话状态，无法进行呼叫保持、发起磋商等操作，只能挂机或由其他方挂机离开三方会话；

**触发事件**

| 事件     | 设置回调          | 说明                       |
|:-------|:--------------|:-------------------------|
| 建立三方会话 | RecvDtmfEvent | setRecvDtmfEventListener | 详见4.2.10 |

#### 3.2.9 单步呼叫转移(座席) singleStepTransfer()

| 字段名称     | 字段类型 | 说明                                | 示例值               |
|:---------|:-----|:----------------------------------|:------------------|
| target   | 字符串  | 转移座席号码                            | 8003              |
| callData | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换成字符串传递 | { “a” :1,” b” :2} |

目前返回值总是true，可忽略。
座席仅能在通话时发起单步呼叫转移。单步呼叫转移发起后原座席会先挂机，然后程序会将客户和另一个客服呼叫桥接起来。

**本端触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**对端客服触发事件**

| 事件   | 设置回调         | 说明                      |
|:-----|:-------------|:------------------------|
| 被呼振铃 | RingingEvent | setRingingEventListener | 详见4.2.1 |

#### 3.2.10 单步呼叫转移(外线) singleStepTransferPhone()

| 字段名称        | 字段类型 | 说明                                        | 示例值                                  |
|:------------|:-----|:------------------------------------------|:-------------------------------------|
| target      | 字符串  | 转移外线号码                                    | 18122223333                          |
| callData    | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换成字符串传递         | { “a” :1,” b” :2}                    |
| trunkNumber | 字符串  | 可为空，若不满足配置里设置的外显号码，需要客服选择输入则可自行输入外显号码进行呼叫 | +862160568356                        |
| gateway     | 字符串  | 可为空，外显号码的网关                               | dfdea3e4-ba6c-4398-8b9d-781cc71dbb98 |
| outno       | 字符串  | 可为空，外显号码的拨号前缀                             | 98                                   |

目前返回值总是true，可忽略。
座席仅能在通话时发起单步呼叫转移。单步呼叫转移发起后原座席会先挂机，然后程序会将客户和另一个外线号码呼叫桥接起来。

**本端触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

#### 3.2.11 单步转技能组 transferAgentGroup()

| 字段名称     | 字段类型 | 说明                                | 示例值               |
|:---------|:-----|:----------------------------------|:------------------|
| target   | 字符串  | 转移技能组                             | 888888            |
| callData | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换成字符串传递 | { “a” :1,” b” :2} |

目前返回值总是true，可忽略。
座席仅能在通话时发起单步呼叫转移。单步呼叫转移发起后原座席会先挂机，然后程序会将客户和另一个技能组的随机客服桥接起来。

**本端触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**对端客服触发事件**

| 事件   | 设置回调         | 说明                      |
|:-----|:-------------|:------------------------|
| 被呼振铃 | RingingEvent | setRingingEventListener | 详见4.2.1 |

#### 3.2.12 满意度评分 agentEvaluate()

| 字段名称     | 字段类型 | 说明                                | 示例值               |
|:---------|:-----|:----------------------------------|:------------------|
| callData | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换成字符串传递 | { “a” :1,” b” :2} |

座席在结束对客户的服务后，可以邀请客户对本次服务进行满意度评分。满意度评分实际上是将呼叫转移到某个评分的自助服务流程，由客户按语音菜单提示对座席的服务进行满意度打分。

**触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

#### 3.2.13 静音mute()

座席仅能在通话时发起静音，注意三方通话时无法发起静音操作。发起静音后会收到AgentCallMuteEvent。

**触发事件**

| 事件    | 设置回调               | 说明                          |
|:------|:-------------------|:----------------------------|
| 呼叫已静音 | AgentCallMuteEvent | setChannelMuteEventListener | 详见4.2.12 |

#### 3.2.14 呼叫取消静音 unMute()

座席仅能在呼叫静音时发起呼叫取消静音，呼叫取消静音后会收到AgentCallUnMuteEvent。

**触发事件**

| 事件    | 设置回调                 | 说明                            |
|:------|:---------------------|:------------------------------|
| 呼叫已恢复 | AgentCallUnMuteEvent | setChannelUnMuteEventListener | 详见4.2.13 |

#### 3.2.15 代接 proxyAnswer()

| 字段名称   | 字段类型 | 说明   | 示例值  |
|:-------|:-----|:-----|:-----|
| target | 字符串  | 座席分机 | 8002 |

座席发现其他座席振铃却因故不能接听，可改座席分机号进行代接电话。

**触发事件**

| 事件   | 设置回调         | 说明                      |
|:-----|:-------------|:------------------------|
| 被呼振铃 | RingingEvent | setRingingEventListener | 详见4.2.1 |
| 状态变化 |              |                         | 详见4.1.4 |

#### 3.2.16 外呼时发送DTMF

使用场景：客服呼出到外线座机遇到需要输入分机等情况

| 字段名称   | 字段类型 | 说明       | 示例值  |
|:-------|:-----|:---------|:-----|
| target | 字符串  | dtmf拨号信息 | 5623 |

#### 3.2.17 显示视频 showCustomerViewer()

该接口用于视频媒体的显示：

| 字段名称      | 字段类型 | 说明     | 示例值       |
|:----------|:-----|:-------|:----------|
| elementId | 字符串  | 视频窗口id | container |
| href      | 字符串  | 视频地址   | http://#@ |

**注：该接口内置于API，若不想使用内置的视频控件，用户在接入获取视频流URL之后可自行选择其他合适的控件去显示视频。**

#### 3.2.18 关闭视频 closeCustomerViewer()

该接口用于视频媒体的关闭，关闭`<video>`控件的视频显示：

| 字段名称      | 字段类型 | 说明     | 示例值       |
|:----------|:-----|:-------|:----------|
| elementId | 字符串  | 视频窗口id | container |

**注：该接口内置于API，若不想使用内置的视频控件，用户在接入获取视频流URL之后可自行选择其他合适的控件去显示视频。**

#### 3.2.19 预开启桌面推流 publishLocalStream()

该接口用于预备本地桌面推流和显示，建立和monibuca流服务器的推流连接，依赖配置项videoPublishUrl:

| 字段名称      | 字段类型 | 说明     | 示例值       |
|:----------|:-----|:-------|:----------|
| elementId | 字符串  | 视频窗口id | container |

**注：该接口内置于API，若不想使用内置的视频控件，用户在接入获取视频流URL之后可自行选择其他合适的控件去显示视频。该接口正在实验中,不建议接入
**

**触发事件**

| 事件     | 设置回调                      | 说明                                   |
|:-------|:--------------------------|:-------------------------------------|
| 推流预备事件 | PublishStreamPrepareEvent | setPublishStreamPrepareEventListener | |

#### 3.2.20 开始桌面推流 startPushVideoStream()

该接口将桌面rtmp流推送到客户终端：

| 字段名称       | 字段类型 | 说明  | 示例值                                                                                       |
|:-----------|:-----|:----|:------------------------------------------------------------------------------------------|
| streamPath | 字符串  | 流路径 | 使用PublishStreamPrepareEvent事件stream_path字段的返回，如：20230920/AoiaiQoAxjHW3rvc1XLlk6kCEjxgDgVl |

#### 3.2.21 停止桌面推流 closeLocalStream()

该接口用于本地视频rtmp流关闭，关闭`<video>`控件的视频显示：

| 字段名称      | 字段类型 | 说明     | 示例值       |
|:----------|:-----|:-------|:----------|
| elementId | 字符串  | 视频窗口id | container |

**注：该接口内置于API，若不想使用内置的视频控件，用户在接入获取视频流URL之后可自行选择其他合适的控件去显示视频。该接口正在实验中,不建议接入
**

#### 3.2.22 主动请求视频模式 requestVideoMode()

当通话是音频模式时，客服主动请求客户切换成视频模式：

**注：若当前已经是视频模式，再调用本接口将会收到以下报错事件：**

| 事件 | 设置回调            | 说明                         |
|:---|:----------------|:---------------------------|
| 报错 | AgentErrorEvent | setAgentErrorEventListener | 详见4.1.7 |

#### 3.2.23 主动请求音频模式 requestAudioMode()

当通话是视频模式时，客服主动请求客户切换成音频模式：

**注：若当前已经是音频模式，再调用本接口将会收到以下报错事件：**

| 事件 | 设置回调            | 说明                         |
|:---|:----------------|:---------------------------|
| 报错 | AgentErrorEvent | setAgentErrorEventListener | 详见4.1.7 |

#### 3.2.24 开启媒体文件推送 startPushMediaFile()

当通话是视频模式时，客服可用该接口推送一段mp4或者png图片：

| 字段名称      | 字段类型 | 说明     | 示例值                      |
|:----------|:-----|:-------|:-------------------------|
| localPath | 字符串  | 文件地址路径 | /home/app/media/test.mp4 |

**注：仅限于视频模式时使用该接口，仅支持MP4或PNG，调用该接口目前需要先将素材上传至vcswitch同服务器的文件服务系统，localPath参数支持上传文件后的服务器绝对路径，如：/home/app/media/test.mp4
也支持http访问地址传递。**
**文件服务传输服务接口为：http://###.###.##:####/upload,请求成功后结果会返回文件的绝对路径和htto访问路径可供选择。**

#### 3.2.25 关闭推送 stopPushMedia()

关闭客户端媒体画面显示，该方法适用于媒体文件推送的关闭以及rtmp推流的关闭

#### 3.2.26 变音:粗 voiceChangeThick()

将客服讲话声音变粗：

| 字段名称           | 字段类型 | 说明                                                                                      | 示例值    |
|:---------------|:-----|:----------------------------------------------------------------------------------------|:-------|
| frequencyKey   | 字符串  | 变音频率：<br> low 低频0.9 <br> middle 中频0.8 <br> high 高频0.7 <br> restore 归正 1 <br> custom 自定义 | middle |
| destType       | 字符串  | 变音对象：<br> Agent 客服 <br> Customer 客户                                                     | Agent  |
| value          | 字符串  | 可为空，若frequencyKey选择custom则必填                                                            | 0.72   |
| isChangeRecord | 字符串  | 是否需要支持录音文件变音（1是0否）                                                                      | 0      |

**触发事件**

| 事件 | 设置回调             | 说明                          |
|:---|:-----------------|:----------------------------|
| 报错 | VoiceChangeEvent | setVoiceChangeEventListener | 详见4.1.19 |

**注：变粗的自定义取值控制范围为(>=0.6,<1)，精度为2位小数**

#### 3.2.27 变音:细 voiceChangeThin()

将客服讲话声音变细：

| 字段名称           | 字段类型 | 说明                                                                                      | 示例值    |
|:---------------|:-----|:----------------------------------------------------------------------------------------|:-------|
| frequencyKey   | 字符串  | 变音频率：<br> low 低频1.1 <br> middle 中频1.3 <br> high 高频1.5 <br> restore 归正 1 <br> custom 自定义 | middle |
| destType       | 字符串  | 变音对象：<br> Agent 客服 <br> Customer 客户                                                     | Agent  |
| value          | 字符串  | 可为空，若frequencyKey选择custom则必填                                                            | 1.32   |
| isChangeRecord | 字符串  | 是否需要支持录音文件变音（1是0否）                                                                      | 0      |

**触发事件**

| 事件 | 设置回调             | 说明                          |
|:---|:-----------------|:----------------------------|
| 报错 | VoiceChangeEvent | setVoiceChangeEventListener | 详见4.1.19 |

**注：变细的自定义取值控制范围为(>1,<=2)，精度为2位小数**

#### 3.2.28 接听 answer()

当前配置使用softphone模式（SIP软件+web）时：
a. 使用SIP软件弹出的框进行按钮接听，此时本按钮接口点击不生效，可在页面隐藏，避免客服混淆。
b. 使用接听工具服务，模拟行使SIP软件弹出框的接听点击操作，此时可通过本接口进行接听。前提是需要在配置项中配置answerToolUrl。

当前配置使用webphone模式（webrtc）时（V1.1.0更新）：
a. 配置项配置成自动接听，呼入呼出自动接听。
b. 配置项配置成非自动接听，使用本接口在振铃后点击实现接听功能。

| 返回结果       |                      |
|:-----------|:---------------------|
| true/false | true:接听成功、false:接听失败 |

#### 3.2.29 验证接听工具是否启动 checkAnswerTool()

注：当使用接听工具时，可做轮询接口判断接听工具的实时运作情况。

| 返回结果       |                         |
|:-----------|:------------------------|
| true/false | true:工具稳定运行、false:工具未运行 |

**注：v1.0.9新增内容，配合接听小程序**

#### 3.2.30 分机注册网关单步语音外呼singleStepTransferErgtoc()

| 字段名称        | 字段类型 | 说明                                      | 示例值                                  |
|:------------|:-----|:----------------------------------------|:-------------------------------------|
| target      | 字符串  | 转移外线号码                                  | 18122223333                          |
| callData    | 字符串  | 可为空，自定义传输随路数据,若传递json格式需要转换字符传递         | { “a” :1,” b” :2}                    |
| trunkNumber | 字符串  | 可为空，若不满足配置里设置的外显号码，需要客服选择输入则可自行输入外显号码呼叫 | +862160568356                        |
| gateway     | 字符串  | 可为空，外显号码的网关                             | dfdea3e4-ba6c-4398-8b9d-781cc71dbb98 |
| outno       | 字符串  | 可为空，外显号码的拨号前缀                           | 98                                   |

目前返回值总是true，可忽略。
座席仅能在通话时发起单步呼叫转移。单步呼叫转移发起后原座席会先挂机，然后程序会将客户和另一个外线号码呼叫桥接起来。

**本端触发事件**

| 事件    | 设置回调                   | 说明                                |
|:------|:-----------------------|:----------------------------------|
| 呼叫已挂断 | AgentCallHangupEvent   | setChannelDestroyEventListener    | 详见4.2.5 |
| 状态变化  | AgentStatusChangeEvent | setAgentStatusChangeEventListener | 详见4.1.4 |

**V1.0.11新增功能**

#### 3.2.31 分机注册网关语音外呼 makeCallRegGWExtNo()

通过分机注册网关的中转方式呼叫到手机，若无其他需要，默认以配置项中的外显号码、网关、拨号前缀进行呼出

| 字段名称        | 字段类型 | 说明                                        | 示例值                                  |
|:------------|:-----|:------------------------------------------|:-------------------------------------|
| target      | 字符串  | 呼出电话号码，可以是外线或内线分机号码                       | 18122223333                          |
| trunkNumber | 字符串  | 可为空，若不满足配置里设置的外显号码，需要客服选择输入则可自行输入其他外显号码呼叫 | +862160568356                        |
| gateway     | 字符串  | 可为空，当前外显号码的网关                             | dfdea3e4-ba6c-4398-8b9d-781cc71dbb98 |
| outno       | 字符串  | 可为空，当前外显号码的拨号前缀                           | 98                                   |

**触发事件**

| 事件    | 设置回调                   | 说明                              |
|:------|:-----------------------|:--------------------------------|
| 对方振铃时 | AgentCallRingBackEvent | setChannelRingBackEventListener | 详见4.2.2 |

**V1.0.11新增功能**

## 4. API事件定义

以下是呼入和呼出流程状态图：
(此处为流程图图片)

### 4.1 AgentControl事件定义

事件中返回allow代表该事件后续可允许的操作，若返回的allow为空数组，则使用return语句跳出按钮设定逻辑。

#### 4.1.1 登录成功回调 setAgentLoginSuccessEventListener()

座席登录成功后回调该接口，其中LoginSuccessEvent定义如下：

| 字段名称      | 字段类型  | 说明                       |
|:----------|:------|:-------------------------|
| type      | 字符串   | 事件类型 “LoginSuccessEvent” |
| agent_id  | 字符串   | 座席工号                     |
| device    | 字符串   | 分机号                      |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表     |
| date_time | 字符串   | 事件时间                     |

#### 4.1.2 登录失败回调 setAgentLoginFailedEventListener()

座席登录失败后回调该接口，其中LoginSuccessEvent定义如下：

| 字段名称      | 字段类型 | 说明                      |
|:----------|:-----|:------------------------|
| type      | 字符串  | 事件类型 “LoginFailedEvent” |
| agent_id  | 字符串  | 座席工号                    |
| device    | 字符串  | 分机号                     |
| cause     | 字符串  | 失败原因                    |
| date_time | 字符串  | 事件时间                    |

#### 4.1.3 登出回调 setAgentLogoutEventListener()

状态改变后回调该接口，其中Logout定义如下：

| 字段名称      | 字段类型  | 说明                                                                                                                                                                                                                                 |
|:----------|:------|:-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| type      | 字符串   | 事件类型 “Logout”                                                                                                                                                                                                                      |
| agent_id  | 字符串   | 座席工号                                                                                                                                                                                                                               |
| device    | 字符串   | 分机号                                                                                                                                                                                                                                |
| cause     | 字符串   | 登出原因                                                                                                                                                                                                                               |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表                                                                                                                                                                                                               |
| code      | 数值常量  | WS_CLOSE_NORMAL=1000 正常登出 <br> WS_CLOSE_ABNORMAL=1006连接非正常关闭 <br> WS_CLOSE_SAME_AGENT=4802 同工号在别处登录已断连 <br> WS_CLOSE_BY_MONITOR=4803 已被班长席强制退出 <br> WS_CLOSE_BY_UNREG=4804 SIP话机注册失效已断连 <br> WS_CLOSE_SAME_DEVICE=4805 同分机在别处登录已断连 |
| date_time | 字符串   | 事件时间                                                                                                                                                                                                                               |

#### 4.1.4 座席状态回调 setAgentStatusChangeEventListener()

状态改变后回调该接口(在监控系统种班长席能收到组内其他座席的状态改变事件)，
其中AgentStatusEvent定义如下：

| 字段名称       | 字段类型  | 说明                                                       |
|:-----------|:------|:---------------------------------------------------------|
| type       | 字符串   | 事件类型 “AgentStatusEvent”                                  |
| agent_id   | 字符串   | 座席工号                                                     |
| device     | 字符串   | 分机号                                                      |
| isMntForce | 布尔    | 是否班长强制改变                                                 |
| status     | 字符串常量 | STATUS_READY 就绪(Available(On Demand)), 即”0” ... (同3.1.3) |
| allow      | 字符串数组 | 允许的操作，参见”allow字段说明”表                                     |
| date_time  | 字符串   | 事件时间                                                     |

#### 4.1.5 队列等待回调 setMembersCountEventListener()

用户来电能获取队列正在排队的数量，其中MembersCountEvent定义如下：

| 字段名称         | 字段类型 | 说明                       |
|:-------------|:-----|:-------------------------|
| type         | 字符串  | 事件类型 “MembersCountEvent” |
| agent_party  | 字符串  | 座席工号                     |
| queue        | 字符串  | 队列组                      |
| in_queue_num | 字符串  | 队列排队数量                   |
| date_time    | 字符串  | 事件时间                     |

#### 4.1.6 班长电话切入回调setMonitorForceActionAnsweredEvent()

班长对座席监听、强插、耳语时回调，其中MonitorForceActionAnsweredEvent定义如下：

| 字段名称         | 字段类型  | 说明                                                          |
|:-------------|:------|:------------------------------------------------------------|
| type         | 字符串   | 事件类型 “MonitorForceActionAnsweredEvent”                      |
| agent_id     | 字符串   | 座席工号                                                        |
| device       | 字符串   | 分机号                                                         |
| other_device | 字符串   | 目标座席分机                                                      |
| force_type   | 字符串   | force_listen、force_whisper、force_conference、force_intercept |
| uuid         | 字符串   | 呼叫唯一编号                                                      |
| video_stream | 字符串   | 呼入侧可切换音视频模式,客服查看流媒体视频                                       |
| status       | 字符串常量 | STATUS_READY 就绪... (同3.1.3)                                 |
| allow        | 字符串数组 | 允许的操作，参见”allow字段说明”表                                        |
| date_time    | 字符串   | 事件时间                                                        |

#### 4.1.7 错误消息回调 setAgentErrorEventListener()

有UI错误信息需显示时回调该接口，其中AgentErrorEvent定义如下：

| 字段名称    | 字段类型 | 说明                     |
|:--------|:-----|:-----------------------|
| type    | 字符串  | 事件类型 “AgentErrorEvent” |
| code    | 整型   | 错误码                    |
| message | 字符串  | 错误描述                   |

错误编码定义如下：

| 常量定义                           | 数值    | 说明                      |
|:-------------------------------|:------|:------------------------|
| ERROR_DEVICE_NOT_REGISTERED    | 1001  | 分机尚未登录                  |
| ERROR_AGENT_FORMAT             | 1002  | 客服工号格式错误                |
| ERROR_CURRENT_VIDEO_MODE       | 1003  | 当前已经是视频模式               |
| ERROR_CURRENT_AUDIO_MODE       | 1004  | 当前已经是音频模式               |
| ERROR_TOO_MANY_MEMBER          | 1005  | 当前通话人数多于两方不允许推流         |
| ERROR_CANNOT_CONSULT_WHEN_PUSH | 1006  | 磋商时不允许推流                |
| ERROR_NOT_VIDEO_MODE           | 1007  | 视频模式通话才能发起推流            |
| ERROR_FORMAT_STREAM_PATH       | 1008  | 推流地址格式错误                |
| ERROR_MEDIA_FILE_FORMAT        | 1009  | 推送的媒体文件格式不支持，仅支持mp4和png |
| ERROR_AGENT_OR_PASSWORD        | 10001 | 账号或密码错误                 |
| ERROR_BELONG_QUEUES            | 10002 | 工号至少应该属于一个技能组           |
| ERROR_IN_LOGIN                 | 10003 | 已经在登录中                  |
| ERROR_NOT_REGISTERED           | 10004 | 工号尚未登录,请先在SIP话机上登录      |
| ERROR_EMPTY_AGENT              | 10006 | 座席工号不能为空                |
| ERROR_TARGET_NUMBER            | 10007 | 不是有效的电话号码               |
| ERROR_AGENT_ID                 | 10008 | 座席工号输入错误                |
| ERROR_EMPTY_DEVICE             | 10014 | 分机号不能为空                 |
| ERROR_DEVICE_ID                | 10015 | 分机号输入错误                 |
| ERROR_PARAMS_EMPTY             | 10016 | 参数为空                    |
| ERROR_OPERATION_HEHAVIOR       | 10020 | 不允许的操作错误                |

#### 4.1.8 获取队列列表回调 setAllQueueListEventListener

用户请求队列信息时返回所有队列列表，其中AllQueueListEvent定义如下：

| 字段名称      | 字段类型         | 说明                       |
|:----------|:-------------|:-------------------------|
| type      | 字符串          | 事件类型 “AllQueueListEvent” |
| agent_id  | 字符串          | 座席工号                     |
| device    | 字符串          | 分机号                      |
| date_time | 字符串          | 事件时间                     |
| list      | String Array | 队列列表                     |

#### 4.1.9 获取队列座席对照回调 setAllQueueAgentMapListener

用户请求队列信息时返回所有队列座席对照，其中AllQueueAgentMapEvent定义如下：

| 字段名称      | 字段类型                 | 说明                           |
|:----------|:---------------------|:-----------------------------|
| type      | 字符串                  | 事件类型 “AllQueueAgentMapEvent” |
| agent_id  | 字符串                  | 座席工号                         |
| device    | 字符串                  | 分机号                          |
| date_time | 字符串                  | 事件时间                         |
| data      | Object{key:[]string} | 队列座席对照表                      |

data内结构，键值对例如：666666@172.20.12.205:[ “8029@172.20.12.205” ,” 8002@172.20.12.205” ]

#### 4.1.10 座席加入队列回调事件setTierOnResultEventListener

座席加入队列后收到反馈回调事件，其中TierOnResultEvent定义如下：

| 字段名称      | 字段类型 | 说明                       |
|:----------|:-----|:-------------------------|
| type      | 字符串  | 事件类型 “TierOnResultEvent” |
| agent_id  | 字符串  | 座席工号                     |
| queue     | 字符串  | 队列                       |
| date_time | 字符串  | 事件时间                     |

#### 4.1.11 座席退出队列回调事件setTierOffResultEventListener

座席退出队列后收到反馈回调事件，其中TierOffResultEvent定义如下：

| 字段名称      | 字段类型 | 说明                        |
|:----------|:-----|:--------------------------|
| type      | 字符串  | 事件类型 “TierOffResultEvent” |
| agent_id  | 字符串  | 座席工号                      |
| queue     | 字符串  | 队列                        |
| date_time | 字符串  | 事件时间                      |

### 4.2 CallControl事件定义

事件中返回allow代表该事件后续可允许的操作，若返回的allow为空数组，则使用return语句跳出按钮设定逻辑。

#### 4.2.1 呼入振铃回调 setChannelRingingEventListener()

座席被呼入成功后回调该接口，其中RingingEvent定义如下：

| 字段名称             | 字段类型  | 说明                          |
|:-----------------|:------|:----------------------------|
| type             | 字符串   | 事件类型 “RingingEvent”         |
| agent_id         | 字符串   | 座席工号                        |
| device           | 字符串   | 分机号                         |
| agent_party      | 字符串   | 座席分机                        |
| uuid             | 字符串   | 呼叫唯一编号                      |
| other_party      | 字符串   | 对端号码                        |
| other_party_uuid | 字符串   | 对端通话uuid                    |
| is_inbound       | 布尔    | 是否呼入 是true，否false           |
| status           | 字符串常量 | STATUS_READY 就绪... (同3.1.3) |
| allow            | 字符串数组 | 允许的操作，参见”allow字段说明”表        |
| user_data        | 字符串   | 随路数据                        |
| date_time        | 字符串   | 事件时间                        |

**注：若要获取会话ID，首通呼入回调可取other_party_uuid作为会话id，后续触发转接等操作可将记录的会话id往下文传递。**

#### 4.2.2 呼出振铃回调 setChannelRingBackEventListener()

座席呼出成功后回调该接口，其中RingBackEvent定义如下：

| 字段名称             | 字段类型  | 说明                          |
|:-----------------|:------|:----------------------------|
| type             | 字符串   | 事件类型 “RingBackEvent”        |
| agent_id         | 字符串   | 座席工号                        |
| device           | 字符串   | 分机号                         |
| agent_party      | 字符串   | 座席分机                        |
| uuid             | 字符串   | 呼叫唯一编号                      |
| other_party      | 字符串   | 对端号码                        |
| other_party_uuid | 字符串   | 对端通话uuid                    |
| is_consult       | 布尔    | 是否磋商 是true，否false           |
| is_inbound       | 布尔    | 是否呼入 是true，否false           |
| status           | 字符串常量 | STATUS_READY 就绪... (同3.1.3) |
| allow            | 字符串数组 | 允许的操作，参见”allow字段说明”表        |
| date_time        | 字符串   | 事件时间                        |

**注：若要获取会话ID，首通呼出回调可取uuid作为会话id，后续触发转接等操作可将记录的会话id往下文传递。**

#### 4.2.3 被呼叫接通后回调 setChannelThisPartyAnsweredEventListener()

被呼叫接通后回调该接口，其中ThisPartyAnsweredEvent定义如下：

| 字段名称             | 字段类型  | 说明                            |
|:-----------------|:------|:------------------------------|
| type             | 字符串   | 事件类型 “ThisPartyAnsweredEvent” |
| agent_id         | 字符串   | 座席工号                          |
| device           | 字符串   | 分机号                           |
| uuid             | 字符串   | 呼叫唯一编号                        |
| other_party      | 字符串   | 对端号码                          |
| other_party_uuid | 字符串   | 对端通话uuid                      |
| is_inbound       | 布尔    | 是否呼入 是true，否false             |
| status           | 字符串常量 | STATUS_READY 就绪... (同3.1.3)   |
| allow            | 字符串数组 | 允许的操作，参见”allow字段说明”表          |
| video_stream     | 字符串   | 呼入侧可切换音视频模式,客服查看流媒体视频         |
| user_data        | 字符串   | 随路数据                          |
| date_time        | 字符串   | 事件时间                          |

#### 4.2.4 呼叫接通后回调 setChannelOtherPartyAnsweredEventListener()

呼叫接通后回调该接口，其中OtherPartyAnsweredEvent定义如下：

| 字段名称             | 字段类型  | 说明                             |
|:-----------------|:------|:-------------------------------|
| type             | 字符串   | 事件类型 “OtherPartyAnsweredEvent” |
| agent_id         | 字符串   | 座席工号                           |
| device           | 字符串   | 分机号                            |
| uuid             | 字符串   | 呼叫唯一编号                         |
| other_party      | 字符串   | 对端号码                           |
| other_party_uuid | 字符串   | 对端通话uuid                       |
| is_inbound       | 布尔    | 是否呼入 是true，否false              |
| status           | 字符串常量 | STATUS_READY 就绪... (同3.1.3)    |
| allow            | 字符串数组 | 允许的操作，参见”allow字段说明”表           |
| video_stream     | 字符串   | 呼入侧可切换音视频模式,客服查看流媒体视频          |
| user_data        | 字符串   | 随路数据                           |
| date_time        | 字符串   | 事件时间                           |

#### 4.2.5 呼叫挂断回调 setChannelDestroyEventListener()

呼叫挂断后回调该接口，其中HangupEvent定义如下：

| 字段名称        | 字段类型  | 说明                   |
|:------------|:------|:---------------------|
| type        | 字符串   | HangupEvent          |
| agent_id    | 字符串   | 座席工号                 |
| device      | 字符串   | 分机号                  |
| uuid        | 字符串   | 呼叫唯一编号               |
| other_party | 字符串   | 对端号码                 |
| allow       | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| user_data   | 字符串   | 随路数据                 |
| date_time   | 字符串   | 事件时间                 |

#### 4.2.6 呼叫保持回调 setChannelHoldEventListener()

呼叫保持后回调该接口，其中HoldEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “HoldEvent”     |
| agent_id  | 字符串   | 座席工号                 |
| device    | 字符串   | 分机号                  |
| uuid      | 字符串   | 呼叫唯一编号               |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.7 呼叫恢复回调 setChannelUnHoldEventListener()

呼叫恢复后回调该接口，其中UnHoldEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “UnHoldEvent”   |
| agent_id  | 字符串   | 座席工号                 |
| device    | 字符串   | 分机号                  |
| uuid      | 字符串   | 呼叫唯一编号               |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.8 磋商事件回调 setConsultEventListener()

磋商后回调该接口，其中ConsultEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “ConsultEvent”  |
| device    | 字符串   | 分机号                  |
| cause     | 字符串   | HangUp、Success、Fail  |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.9 磋商外线挂机取回回调 setOtherPartyCanceledEventListener()

磋商成功后回调该接口，其中OtherPartyCanceledEvent定义如下：

| 字段名称      | 字段类型  | 说明                             |
|:----------|:------|:-------------------------------|
| type      | 字符串   | 事件类型 “OtherPartyCanceledEvent” |
| device    | 字符串   | 分机号                            |
| cause     | 字符串   | HangUp、Success、Fail            |
| status    | 字符串常量 | STATUS_READY 就绪... (同3.1.3)    |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表           |
| date_time | 字符串   | 事件时间                           |

#### 4.2.10 进入三方会话回调 setRecvDtmfEventListener()

三方会话成功后回调该接口，其中RecvDtmfEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “RecvDtmfEvent” |
| uuid      | 字符串   | 呼叫唯一编号               |
| opt       | 字符串   | 0                    |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.11 提示被呼客服正通话中回调setAgentOnCallEventListener()

客服a磋商、转接客服b等操作时发现现在通话中回调接口，其中AgentOnCallEvent定义如下：

| 字段名称        | 字段类型 | 说明                      |
|:------------|:-----|:------------------------|
| type        | 字符串  | 事件类型 “AgentOnCallEvent” |
| agent_id    | 字符串  | 座席工号                    |
| device      | 字符串  | 分机号                     |
| other_party | 字符串  | 被呼客服分机                  |
| state       | 字符串  | calling                 |
| cause       | 字符串  | 原因                      |
| date_time   | 字符串  | 事件时间                    |

#### 4.2.12 呼叫静音回调 setChannelMuteEventListener()

呼叫静音后回调该接口，其中MuteEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “MuteEvent”     |
| agent_id  | 字符串   | 座席工号                 |
| device    | 字符串   | 分机号                  |
| uuid      | 字符串   | 呼叫对端唯一编号             |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.13 呼叫恢复静音回调 setChannelUnMuteEventListener()

呼叫恢复后回调该接口，其中UnMuteEvent定义如下：

| 字段名称      | 字段类型  | 说明                   |
|:----------|:------|:---------------------|
| type      | 字符串   | 事件类型 “UnMuteEvent”   |
| agent_id  | 字符串   | 座席工号                 |
| device    | 字符串   | 分机号                  |
| uuid      | 字符串   | 呼叫对端唯一编号             |
| allow     | 字符串数组 | 允许的操作，参见”allow字段说明”表 |
| date_time | 字符串   | 事件时间                 |

#### 4.2.14 视频流播放错误回调 setVideoStreamErrorListener()

视频流播放报错时调用接口，其中消息定义如下：

| 字段名称  | 字段类型  | 说明   |
|:------|:------|:-----|
| error | Error | 错误信息 |

#### 4.2.15 通话语音识别翻译回调 setDetectSpeechEventListener()

通话中翻译客服和客户的语音实时翻译，其中DeteceSpeechEvent定义如下：

| 字段名称      | 字段类型 | 说明                       |
|:----------|:-----|:-------------------------|
| type      | 字符串  | 事件类型 “DetectSpeechEvent” |
| agent_id  | 字符串  | 座席工号                     |
| device    | 字符串  | 分机号                      |
| speaker   | 字符串  | 说话的对象                    |
| content   | 字符串  | 说话的内容                    |
| date_time | 字符串  | 事件时间                     |

**注：配置文件项isDetectSpeech需开通使用语音翻译功能**

#### 4.2.16 推流准备反馈 setPublishStreamPrepareEventListener()

使用桌面推流时，先将流推至服务器，再发送事件告知准备发送指令，其中PublishStreamPrepareEvent定义如下：

| 字段名称        | 字段类型 | 说明                               |
|:------------|:-----|:---------------------------------|
| type        | 字符串  | 事件类型 “PublishStreamPrepareEvent” |
| stream_path | 字符串  | 流地址                              |

**注：该事件的出现为了避免先发送命令后推流的情况。**

#### 4.2.17 通知开始视频推流 setStartVideoPushEventListener()

当通话大于两方通话时，被磋商方的客服收到该通知以获取前客服的视频推流。其中StartVideoPushEvent定义如下

| 字段名称        | 字段类型 | 说明                         |
|:------------|:-----|:---------------------------|
| type        | 字符串  | 事件类型 “StartVideoPushEvent” |
| agent_id    | 字符串  | 座席工号                       |
| device      | 字符串  | 分机号                        |
| push_device | 字符串  | 推视频流的座席                    |
| stream_path | 字符串  | 流地址                        |
| date_time   | 字符串  | 事件时间                       |

**注：人数大于两方的通话，第三方将限制推流**

#### 4.2.18 通知结束视频推流 setStopVideoPushEventListener()

当通话大于两方通话时，收到前客服结束视频推流的通知。

| 字段名称        | 字段类型 | 说明                        |
|:------------|:-----|:--------------------------|
| type        | 字符串  | 事件类型 “StopVideoPushEvent” |
| agent_id    | 字符串  | 座席工号                      |
| device      | 字符串  | 分机号                       |
| push_device | 字符串  | 推视频流的座席                   |
| date_time   | 字符串  | 事件时间                      |

**注：人数大于两方的通话，第三方将限制推流**

#### 4.2.19 变音反馈事件setVoiceChangeEventListener()

当启用变音模式，调整变音频率时触发该事件。

| 字段名称      | 字段类型 | 说明                            |
|:----------|:-----|:------------------------------|
| type      | 字符串  | 事件类型 “VoiceChangeEvent”       |
| agent_id  | 字符串  | 座席工号                          |
| device    | 字符串  | 分机号                           |
| info      | 字符串  | 指返回变音回调频数值，若变音控件加载问题也会从字段吐出信息 |
| date_time | 字符串  | 事件时间                          |

**注：变音反馈事件只有操作客服测变音的时候才会发送反馈事件。**
