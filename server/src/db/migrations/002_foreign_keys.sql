-- Add foreign key constraints for referential integrity
-- Migration 002: All tables that reference other tables now have proper FK constraints

-- store -> region
ALTER TABLE store ADD CONSTRAINT fk_store_region FOREIGN KEY (region_id) REFERENCES region(id);

-- app_user -> region
ALTER TABLE app_user ADD CONSTRAINT fk_app_user_region FOREIGN KEY (region_id) REFERENCES region(id);

-- user_identity -> app_user
ALTER TABLE user_identity ADD CONSTRAINT fk_user_identity_user FOREIGN KEY (user_id) REFERENCES app_user(id);

-- activity -> region
ALTER TABLE activity ADD CONSTRAINT fk_activity_region FOREIGN KEY (region_id) REFERENCES region(id);

-- activity_rule -> activity
ALTER TABLE activity_rule ADD CONSTRAINT fk_activity_rule_activity FOREIGN KEY (activity_id) REFERENCES activity(id);

-- design_template -> activity (if template is activity-scoped)
-- Note: design_template has no activity_id in current schema, skip

-- ai_generation_record -> app_user, activity
ALTER TABLE ai_generation_record ADD CONSTRAINT fk_ai_gen_user FOREIGN KEY (user_id) REFERENCES app_user(id);
ALTER TABLE ai_generation_record ADD CONSTRAINT fk_ai_gen_activity FOREIGN KEY (activity_id) REFERENCES activity(id);

-- contest_entry -> activity, app_user
ALTER TABLE contest_entry ADD CONSTRAINT fk_entry_activity FOREIGN KEY (activity_id) REFERENCES activity(id);
ALTER TABLE contest_entry ADD CONSTRAINT fk_entry_user FOREIGN KEY (user_id) REFERENCES app_user(id);

-- vote_record -> activity, contest_entry, app_user
ALTER TABLE vote_record ADD CONSTRAINT fk_vote_activity FOREIGN KEY (activity_id) REFERENCES activity(id);
ALTER TABLE vote_record ADD CONSTRAINT fk_vote_entry FOREIGN KEY (entry_id) REFERENCES contest_entry(id);
ALTER TABLE vote_record ADD CONSTRAINT fk_vote_user FOREIGN KEY (voter_user_id) REFERENCES app_user(id);

-- risk_event -> activity
ALTER TABLE risk_event ADD CONSTRAINT fk_risk_event_activity FOREIGN KEY (activity_id) REFERENCES activity(id);

-- winner_record -> activity, contest_entry, app_user
ALTER TABLE winner_record ADD CONSTRAINT fk_winner_activity FOREIGN KEY (activity_id) REFERENCES activity(id);
ALTER TABLE winner_record ADD CONSTRAINT fk_winner_entry FOREIGN KEY (entry_id) REFERENCES contest_entry(id);
ALTER TABLE winner_record ADD CONSTRAINT fk_winner_user FOREIGN KEY (user_id) REFERENCES app_user(id);

-- reward_order -> winner_record, store
ALTER TABLE reward_order ADD CONSTRAINT fk_order_winner FOREIGN KEY (winner_id) REFERENCES winner_record(id);
ALTER TABLE reward_order ADD CONSTRAINT fk_order_store FOREIGN KEY (store_id) REFERENCES store(id);

-- production_batch -> reward_order (via production_task.order_id, not a direct FK)
-- production_batch has no order_id column — batch groups tasks that may span multiple orders
-- ALTER TABLE production_batch ADD CONSTRAINT fk_production_batch_order FOREIGN KEY (order_id) REFERENCES reward_order(id);

-- production_task -> production_batch, store, reward_order
ALTER TABLE production_task ADD CONSTRAINT fk_production_task_batch FOREIGN KEY (batch_id) REFERENCES production_batch(id);
ALTER TABLE production_task ADD CONSTRAINT fk_production_task_store FOREIGN KEY (store_id) REFERENCES store(id);
ALTER TABLE production_task ADD CONSTRAINT fk_production_task_order FOREIGN KEY (order_id) REFERENCES reward_order(id);

-- redeem_code -> reward_order
ALTER TABLE redeem_code ADD CONSTRAINT fk_redeem_code_order FOREIGN KEY (order_id) REFERENCES reward_order(id);

-- redeem_record -> reward_order, redeem_code, store
ALTER TABLE redeem_record ADD CONSTRAINT fk_redeem_record_order FOREIGN KEY (order_id) REFERENCES reward_order(id);
ALTER TABLE redeem_record ADD CONSTRAINT fk_redeem_record_code FOREIGN KEY (redeem_code_id) REFERENCES redeem_code(id);
ALTER TABLE redeem_record ADD CONSTRAINT fk_redeem_record_store FOREIGN KEY (store_id) REFERENCES store(id);

-- inventory_item -> store
ALTER TABLE inventory_item ADD CONSTRAINT fk_inventory_store FOREIGN KEY (store_id) REFERENCES store(id);

-- inventory_txn -> inventory_item
ALTER TABLE inventory_txn ADD CONSTRAINT fk_inventory_txn_item FOREIGN KEY (item_id) REFERENCES inventory_item(id);

-- staff -> store
ALTER TABLE staff ADD CONSTRAINT fk_staff_store FOREIGN KEY (store_id) REFERENCES store(id);

-- attendance_record -> staff, store
ALTER TABLE attendance_record ADD CONSTRAINT fk_attendance_staff FOREIGN KEY (staff_id) REFERENCES staff(id);
ALTER TABLE attendance_record ADD CONSTRAINT fk_attendance_store FOREIGN KEY (store_id) REFERENCES store(id);
