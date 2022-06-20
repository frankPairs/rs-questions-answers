use parking_lot::RwLock;
use std::{collections::HashMap, sync::Arc};

use crate::types::{answers::Answer, questions::Question};

#[derive(Clone)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<String, Question>>>,
    pub answers: Arc<RwLock<HashMap<String, Answer>>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            // Arc allows use to modify questions safely between threads. RwLock provides a mechanism
            // where multiple threads can read questions at the same time but only one can be written.
            questions: Arc::new(RwLock::new(Self::init())),
            answers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn init() -> HashMap<String, Question> {
        let file = include_str!("../db/questions.json");

        serde_json::from_str(file).expect("can't read questions.json")
    }
}
