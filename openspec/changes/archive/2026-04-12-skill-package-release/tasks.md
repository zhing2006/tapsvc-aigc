## 1. SKILL.md 更新

- [x] 1.1 在 Step 0 Preflight 中补充 `.env` 文件检测逻辑：若 CWD 下存在 `.env` 则跳过环境变量检查
- [x] 1.2 在 SKILL.md 中补充 CWD 执行约束说明：CLI 必须在用户工作目录执行，不能 cd 到技能目录

## 2. README 双语安装手册

- [x] 2.1 编写 `README.md`（英文版）：项目介绍 + Agent 安装/更新指引（平台检测、GitHub API 下载、解压部署、验证）
- [x] 2.2 编写 `README_CN.md`（中文版）：与英文版内容对应
- [x] 2.3 两个 README 文件头添加语言切换链接

## 3. Release workflow 重构

- [x] 3.1 在 release.yml matrix 中添加压缩格式变量（zip/tar.gz）和技能包名变量
- [x] 3.2 添加 package step：组装技能包目录结构 — 复制 skills/tapsvc-aigc/ 内容到包顶层，将 binary 放入 scripts/ 子目录
- [x] 3.3 Windows 平台使用 zip 打包，Unix 平台使用 tar.gz 打包
- [x] 3.4 替换 artifact upload：上传技能包替代裸二进制
- [x] 3.5 更新 release job：上传技能包到 GitHub Release

## 4. SKILL.md Preflight 平台感知

- [x] 4.1 更新 SKILL.md Preflight 中的 binary 查找逻辑：支持平台感知（技能目录变量优先，PATH 回退，Windows .exe）

## 5. 验证

- [x] 5.1 检查 workflow YAML 语法正确性
- [x] 5.2 确认 5 个平台的包名与 spec 定义一致
- [x] 5.3 确认 workflow 打包产出的包内结构正确：顶层为 tapsvc-aigc/，包含 SKILL.md、references/（3 个 md）、scripts/（binary）
- [x] 5.4 确认 release workflow 产出中无裸二进制，只有技能压缩包
- [x] 5.5 确认 README 双语文件头有语言切换链接
- [x] 5.6 确认 SKILL.md Preflight 包含 Windows .exe 和平台感知查找逻辑（技能目录优先 → PATH 回退 → 两步均失败才报错）
