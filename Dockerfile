FROM registry.cn-beijing.aliyuncs.com/banbridge/build_base:rust-1.92-v1 AS builder
WORKDIR /app

ARG PACKAGE_NAME=""

# 3. 复制源代码并构建应用程序
COPY . .
# 根据你的项目调整构建参数，例如启用特定 features
RUN cargo build --release --bin ${PACKAGE_NAME}

################################################################################
# 阶段 3: 运行时 - 创建最终用于部署的精简镜像
# 目的：仅包含运行应用所必需的系统库和二进制文件，镜像极小。
################################################################################
FROM registry.cn-beijing.aliyuncs.com/banbridge/build_base:debian-trixie-slim-1.1 AS runtime
WORKDIR /app

ARG PACKAGE_NAME=""

COPY config/ /app/config/

# 从构建阶段复制编译好的二进制文件
COPY --from=builder /app/target/release/${PACKAGE_NAME} /app/

# 安全实践：使用非 root 用户运行
RUN useradd -m -u 1000 appuser
USER appuser

# 设置容器启动命令
ENTRYPOINT ["/app/${PACKAGE_NAME}"]
# 如果需要传递参数，可以使用 CMD
# CMD ["--help"]