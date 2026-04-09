# CI/CD 工作流

本文档说明 `todo-cli` 项目的 Rust CI/CD 实战流程。

## 概述

工作流在以下情况下自动触发：
- 推送代码到 `main` 分支
- 提交 Pull Request
- 推送 `v*` 版本标签
- 在 GitHub Actions 页面手动触发

## 工作流程

项目的工作流分成三层：

### 1. 验证层 (Verify)

包含三个并行 job：
- `fmt`：运行 `cargo fmt --check`
- `clippy`：运行 `cargo clippy --all-targets --all-features -- -D warnings`
- `test`：运行 `cargo test --all-features`

这些 job 是后续构建和发布的门禁，任何一步失败都不会进入后续阶段。

### 2. 构建层 (Build)

仅在推送到 `main` 时运行：

```bash
cargo build --release
```

构建成功后会上传 Linux 二进制 artifact，便于下载和验证。

### 3. 发布演练层 (Release)

仅在推送类似 `v0.1.0` 的标签时运行：

```bash
cargo build --release
```

构建后会上传带版本号的 release artifact，用来模拟正式发布前的产物准备。

## 自动检查

### 1. 代码格式检查 (Formatting)

```bash
cargo fmt --check
```

验证代码是否符合 Rust 的官方格式标准。使用 `rustfmt` 自动格式化工具。

**本地运行：**

```bash
cargo fmt --check
cargo fmt
```

### 2. 代码检查 (Linting)

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

使用 `clippy` 检测代码中的常见错误、性能问题和非惯用代码。所有警告都被视为错误。

**本地运行：**

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### 3. 测试 (Tests)

```bash
cargo test --all-features
```

运行项目的所有单元测试和集成测试，启用所有功能标志。当前仓库已经包含 CLI 解析测试和基于内存 SQLite 的服务层测试，适合拿来练习真实的 CI 失败排查。

## 在本地运行完整 CI 检查

提交前建议运行：

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

如果要顺带验证构建层，可再运行：

```bash
cargo build --release
```

## 故障排除

### 格式检查失败

**解决方案：**

```bash
cargo fmt
```

### Clippy 检查失败

**解决方案：**
1. 运行 `cargo clippy --all-targets --all-features -- -D warnings`
2. 根据提示修改代码
3. 重新运行相同命令确认通过

### 测试失败

**解决方案：**
1. 运行 `cargo test --all-features`
2. 如需更多输出，运行 `cargo test --all-features -- --nocapture`
3. 修复逻辑或测试后再次验证

## CI/CD 配置文件

配置位于 [`.github/workflows/ci.yml`](workflows/ci.yml)

使用的工具：
- **Rust Toolchain**: 最新稳定版本
- **格式检查**: `rustfmt`
- **代码检查**: `clippy`
- **测试框架**: 内置 `cargo test`
- **缓存**: `Swatinem/rust-cache@v2`
- **构建产物**: `actions/upload-artifact@v4`

## 预期结果

- Pull Request 会运行验证层
- 推送到 `main` 会额外生成构建 artifact
- 推送 `v*` 标签会生成带标签名的 release artifact

## 贡献者指南

提交代码前，请确保：
1. 代码格式正确：`cargo fmt`
2. 没有 clippy 警告：`cargo clippy --all-targets --all-features -- -D warnings`
3. 所有测试通过：`cargo test --all-features`
4. 理解 `main` 构建和 `tag` 发布演练的区别
