FROM ubuntu:22.04
LABEL authors="metaforge"

WORKDIR /metaforge
COPY target/release/metaforge /metaforge/
COPY metaforge/conf /metaforge/conf
COPY metaforge/templates /metaforge/templates
COPY metaforge/www /metaforge/www
# 安装 net-tools (为了 ifconfig), netcat 和 iputils-ping (为了 ping), curl 和 vim
RUN apt-get update && apt-get install -y net-tools netcat iputils-ping curl vim
# 设置时区环境变量为上海
RUN DEBIAN_FRONTEND="noninteractive" apt-get update && apt-get install -y tzdata
RUN ln -fs /usr/share/zoneinfo/Asia/Shanghai /etc/localtime
RUN dpkg-reconfigure -f noninteractive tzdata

ENTRYPOINT ["/metaforge/metaforge"]