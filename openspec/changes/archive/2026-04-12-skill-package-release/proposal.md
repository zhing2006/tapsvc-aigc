## Why

当前 release workflow 只产出 5 个裸二进制文件。用户需要手动拷贝 SKILL.md、references 等文件才能使用技能。
改为构建**可安装的技能包**（包含 CLI binary + Skill 文件），Agent 可以自动从 GitHub Release 下载并安装到技能目录，实现一键部署。

## What Changes

- **release workflow 重构**: 从产出裸二进制改为产出 5 个平台技能压缩包（Windows `.zip`，Linux/macOS `.tar.gz`）
- **SKILL.md 更新**: 补充 CLI 工作目录说明 — CLI 必须在用户当前工作目录执行，不能 cd 到 skill 目录
- **README.md 重写**: 仓库根 README.md 改为双语（英文 + 中文 README_CN.md），包含项目介绍和 Agent 可执行的安装/更新指引
- 技能包内部结构统一为 `tapsvc-aigc/{SKILL.md, references/, scripts/}`（README 不打包，仅在仓库中供安装前阅读）

## Capabilities

### New Capabilities

- `Skill/Distribution/skill-package`: 技能包构建与分发 — 定义包结构、压缩格式、平台映射、资产命名
- `Skill/Distribution/skill-install`: Agent 驱动的安装/更新流程 — 平台检测、Release 下载、解压部署、验证

### Modified Capabilities

- `Skill/Core/skill-workflow`: Preflight 和 Step 2 补充 CWD 执行约束 — CLI 必须在用户工作目录运行，.env 从 CWD 加载

## Impact

- `.github/workflows/release.yml` — 重构打包流程
- `skills/tapsvc-aigc/SKILL.md` — 补充 CWD 说明
- `README.md` — 重写为双语安装手册
- 新增 `README_CN.md` — 中文版
- 技能包新增 `scripts/` 目录（运行时由包携带，不进仓库）
