-- Add up migration script here
ALTER TABLE todos
    MODIFY COLUMN id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT;