pub struct NotificationService;

impl NotificationService {
    pub async fn send_settle_notification(_activity_id: i64) {}
    pub async fn send_inventory_alert(_store_id: i64, _item_id: i64) {}
}
