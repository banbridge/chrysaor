FROM debian:trixie-slim

LABEL authors="banbridge"

#CMD ["/bin/bash"]

SHELL ["/bin/bash", "-c"]

# 步骤 1：更新 Ubuntu 软件源并安装必要依赖
# 1. 替换 Ubuntu 国内 apt 源（阿里云，提升后续包安装速度）
# 2. 安装 curl（用于下载 rustup）、build-essential（提供 gcc 编译工具链）、ca-certificates（解决 HTTPS 证书验证问题）
# 3. 清理 apt 缓存，减小镜像体积
RUN  sed -i 's/deb.debian.org/mirrors.tuna.tsinghua.edu.cn/g' /etc/apt/sources.list.d/debian.sources \
    && sed -i 's|security.debian.org/debian-security|mirrors.tuna.tsinghua.edu.cn/debian-security|g' /etc/apt/sources.list.d/debian.sources \

