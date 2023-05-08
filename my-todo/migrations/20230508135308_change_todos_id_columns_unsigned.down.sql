-- Add down migration script here
ALTER TABLE todos
    MODIFY COLUMN id BIGINT NOT NULL AUTO_INCREMENT;
