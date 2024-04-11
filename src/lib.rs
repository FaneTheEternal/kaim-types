use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntryKind {
    Call,
    CCall,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub id: usize,
    pub kind: EntryKind,
    pub called: String,
    pub info: String,
    pub time: (f64, f64),
}

impl Entry {
    fn start(id: usize, kind: EntryKind, called: String, info: String, ts: f64) -> Self {
        Self {
            id,
            kind,
            called,
            info,
            time: (ts, 0.0),
        }
    }
    pub fn call(id: usize, called: String, info: String, ts: f64) -> Self {
        Self::start(id, EntryKind::Call, called, info, ts)
    }

    pub fn ccall(id: usize, called: String, info: String, ts: f64) -> Self {
        Self::start(id, EntryKind::CCall, called, info, ts)
    }

    pub fn fin(&mut self, ts: f64) {
        self.time.1 = ts;
    }
}