CREATE FUNCTION `authSignInLog`(p_uid BIGINT UNSIGNED,p_ip VARBINARY(16),p_timezone TINYINT,p_dpi TINYINT UNSIGNED,p_w SMALLINT UNSIGNED,p_h SMALLINT UNSIGNED,p_os_ver VARBINARY(255),p_arch VARBINARY(255),p_cpu_num MEDIUMINT UNSIGNED,p_gpu VARBINARY(255),p_brand VARBINARY(255),p_os_name VARBINARY(255),p_browser_name VARBINARY(255),p_browser_ver INT UNSIGNED,p_browser_lang VARBINARY(255)
) RETURNS BIGINT UNSIGNED
    MODIFIES SQL DATA
BEGIN
    DECLARE v_brand_id,v_gpu_id,v_arch_id,v_os_name_id,v_os_ver_id,v_browser_name_id,v_browser_ver_id,v_browser_lang_id,v_os_id,v_browser_id,v_device_id,v_log_id BIGINT UNSIGNED;
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
    IF v_os_id IS NULL THEN INSERT INTO authOs (name_id,ver_id) VALUES (v_os_name_id,v_os_ver_id); SET v_os_id = LAST_INSERT_ID(); END IF;
    SELECT id INTO v_browser_id FROM authBrowser WHERE name_id = v_browser_name_id AND lang_id = v_browser_lang_id AND ver_id = v_browser_ver_id;
    IF v_browser_id IS NULL THEN INSERT INTO authBrowser (name_id,lang_id,ver_id) VALUES (v_browser_name_id,v_browser_lang_id,v_browser_ver_id); SET v_browser_id = LAST_INSERT_ID(); END IF;
    SELECT id INTO v_device_id FROM authDevice WHERE brand_id = v_brand_id AND arch_id = v_arch_id AND gpu_id = v_gpu_id AND cpu_num = p_cpu_num AND w = p_w AND h = p_h AND dpi = p_dpi;
    IF v_device_id IS NULL THEN
        INSERT INTO authDevice (brand_id,arch_id,gpu_id,cpu_num,w,h,dpi) VALUES (v_brand_id,v_arch_id,v_gpu_id,p_cpu_num,p_w,p_h,p_dpi);
        SET v_device_id = LAST_INSERT_ID();
    END IF;
    INSERT INTO authSignInLog (uid,ip,device_id,browser_id,os_id,timezone,ts) VALUES (p_uid,p_ip,v_device_id,v_browser_id,v_os_id,p_timezone,UNIX_TIMESTAMP());
    SET v_log_id = LAST_INSERT_ID();
    RETURN v_log_id;
END ;;