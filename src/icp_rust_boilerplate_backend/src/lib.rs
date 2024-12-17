#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// UserRole Enum
#[derive(
    candid::CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug,
)]
enum UserRole {
    #[default]
    Admin,
    CampaignManager,
    Donor,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Campaign {
    id: u64,
    name: String,
    description: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Donation {
    id: u64,
    campaign_id: u64,
    donor_name: String,
    amount: u64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Expense {
    id: u64,
    campaign_id: u64,
    description: String,
    amount: u64,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct VoterOutreach {
    id: u64,
    campaign_id: u64,
    activity: String,
    date: u64,
    status: String, // e.g., "planned", "completed"
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct SecureMessage {
    id: u64,
    campaign_id: u64,
    sender: String,
    content: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Notification {
    id: u64,
    campaign_id: u64,
    message: String,
    created_at: u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    role: UserRole,
    created_at: u64,
}

impl Storable for Campaign {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Campaign {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Donation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Donation {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Expense {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Expense {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for VoterOutreach {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for VoterOutreach {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for SecureMessage {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for SecureMessage {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Notification {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Notification {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static CAMPAIGN_STORAGE: RefCell<StableBTreeMap<u64, Campaign, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static DONATION_STORAGE: RefCell<StableBTreeMap<u64, Donation, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static EXPENSE_STORAGE: RefCell<StableBTreeMap<u64, Expense, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static OUTREACH_STORAGE: RefCell<StableBTreeMap<u64, VoterOutreach, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));

    static MESSAGE_STORAGE: RefCell<StableBTreeMap<u64, SecureMessage, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));

    static NOTIFICATION_STORAGE: RefCell<StableBTreeMap<u64, Notification, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6)))
    ));

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(7)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct CampaignPayload {
    name: String,
    description: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct DonationPayload {
    campaign_id: u64,
    donor_name: String,
    amount: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct ExpensePayload {
    campaign_id: u64,
    description: String,
    amount: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct VoterOutreachPayload {
    campaign_id: u64,
    activity: String,
    date: u64,
    status: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct MessagePayload {
    campaign_id: u64,
    sender: String,
    content: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct UpdateCampaignPayload {
    id: u64,
    name: Option<String>,
    description: Option<String>,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    role: UserRole,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    UnAuthorized(String),
}

#[ic_cdk::update]
fn create_campaign(
    payload: CampaignPayload,
    user_payload: UserPayload,
) -> Result<Campaign, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::Admin && user.role != UserRole::CampaignManager {
        return Err(Message::UnAuthorized(
            "You do not have permission to create a campaign".to_string(),
        ));
    }

    // Ensure 'name' and 'description' are provided
    if payload.name.is_empty() || payload.description.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'name' and 'description' are provided.".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let campaign = Campaign {
        id,
        name: payload.name,
        description: payload.description,
        created_at: current_time(),
    };
    CAMPAIGN_STORAGE.with(|storage| storage.borrow_mut().insert(id, campaign.clone()));
    notify_participants(campaign.id, "New campaign created.".to_string());
    Ok(campaign)
}

#[ic_cdk::update]
fn update_campaign(payload: UpdateCampaignPayload) -> Result<Campaign, Message> {
    CAMPAIGN_STORAGE.with(|storage| {
        if let Some(mut campaign) = storage.borrow_mut().get(&payload.id) {
            if let Some(name) = payload.name {
                campaign.name = name;
            }
            if let Some(description) = payload.description {
                campaign.description = description;
            }
            storage.borrow_mut().insert(payload.id, campaign.clone());
            notify_participants(campaign.id, "Campaign updated.".to_string());
            Ok(campaign)
        } else {
            Err(Message::NotFound("Campaign not found".to_string()))
        }
    })
}

#[ic_cdk::query]
fn get_campaigns() -> Result<Vec<Campaign>, Message> {
    CAMPAIGN_STORAGE.with(|storage| {
        let campaigns: Vec<Campaign> = storage
            .borrow()
            .iter()
            .map(|(_, campaign)| campaign.clone())
            .collect();

        if campaigns.is_empty() {
            Err(Message::NotFound("No campaigns found".to_string()))
        } else {
            Ok(campaigns)
        }
    })
}

#[ic_cdk::query]
fn get_campaign_by_id(id: u64) -> Result<Campaign, Message> {
    CAMPAIGN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, campaign)| campaign.id == id)
            .map(|(_, campaign)| campaign.clone())
            .ok_or(Message::NotFound("Campaign not found".to_string()))
    })
}

#[ic_cdk::update]
fn create_donation(
    payload: DonationPayload,
    user_payload: UserPayload,
) -> Result<Donation, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::Donor {
        return Err(Message::UnAuthorized(
            "You do not have permission to create a donation.".to_string(),
        ));
    }

    // Ensure 'donor_name' and 'amount' are provided
    if payload.donor_name.is_empty() || payload.amount == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'donor_name' and 'amount' are provided.".to_string(),
        ));
    }

    // Check if the campaign exists
    let campaign = CAMPAIGN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, campaign)| campaign.id == payload.campaign_id)
            .map(|(_, campaign)| campaign.clone())
    });
    if campaign.is_none() {
        return Err(Message::NotFound("Campaign not found".to_string()));
    }

    // Create a new donation
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let donation = Donation {
        id,
        campaign_id: payload.campaign_id,
        donor_name: payload.donor_name,
        amount: payload.amount,
        created_at: current_time(),
    };
    DONATION_STORAGE.with(|storage| storage.borrow_mut().insert(id, donation.clone()));
    notify_participants(donation.campaign_id, "New donation received.".to_string());
    Ok(donation)
}

