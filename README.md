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

## TODO List

-  [ ] 子命令 `auto`，自动检测各种语言配置文件的 `license` 字段进行生成
   -  [ ] `package.json`
   -  [ ] `pyproject.toml`
   -  [ ] `Cargo.toml`
