SET NAMES utf8mb4;

CREATE TABLE users (
    `id` BIGINT(20) UNSIGNED NOT NULL PRIMARY KEY,
    `username` VARCHAR(128) NOT NULL,
    `email` VARCHAR(255) NOT NULL,
    `password` VARCHAR(128) NOT NULL,
    `description` TEXT,
    `authenticated_at` DATETIME DEFAULT NULL,
    `created_at` DATETIME DEFAULT CURRENT_TIMESTAMP,
    `deleted_at` DATETIME DEFAULT NULL,
    KEY `idx_users_username` (`username`),
    KEY `idx_users_email` (`email`),
    KEY `idx_users_authenticated_at` (`authenticated_at`),
    KEY `idx_users_deleted_at` (`deleted_at`)
) ENGINE=InnoDB CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;
