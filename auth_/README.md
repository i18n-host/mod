# 表结构

1. 创建表

请写mariadb表的创建语句,要求如下
id的类型为 BIGINT UNSIGNED NOT NULL AUTO_INCREMENT
ts的类型为 BIGINT UNSIGNED NOT NULL,默认值为当前unix时间的秒数
v的类型为 varbinary(255) , 并且唯一
所有字段都是NOT NULL的
其他id的组合,也需要联合唯一，表不需要写默认引擎、字符集等信息。

2. 创建函数

代码风格:SQL代码应尽量紧凑，减少不必要的换行。无注释
函数参数名称都用p_开头。
实现方法: 严格遵循 SELECT ... INTO a_variable; IF a_variable IS NULL THEN INSERT ...; SET a_variable = LAST_INSERT_ID(); END IF; 的逻辑模式。禁止使用 INSERT ... ON DUPLICATE KEY UPDATE 或 DECLARE HANDLER。
---

修改下面mariadb表authBrowserVer结构和authOsVer结构一致，然后修改对应的函数参数和调用，输出用```包裹


DROP DATABASE IF EXISTS dev;

CREATE DATABASE dev CHARACTER SET binary;

USE dev;

SET SESSION default_storage_engine = rocksdb;

CREATE TABLE authGpu (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authArch (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authModel (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authOsName (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authOsVer (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v1 INT UNSIGNED NOT NULL,
    v2 INT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v1, v2)
);

CREATE TABLE authBrowserName (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authBrowserVer (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v1 INT UNSIGNED NOT NULL,
    v2 INT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v1, v2)
);

CREATE TABLE authBrowserLang (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authOs (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name_id BIGINT UNSIGNED NOT NULL,
    ver_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_os (name_id, ver_id)
);

CREATE TABLE authBrowser (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name_id BIGINT UNSIGNED NOT NULL,
    lang_id BIGINT UNSIGNED NOT NULL,
    ver_id BIGINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_browser (name_id, lang_id, ver_id)
);

CREATE TABLE authDevice (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    model_id BIGINT UNSIGNED NOT NULL,
    arch_id BIGINT UNSIGNED NOT NULL,
    gpu_id BIGINT UNSIGNED NOT NULL,
    cpu_num MEDIUMINT UNSIGNED NOT NULL,
    w SMALLINT UNSIGNED NOT NULL,
    h SMALLINT UNSIGNED NOT NULL,
    dpi TINYINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_hardware (model_id, arch_id, gpu_id, cpu_num, w, h, dpi)
);

CREATE TABLE authSignInLog (
  id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  uid BIGINT UNSIGNED NOT NULL,
  ip VARBINARY(16) NOT NULL,
  device_id BIGINT UNSIGNED NOT NULL,
  browser_id BIGINT UNSIGNED NOT NULL,
  os_id BIGINT UNSIGNED NOT NULL,
  timezone TINYINT NOT NULL,
  ts BIGINT UNSIGNED NOT NULL DEFAULT UNIX_TIMESTAMP(),
  PRIMARY KEY (id)
);

DELIMITER $$

CREATE FUNCTION `authSignInLog`(
    p_uid BIGINT UNSIGNED,
    p_ip VARBINARY(16),
    p_timezone TINYINT,
    p_dpi TINYINT UNSIGNED,
    p_w SMALLINT UNSIGNED,
    p_h SMALLINT UNSIGNED,
    p_arch VARBINARY(255),
    p_model VARBINARY(255),
    p_cpu_num MEDIUMINT UNSIGNED,
    p_gpu VARBINARY(255),
    p_os_v1 INT UNSIGNED,
    p_os_v2 INT UNSIGNED,
    p_os_name VARBINARY(255),
    p_browser_name VARBINARY(255),
    p_browser_v1 INT UNSIGNED,
    p_browser_v2 INT UNSIGNED,
    p_browser_lang VARBINARY(255)
)
RETURNS BIGINT UNSIGNED
MODIFIES SQL DATA
BEGIN
    DECLARE v_gpu_id, v_arch_id, v_model_id, v_os_name_id, v_os_ver_id, v_browser_name_id, v_browser_ver_id, v_browser_lang_id, v_os_id, v_browser_id, v_device_id, v_log_id BIGINT UNSIGNED;

    SELECT id INTO v_gpu_id FROM authGpu WHERE v = p_gpu;
    IF v_gpu_id IS NULL THEN INSERT INTO authGpu (v) VALUES (p_gpu); SET v_gpu_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_arch_id FROM authArch WHERE v = p_arch;
    IF v_arch_id IS NULL THEN INSERT INTO authArch (v) VALUES (p_arch); SET v_arch_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_model_id FROM authModel WHERE v = p_model;
    IF v_model_id IS NULL THEN INSERT INTO authModel (v) VALUES (p_model); SET v_model_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_name_id FROM authOsName WHERE v = p_os_name;
    IF v_os_name_id IS NULL THEN INSERT INTO authOsName (v) VALUES (p_os_name); SET v_os_name_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_ver_id FROM authOsVer WHERE v1 = p_os_v1 AND v2 = p_os_v2;
    IF v_os_ver_id IS NULL THEN
        INSERT INTO authOsVer (v1, v2) VALUES (p_os_v1, p_os_v2);
        SET v_os_ver_id = LAST_INSERT_ID();
    END IF;

    SELECT id INTO v_browser_name_id FROM authBrowserName WHERE v = p_browser_name;
    IF v_browser_name_id IS NULL THEN INSERT INTO authBrowserName (v) VALUES (p_browser_name); SET v_browser_name_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_ver_id FROM authBrowserVer WHERE v1 = p_browser_v1 AND v2 = p_browser_v2;
    IF v_browser_ver_id IS NULL THEN
        INSERT INTO authBrowserVer (v1, v2) VALUES (p_browser_v1, p_browser_v2);
        SET v_browser_ver_id = LAST_INSERT_ID();
    END IF;

    SELECT id INTO v_browser_lang_id FROM authBrowserLang WHERE v = p_browser_lang;
    IF v_browser_lang_id IS NULL THEN INSERT INTO authBrowserLang (v) VALUES (p_browser_lang); SET v_browser_lang_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_id FROM authOs WHERE name_id = v_os_name_id AND ver_id = v_os_ver_id;
    IF v_os_id IS NULL THEN INSERT INTO authOs (name_id, ver_id) VALUES (v_os_name_id, v_os_ver_id); SET v_os_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_id FROM authBrowser WHERE name_id = v_browser_name_id AND lang_id = v_browser_lang_id AND ver_id = v_browser_ver_id;
    IF v_browser_id IS NULL THEN INSERT INTO authBrowser (name_id, lang_id, ver_id) VALUES (v_browser_name_id, v_browser_lang_id, v_browser_ver_id); SET v_browser_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_device_id FROM authDevice WHERE model_id = v_model_id AND arch_id = v_arch_id AND gpu_id = v_gpu_id AND cpu_num = p_cpu_num AND w = p_w AND h = p_h AND dpi = p_dpi;
    IF v_device_id IS NULL THEN
        INSERT INTO authDevice (model_id, arch_id, gpu_id, cpu_num, w, h, dpi) VALUES (v_model_id, v_arch_id, v_gpu_id, p_cpu_num, p_w, p_h, p_dpi);
        SET v_device_id = LAST_INSERT_ID();
    END IF;

    INSERT INTO authSignInLog (uid, ip, device_id, browser_id, os_id, timezone, ts) VALUES (p_uid, p_ip, v_device_id, v_browser_id, v_os_id, p_timezone, UNIX_TIMESTAMP());

    SET v_log_id = LAST_INSERT_ID();
    RETURN v_log_id;
END$$

DELIMITER ;

SELECT authSignInLog(
    1001,
    INET6_ATON('198.51.100.10'),
    -48,
    20,
    1920,
    1080,
    'x86_64',
    'XPS 15 9520',
    10,
    'NVIDIA GeForce RTX 3080',
    10,
    0,
    'Windows',
    'Chrome',
    108,
    0,
    'en-US'
);
