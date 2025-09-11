use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, DefaultMemoryImpl, StableBTreeMap, Cell};
use ic_stable_structures::storable::{Storable, Bound};
use std::borrow::Cow;
use std::cell::RefCell;

type VM = VirtualMemory<DefaultMemoryImpl>;

#[derive(CandidType, Deserialize, Clone)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub content: String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransferEvent {
    pub id: u64,
    pub from: Principal,
    pub to: Principal,
    pub amount: u128,
    pub timestamp_ns: u64,
}

#[derive(Clone, Eq, PartialEq)]
struct UserNoteKey {
    user: Principal,
    id: u64,
}

impl Ord for UserNoteKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.user.as_slice();
        let b = other.user.as_slice();
        match a.cmp(b) {
            std::cmp::Ordering::Equal => self.id.cmp(&other.id),
            ord => ord,
        }
    }
}

impl PartialOrd for UserNoteKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Storable for UserNoteKey {
    const BOUND: Bound = Bound::Bounded { max_size: 128, is_fixed_size: false };
    fn to_bytes(&self) -> Cow<[u8]> {
        let mut bytes = Vec::with_capacity(self.user.as_slice().len() + 8);
        bytes.extend_from_slice(self.user.as_slice());
        bytes.extend_from_slice(&self.id.to_be_bytes());
        Cow::Owned(bytes)
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let data = bytes.as_ref();
        let (pbytes, id_bytes) = data.split_at(data.len() - 8);
        let id = u64::from_be_bytes(id_bytes.try_into().unwrap());
        Self { user: Principal::from_slice(pbytes), id }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct PrincipalKey(Principal);

impl Storable for PrincipalKey {
    const BOUND: Bound = Bound::Bounded { max_size: 64, is_fixed_size: false };
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(self.0.as_slice().to_vec()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { PrincipalKey(Principal::from_slice(&bytes)) }
}

#[derive(Clone, Copy, Default)]
struct Amount(u128);

impl Storable for Amount {
    const BOUND: Bound = Bound::Bounded { max_size: 16, is_fixed_size: true };
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(self.0.to_be_bytes().to_vec()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        let mut arr = [0u8; 16];
        arr.copy_from_slice(&bytes);
        Amount(u128::from_be_bytes(arr))
    }
}

impl Storable for Note {
    const BOUND: Bound = Bound::Unbounded;
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(candid::encode_one(self).unwrap())
    }
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        candid::decode_one(&bytes).unwrap()
    }
}

impl Storable for TransferEvent {
    const BOUND: Bound = Bound::Unbounded;
    fn to_bytes(&self) -> Cow<[u8]> { Cow::Owned(candid::encode_one(self).unwrap()) }
    fn from_bytes(bytes: Cow<[u8]>) -> Self { candid::decode_one(&bytes).unwrap() }
}

thread_local! {
    static MEMORY_MANAGER: MemoryManager<DefaultMemoryImpl> = MemoryManager::init(DefaultMemoryImpl::default());
    static NOTES: RefCell<StableBTreeMap<UserNoteKey, Note, VM>> = MEMORY_MANAGER.with(|m| RefCell::new(StableBTreeMap::init(m.get(MemoryId::new(0)))));
    static NEXT_ID_CELL: RefCell<Cell<u64, VM>> = MEMORY_MANAGER.with(|m| RefCell::new(Cell::init(m.get(MemoryId::new(1)), 1).expect("init NEXT_ID cell")));
    static BALANCES: RefCell<StableBTreeMap<PrincipalKey, Amount, VM>> = MEMORY_MANAGER.with(|m| RefCell::new(StableBTreeMap::init(m.get(MemoryId::new(2)))));
    static TRANSFERS: RefCell<StableBTreeMap<u64, TransferEvent, VM>> = MEMORY_MANAGER.with(|m| RefCell::new(StableBTreeMap::init(m.get(MemoryId::new(3)))));
    static NEXT_TX_ID_CELL: RefCell<Cell<u64, VM>> = MEMORY_MANAGER.with(|m| RefCell::new(Cell::init(m.get(MemoryId::new(4)), 1).expect("init NEXT_TX_ID cell")));
}

fn caller() -> Principal { ic_cdk::api::caller() }

fn next_id() -> u64 {
    NEXT_ID_CELL.with(|c| {
        let mut cell = c.borrow_mut();
        let id = *cell.get();
        cell.set(id + 1).expect("increment NEXT_ID");
        id
    })
}

fn next_tx_id() -> u64 {
    NEXT_TX_ID_CELL.with(|c| {
        let mut cell = c.borrow_mut();
        let id = *cell.get();
        cell.set(id + 1).expect("increment NEXT_TX_ID");
        id
    })
}

#[ic_cdk::query]
fn get_all_notes() -> Vec<Note> {
    let user = caller();
    let start = UserNoteKey { user: user.clone(), id: 0 };
    let end = UserNoteKey { user, id: u64::MAX };
    NOTES.with(|notes|
        notes.borrow()
            .range(start..=end)
            .map(|(_, v)| v)
            .collect()
    )
}

#[ic_cdk::update]
fn create_note(title: String, content: String) -> Note {
    let user = caller();
    if user == Principal::anonymous() { ic_cdk::trap("Unauthenticated"); }
    let id = next_id();
    let note = Note { id, title, content };
    NOTES.with(|notes| {
        notes.borrow_mut().insert(UserNoteKey { user, id }, note.clone());
    });
    note
}

#[ic_cdk::update]
fn update_note(id: u64, title: String, content: String) -> Option<Note> {
    let user = caller();
    if user == Principal::anonymous() { ic_cdk::trap("Unauthenticated"); }
    let key = UserNoteKey { user, id };
    NOTES.with(|notes| {
        let mut map = notes.borrow_mut();
        map.get(&key).map(|mut existing| {
            existing.title = title;
            existing.content = content;
            map.insert(key, existing.clone());
            existing
        })
    })
}

#[ic_cdk::update]
fn delete_note(id: u64) -> bool {
    let user = caller();
    if user == Principal::anonymous() { ic_cdk::trap("Unauthenticated"); }
    let key = UserNoteKey { user, id };
    NOTES.with(|notes| notes.borrow_mut().remove(&key).is_some())
}

#[ic_cdk::query]
fn whoami() -> Principal { caller() }

#[ic_cdk::query]
fn balance_of(owner: Principal) -> u128 {
    BALANCES.with(|b| b.borrow().get(&PrincipalKey(owner)).unwrap_or(Amount(0)).0)
}

#[ic_cdk::query]
fn my_balance() -> u128 {
    let me = caller();
    balance_of(me)
}

#[ic_cdk::update]
fn transfer(to: Principal, amount: u128) -> Result<(), String> {
    let from = caller();
    if from == Principal::anonymous() { return Err("Unauthenticated".into()); }
    if amount == 0 { return Ok(()); }
    if from == to { return Ok(()); }

    let result = BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        let from_key = PrincipalKey(from);
        let to_key = PrincipalKey(to);
        let from_bal = map.get(&from_key).unwrap_or(Amount(0)).0;
        if from_bal < amount { return Err("Insufficient balance".to_string()); }
        map.insert(from_key.clone(), Amount(from_bal - amount));
        let to_bal = map.get(&to_key).unwrap_or(Amount(0)).0;
        map.insert(to_key, Amount(to_bal + amount));
        Ok(())
    });

    if result.is_ok() {
        let event = TransferEvent {
            id: next_tx_id(),
            from,
            to,
            amount,
            timestamp_ns: ic_cdk::api::time(),
        };
        TRANSFERS.with(|t| { t.borrow_mut().insert(event.id, event); });
    }

    result
}

