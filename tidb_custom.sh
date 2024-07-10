#!/bin/bash

# 替换yum源为阿里云镜像
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 替换yum源为阿里云镜像 >> /etc/tidb/metaforge_db.log
sed -e 's|^mirrorlist=|#mirrorlist=|g' \
    -e 's|^#baseurl=http://dl.rockylinux.org/\$contentdir|baseurl=https://mirrors.aliyun.com/rockylinux|g' \
    -i.bak /etc/yum.repos.d/rocky*.repo

# 更新yum缓存
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 更新yum缓存 >> /etc/tidb/metaforge_db.log
dnf makecache

# 更新所有包
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 更新所有包 >> /etc/tidb/metaforge_db.log
yum update -y
yum update -y
yum update -y

# 安装mysql 客户端
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 安装mysql 客户端 >> /etc/tidb/metaforge_db.log
yum install mysql -y

# 安装 nc 命令
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 安装 nc 命令 >> /etc/tidb/metaforge_db.log
yum install nc -y

# 等待 tidb 服务启动，例如通过检查端口或文件锁
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 等待 tidb 服务启动\，例如通过检查端口或文件锁 >> /etc/tidb/metaforge_db.log
while ! nc -z 127.0.0.1 4000; do
  sleep 1
done

# 导入MySQL数据库
echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 导入MySQL数据库 >> /etc/tidb/metaforge_db.log
mysql -uroot --host 127.0.0.1 --port 4000 -e 'source /etc/tidb/metaforge_db.sql'

echo $(date '+%Y-%m-%d %H:%M:%S.%9N') 成功导入MySQL数据库 >> /etc/tidb/metaforge_db.log