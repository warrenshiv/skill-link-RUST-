use candid::{CandidType, Deserialize, Encode, Decode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{StableBTreeMap, DefaultMemoryImpl, Storable, BoundedStorable};
use std::{borrow::Cow, cell::RefCell};

// Type Definitions

type Memory = VirtualMemory<DefaultMemoryImpl>;
type UserStorage = StableBTreeMap<u64, User, Memory>;
type WorkerProfileStorage = StableBTreeMap<u64, WorkerProfile, Memory>;
type EmployerProfileStorage = StableBTreeMap<u64, EmployerProfile, Memory>;
type JobPostingStorage = StableBTreeMap<u64, JobPosting, Memory>;
type JobApplicationStorage = StableBTreeMap<u64, JobApplication, Memory>;
type ProjectStorage = StableBTreeMap<u64, Project, Memory>;

type IdCounter = u64;

// Enums

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum UserRole {
    Worker,
    Employer,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum JobCategory {
    WebDevelopment,
    MobileDevelopment,
    DataScience,
    ArtificialIntelligence,
    GraphicDesign,
    ContentWriting,
    Other,
}

// Struct Definitions

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct User {
    id: u64,
    owner: Principal,
    full_name: String,
    user_type: UserRole,
    email: String,
    address: String,
    phone_number: String,
    created_at: u64,
    updated_at: u64,
    is_verified: bool,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WorkerProfile {
    id: u64,
    user_id: u64,
    professional_summary: String,
    skills: Vec<String>,
    certifications: Vec<String>,
    average_rating: u64,
    completed_jobs: u64,
    total_earnings: u64,
    created_at: u64,
    updated_at: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EmployerProfile {
    id: u64,
    user_id: u64,
    company_name: String,
    industry: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct JobPosting {
    id: u64,
    employer_id: u64,
    title: String,
    description: String,
    required_skills: Vec<String>,
    job_category: JobCategory,
    project_duration: String,
    status: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct JobApplication {
    id: u64,
    job_id: u64,
    worker_id: u64,
    cover_letter: String,
    status: String,
    created_at: u64,
    updated_at: u64,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Project {
    id: u64,
    job_id: u64,
    employer_id: u64,
    worker_id: u64,
    status: String,
    payment_status: PaymentStatus,
    created_at: u64,
    updated_at: u64,
}

thread_local! {
    static ID_COUNTER: RefCell<IdCounter> = RefCell::new(0);
    static USER_STORAGE: RefCell<UserStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
    static WORKER_PROFILE_STORAGE: RefCell<WorkerProfileStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
    static EMPLOYER_PROFILE_STORAGE: RefCell<EmployerProfileStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
    static JOB_POSTING_STORAGE: RefCell<JobPostingStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
    static JOB_APPLICATION_STORAGE: RefCell<JobApplicationStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
    static PROJECT_STORAGE: RefCell<ProjectStorage> = RefCell::new(StableBTreeMap::init(VirtualMemory::default()));
}

// Utility Functions

fn generate_id() -> u64 {
    ID_COUNTER.with(|counter| {
        let mut counter = counter.borrow_mut();
        *counter += 1;
        *counter
    })
}

fn current_time() -> u64 {
    time()
}

// CRUD Functions

#[ic_cdk::update]
pub fn create_user(owner: Principal, full_name: String, email: String, address: String, phone_number: String, user_type: UserRole) -> User {
    let id = generate_id();
    let user = User {
        id,
        owner,
        full_name,
        email,
        address,
        phone_number,
        user_type,
        created_at: current_time(),
        updated_at: current_time(),
        is_verified: false,
    };
    
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    user
}

#[ic_cdk::query]
pub fn get_user_by_id(id: u64) -> Option<User> {
    USER_STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}

#[ic_cdk::update]
pub fn create_job_posting(employer_id: u64, title: String, description: String) -> JobPosting {
    let id = generate_id();
    let job_posting = JobPosting {
        id,
        employer_id,
        title,
        description,
        required_skills: vec![],
        job_category: JobCategory::Other,
        project_duration: "1 month".to_string(),
        status: "Open".to_string(),
        created_at: current_time(),
        updated_at: current_time(),
    };
    
    JOB_POSTING_STORAGE.with(|storage| storage.borrow_mut().insert(id, job_posting.clone()));
    job_posting
}

#[ic_cdk::query]
pub fn get_job_posting(id: u64) -> Option<JobPosting> {
    JOB_POSTING_STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}

#[ic_cdk::update]
pub fn apply_for_job(job_id: u64, worker_id: u64, cover_letter: String) -> JobApplication {
    let id = generate_id();
    let job_application = JobApplication {
        id,
        job_id,
        worker_id,
        cover_letter,
        status: "Pending".to_string(),
        created_at: current_time(),
        updated_at: current_time(),
    };
    
    JOB_APPLICATION_STORAGE.with(|storage| storage.borrow_mut().insert(id, job_application.clone()));
    job_application
}

#[ic_cdk::query]
pub fn get_job_application(id: u64) -> Option<JobApplication> {
    JOB_APPLICATION_STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}
