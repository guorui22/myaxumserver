-- 创建数据库 metaforge_db
CREATE DATABASE `metaforge_db` /*!40100 DEFAULT CHARACTER SET utf8mb4 */;

-- 系统用户表
CREATE TABLE `sys_user` (
  `id` varchar(64) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL COMMENT '用户ID',
  `user_code` varchar(64) NOT NULL COMMENT '用户编号',
  `user_name` varchar(128) DEFAULT NULL COMMENT '用户名称',
  `user_password` varchar(128) NOT NULL COMMENT '用户密码',
  `status` tinyint(4) NOT NULL DEFAULT '0' COMMENT '用户状态(0-正常，1-删除，2-冻结)',
  `submit_time` timestamp NOT NULL COMMENT '提交时间',
  `submit_user` varchar(64) DEFAULT NULL COMMENT '提交用户(编号)',
  PRIMARY KEY (`id`) /*T![clustered_index] NONCLUSTERED */,
  UNIQUE KEY `sys_user_code` (`user_code`),
  KEY `sys_user_user_name_IDX` (`user_name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='系统用户表';

-- 系统用户表 TTL
CREATE TABLE `sys_user_ttl` (
  `id` varchar(64) CHARACTER SET utf8 COLLATE utf8_bin NOT NULL COMMENT '用户ID',
  `user_code` varchar(64) NOT NULL COMMENT '用户编号',
  `user_name` varchar(128) DEFAULT NULL COMMENT '用户名称',
  `user_password` varchar(128) NOT NULL COMMENT '用户密码',
  `status` tinyint(4) NOT NULL DEFAULT '0' COMMENT '用户状态(0-正常，1-删除，2-冻结)',
  `submit_time` timestamp NOT NULL COMMENT '提交时间',
  `submit_user` varchar(64) NOT NULL COMMENT '提交用户(编号)',
  KEY `sys_user_user_name_IDX` (`user_name`),
  PRIMARY KEY (`id`,`submit_time`,`submit_user`) /*T![clustered_index] NONCLUSTERED */
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_bin COMMENT='系统用户表';

-- 系统用户数据
INSERT INTO metaforge_db.sys_user (id,user_code,user_name,user_password,status,submit_time,submit_user) VALUES
	 ('10000','07788','郭睿_1','123581321',0,'2024-05-10 18:12:52','07788'),
	 ('10001','07799','郭紫姝','123581321',0,'2024-05-10 20:38:52','07788');

-- 系统用户数据 TTL
INSERT INTO metaforge_db.sys_user_ttl (id,user_code,user_name,user_password,status,submit_time,submit_user) VALUES
	 ('10000','07788','郭睿_1','123581321',0,'2024-05-10 18:12:52','07788'),
	 ('10001','07799','郭紫姝','123581321',0,'2024-05-10 20:38:52','07788');
