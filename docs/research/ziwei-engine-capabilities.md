# `matharts/ziwei` 引擎现有能力与约束

## 结论

以 `matharts/ziwei` 的 `main` 分支提交
[`f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c`](https://github.com/matharts/ziwei/commit/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c)
为证据基线：当前引擎已经交付的是**不可变本命盘领域计算**与**借用本命盘的只读查询**；历法、命理分析、产品数据管理、展示、绑定等仍是引擎外职责或空边界。

对 Ziwei Atlas 最安全的集成结论是：

1. 只通过 `ziwei` facade 使用当前公开面，不直接依赖内部模块。
2. 把 `Natal` 当作一次排盘产生的不可变领域事实，而不是可附加命例、分析、论断或教学内容的产品实体。
3. Atlas 自己拥有命例、命盘记录、关系、分析、论断、统计、教学与持久化；引擎只负责可重复计算和只读领域查询。
4. Atlas 已讨论的“直接指定紫微所在地支”模拟输入**不能映射到当前构造 API**。现有引擎要求农历日，并由日数与五行局计算紫微位置；紫微位置被明确禁止作为排盘结果注入。
5. 仓库虽然把选择性重导出的领域身份称为“稳定”，但所有 crate 仍是 `0.1.0`、`publish = false`，README 也标明积极开发中。因此这里的“稳定公开面”只能理解为**当前提交已确认的仓库内契约**，不能理解为已有 crates.io/语义化版本兼容承诺。

## 研究基线

- checkout：`/Users/lzm0x219/Code/Github/matharts/ziwei`
- branch：`main`，与 `origin/main` 同步
- commit：`f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c`
- 工作树：研究开始和验证结束时均干净；本次只读分析，没有修改该仓库
- 工具链：`rustc 1.97.1 (8bab26f4f 2026-07-14)`、`cargo 1.97.1 (c980f4866 2026-06-30)`；仓库声明 Rust 2024 edition 与 `rust-version = 1.97.1`（[`Cargo.toml#L5-L10`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/Cargo.toml#L5-L10)）

README 只把项目称为“标准驱动紫微斗数排盘引擎”，并明确提示仍在积极开发（[`README.md#L1-L6`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/README.md#L1-L6)）。

## 已实现的领域能力

### 1. 两种已验证输入

`ziwei_core` 暴露两条构造路径（[`ziwei_core/src/lib.rs#L1-L17`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/lib.rs#L1-L17)）：

- `ZiweiBirth -> create_from_birth -> Natal`：输入性别、归一化农历年序号、月、日、时；年干支由引擎推导。字段和范围见 [`input.rs#L14-L80`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/input.rs#L14-L80)。
- `ZiweiInput -> create_from_input -> Natal`：输入性别、生年天干、生年地支、月、日、时；验证月日时范围及干支阴阳配对。字段和构造契约见 [`input.rs#L84-L173`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/input.rs#L84-L173)。

两种输入都必须先通过 `try_new`；此后 `create_from_*` 直接返回 `Natal`，不保留不可能发生的错误分支（[`public-interface-and-domain-types.md#L3-L12`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/public-interface-and-domain-types.md#L3-L12)）。

### 2. 不可变本命盘事实

`Natal` 的构造流水线实际计算并装配（[`natal.rs#L104-L195`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/natal.rs#L104-L195)）：

- 十二宫及命宫、身宫、来因宫坐标；
- 十八颗星曜：十四正曜，加左辅、右弼、文昌、文曲；
- 生年四化、每宫四条宫位四化关系、星曜向心/离心自化；
- 生肖、五行局、大限方向；
- 十二个大限，每限十个最小 `DecadeYear` 事实。

结果字段私有、由内核装配、对外只读；`Natal` getter 见 [`natal.rs#L198-L252`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/natal.rs#L198-L252)，完整模型决策见 [`ADR-0011#L7-L18`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/adr/0011-immutable-natal-model.md#L7-L18)。

有绝对农历年序号的 `ZiweiBirth` 会为大限年保存 `Some(year)`；只有年干支的 `ZiweiInput` 仍可计算虚岁，但大限年的绝对农历年为 `None`。这不是缺陷，而是两种输入能力的显式区别（[`workspace-boundaries.md#L20-L24`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/workspace-boundaries.md#L20-L24)）。

### 3. 只读查询

`ziwei_query` 只借用 `&Natal`，不复制、修改或重算内核事实（[`ziwei_query/src/lib.rs#L1-L15`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_query/src/lib.rs#L1-L15)）。当前实现包括：

- `query(&Natal)` 入口、本命 scope、任意宫位立极 `ReframeScope`；
- 大限 `DecadeScope`，按大限序号、虚岁或农历年定位，及跨限前后年份导航；入口见 [`scope.rs#L11-L92`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_query/src/scope.rs#L11-L92)；
- 按相对宫名、本命宫名、地支、宫干导航宫位；按身份、类别、斗系查星；
- 生年四化、宫位四化入边/出边、星曜自化查询；
- 固定关系：立极、叠宫、对宫、宫线、三方、四正、河图宫位、暗合宫；
- 首批条件组合：空宫、星曜集合、同宫、会、照、冲及自化条件。

当前能力清单与边界见 [`ziwei-query.md#L23-L91`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/ziwei-query.md#L23-L91)，L2 大限定位与查询语义见 [`ziwei-query.md#L93-L115`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/ziwei-query.md#L93-L115)。

### 4. 统一 facade

`ziwei` crate 是面向调用方的门面，只选择性重导出已经确认的 core 事实与 query 类型（[`ziwei/src/lib.rs#L1-L28`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/src/lib.rs#L1-L28)）。依赖方向是：

```text
ziwei_query -> ziwei_core
ziwei       -> ziwei_core + ziwei_query
```

这一边界由 manifests 与设计文档共同固定（[`workspace-boundaries.md#L5-L18`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/workspace-boundaries.md#L5-L18)）。Atlas 不应依赖 `ziwei_core` 或 `ziwei_query` 的私有模块、字段和索引表示。

## Atlas 不能当作已交付事实的能力

### 1. 历法与原始出生时间归一化

引擎不处理公历转农历、时区、真太阳时、闰月归属、晚子时或日界；调用方必须先完成归一化（[`ADR-0001#L1-L9`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/adr/0001-lunar-birth-input-contract.md#L1-L9)）。`ziwei_calendar` 当前只有一行 crate 文档，没有实现或依赖；仓库也明确称其为“保留的历法边界，当前不实现产品能力”（[`workspace-boundaries.md#L5-L18`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/workspace-boundaries.md#L5-L18)）。

因此，Atlas 不能把“输入公历出生时间即可得到规范农历输入”归因于现有引擎。

### 2. 命理分析、论断与格局判断

`ziwei_analysis` 同样只是空边界。现有 query 明确不做排盘、历法换算、解释、断语或命理格局判断（[`ziwei-query.md#L133-L138`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/ziwei-query.md#L133-L138)）。

因此，Atlas 的分析、论断、验证、知识引用和统计洞察必须是产品领域能力；不能把 `ziwei_analysis` 的 crate 名称当成已有实现。

### 3. 流年、流月等完整时态盘

当前 `Natal` 保存的是十二个大限及每限十个最小年份/虚岁事实；`DecadeYearSelection` 只是定位结果，不是流年 scope。只有 core 未来保存了完整时态事实，query 才会增加流年、流月层级（[`ziwei-query.md#L13-L21`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/ziwei-query.md#L13-L21)）。

Atlas 不能宣称现有引擎已经提供流年宫位、流年星曜、流年四化或完整限运推演。

### 4. 更完整的星曜与显示资料

当前模型只有十八颗星曜。测试文件也明确把星曜亮度、不支持的星曜、流年映射和小限排除在当前 `Natal` 之外（[`natal_examples.rs#L1-L4`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/natal_examples.rs#L1-L4)）。简繁体名称、文案和国际化属于外层适配器，不属于 core（[`public-interface-and-domain-types.md#L37-L43`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/public-interface-and-domain-types.md#L37-L43)）。

因此不能把禄存、羊陀、魁钺、火铃、空劫、完整杂曜神煞、庙旺利陷、小限或展示文本当成已交付。

### 5. 产品与数据层能力

现有 workspace 没有提供：

- 命例、命盘记录及二者的一对多关系；
- 命例之间的亲属/社会关系；
- 分析、论断、验证、教学批注；
- 数据库、序列化、导入导出、规则版本快照；
- 仪表板、跨命例统计、多盘比较、知识库；
- UI、语言绑定、同步或协作。

这些内容不在 `ziwei` facade 的公开面中；bindings 也仍是未来边界（[`workspace-boundaries.md#L5-L14`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/workspace-boundaries.md#L5-L14)）。

## 与 Atlas 命盘模拟器的关键冲突

Atlas 当前讨论的模拟器参数是：性别、生年天干、生年地支、紫微所在地支、出生月份、出生时辰。现有引擎的 `ZiweiInput` 则要求：性别、生年天干、生年地支、出生月份、**出生日期**、出生时辰；它没有“紫微所在地支”字段（[`input.rs#L84-L142`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/input.rs#L84-L142)）。

实现会根据 `day + FiveElementBureau` 查出紫微位置，再安其余星曜（[`placement.rs#L147-L169`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei_core/src/domain/placement.rs#L147-L169)）。ADR 更明确规定：命宫、紫微、宫干、局数、星曜落位和大限都属于结果，禁止作为输入注入（[`ADR-0002#L1-L24`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/adr/0002-engine-compute-vs-inject.md#L1-L24)）。

所以当前不能把该模拟器简单实现为 `ZiweiInput -> Natal`。后续必须先决定：

- **规则有效模拟**：改为输入出生日期，让引擎完整计算紫微位置；或
- **自由教学推演**：保留直接指定紫微位置，但使用与 `Natal` 明确区分的模拟模型；若要改动引擎边界，则需单独决策并重新定义真实性、校验与测试契约。

在决策完成前，不应把自由组合结果称为“引擎排出的有效 `Natal`”。

## 公开 API 的稳定性判断

仓库文档把 `Gender`、`Branch`、`Stem`、`PalaceName`、`StarName`、`Transformation` 等称为稳定领域身份，并由 `ziwei` facade 选择性重导出（[`public-interface-and-domain-types.md#L37-L59`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/public-interface-and-domain-types.md#L37-L59)）。这是当前最适合作为 Atlas 集成面的 API。

但应同时保留三项限制：

1. 所有 crate 当前都是 `0.1.0` 且 `publish = false`；例如 [`ziwei/Cargo.toml#L1-L14`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/Cargo.toml#L1-L14)。
2. 仓库明确不保留旧 API 兼容别名（[`public-interface-and-domain-types.md#L45-L55`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/docs/design/public-interface-and-domain-types.md#L45-L55)）。
3. 当前公开输入没有流派、规则集或规则版本参数；`Natal` 也没有 provenance 字段。若 Atlas 要长期保存命盘并支持重算，应自行记录所用引擎 commit/版本和归一化输入，避免未来规则变化后无法解释差异。

因此更准确的措辞是：**当前提交已有明确、测试覆盖的公开 facade，但尚不是承诺长期兼容的已发布 SDK。**

## 验证证据与边界

在上述 clean checkout 上实际运行：

```text
rtk cargo test --workspace --all-targets
=> 65 passed, 1 ignored

rtk cargo test --release -p ziwei --test natal_exhaustive -- --ignored
=> 1 passed

rtk cargo test --workspace --doc
=> 2 passed
```

主要契约证据：

- facade 输入与不可变 `Natal` 图：[`natal_public_api.rs#L8-L65`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/natal_public_api.rs#L8-L65)；
- 宫位顺序、唯一身份、四化与坐标不变量：[`natal_contract.rs#L73-L188`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/natal_contract.rs#L73-L188)；
- 10 个用户确认的完整本命示例：[`natal_examples.rs#L41-L1362`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/natal_examples.rs#L41-L1362)；
- 全部合法归一化输入组合的结果图不变量（默认忽略，本次显式 release 运行通过）：[`natal_exhaustive.rs#L159-L180`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/natal_exhaustive.rs#L159-L180)；
- 查询的立极 12×12 双射、固定关系、星曜条件、四化及大限/年份边界：[`query_contract.rs#L171-L685`](https://github.com/matharts/ziwei/blob/f19182e3271c7dd2a09fa80a8c19d6b7e0861c8c/crates/ziwei/tests/query_contract.rs#L171-L685)。

这些运行结果证明：在 commit `f19182e` 和本机工具链上，仓库定义的测试契约通过。它们**不等于**：

- 已验证所有流派或所有古籍版本的规则一致性；
- 已由独立排盘实现做全量交叉验证；
- 已验证尚未实现的 calendar/analysis/product 能力；
- 已证明 API 长期兼容、生产可用性或 Atlas 集成正确性。

## 对后续决策的输入

建议后续精确提出、但不在本研究票内自行解决：

1. **定义 Atlas 模拟器输入与 `Natal` 的真实性边界**：规则有效模拟和自由教学推演是否是两种不同产物。
2. **定义 Atlas 的历法归一化契约**：公历/农历、时区、真太阳时、闰月、晚子时和日界由哪个组件负责。
3. **定义 Atlas 保存命盘时的规则 provenance**：至少记录引擎 commit/版本、归一化输入和必要的规则集标识。
4. **确定 Atlas 对引擎的依赖策略**：在 crates 未发布且仍处于 `0.1.0` 时，是同仓 workspace/path 依赖，还是固定 git revision。

