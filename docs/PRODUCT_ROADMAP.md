# Productization Roadmap

本文档描述将 `mquickjs-rs` 演进为 ESP32 LED 特效产品运行时的优先级路线。

## 目标

- 设备端稳定执行受限 JavaScript 特效脚本
- 内存、执行时间、功能边界可控
- 文档、测试、生成器、运行时保持一致

## 当前主要差距

### P0 问题

- 文档仍以通用/学习型引擎表述为主
- LED Profile 尚未成为唯一规范源
- `effects` 集成测试尚未通过
- `Uint8Array` 相关能力对 LED 场景仍不完整
- 公开 API 仍偏通用 `eval`，宿主接口不稳定

### P1 问题

- 内存限制与真实运行时分配未统一计量
- GC 设计与实际运行时对象管理尚未闭环
- 缺少执行预算、watchdog、中断机制
- 缺少面向设备部署的字节码发布流程

### P2 问题

- 缺少 Profile 一致性测试矩阵
- 缺少 ESP32 端端到端性能与压力数据
- 缺少版本化兼容策略

## 路线分阶段

## Phase 1：规范冻结

目标：先统一“允许什么、不允许什么”。

- 建立 `docs/LED_PROFILE.md` 作为唯一产品脚本规范
- README 与生成器说明统一改为“受限 ES6 风格 Profile”
- 将现有 `JS_FEATURE_SPEC` 与 Profile 建立引用关系
- 新增禁止特性测试，避免脚本能力漂移

验收标准：

- Profile 文档可单独指导脚本编写
- 代码生成器、测试、README 描述一致

## Phase 2：LED 最小闭环

目标：让核心 effect 用例稳定跑通。

- 修复 `tests/effects.rs`
- 补齐 `Uint8Array` 在 LED 场景必要方法
- 稳定对象字面量、闭包、配置对象路径
- 增加 effect 生命周期回归测试

验收标准：

- `cargo test --test effects` 全通过
- `blink/chase/rainbow/wave` 四类效果稳定执行

## Phase 3：宿主接口产品化

目标：把引擎改造成稳定的设备端组件，而不是仅供 `eval` 使用的库。

- 设计 effect 实例 API
- 增加读取 LED buffer 的宿主接口
- 增加加载 bytecode / 重置实例 / 更新配置能力
- 明确单脚本/多脚本运行模型

建议 API 方向：

- `load_effect(bytecode)`
- `instantiate(config)`
- `tick()`
- `led_buffer()`
- `set_config(key, value)`
- `reset()`

验收标准：

- 宿主侧无需依赖通用 `eval` 即可完整驱动 effect

## Phase 4：资源模型重构

目标：建立真实可靠的 MCU 资源边界。

- 统一对象、数组、闭包、TypedArray 的内存计量
- 决定保留并补齐 GC，或改为更简单的句柄表/arena 模型
- 增加最大对象数、最大数组长度、最大 TypedArray bytes 限制
- 修复 `memory_stats` 口径与真实分配不一致的问题

验收标准：

- 任意脚本都不能绕过内存上限
- 内存统计可用于线上观测与问题定位

## Phase 5：执行安全

目标：避免脚本卡死主循环。

- 增加执行步数预算
- 增加递归深度预算
- 增加长循环中断能力
- 增加超时/看门狗联动策略

验收标准：

- 恶意或错误脚本不会长期阻塞 LED 主循环

## Phase 6：离线工具链

目标：降低设备端复杂度并提升发布可靠性。

- 构建源码到 bytecode 的离线编译流程
- 加入 Profile 校验
- 加入 bytecode 版本号与兼容性检查
- 建立 effect 发布包格式

验收标准：

- 设备端仅加载已验证字节码
- 字节码版本不兼容时可明确报错

## Phase 7：ESP32 集成验收

目标：验证产品化指标，而不是只看桌面测试。

- 启动时间测试
- 单帧执行延迟测试
- 内存峰值测试
- 长时间稳定性测试
- 异常脚本恢复测试

验收标准：

- 在目标芯片与目标 LED 数量上满足产品 KPI

## 近期建议优先级

### 立即执行

- 冻结 LED Profile
- 更新 README 与相关文档
- 修复 `effects` 测试

### 紧接执行

- 补齐 `Uint8Array` 必要能力
- 设计宿主接口
- 建立真实内存计量

### 之后执行

- 引入执行预算
- 上线离线字节码流程
- 完成 ESP32 压测
