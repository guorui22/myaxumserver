# 指定基础镜像为ubuntu:22.04版本
FROM ubuntu:22.04

# 为镜像添加一个标签，标明作者为metaforge
LABEL authors="郭睿"

# 设置容器内的工作目录为/metaforge
WORKDIR /metaforge

# 将当前上下文中的target/release/metaforge复制到容器内的/metaforge目录下
COPY target/release/metaforge /metaforge/

# 将当前上下文中的metaforge/conf目录复制到容器内的/metaforge/conf目录下
COPY metaforge/conf /metaforge/conf

# 将当前上下文中的metaforge/templates目录复制到容器内的/metaforge/templates目录下
COPY metaforge/templates /metaforge/templates

# 将当前上下文中的metaforge/www目录复制到容器内的/metaforge/www目录下
COPY metaforge/www /metaforge/www

# 更新apt包索引，然后安装net-tools（用于ifconfig命令）、netcat(端口扫描，比如 nc -zvw3 127.0.0.1 5000)、iputils-ping（用于ping命令）、curl和vim(文本编辑)
# 这些软件包用于网络诊断和文件编辑
RUN apt-get update && apt-get install -y net-tools netcat iputils-ping curl vim

# 设置环境变量DEBIAN_FRONTEND为noninteractive，以非交互模式安装tzdata（时区数据包）
RUN DEBIAN_FRONTEND="noninteractive" apt-get update && apt-get install -y tzdata

# 创建符号链接，将上海时区设置为容器的默认时区
RUN ln -fs /usr/share/zoneinfo/Asia/Shanghai /etc/localtime

# 重新配置tzdata，以应用时区设置
RUN dpkg-reconfigure -f noninteractive tzdata

# 设置容器启动时执行的命令，即运行/metaforge目录下的metaforge程序
ENTRYPOINT ["/metaforge/metaforge"]

# Dockerfile文件使用说明：
# 1. 生成镜像文件：docker build -f ./Dockerfile -t harbor.sunnercn.com/library/metaforge:v0.1.0 ./
# 2. 登录私有镜像仓库(用户：admin 密码：Harbor_sn1983)：docker login -u admin harbor.sunnercn.com
# 3. 发布镜像：docker push harbor.sunnercn.com/library/metaforge:v0.1.0
# 4. 下载镜像：docker pull harbor.sunnercn.com/library/metaforge:v0.1.0
# 5. 运行容器：docker run -tid --rm -p 5000:5000 -p 29029:29029 --name metaforge-server harbor.sunnercn.com/library/metaforge:v0.1.0
# 6. 停止容器：docker stop metaforge-server
# 7. 进入容器终端：docker exec -it metaforge-server /bin/bash
# 8. 查看容器日志：docker exec -it metaforge-server tail -f /home/gr/桌面/my_log.2024-07-03
# 9. 删除容器：docker rm metaforge-server
# 10. 删除镜像：docker rmi harbor.sunnercn.com/library/metaforge:v0.1.0
# 11. 查看镜像：docker images
# 12. 查看容器：docker ps -a
# 13. 查看容器IP：docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' metaforge-server
# 14. 查看容器端口：docker port metaforge-server
# 15. 查看容器内进程：docker top metaforge-server
# 16. 查看容器内环境变量：docker exec -it metaforge-server env
# 17. 查看容器内网络：docker exec -it metaforge-server ifconfig
# 18. 查看容器内进程：docker exec -it metaforge-server ps -ef
# 19. 查看镜像版本：docker run --rm redis:latest --version
# 20. 重置 docker 服务以恢复网络：sudo systemctl restart docker
