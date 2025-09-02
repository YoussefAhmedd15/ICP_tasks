# 📝 Secure Note-Taking dApp with Internet Identity

A decentralized note-taking application built on the Internet Computer (ICP) with Internet Identity authentication. Users can securely create, read, update, and delete their personal notes after authenticating with their Internet Identity.

## ✨ Features

- **🔐 Internet Identity Authentication**: Secure login using Internet Identity service
- **👤 User-Specific Notes**: Each user can only access their own notes
- **📝 Full CRUD Operations**: Create, read, update, and delete notes
- **🎨 Modern UI**: Clean and responsive user interface
- **🔒 Privacy-First**: Notes are stored securely per user Principal

## 🏗️ Architecture

### Backend (Rust)
- **Canister**: `ICP_backend`
- **Storage**: `HashMap<Principal, HashMap<u64, Note>>` - Notes stored per user
- **Authentication**: Uses `ic_cdk::caller()` to identify authenticated users
- **Functions**:
  - `get_all_notes()` - Get all notes for the authenticated user
  - `create_note(title, content)` - Create a new note
  - `update_note(id, title, content)` - Update an existing note
  - `delete_note(id)` - Delete a note
  - `whoami()` - Get the current user's Principal

### Frontend (React)
- **Authentication**: Internet Identity integration using `@dfinity/auth-client`
- **State Management**: React Context for authentication state
- **UI Components**: Modern, responsive design with authentication flow

## 🚀 Getting Started

### Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install) installed
- Node.js (>=16.0.0) and npm (>=7.0.0)
- Internet Identity account (you can create one during the login process)

### Installation

1. **Clone and navigate to the project**:
```bash
cd "ICP task 1"
```

2. **Install dependencies**:
```bash
npm install
```

3. **Start the local Internet Computer replica**:
```bash
dfx start --background
```

4. **Deploy the canisters**:
```bash
dfx deploy
```

5. **Start the frontend development server**:
```bash
npm start
```

The application will be available at `http://localhost:8080`.

## 🔧 Development Commands

```bash
# Generate Candid interface (run after backend changes)
npm run generate

# Build the project
npm run build

# Format code
npm run format
```

## 🧪 Testing the Authentication Flow

### 1. **Initial Setup**
After deploying and starting the application:

1. Open `http://localhost:8080` in your browser
2. You should see the login screen with "Login with Internet Identity" button

### 2. **First-Time Login**
1. Click "🔐 Login with Internet Identity"
2. You'll be redirected to the Internet Identity service
3. If you don't have an account, create one by:
   - Choosing a device (computer, phone, etc.)
   - Creating a passphrase
   - Following the setup instructions
4. After successful authentication, you'll be redirected back to the app

### 3. **Using the App**
1. Once logged in, you'll see your Principal ID in the header
2. Create notes using the form
3. Edit existing notes by clicking "Edit"
4. Delete notes by clicking "Delete"
5. Logout using the "Logout" button

### 4. **Testing User Isolation**
1. Logout from the current session
2. Create a new Internet Identity account (or use a different device)
3. Login with the new account
4. Verify that you can't see notes from the previous account
5. Create new notes and verify they're isolated to this account

## 🔍 Key Features Demonstrated

- **Authentication**: Internet Identity integration working seamlessly
- **User Isolation**: Notes are properly isolated per user Principal
- **Security**: Only authenticated users can access their own notes
- **UI/UX**: Clean interface showing user's Principal and authentication status

## 🛠️ Technical Implementation Details

### Backend Changes
- Modified storage from global `HashMap<u64, Note>` to `HashMap<Principal, HashMap<u64, Note>>`
- Added `caller()` function to get the authenticated user's Principal
- Updated all CRUD operations to work with user-specific data
- Added `whoami()` query function for frontend user identification

### Frontend Changes
- Added `@dfinity/auth-client` dependency
- Created `AuthContext` for authentication state management
- Implemented login/logout flow with Internet Identity
- Updated UI to show authentication status and user Principal
- Added loading states and error handling

## 📚 Additional Resources

- [Internet Identity Documentation](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/)
- [DFX Documentation](https://internetcomputer.org/docs/current/developer-docs/setup/install)
- [Candid Interface](https://internetcomputer.org/docs/building-apps/interact-with-canisters/candid/candid-concepts)
- [Rust Canister Development](https://internetcomputer.org/docs/current/developer-docs/backend/rust/)
