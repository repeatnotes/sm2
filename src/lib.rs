extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

use crate::srs::card::Card;
use crate::srs::config::Config;
use crate::srs::scheduler::{Choice, Sched, Scheduler};
use crate::svc::timestamp::Timestamp;

// Import 'window.alert'
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a 'hello' function
#[wasm_bindgen]
pub fn hello(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct Sm2 {
    scheduler: Scheduler,
}

#[wasm_bindgen]
impl Sm2 {
    #[wasm_bindgen(constructor)]
    pub fn new(config: &JsValue) -> Self {
        let config: Config = config.into_serde().unwrap();
        Self {
            scheduler: Scheduler::new(config, Timestamp::day_cut_off()),
        }
    }
}

#[wasm_bindgen]
impl Sm2 {
    pub fn next_interval(&self, card: &JsValue, choice: &JsValue) -> i64 {
        let card: Card = card.into_serde().unwrap();
        let choice: Choice = choice.into_serde().unwrap();
        self.scheduler.next_interval(&card, choice)
    }

    pub fn next_interval_string(&self, card: &JsValue, choice: &JsValue) -> String {
        let card: Card = card.into_serde().unwrap();
        let choice: Choice = choice.into_serde().unwrap();
        self.scheduler.next_interval_string(&card, choice)
    }

    pub fn answer_card(&self, card: &JsValue, choice: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        let choice: Choice = choice.into_serde().unwrap();
        self.scheduler.answer_card(&mut card, choice);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn bury_card(&self, card: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler.bury_card(&mut card);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn unbury_card(&self, card: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler.unbury_card(&mut card);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn suspend_card(&self, card: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler.suspend_card(&mut card);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn unsuspend_card(&self, card: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler.unsuspend_card(&mut card);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn schedule_card_as_new(&self, card: &JsValue) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler.schedule_card_as_new(&mut card);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn schedule_card_as_review(&self, card: &JsValue, min_days: i32, max_days: i32) -> JsValue {
        let mut card: Card = card.into_serde().unwrap();
        self.scheduler
            .schedule_card_as_review(&mut card, min_days, max_days);
        JsValue::from_serde(&card).unwrap()
    }

    pub fn day_cut_off(&self) -> i64 {
        self.scheduler.day_cut_off
    }

    pub fn day_today(&self) -> i64 {
        self.scheduler.day_today
    }
}

pub mod srs;
pub mod svc;
