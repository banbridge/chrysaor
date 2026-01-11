FROM debian:trixie

LABEL authors="banbridge"

#ENV RUSTUP_DIST_SERVER="https://rsproxy.cn"
#ENV RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"

ENV RUSTUP_DIST_SERVER="https://mirrors.hust.edu.cn/rustup"
ENV RUSTUP_UPDATE_ROOT="https://mirrors.hust.edu.cn/rustup/rustup"

#CMD ["/bin/bash"]

SHELL ["/bin/bash", "-c"]

# 步骤 1：更新 Ubuntu 软件源并安装必要依赖
# 1. 替换 Ubuntu 国内 apt 源（阿里云，提升后续包安装速度）
# 2. 安装 curl（用于下载 rustup）、build-essential（提供 gcc 编译工具链）、ca-certificates（解决 HTTPS 证书验证问题）
# 3. 清理 apt 缓存，减小镜像体积
RUN  sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources \
    && sed -i 's|security.debian.org/debian-security|mirrors.tuna.tsinghua.edu.cn/debian-security|g' /etc/apt/sources.list.d/debian.sources \
    && apt-get update && apt-get install -y --no-install-recommends curl ca-certificates gcc wget libc6-dev \
    && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime && echo "Asia/Shanghai" > /etc/timezone \
    && apt-get clean && apt-get autoclean && rm -rf /var/lib/apt/lists/* && apt-get remove -y --auto-remove

# 步骤 2：安装 Rust 环境（通过 rustup 官方脚本，非交互安装）
# 1. 下载并执行 rustup 安装脚本，默认安装 stable 稳定版
# 2. 配置环境变量，使 cargo/rustc 全局可用
# 3. 验证 Rust 安装是否成功（可选，用于构建时校验）
RUN  curl https://mirrors.hust.edu.cn/rustup/rustup.sh > install.sh \
     && chmod +x install.sh \
     && ./install.sh -y \
     && rm install.sh \
     && source ~/.cargo/env \
     && rustup update \
     && rustc --version \
     && cargo --version

COPY config.toml /root/.cargo/config.toml

ENV PATH="~/.cargo/bin:${PATH}"