#[ic_cdk::update]
fn mint_to(to: Principal, amount: u128) -> Result<(), String> {
    if !ic_cdk::api::is_controller(&caller()) { return Err("Not authorized".into()); }
    if amount == 0 { return Ok(()); }
    BALANCES.with(|b| {
        let mut map = b.borrow_mut();
        let key = PrincipalKey(to);
        let cur = map.get(&key).unwrap_or(Amount(0)).0;
        map.insert(key, Amount(cur + amount));
    });
    let event = TransferEvent {
        id: next_tx_id(),
        from: caller(),
        to,
        amount,
        timestamp_ns: ic_cdk::api::time(),
    };
    TRANSFERS.with(|t| { t.borrow_mut().insert(event.id, event); });
    Ok(())
}

#[ic_cdk::query]
fn get_my_transfers() -> Vec<TransferEvent> {
    let me = caller();
    TRANSFERS.with(|t| {
        t.borrow()
            .range(0..=u64::MAX)
            .filter_map(|(_, ev)| {
                if ev.from == me || ev.to == me { Some(ev.clone()) } else { None }
            })
            .collect()
    })
}

candid::export_service!();

// Expose candid interface to dfx so it can generate an up-to-date .did
#[ic_cdk::query(name = "__get_candid_interface_tmp_hack")]
fn export_candid() -> String {
    candid::export_service!();
    __export_service()
}
