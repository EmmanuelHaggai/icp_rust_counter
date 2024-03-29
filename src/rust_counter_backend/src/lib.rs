use std::cell::RefCell;
use candid::types::number::Nat;

thread_local! {
    static COUNTER: RefCell<Nat> = RefCell::new(Nat::from(0));
}

// Get the value of the counter.
#[ic_cdk_macros::query]
fn get() -> Nat {
    COUNTER.with(|counter| (*counter.borrow()).clone())
}

// Set the value of the counter.
#[ic_cdk_macros::update]
fn set(n: Nat) {
    COUNTER.with(|count| *count.borrow_mut() = n);
}

// Increment the value of the counter.
#[ic_cdk_macros::update]
fn increment() {
    COUNTER.with(|counter| *counter.borrow_mut() += 1);
}