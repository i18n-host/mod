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
实现方法: 严格遵循 SELECT ... INTO a_variable; IF a_variable IS NULL THEN INSERT ...; SET a_variable = LAST_INSERT_ID(); END IF; 的逻辑模式。禁止使用 INSERT ... ON DUPLICATE KEY UPDATE 或 DECLARE HANDLER。
---

DROP DATABASE IF EXISTS dev;

CREATE DATABASE dev CHARACTER SET binary;

USE dev;

SET SESSION default_storage_engine = rocksdb;

CREATE TABLE authBrand (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

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

CREATE TABLE authOsName (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authOsVer (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authBrowserName (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
);

CREATE TABLE authBrowserVer (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    v VARBINARY(255) NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_v (v)
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
    brand_id BIGINT UNSIGNED NOT NULL,
    arch_id BIGINT UNSIGNED NOT NULL,
    gpu_id BIGINT UNSIGNED NOT NULL,
    cpu_num MEDIUMINT UNSIGNED NOT NULL COMMENT 'CPU核心数',
    w SMALLINT UNSIGNED NOT NULL COMMENT '分辨率宽',
    h SMALLINT UNSIGNED NOT NULL COMMENT '分辨率高',
    dpi TINYINT UNSIGNED NOT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY uq_hardware (brand_id, arch_id, gpu_id, cpu_num, w, h, dpi)
);

CREATE TABLE authSignInLog (
  id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
  uid BIGINT UNSIGNED NOT NULL,
  ip VARBINARY(16) NOT NULL,
  device_id BIGINT UNSIGNED NOT NULL,
  browser_id BIGINT UNSIGNED NOT NULL,
  os_id BIGINT UNSIGNED NOT NULL,
  timezone TINYINT NOT NULL COMMENT '时区, 来自JS: new Date().getTimezoneOffset() / 10',
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
    p_os_ver VARBINARY(255),
    p_arch VARBINARY(255),
    p_cpu_num MEDIUMINT UNSIGNED,
    p_gpu VARBINARY(255),
    p_brand VARBINARY(255),
    p_os_name VARBINARY(255),
    p_browser_name VARBINARY(255),
    p_browser_ver VARBINARY(255),
    p_browser_lang VARBINARY(255)
)
RETURNS BIGINT UNSIGNED
MODIFIES SQL DATA
BEGIN
    DECLARE v_brand_id, v_gpu_id, v_arch_id, v_os_name_id, v_os_ver_id, v_browser_name_id, v_browser_ver_id, v_browser_lang_id, v_os_id, v_browser_id, v_device_id, v_log_id BIGINT UNSIGNED;

    SELECT id INTO v_brand_id FROM authBrand WHERE v = p_brand;
    IF v_brand_id IS NULL THEN INSERT INTO authBrand (v) VALUES (p_brand); SET v_brand_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_gpu_id FROM authGpu WHERE v = p_gpu;
    IF v_gpu_id IS NULL THEN INSERT INTO authGpu (v) VALUES (p_gpu); SET v_gpu_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_arch_id FROM authArch WHERE v = p_arch;
    IF v_arch_id IS NULL THEN INSERT INTO authArch (v) VALUES (p_arch); SET v_arch_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_name_id FROM authOsName WHERE v = p_os_name;
    IF v_os_name_id IS NULL THEN INSERT INTO authOsName (v) VALUES (p_os_name); SET v_os_name_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_ver_id FROM authOsVer WHERE v = p_os_ver;
    IF v_os_ver_id IS NULL THEN INSERT INTO authOsVer (v) VALUES (p_os_ver); SET v_os_ver_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_name_id FROM authBrowserName WHERE v = p_browser_name;
    IF v_browser_name_id IS NULL THEN INSERT INTO authBrowserName (v) VALUES (p_browser_name); SET v_browser_name_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_ver_id FROM authBrowserVer WHERE v = p_browser_ver;
    IF v_browser_ver_id IS NULL THEN INSERT INTO authBrowserVer (v) VALUES (p_browser_ver); SET v_browser_ver_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_lang_id FROM authBrowserLang WHERE v = p_browser_lang;
    IF v_browser_lang_id IS NULL THEN INSERT INTO authBrowserLang (v) VALUES (p_browser_lang); SET v_browser_lang_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_os_id FROM authOs WHERE name_id = v_os_name_id AND ver_id = v_os_ver_id;
    IF v_os_id IS NULL THEN INSERT INTO authOs (name_id, ver_id) VALUES (v_os_name_id, v_os_ver_id); SET v_os_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_browser_id FROM authBrowser WHERE name_id = v_browser_name_id AND lang_id = v_browser_lang_id AND ver_id = v_browser_ver_id;
    IF v_browser_id IS NULL THEN INSERT INTO authBrowser (name_id, lang_id, ver_id) VALUES (v_browser_name_id, v_browser_lang_id, v_browser_ver_id); SET v_browser_id = LAST_INSERT_ID(); END IF;

    SELECT id INTO v_device_id FROM authDevice WHERE brand_id = v_brand_id AND arch_id = v_arch_id AND gpu_id = v_gpu_id AND cpu_num = p_cpu_num AND w = p_w AND h = p_h AND dpi = p_dpi;
    IF v_device_id IS NULL THEN
        INSERT INTO authDevice (brand_id, arch_id, gpu_id, cpu_num, w, h, dpi) VALUES (v_brand_id, v_arch_id, v_gpu_id, p_cpu_num, p_w, p_h, p_dpi);
        SET v_device_id = LAST_INSERT_ID();
    END IF;

    INSERT INTO authSignInLog (uid, ip, device_id, browser_id, os_id, timezone, ts) VALUES (p_uid, p_ip, v_device_id, v_browser_id, v_os_id, p_timezone, UNIX_TIMESTAMP());

    SET v_log_id = LAST_INSERT_ID();
    RETURN v_log_id;
END$$

DELIMITER ;

SELECT authSignInLog(
    1001,                           -- p_uid
    INET6_ATON('198.51.100.10'),    -- p_ip
    -48,                            -- p_timezone
    20,                             -- p_dpi
    1920,                           -- p_w
    1080,                           -- p_h
    '10.0',                         -- p_os_ver
    'x86_64',                       -- p_arch
    10,                             -- p_cpu_num
    'NVIDIA GeForce RTX 3080',      -- p_gpu
    'Dell',                         -- p_brand
    'Windows',                      -- p_os_name
    'Chrome',                       -- p_browser_name
    '108.0.0.0',                    -- p_browser_ver
    'en-US'                         -- p_browser_lang
);
