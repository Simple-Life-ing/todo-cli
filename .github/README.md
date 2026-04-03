# CI/CD 工作流

本文档说明 `todo-cli` 项目的持续集成（CI）流程。

## 概述

CI 流程在以下情况下自动触发：
- 推送代码到 `main` 分支
- 提交 Pull Request

## 工作流程

### 自动检查

项目的 CI 流程包括以下步骤：

#### 1. 代码格式检查 (Formatting)
```bash
cargo fmt --check
```
验证代码是否符合 Rust 的官方格式标准。使用 `rustfmt` 自动格式化工具。

**本地运行：**
```bash
cargo fmt --check    # 检查格式
cargo fmt           # 自动修复格式
```

#### 2. 代码检查 (Linting)
```bash
cargo clippy -- -D warnings
```
使用 `clippy` 检测代码中的常见错误、性能问题和非惯用代码。所有警告都被视为错误 (`-D warnings`)。

**本地运行：**
```bash
cargo clippy       # 查看所有建议
cargo clippy -- -D warnings  # 严格模式（与 CI 相同）
```

#### 3. 测试 (Tests)
```bash
cargo test --all-features
```
运行项目的所有单元测试和集成测试，启用所有功能标志。

**本地运行：**
```bash
cargo test              # 运行所有测试
cargo test --all-features  # 启用所有功能（与 CI 相同）
```

## 在本地运行完整 CI 检查

要在提交前在本地验证所有检查，运行：

```bash
cargo fmt --check && cargo clippy -- -D warnings && cargo test --all-features
```

或分别运行：

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test --all-features
```

## 故障排除

### 格式检查失败

**错误信息：** `error: line formatted incorrectly`

**解决方案：**
```bash
cargo fmt
```
这将自动修复格式问题。

### Clippy 检查失败

**错误信息：** `error: aborting due to previous error`

**解决方案：**
1. 运行 `cargo clippy` 查看具体的建议
2. 按照建议修改代码
3. 某些情况下可能需要添加 `#[allow(clippy::...)]` 注解

### 测试失败

**错误信息：** `test result: FAILED`

**解决方案：**
1. 运行 `cargo test -- --nocapture` 查看详细输出
2. 修复测试或修复代码逻辑
3. 运行 `cargo test --all-features` 再次验证

## CI 配置文件

CI 配置位于 [`.github/workflows/ci.yml`](workflows/ci.yml)

使用的工具：
- **Rust Toolchain**: 最新稳定版本
- **格式检查**: `rustfmt`
- **代码检查**: `clippy`
- **测试框架**: 内置 `cargo test`

## 预期结果

所有检查通过后，CI 将显示绿色的 ✓ 标记。CI 状态显示在 Pull Request 页面上。

## 贡献者指南

提交代码前，请确保：
1. ✅ 代码格式正确：`cargo fmt`
2. ✅ 没有 clippy 警告：`cargo clippy -- -D warnings`
3. ✅ 所有测试通过：`cargo test --all-features`

如果 CI 检查失败，请根据错误信息修复问题，然后推送更新。
