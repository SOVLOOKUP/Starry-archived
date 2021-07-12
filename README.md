<div align="center">
    <h1>✨Starry ｜ 繁星✨</h1>
    <ul>去中心化生态系统</ul>
    <ul>构建全端跨平台生态，一次编写全端运行，并能通过加载插件拓展 API </ul>
<!--   <a href="https://codecov.io/gh/SOVLOOKUP/starry">
    <img src="https://codecov.io/gh/SOVLOOKUP/starry/branch/master/graph/badge.svg?token=gtYREOVInH"/>
  </a> -->
</div>

## 特性

- 使用 Rust 实现 API
- 一切皆插件
- 完全开源
- 兼容 PWA、utools 等应用
- 安全
- 去中心化插件市场
- 极小的安装文件 < 10M


## 兼容性

Tauri 规范是我们推荐的 API 规范，但是我们还计划兼容以下规范:

- [ ] Tauri
- [ ] PWA
- [ ] utools
- [ ] uniapp

平台适配情况:

- [x] Windows

- [x] Macos

- [x] Linux

- [ ] Andorid
- [ ] IOS

## Todo

- [ ] Rust plugin extend

## Contribution

### Prepare

1. rust cargo

`cargo install tauri-bundler --force`

2. nodejs

`npm run install`

### Develop

1. Browser

`npm run dev`

2. Tauri desktop

`npm run start`

### Anti Bloat

compress to at most 959KB

```
rustup toolchain install nightly && cargo +nightly build --release -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu
strip your-app
upx --lzma your-app
```

## Credit

- [svelte kit](https://kit.svelte.dev/)
- [tauri](https://tauri.studio/)

