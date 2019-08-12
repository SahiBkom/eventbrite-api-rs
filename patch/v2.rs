use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    pub answer: String,
    pub attendee_id: String,
    pub question: String,
    pub question_id: String,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Attendee {
    pub affiliate: Option<String>,
    pub answers: Option<Vec<Answer>>,
    pub assigned_unit: Option<AttendeeAssignedUnit>,
    pub barcodes: Vec<AttendeeBarcode>,
    pub cancelled: bool,
    pub changed: String,
    pub checked_in: bool,
    pub costs: Cost,
    pub created: String,
    pub delivery_method: String,
    #[serde(default)] 
    pub event: Option<Event>,
    pub event_id: String,
    pub guestlist_id: Option<String>,
    pub id: String,
    pub invited_by: Option<String>,
    pub order: Option<Order>,
    pub order_id: String,
    pub profile: AttendeeProfile,
    pub questions: Option<Vec<Question>>,
    pub refunded: bool,
    pub resource_uri: String,
    pub status: String,
    pub team: Option<AttendeeTeam>,
    pub ticket_class_id: String,
    pub ticket_class_name: String,
    pub variant_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeAddress {
    pub bill: Option<Address>,
    pub home: Option<Address>,
    pub ship: Option<Address>,
    pub work: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeAssignedUnit {
    pub description: String,
    pub labels: Vec<String>,
    pub location_image: UnitLocationImage,
    pub titles: Vec<String>,
    pub unit_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeBarcode {
    pub barcode: String,
    pub changed: String,
    pub checkin_type: i64,
    pub created: String,
    pub is_printed: bool,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeProfile {
    pub addresses: AttendeeAddress,
    pub age: Option<i64>,
    pub birth_date: Option<String>,
    pub blog: Option<String>,
    pub cell_phone: Option<String>,
    pub company: Option<String>,
    pub email: String,
    pub first_name: String,
    pub gender: Option<String>,
    pub job_title: Option<String>,
    pub last_name: String,
    pub name: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub website: Option<String>,
    pub work_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeTeam {
    pub date_joined: String,
    pub event_id: String,
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeTeamCreatorEmails {
    pub email: String,
    pub primary: bool,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttendeeTeamResponse {
    pub attendee_count: i64,
    pub changed: String,
    pub created: String,
    pub creator: String,
    pub date_joined: String,
    pub event_id: String,
    pub id: String,
    pub name: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseAddress {
    pub address_1: String,
    pub address_2: String,
    pub city: String,
    pub country: String,
    pub latitude: String,
    pub longitude: String,
    pub postal_code: String,
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseQuestion {
    pub choices: Vec<QuestionChoices>,
    pub id: String,
    pub question: String,
    pub r#type: String,
    pub required: bool,
    pub resource_uri: String,
    pub respondent: String,
    pub ticket_classes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseTicketClass {
    pub auto_hide: Option<bool>,
    pub auto_hide_after: Option<String>,
    pub auto_hide_before: Option<String>,
    pub capacity: Option<i64>,
    pub category: String,
    pub cost: Option<TicketClassCost>,
    pub delivery_methods: Option<Vec<String>>,
    pub description: Option<String>,
    pub donation: Option<bool>,
    pub free: Option<bool>,
    pub hidden: Option<bool>,
    pub hide_description: Option<bool>,
    pub image_id: Option<String>,
    pub include_fee: Option<bool>,
    pub maximum_quantity: Option<i64>,
    pub minimum_quantity: Option<i64>,
    pub order_confirmation_message: Option<String>,
    pub quantity_sold: Option<i64>,
    pub quantity_total: Option<i64>,
    pub resource_uri: String,
    pub sales_channels: Option<Vec<String>>,
    pub sales_end: Option<String>,
    pub sales_start: Option<String>,
    pub sales_start_after: Option<String>,
    pub split_fee: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BasicInventoryInfo {
    pub has_add_ons: bool,
    pub has_admission_tiers: bool,
    pub has_donations: bool,
    pub has_inventory_tiers: bool,
    pub has_ticket_classes: bool,
    pub has_ticket_rules: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookmarkInfo {
    pub bookmarked: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CannedQuestion {
    pub choices: String,
    pub default_value: Option<String>,
    pub group_id: String,
    pub id: String,
    pub question: String,
    pub r#type: String,
    pub required: bool,
    pub resource_uri: String,
    pub respondent: String,
    pub ticket_classes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CannedQuestionChoices {
    pub answer: String,
    pub html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CannedQuestionCreate {
    pub canned_type: String,
    pub choices: Option<Vec<CannedQuestionChoices>>,
    pub display_answer_on_order: Option<bool>,
    pub parent_choice_id: Option<String>,
    pub question: Option<String>,
    pub r#type: Option<String>,
    pub required: Option<bool>,
    pub respondent: String,
    pub ticket_classes: Option<Vec<QuestionTicketClass>>,
    pub waiver: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub name_localized: String,
    pub resource_uri: String,
    pub short_name: String,
    pub short_name_localized: String,
    pub subcategories: Vec<Subcategory>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckoutSettings {
    pub changed: String,
    pub checkout_method: String,
    pub country_code: String,
    pub created: String,
    pub currency_code: String,
    pub offline_settings: Vec<OfflineSettings>,
    pub user_instrument_vault_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmationMessage {
    pub html: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerTicketVariant {
    pub amount_off: Option<CurrencyCost>,
    pub category: String,
    pub code: Option<String>,
    pub color: Option<String>,
    pub cost: Option<CurrencyCost>,
    pub description: Option<String>,
    pub display_name: String,
    pub fee: Option<CurrencyCost>,
    pub free: bool,
    pub id: String,
    pub image_id: Option<String>,
    pub name: Option<String>,
    pub original_cost: Option<CurrencyCost>,
    pub original_fee: Option<CurrencyCost>,
    pub original_tax: Option<CurrencyCost>,
    pub original_total_cost: Option<CurrencyCost>,
    pub percent_off: Option<String>,
    pub primary: Option<bool>,
    pub tax: Option<CurrencyCost>,
    pub tax_and_fee: Option<CurrencyCost>,
    pub total_cost: Option<CurrencyCost>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Continuation {
    pub continuation: Option<String>,
    pub has_more_items: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cost {
    pub base_price: CurrencyCost,
    pub eventbrite_fee: CurrencyCost,
    pub gross: CurrencyCost,
    pub payment_fee: CurrencyCost,
    pub tax: CurrencyCost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropMask {
    pub height: Option<i64>,
    pub top_left: Option<CropMaskCoordinate>,
    pub width: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CropMaskCoordinate {
    pub x: Option<i64>,
    pub y: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyCost {
    pub currency: String,
    pub display: String,
    pub major_value: String,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountBase {
    pub amount_off: String,
    pub code: String,
    pub end_date: String,
    pub end_date_relative: i64,
    pub event_id: String,
    pub hold_ids: Vec<String>,
    pub percent_off: String,
    pub quantity_available: i64,
    pub r#type: String,
    pub start_date: String,
    pub start_date_relative: i64,
    pub ticket_class_ids: Vec<String>,
    pub ticket_group_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub show_end_date: Option<bool>,
    pub show_facebook_friends_going: Option<bool>,
    pub show_map: Option<bool>,
    pub show_organizer_facebook: Option<bool>,
    pub show_organizer_twitter: Option<bool>,
    pub show_remaining: Option<bool>,
    pub show_start_date: Option<bool>,
    pub show_start_end_time: Option<bool>,
    pub show_timezone: Option<bool>,
    pub terminology: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_description: String,
    pub error_detail: String,
    pub status_code: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub bookmark_info: Option<BookmarkInfo>,
    pub capacity: i64,
    pub capacity_is_custom: bool,
    pub category: Option<Category>,
    pub changed: String,
    pub checkout_settings: Option<CheckoutSettings>,
    pub created: String,
    pub currency: String,
    pub description: Option<multipart_text>,
    pub end: datetime_tz,
    pub event_sales_status: Option<EventSalesStatus>,
    pub external_ticketing: Option<ExternalTicketing>,
    pub format: Option<Format>,
    pub hide_end_date: bool,
    pub hide_start_date: bool,
    pub id: String,
    pub invite_only: bool,
    pub is_externally_ticketed: bool,
    pub is_free: bool,
    pub is_locked: bool,
    pub is_reserved_seating: bool,
    pub is_series: bool,
    pub is_series_parent: bool,
    pub listed: bool,
    pub locale: String,
    pub logo: Option<image_logo>,
    pub logo_id: Option<String>,
    pub music_properties: Option<MusicProperties>,
    pub name: multipart_text,
    pub online_event: bool,
    pub organization_id: String,
    pub organizer: Option<Organizer>,
    pub organizer_id: String,
    pub password: String,
    pub privacy_setting: String,
    pub published: Option<String>,
    pub refund_policy: Option<String>,
    pub resource_uri: String,
    pub shareable: bool,
    pub show_colors_in_seatmap_thumbnail: bool,
    pub show_pick_a_seat: bool,
    pub show_remaining: bool,
    pub show_seatmap_thumbnail: bool,
    pub source: String,
    pub start: datetime_tz,
    pub status: Option<String>,
    pub subcategory: Option<Subcategory>,
    pub ticket_availability: Option<TicketAvailability>,
    pub tx_time_limit: String,
    pub url: String,
    pub vanity_url: Option<String>,
    pub venue: Option<Venue>,
    pub venue_id: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCopy {
    pub end_date: Option<String>,
    pub name: Option<String>,
    pub start_date: Option<String>,
    pub summary: Option<String>,
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventCreate {
    pub capacity: Option<i64>,
    pub category_id: Option<String>,
    pub currency: String,
    pub description: Option<htmltext>,
    pub end: datetime_tz_utc,
    pub format_id: Option<String>,
    pub hide_end_date: Option<bool>,
    pub hide_start_date: Option<bool>,
    pub invite_only: bool,
    pub is_reserved_seating: Option<bool>,
    pub listed: Option<bool>,
    pub logo_id: Option<String>,
    pub name: htmltext,
    pub online_event: Option<bool>,
    pub organizer_id: String,
    pub password: String,
    pub shareable: Option<bool>,
    pub show_colors_in_seatmap_thumbnail: Option<bool>,
    pub show_pick_a_seat: Option<bool>,
    pub show_remaining: Option<bool>,
    pub show_seatmap_thumbnail: Option<bool>,
    pub source: Option<String>,
    pub start: datetime_tz_utc,
    pub subcategory_id: Option<String>,
    pub summary: Option<String>,
    pub venue_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSalesStatus {
    pub message: Option<String>,
    pub message_code: Option<String>,
    pub message_type: Option<String>,
    pub sales_status: String,
    pub start_sales_date: datetime_tz,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSeriesCreate {
    pub capacity: Option<i64>,
    pub category_id: Option<String>,
    pub currency: String,
    pub description: Option<htmltext>,
    pub end: datetime_tz_utc,
    pub format_id: Option<String>,
    pub hide_end_date: Option<bool>,
    pub hide_start_date: Option<bool>,
    pub invite_only: Option<bool>,
    pub listed: Option<bool>,
    pub logo_id: Option<String>,
    pub name: htmltext,
    pub online_event: Option<bool>,
    pub organizer_id: Option<String>,
    pub password: Option<String>,
    pub shareable: Option<bool>,
    pub show_remaining: Option<bool>,
    pub start: datetime_tz_utc,
    pub subcategory_id: Option<String>,
    pub summary: Option<String>,
    pub venue_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSeriesDate {
    pub end: datetime_tz,
    pub id: String,
    pub locale: String,
    pub start: datetime_tz,
    pub status: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventSeriesDateCreate {
    pub end: datetime_tz_utc,
    pub start: datetime_tz_utc,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTextsContent {
    pub message_code: Option<String>,
    pub message_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTextsRequest {
    pub event_text_code: EventTextsRequestContent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTextsRequestContent {
    pub message: Option<String>,
    pub message_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTextsResponse {
    pub event_text_code: EventTextsContent,
    pub locale: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventUpdate {
    pub capacity: Option<i64>,
    pub category_id: Option<String>,
    pub currency: Option<String>,
    pub description: Option<htmltext>,
    pub end: Option<datetime_tz_utc>,
    pub format_id: Option<String>,
    pub hide_end_date: Option<bool>,
    pub hide_start_date: Option<bool>,
    pub invite_only: bool,
    pub is_reserved_seating: Option<bool>,
    pub listed: Option<bool>,
    pub logo_id: Option<String>,
    pub name: Option<htmltext>,
    pub online_event: Option<bool>,
    pub organizer_id: String,
    pub password: String,
    pub shareable: Option<bool>,
    pub show_colors_in_seatmap_thumbnail: Option<bool>,
    pub show_pick_a_seat: Option<bool>,
    pub show_remaining: Option<bool>,
    pub show_seatmap_thumbnail: Option<bool>,
    pub source: Option<String>,
    pub start: Option<datetime_tz_utc>,
    pub subcategory_id: Option<String>,
    pub summary: Option<String>,
    pub venue_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalIdentity {
    pub external_provider: String,
    pub external_user_id: String,
    pub id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalTicketing {
    pub external_url: String,
    pub is_free: bool,
    pub maximum_ticket_price: CurrencyCost,
    pub minimum_ticket_price: CurrencyCost,
    pub sales_end: String,
    pub sales_start: String,
    pub ticketing_provider_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeeRate {
    pub channel: String,
    pub country: String,
    pub currency: String,
    pub fee_name: String,
    pub fixed: CurrencyCost,
    pub item_type: String,
    pub maximum: Option<CurrencyCost>,
    pub minimum: Option<CurrencyCost>,
    pub payment_type: String,
    pub percent: String,
    pub plan: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Format {
    pub id: String,
    pub name: String,
    pub name_localized: String,
    pub resource_uri: String,
    pub short_name: String,
    pub short_name_localized: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FulfillmentSummary {
    pub data: String,
    pub event_ids: Vec<String>,
    pub timezone: String,
    pub topics_totals: String,
    pub totals: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    pub html: Option<String>,
    pub text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaUpload {
    pub r#type: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaUploadPost {
    pub crop_mask: Option<CropMask>,
    pub upload_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MusicProperties {
    pub age_restriction: Option<String>,
    pub door_time: Option<String>,
    pub presented_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub attendees: Option<Vec<Attendee>>,
    pub changed: String,
    pub costs: Option<Cost>,
    pub created: String,
    pub email: String,
    #[serde(default)] 
    pub event: Option<Event>,
    pub event_id: String,
    pub first_name: String,
    pub id: String,
    pub last_name: String,
    pub name: String,
    pub resource_uri: String,
    pub status: Option<String>,
    pub time_remaining: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub image_id: Option<String>,
    pub name: String,
    pub vertical: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organizer {
    pub description: multipart_text,
    pub facebook: Option<String>,
    pub id: String,
    pub logo: Option<image_logo>,
    pub logo_id: Option<String>,
    pub long_description: multipart_text,
    pub name: String,
    pub num_future_events: i64,
    pub num_past_events: i64,
    pub resource_uri: String,
    pub twitter: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
  //  pub continuation: Option<String>,
    pub has_more_items: bool,
    pub object_count: i64,
    pub page_count: i64,
    pub page_number: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentConstraintCreate {
    pub instrument_type: InstrumentTypeEnum,
    pub payment_method: Option<PaymentMethodEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricingresponseforitems {
    pub organizer_share: CurrencyCost,
    pub total_fees: CurrencyCost,
    pub total_taxes: CurrencyCost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub label: String,
    pub r#type: String,
    pub required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionChoices {
    pub answer: String,
    pub id: String,
    pub subquestion_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionChoicesAnswer {
    pub answer: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionCreate {
    pub choices: Option<Vec<QuestionChoicesAnswer>>,
    pub display_answer_on_order: Option<bool>,
    pub parent_choice_id: Option<String>,
    pub parent_id: Option<String>,
    pub question: Option<String>,
    pub r#type: Option<String>,
    pub required: Option<bool>,
    pub respondent: String,
    pub ticket_classes: Option<Vec<QuestionTicketClass>>,
    pub waiver: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionTicketClass {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDataAttendees {
    pub date: String,
    pub date_localized: String,
    pub totals: ReportTotalsAttendees,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportDataSales {
    pub date: String,
    pub date_localized: String,
    pub totals: ReportTotalsSales,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponseAttendees {
    pub data: Vec<ReportDataAttendees>,
    pub event_ids: Vec<String>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportResponseSales {
    pub data: Vec<ReportDataSales>,
    pub event_ids: Vec<String>,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportTotalsAttendees {
    pub num_attendees: i64,
    pub num_orders: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportTotalsSales {
    pub currency: String,
    pub fees: i64,
    pub gross: i64,
    pub net: i64,
    pub quantity: i64,
    pub royalty: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub description: String,
    pub id: String,
    pub name: String,
    pub organization_id: String,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScheduledEmailReport {
    pub id: String,
    pub recipients: Vec<String>,
    pub report_options: String,
    pub report_status: String,
    pub schedule_frequency: String,
    pub timezone: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeatMap {
    pub capacity: i64,
    pub event_id: String,
    pub name: Option<String>,
    pub thumbnail_url: Option<String>,
    pub venue_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredContentImageModuleData {
    pub alt: String,
    pub corner_style: String,
    pub display_size: String,
    pub image_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredContentModule {
    pub data: StructuredContentModuleData,
    pub r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredContentModuleData {
    pub body: Option<StructuredContentTextModuleData>,
    pub image: Option<StructuredContentImageModuleData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StructuredContentTextModuleData {
    pub alignment: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subcategory {
    pub id: String,
    pub name: String,
    pub parent_category: Category,
    pub resource_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOverridesRequestContent {
    pub message: Option<String>,
    pub message_code: Option<String>,
    pub text_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextOverridesResponseContent {
    pub default_message: String,
    pub message: Option<String>,
    pub message_code: Option<String>,
    pub message_type: String,
    pub text_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketAvailability {
    pub has_available_tickets: bool,
    pub is_sold_out: bool,
    pub maximum_ticket_price: CurrencyCost,
    pub minimum_ticket_price: CurrencyCost,
    pub start_sales_date: datetime_tz,
    pub waitlist_available: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketBuyerSettings {
    pub confirmation_message: Option<ConfirmationMessage>,
    pub event_id: Option<String>,
    pub instructions: Option<Instructions>,
    pub redirect_url: Option<String>,
    pub refund_request_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketBuyerSettingsUpdate {
    pub confirmation_message: Option<ConfirmationMessageUpdate>,
    pub instructions: Option<InstructionsUpdate>,
    pub redirect_url: Option<String>,
    pub refund_request_enabled: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketClassCost {
    pub actual_cost: CurrencyCost,
    pub actual_fee: CurrencyCost,
    pub cost: CurrencyCost,
    pub fee: CurrencyCost,
    pub tax: CurrencyCost,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketGroup {
    pub event_ticket_ids: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
    pub tickets: Option<TicketGroupTickets>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TicketGroupCreate {
    pub event_ticket_ids: Option<String>,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackingBeacon {
    pub event_id: String,
    pub pixel_id: String,
    pub triggers: Triggers,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitLocationImage {
    pub url: String,
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateExternalTicketing {
    pub external_url: Option<String>,
    pub is_free: Option<bool>,
    pub maximum_ticket_price: Option<CurrencyCost>,
    pub minimum_ticket_price: Option<CurrencyCost>,
    pub sales_end: Option<String>,
    pub sales_start: Option<String>,
    pub ticketing_provider_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub emails: Vec<UserEmail>,
    pub first_name: String,
    pub image_id: Option<String>,
    pub is_public: bool,
    pub last_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserEmail {
    pub email: String,
    pub verified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VenueBase {
    pub age_restriction: Option<String>,
    pub capacity: Option<i64>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct datetime_tz {
    pub local: String,
    pub timezone: String,
    pub utc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct datetime_tz_utc {
    pub timezone: String,
    pub utc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct htmltext {
    pub html: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct local_datetime {
    pub local: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct multipart_text {
    pub html: String,
    pub text: String,
}
