# Decentralized-election-campaign-manager

This project is a decentralized platform built on the Internet Computer, designed to facilitate the management of campaigns, donations, expenses, voter outreach, and secure communication within the campaign community. It leverages blockchain technology to ensure transparency and reliability in the management processes.

## Key Features

### Campaign Management
- **Create Campaign**: Allows the creation of new campaigns by Admins and Campaign Managers.
- **Update Campaign**: Enables the update of campaign details.
- **Get Campaigns**: Retrieves all registered campaigns.
- **Get Campaign by ID**: Retrieves the details of a specific campaign by its unique ID.

### Donation Management
- **Create Donation**: Allows Donors to create donations for specific campaigns.
- **Get Donations**: Retrieves all donations for a specific campaign.

### Expense Management
- **Create Expense**: Allows Admins and Campaign Managers to create new expenses for campaigns.
- **Get Expenses**: Retrieves all expenses for a specific campaign.

### Voter Outreach Management
- **Create Voter Outreach Activity**: Allows the creation of voter outreach activities for campaigns.
- **Get Voter Outreach Activities**: Retrieves all voter outreach activities for a specific campaign.

### Communication
- **Send Message to Campaign**: Allows users to send secure messages to campaign members.
- **Get Messages**: Retrieves all messages for a specific campaign.

### Notification System
- **Notify Participants**: Notifies donors and participants about campaign updates and new activities.

### User Management
- **Create User**: Allows the creation of new users with roles (Admin, Campaign Manager, Donor).
- **Get Users**: Retrieves all registered users.
- **Get User by ID**: Retrieves the profile of a specific user by their unique ID.
- **Authenticate User**: Validates user credentials for role-based access control.

### Error Handling
- **Not Found**: Returns an error if a requested resource (campaign, donation, expense, voter outreach activity, message, user) is not found.
- **Invalid Input**: Handles errors related to invalid input fields.
- **UnAuthorized**: Returns an error if a user does not have the necessary permissions to perform an action.

## Data Structures

### Enums

#### UserRole
Defines the roles a user can have:
- Admin
- CampaignManager
- Donor

### Structs

#### Campaign
Represents a campaign with attributes:
- `id: u64`
- `name: String`
- `description: String`
- `created_at: u64`

#### Donation
Represents a donation with attributes:
- `id: u64`
- `campaign_id: u64`
- `donor_name: String`
- `amount: u64`
- `created_at: u64`

#### Expense
Represents an expense with attributes:
- `id: u64`
- `campaign_id: u64`
- `description: String`
- `amount: u64`
- `created_at: u64`

#### VoterOutreach
Represents a voter outreach activity with attributes:
- `id: u64`
- `campaign_id: u64`
- `activity: String`
- `date: u64`
- `status: String`
- `created_at: u64`

#### SecureMessage
Represents a secure message with attributes:
- `id: u64`
- `campaign_id: u64`
- `sender: String`
- `content: String`
- `created_at: u64`

#### Notification
Represents a notification with attributes:
- `id: u64`
- `campaign_id: u64`
- `message: String`
- `created_at: u64`

#### User
Represents a user with attributes:
- `id: u64`
- `username: String`
- `role: UserRole`
- `created_at: u64`

### Payload Structs

#### CampaignPayload
Payload for creating a campaign:
- `name: String`
- `description: String`

#### DonationPayload
Payload for creating a donation:
- `campaign_id: u64`
- `donor_name: String`
- `amount: u64`

#### ExpensePayload
Payload for creating an expense:
- `campaign_id: u64`
- `description: String`
- `amount: u64`

#### VoterOutreachPayload
Payload for creating a voter outreach activity:
- `campaign_id: u64`
- `activity: String`
- `date: u64`
- `status: String`

#### MessagePayload
Payload for sending a message to a campaign:
- `campaign_id: u64`
- `sender: String`
- `content: String`

#### UpdateCampaignPayload
Payload for updating a campaign:
- `id: u64`
- `name: Option<String>`
- `description: Option<String>`

#### UserPayload
Payload for creating a user:
- `username: String`
- `role: UserRole`

### Messages
Represents possible messages returned by the platform:
- `Success(String)`
- `Error(String)`
- `NotFound(String)`
- `InvalidPayload(String)`
- `UnAuthorized(String)`

## Storage and Thread Safety
- Uses `ic_stable_structures` for managing stable memory storage.
- Thread-local storage ensures safe concurrent access to shared resources.

## Helper Functions
- `current_time`: Returns the current time in `u64` format.
- `notify_participants`: Sends notifications to participants about campaign updates.

## Export Candid Functions
The platform exports candid functions for interaction with the Internet Computer.




## Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```