#[ic_cdk::query]
fn get_donations(campaign_id: u64) -> Result<Vec<Donation>, Message> {
    DONATION_STORAGE.with(|storage| {
        let donations: Vec<Donation> = storage
            .borrow()
            .iter()
            .filter(|(_, donation)| donation.campaign_id == campaign_id)
            .map(|(_, donation)| donation.clone())
            .collect();

        if donations.is_empty() {
            Err(Message::NotFound("No donations found".to_string()))
        } else {
            Ok(donations)
        }
    })
}

#[ic_cdk::update]
fn create_expense(payload: ExpensePayload, user_payload: UserPayload) -> Result<Expense, Message> {
    // Authenticate the user
    let user = authenticate_user(user_payload)?;
    if user.role != UserRole::CampaignManager && user.role != UserRole::Admin {
        return Err(Message::UnAuthorized(
            "You do not have permission to create an expense.".to_string(),
        ));
    }

    // Ensure 'description' and 'amount' are provided
    if payload.description.is_empty() || payload.amount == 0 {
        return Err(Message::InvalidPayload(
            "Ensure 'description' and 'amount' are provided.".to_string(),
        ));
    }

    // Check if the campaign exists
    let campaign = CAMPAIGN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, campaign)| campaign.id == payload.campaign_id)
            .map(|(_, campaign)| campaign.clone())
    });
    if campaign.is_none() {
        return Err(Message::NotFound("Campaign not found".to_string()));
    }

    // Create a new expense
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let expense = Expense {
        id,
        campaign_id: payload.campaign_id,
        description: payload.description,
        amount: payload.amount,
        created_at: current_time(),
    };
    // Insert the expense into the storage
    EXPENSE_STORAGE.with(|storage| storage.borrow_mut().insert(id, expense.clone()));
    notify_participants(expense.campaign_id, "New expense added.".to_string());
    Ok(expense)
}

#[ic_cdk::query]
fn get_expenses(campaign_id: u64) -> Result<Vec<Expense>, Message> {
    EXPENSE_STORAGE.with(|storage| {
        let expenses: Vec<Expense> = storage
            .borrow()
            .iter()
            .filter(|(_, expense)| expense.campaign_id == campaign_id)
            .map(|(_, expense)| expense.clone())
            .collect();

        if expenses.is_empty() {
            Err(Message::NotFound("No expenses found".to_string()))
        } else {
            Ok(expenses)
        }
    })
}

