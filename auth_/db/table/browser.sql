CREATE TABLE `authBrowser` (
  `id` BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,`name_id` BIGINT UNSIGNED NOT NULL,`lang_id` BIGINT UNSIGNED NOT NULL,`ver_id` BIGINT UNSIGNED NOT NULL,PRIMARY KEY (`id`),UNIQUE KEY `uq_browser` (`name_id`,`lang_id`,`ver_id`)
);