# lic

唔，就是一个生成 SPDX 的 LICENSE 的 cli 小工具。

## Install

```bash
cargo install lic
```

## Usage

```bash
lic new MIT --width 80 > LICENSE
```

Automaticly detect license field from package manager manifest.

-  Node.js: `package.json`
-  Rust: `Cargo.toml`
-  Python Poetry: `pyproject.toml`

```bash
lic auto --width 80
```

Search licenses from spdx list.

```bash
lic search gpl --number 20
```

## TODO List

-  [ ] `auto` 时如果当前层找不到的话向上层查找试试
