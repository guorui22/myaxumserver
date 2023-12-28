FROM ubuntu:22.04
LABEL authors="grpc"
# 设置时区环境变量为上海
ENV TZ=Asia/Shanghai

COPY target/release/libgrpc  /grpc/
# 设置容器时区
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
# 安装 net-tools (为了 ifconfig), netcat 和 iputils-ping (为了 ping), curl 和 vim
RUN apt-get update && apt-get install -y net-tools netcat iputils-ping curl vim
# 清除软件包缓存
RUN apt-get clean
RUN apt-get autoremove -y

ENTRYPOINT ["/grpc/libgrpc"]