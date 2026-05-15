#                                           Vane

还没补齐的功能（按影响排序）
WebService 仍是 TCP 透传，不是完整 HTTP 反向代理

目前引擎是 TcpListener + copy_bidirectional，没有 Host/Path 路由匹配、Header 透传控制、HTTPS 强制跳转等 HTTP 语义。.

虽然有 /webservice/:id/routes 接口，但路由规则并未驱动真正的 HTTP 路由引擎。.

TLS 证书“签发/续期”未真实实现

issue_tls 现在只是写入字符串占位（"issued cert for ..."），不是真正 ACME 签发。.

TLS 引擎只是周期 metadata 检查证书文件存在性，不是自动续期与装载。.

PortForward 只支持 TCP，缺 UDP

明确判断非 TCP 直接返回，UDP 转发未实现。.

DDNS 仅 Cloudflare 单 provider

当前同步逻辑只实现 Cloudflare API 路径；原先多 provider（阿里云/DNSPod/腾讯云）仍缺。.

统计与日志多数是“接口有了、数据弱实现”

portforward stats 目前返回固定 0，占位值。.

Web 访问日志由管理接口手动 append，不是代理流量自动采集。.

IPFilter 目标语义仍较粗

已有 targets/upload，但规则匹配逻辑主要是全局 CIDR 命中放行，target 维度精细执行链路还不完整。.

会话与安全机制仍是简化版

会话是内存 map + 持久化元数据，缺 token 过期、签名、刷新策略等。.

系统能力缺口

旧版里的部分系统级行为（如更完整监控/系统信息/更细粒度运行控制）在 Rust 版仍未对齐（从当前路由与引擎可见）。.







> 轻量级网络服务管理工具 — 端口转发 · DDNS · Web服务 · TLS证书

<div align="center">
<br>
<img width="200" src="https://raw.githubusercontent.com/evecus/Vane/master/web/public/icon-512.png" alt="Vane">
<br>
</div>

## ✨ 功能特性

| 模块 | 功能 |
|------|------|
| 🔵 **端口转发** | TCP/UDP 端口转发，实时流量监控 |
| 🟢 **DDNS** | 动态域名，支持 Cloudflare / 阿里云 / DNSPod / 腾讯云 |
| 🟣 **Web 服务** | 反向代理，HTTP → HTTPS，多域名多后端 |
| 🟠 **TLS 证书** | Let's Encrypt DNS-01 自动申请续期 + 手动上传 |

## 🚀 快速开始

```bash
# 下载二进制
wget https://github.com/evecus/vane/releases/latest/download/vane-linux-amd64
chmod +x vane-linux-amd64

# 运行
./vane-linux-amd64 --config /path(可选)

# 访问管理界面
# http://your-ip:4455
# 默认账号: admin / vane1234  （请及时修改密码）
```

## 🏗️ 从源码构建

```bash
# 1. 构建前端
cd web && npm install && npm run build && cd ..

# 2. 构建 Rust 二进制
cargo build --release
cp target/release/vane ./vane

# 3. 运行
./vane --config /path(可选)
```

## 📦 项目结构

```
vane/
├── src/main.rs          # Rust 入口
├── config/              # 配置管理 (vane.json)
├── module/
│   ├── portforward/     # 端口转发
│   ├── ddns/            # DDNS
│   ├── webservice/      # 反向代理
│   └── tls/             # 证书管理
├── api/                 # REST API
├── web/                 # Vue3 前端
└── .github/workflows/   # CI/CD
```

## ⚙️ 配置文件

首次运行自动创建 `vane.json`，所有配置通过 Web 界面管理。

## 📄 License

MIT