#[ic_cdk::update]
fn create_voter_outreach(payload: VoterOutreachPayload) -> Result<VoterOutreach, Message> {
    // Ensure 'activity', 'date', and 'status' are provided
    if payload.activity.is_empty() || payload.status.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'activity' and 'status' are provided.".to_string(),
        ));
    }

    // Check if the campaign exists
    let campaign = CAMPAIGN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, campaign)| campaign.id == payload.campaign_id)
            .map(|(_, campaign)| campaign.clone())
    });
    if campaign.is_none() {
        return Err(Message::NotFound("Campaign not found".to_string()));
    }

    // Create a new voter outreach activity
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let outreach = VoterOutreach {
        id,
        campaign_id: payload.campaign_id,
        activity: payload.activity,
        date: payload.date,
        status: payload.status,
        created_at: current_time(),
    };

    // Insert the voter outreach activity into the storage
    OUTREACH_STORAGE.with(|storage| storage.borrow_mut().insert(id, outreach.clone()));
    Ok(outreach)
}

#[ic_cdk::query]
fn get_voter_outreach(campaign_id: u64) -> Result<Vec<VoterOutreach>, Message> {
    OUTREACH_STORAGE.with(|storage| {
        let outreach: Vec<VoterOutreach> = storage
            .borrow()
            .iter()
            .filter(|(_, outreach)| outreach.campaign_id == campaign_id)
            .map(|(_, outreach)| outreach.clone())
            .collect();

        if outreach.is_empty() {
            Err(Message::NotFound(
                "No voter outreach activities found".to_string(),
            ))
        } else {
            Ok(outreach)
        }
    })
}

#[ic_cdk::update]
fn send_message_to_campaign(payload: MessagePayload) -> Result<SecureMessage, Message> {
    // Ensure 'sender' and 'content' are provided
    if payload.sender.is_empty() || payload.content.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'sender' and 'content' are provided.".to_string(),
        ));
    }

    // Check if the campaign exists
    let campaign = CAMPAIGN_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, campaign)| campaign.id == payload.campaign_id)
            .map(|(_, campaign)| campaign.clone())
    });
    if campaign.is_none() {
        return Err(Message::NotFound("Campaign not found".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let message = SecureMessage {
        id,
        campaign_id: payload.campaign_id,
        sender: payload.sender,
        content: payload.content,
        created_at: current_time(),
    };

    // Insert the message into the storage
    MESSAGE_STORAGE.with(|storage| storage.borrow_mut().insert(id, message.clone()));
    Ok(message)
}

#[ic_cdk::query]
fn get_messages(campaign_id: u64) -> Result<Vec<SecureMessage>, Message> {
    MESSAGE_STORAGE.with(|storage| {
        let messages: Vec<SecureMessage> = storage
            .borrow()
            .iter()
            .filter(|(_, message)| message.campaign_id == campaign_id)
            .map(|(_, message)| message.clone())
            .collect();

        if messages.is_empty() {
            Err(Message::NotFound("No messages found".to_string()))
        } else {
            Ok(messages)
        }
    })
}

// Notification system to inform donors and participants
fn notify_participants(campaign_id: u64, message: String) {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let notification = Notification {
        id,
        campaign_id,
        message,
        created_at: current_time(),
    };

    NOTIFICATION_STORAGE.with(|storage| storage.borrow_mut().insert(id, notification));
}

// Helper function to get the current time
fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    UnAuthorized { msg: String },
}

// User-related functions
#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, Message> {
    if payload.username.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username' and 'role' are provided.".to_string(),
        ));
    }

    // Check if the user already exists
    let user_exists = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.username == payload.username)
    });
    if user_exists {
        return Err(Message::Error("User already exists".to_string()));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        username: payload.username,
        role: payload.role,
        created_at: current_time(),
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::query]
fn get_users() -> Result<Vec<User>, Message> {
    USER_STORAGE.with(|storage| {
        let users: Vec<User> = storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();

        if users.is_empty() {
            Err(Message::NotFound("No users found".to_string()))
        } else {
            Ok(users)
        }
    })
}

#[ic_cdk::query]
fn get_user_by_id(id: u64) -> Result<User, Message> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.id == id)
            .map(|(_, user)| user.clone())
            .ok_or(Message::NotFound("User not found".to_string()))
    })
}

// User authentication
fn authenticate_user(payload: UserPayload) -> Result<User, Message> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.username == payload.username && user.role == payload.role)
            .map(|(_, user)| user.clone())
            .ok_or(Message::UnAuthorized("Invalid credentials".to_string()))
    })
}

// Export the candid functions
ic_cdk::export_candid!();
