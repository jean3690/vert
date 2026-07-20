## 变更说明

<!-- 简要描述这个 PR 做了什么 -->

## 变更类型

- [ ] 新功能（新格式支持 / 新转换路径）
- [ ] Bug 修复
- [ ] 性能优化
- [ ] 代码重构
- [ ] 文档更新
- [ ] CI / 构建

## 涉及前端

<!-- 如果 PR 涉及前端改动（UI 组件、样式、交互、文案等），必须附截图或录屏 -->

- [ ] 本 PR 涉及前端改动，已附截图/录屏（见下方）

<!-- 截图区域 -->

## 涉及格式

<!-- 如果 PR 涉及新格式或格式转换逻辑变更，请确认以下检查项 -->

- [ ] `src/types/index.ts` — `FileFormat` 类型已更新
- [ ] `src/utils/formats.ts` — `VALID_CONVERSIONS` 已更新
- [ ] `src-tauri/src/converter/traits.rs` — `Format` 枚举和 `valid_targets()` 已更新
- [ ] `src-tauri/src/converter/mod.rs` — 分发逻辑已更新
- [ ] `src-tauri/src/cli.rs` — CLI `list()` 已更新（如适用）

## 验证

<!-- 说明如何验证你的改动 -->

- [ ] `pnpm build` 通过（前端 type-check + 构建）
- [ ] `cargo check` / `cargo clippy` / `cargo test` 通过
- [ ] 手动测试：转换示例文件验证输出正确

## 关联 Issue

<!-- 如有，填写 `Close #xxx` 或 `Related #xxx` -->
