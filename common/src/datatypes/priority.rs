use std::ops::Deref;

#[derive(Debug)]
pub struct Priority<Value, Data> {
    v: PriorityDirection<Value>,
    pub data: Data,
}

impl<Value, Data> Priority<Value, Data> {
    pub fn new_min(data: Data, pri: Value) -> Self {
        Self {
            v: PriorityDirection::Min(pri),
            data,
        }
    }

    pub fn new_max(data: Data, pri: Value) -> Self {
        Self {
            v: PriorityDirection::Max(pri),
            data,
        }
    }
}

impl<Value, Data> Deref for Priority<Value, Data> {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

macro_rules! get_value {
    ($enum:expr) => {
        match $enum {
            PriorityDirection::Min(val) => val,
            PriorityDirection::Max(val) => val,
        }
    };
}

impl<Value, Data> Eq for Priority<Value, Data> where Value: PartialEq + PartialOrd + Ord + Eq {}

impl<Value, Data> PartialEq for Priority<Value, Data>
where
    Value: PartialEq + PartialOrd + Ord + Eq,
{
    fn eq(&self, other: &Self) -> bool {
        get_value!(&self.v) == get_value!(&other.v)
    }
}

impl<Value, Data> PartialOrd for Priority<Value, Data>
where
    Value: PartialEq + PartialOrd + Ord + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<Value, Data> Ord for Priority<Value, Data>
where
    Value: PartialEq + PartialOrd + Ord + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match &self.v {
            PriorityDirection::Max(v) => v.cmp(get_value!(&other.v)),
            PriorityDirection::Min(v) => v.cmp(get_value!(&other.v)).reverse(),
        }
    }
}

#[derive(Debug)]
enum PriorityDirection<Value> {
    Max(Value),
    Min(Value),
}
