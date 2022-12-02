//! EAF time order.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::EafError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimeOrder {
    #[serde(rename = "TIME_SLOT", default)]
    pub time_slots: Vec<TimeSlot>
}

impl Default for TimeOrder {
    fn default() -> Self {
        Self {
            time_slots: Vec::new()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub struct TimeSlot {
    pub time_slot_id: String,
    pub time_value: Option<i64> // optional in EAF spec
}

impl TimeSlot {
    /// New time slot from time slot ID and optional millisecond value.
    /// ID:s must be formatted `"ts1"`, `"ts2"`, ..., `"ts23"`, ...,
    /// `"ts10234"`, etc, with no leading zeros.
    pub fn new(id: &str, val: Option<i64>) -> Self {
        TimeSlot {
            time_slot_id: id.to_owned(),
            time_value: val,
        }
    }

    /// Returns `True` if the `TimeSlot`
    /// has a value specified.
    pub fn has_val(&self) -> bool {
        self.time_value.is_some()
    }
}

impl TimeOrder {
    pub fn new(time_slots: &[TimeSlot]) -> Self {
        TimeOrder{time_slots: time_slots.to_owned()}
    }

    /// Pushes a time slot at the end of time slot list, unless it already exists.
    /// Does not verify that specified time slot ID/value is greater
    /// than exisiting ones.
    pub fn add(&mut self, id: &str, val: Option<i64>) {
        let ts = TimeSlot::new(id, val);
        if !self.time_slots.contains(&ts) {
            self.time_slots.push(TimeSlot::new(id, val))
        }
    }

    /// Extends existing time order with specifed time slots.
    /// Does not verify that specified time slot ID:s/values are greater
    /// than exisiting ones.
    pub fn extend(&mut self, time_slots: &[TimeSlot]) {
        self.time_slots.extend(time_slots.to_owned())
    }

    /// Joins one time order with another.
    pub fn join(&mut self, time_order: &TimeOrder) {
        self.time_slots.extend(time_order.time_slots.to_owned())
    }

    /// Generates a new time order from vector containing millisecond time values.
    /// The optional `start_index` corresponds to e.g. the numerical value `45` in "ts45".
    /// Note that time slot values are optional for EAF, hence `Vec<Option<i64>>`.
    pub fn from_values(time_values: Vec<Option<i64>>, start_index: Option<usize>) -> Self {
        let start = start_index.unwrap_or(0);
        TimeOrder {
            time_slots: time_values.iter()
                .enumerate()
                .map(|(i,t)| 
                    TimeSlot::new(&format!("ts{}", start+i+1), t.to_owned())
                )
                .collect()
        }
    }

    /// Generates a new time order from hashmap where key = numerical index for time slot ID (`2` in "ts2"),
    /// and value = time slot millisecond value.
    /// The optional `start_index` corresponds to e.g. the numerical value `45` in "ts45".
    /// Note that time slot values are optional in EAF, hence `Option<i64>`.
    pub fn from_hashmap(id2values: HashMap<String, Option<i64>>) -> Self {
        let mut time_slots: Vec<TimeSlot> = id2values.iter()
            .map(|(ts_id, ts_val)| TimeSlot::new(&ts_id, *ts_val))
            .collect();

        time_slots
            .sort_by_key(|t|
                t.time_slot_id
                    .replace("ts", "")
                    .parse::<usize>()
                    .ok()
            );

        TimeOrder{time_slots}
    }

    /// Sort time slots according to time slot ID.
    pub fn sort_on_id(&mut self) {
    // pub fn sort(&mut self) -> Result<(), EafError> {
        self.time_slots.sort_by_key(|t|
            t.time_slot_id
                .replace("ts", "")
                .parse::<usize>()
                .ok()
                // .or_else(|e| e) // no error handling... will just panic?
        )
        // )?;
        // Ok(())
    }

    /// Sort time slots according to time slot value.
    /// Time slots with no value maintain their position.
    /// Returns hashmap mapping old to new values.
    pub fn sort_on_val(&mut self) {
        // Each time slot cluster starts and ends with a time slot
        // that has a time value set.
        let mut ts_clusters: Vec<Vec<TimeSlot>> = Vec::new();
        let mut ts_cluster: Vec<TimeSlot> = Vec::new();
        let mut ts_prev: Option<TimeSlot> = None;

        // return hasmap old -> new
    }

    /// Returns number of time slots.
    pub fn len(&self) -> usize {
        self.time_slots.len()
    }

    /// Returns true if there are no time slots.
    pub fn is_empty(&self) -> bool {
        self.time_slots.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TimeSlot> {
        self.time_slots.iter()
    }

    pub fn into_iter(self) -> impl IntoIterator<Item = TimeSlot> {
        self.time_slots.into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TimeSlot> {
        self.time_slots.iter_mut()
    }

    pub fn first(&self) -> Option<&TimeSlot> {
        self.time_slots.first()
    }

    pub fn first_mut(&mut self) -> Option<&mut TimeSlot> {
        self.time_slots.first_mut()
    }
    
    pub fn last(&self) -> Option<&TimeSlot> {
        self.time_slots.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut TimeSlot> {
        self.time_slots.last_mut()
    }

    /// Returns the time slot with minimum millisecond value,
    /// or `None` if time order is empty or contains no time values.
    pub fn min(&self) -> Option<TimeSlot> {
        let ts_rev = self.index_rev();
        let min = ts_rev.keys().min()?;
        Some(TimeSlot::new(ts_rev.get(min)?,Some(*min)))
    }

    /// Returns minimum millisecond time value or `None`
    /// if time order is empty or contains no time values.
    pub fn min_val(&self) -> Option<i64> {
        let ts_rev = self.index_rev();
        ts_rev.keys().min().cloned()
    }

    /// Returns the time slot with maximum millisecond value,
    /// or `None` if time order is empty or contains no time values.
    pub fn max(&self) -> Option<TimeSlot> {
        let ts_rev = self.index_rev();
        let max = ts_rev.keys().max()?;
        Some(TimeSlot::new(ts_rev.get(max)?,Some(*max)))
    }

    /// Returns maximum millisecond time value or `None`
    /// if time order is empty or contains no time values.
    pub fn max_val(&self) -> Option<i64> {
        let ts_rev = self.index_rev();
        ts_rev.keys().max().cloned()
    }

    /// Returns reference to `TimeSlot` with specified ID,
    /// or `None`if it does not exist.
    pub fn find(&self, time_slot_id: &str) -> Option<&TimeSlot> {
        self.time_slots.iter()
            .find(|t| t.time_slot_id == time_slot_id)
    }

    /// Remaps time slot ID:s starting on 1, or `start_index`.
    /// Returns hashmap mapping old time slot ID:s to new ones.
    pub fn remap(&mut self, start_index: Option<usize>) -> HashMap<String, String> {
        let mut old2new: HashMap<String, String> = HashMap::new();

        let ts_start_index = start_index.unwrap_or(0);
        
        self.time_slots.iter_mut()
            .enumerate()
            .for_each(|(i,t)| {
                let ts_id_new = format!("ts{}", i+1+ts_start_index);
                old2new.insert(t.time_slot_id.to_owned(), ts_id_new.to_owned());
                t.time_slot_id = ts_id_new;
            });
            
        old2new
    }

    /// Lookup table for time slots.
    /// Time slots with no time value return `None`.
    /// Key: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    /// Value: timeslot value in milliseconds.
    pub fn index(&self) -> HashMap<String, Option<i64>> {
    // pub fn index(&self) -> HashMap<String, Option<u64>> {
        self.time_slots.to_owned().into_iter()
            .map(|ts| (ts.time_slot_id, ts.time_value))
            .collect()
    }
    
    /// Reverse lookup table for time slots.
    /// Important: Only includes time slots with a corresponding time value.
    /// Key: timeslot value in milliseconds.
    /// Value: timeslot reference (e.g. "ts23"), `TIME_SLOT_REF1`/`TIME_SLOT_REF2` in EAF.
    pub fn index_rev(&self) -> HashMap<i64, String> {
    // pub fn index_rev(&self) -> HashMap<u64, String> {
        self.time_slots.to_owned().into_iter()
            .filter_map(|t|
                if let Some(val) = t.time_value {
                    Some((val, t.time_slot_id))
                } else {
                    None
                }
            )
            .collect()
    }

    /// Shift all time values with the specified value in milliseconds.
    /// `allow_negative` ignores negative time values, otherwise
    /// `EafError::ValueTooSmall(time_value)` is raised.
    pub fn shift(&mut self, shift_ms: i64, allow_negative: bool) -> Result<(), EafError> {
        // Check negative values. Cannot check too large values,
        // since this depends on media duration.
        if !allow_negative {
            let min = match self.min_val() {
                Some(val) => val,
                None => 0
            };

            println!("MIN {min}, SHIFT MS {shift_ms}");
    
            // if min - shift_ms < 0 {
            // ensure negative shift values are not less than 0
            if min + shift_ms < 0 {
                return Err(EafError::ValueTooSmall(shift_ms))
            }
        }

        // self.iter_mut().for_each(|ts| ts.time_value.map(|v| *v += shift_ms) += shift_ms);

        // shift values
        for ts in self.iter_mut() {
            if let Some(val) = ts.time_value {
                ts.time_value = Some(val + shift_ms);
            };
        }

        Ok(())
    }

    /// Returns `TimeOrder` containing only time slots between, and including,
    /// millisecond values `start` and `end`. Start and end values may for example
    /// correspond to new media boundaries when a clip has been extracted from a larger
    /// media file.
    /// 
    /// Note that only time slots with a millisecond value can act as the first or
    /// final time slot for the specified time span. Between the first and the final
    /// time slot all time slots will be included, including those without a time value.
    pub fn filter(&self, start: i64, end: i64) -> Option<TimeOrder> {
    // pub fn filter(&self, start: u64, end: u64) -> Option<TimeOrder> {
        // Generate hashmap: time slot value (ms) -> time slot id (String>).
        // Need time slot values to find start/end, hence `index_rev()`.
        // let filtered: HashMap<u64, String> = self.index_rev()
        let filtered: HashMap<i64, String> = self.index_rev()
            .into_iter()
            .filter(|(t, _)| t >= &start && t <= &end)
            .collect();

        // Need to get around time slot values being optional,
        // hence the back and forth below. A simple time value
        // comparison would discard time slots with no time value.
            
        // Get time slot ID for min/max time slot values.
        let id_min = filtered.keys().min()
            .and_then(|min| filtered.get(min))?;
        let id_max = filtered.keys().max()
            .and_then(|max| filtered.get(max))?;

        println!("TS ID MIN: {id_min}");
        println!("TS ID MAX: {id_max}");

        // Indeces for slice containing `TimeSlot`:s within time span.
        let idx1 = self.time_slots.iter()
            .position(|t| &t.time_slot_id == id_min)?;
        let idx2 = self.time_slots.iter()
            .position(|t| &t.time_slot_id == id_max)?;

        println!("TS LEN BEFORE: {}", self.time_slots.len());
        
        let time_slots = self.time_slots[idx1 ..= idx2].iter()
            .map(|t|
                if let Some(val) = t.time_value {
                    TimeSlot {
                        time_value: Some(val - start),
                        ..t.to_owned()
                    }
                } else {
                    t.to_owned()
                }
            )
            .collect::<Vec<TimeSlot>>();
        
        println!("TS LEN AFTER: {}", time_slots.len());
        
        Some(TimeOrder{time_slots})
    }

    /// Set the time slot value for the first time slot, if any exist.
    /// For example to correct a negative start value and set it to 0.
    pub fn set_first(&mut self, time_slot_value: i64) {
    // pub fn set_first(&mut self, time_slot_value: u64) {
        if let Some(ts) = self.first_mut() {
            ts.time_value = Some(time_slot_value);
        }
    }

    /// Set the time slot value for the last time slot, if any exist.
    /// For example if it is greater than the media length.
    pub fn set_last(&mut self, time_slot_value: i64) {
    // pub fn set_last(&mut self, time_slot_value: u64) {
        if let Some(ts) = self.last_mut() {
            ts.time_value = Some(time_slot_value);
        }
    }

    pub fn contains(&self, time_slot: &TimeSlot) -> bool {
        self.time_slots.contains(time_slot)
    }

    pub fn contains_id(&self, time_slot_id: &str) -> bool {
        self.time_slots.iter().any(|t| t.time_slot_id == time_slot_id)
    }

    /// Deduplicates time slots.
    /// IMPORTANT: Time slots with no time value will be discarded.
    /// 
    /// - Time slots with the same millisecond time value will be deduplicated.
    /// - Time slots that have the same ID, and value will be deduplicated.
    pub fn dedup(&mut self) -> HashMap<String, String> {
        let mut old2new: HashMap<String, String> = HashMap::new();

        

        old2new
    }
}
