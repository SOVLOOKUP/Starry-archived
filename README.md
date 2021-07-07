<div align="center">
  <h1>✨Starries ｜ 繁星✨</h1>
  <text>分布式插件生态系统</text>
<!--   <a href="https://codecov.io/gh/SOVLOOKUP/starry">
    <img src="https://codecov.io/gh/SOVLOOKUP/starry/branch/master/graph/badge.svg?token=gtYREOVInH"/>
  </a> -->
</div>

---

1. [ ] 类似小程序的应用部署安装体验
2. [ ] 基于 esm + cdn 的应用市场

## Features

- Decentralized
- Open plugin system
- Fast and convenient

## Prepare

1. rust cargo

`cargo install tauri-bundler --force`

2. nodejs

`npm run install`

## Develop

1. Browser

`npm run dev`

2. Tauri desktop

`npm run tauri:dev` or `cargo tauri dev`

## Build

1. Output static dist

`npm run build`

2. Bundle release

`npm run release` or `cargo tauri build`

## Credit

- [svelte kit](https://kit.svelte.dev/) as framework
- [tauri](https://tauri.studio/) as frontend
- [starry](https://github.com/SOVLOOKUP/starry) as backend
