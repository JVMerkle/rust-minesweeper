use std::fmt::{Debug, Display, Formatter};

use chrono;

type EventType = u16;
type DetailNumber = u16;
type ProductID = u32;
type MetaData = u16;

struct Event {
    evt_type: EventType,
    product: ProductID,
    detail_number: DetailNumber,
    meta: MetaData,
}

impl Event {
    fn new(evt_type: EventType, product: ProductID, detail_number: DetailNumber, meta: MetaData) -> Self {
        Event { evt_type, product, detail_number, meta }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}-{:07}-{:02}", self.evt_type, self.product, self.detail_number)
    }
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}-{:07}-{:02}-{:02}", self.evt_type, self.product, self.detail_number, self.meta)
    }
}

#[derive(Debug)]
enum LogEntryType {
    Coming,
    Going,
}

struct LogEntry {
    timestamp: u32,
    log_type: LogEntryType,
    event: Event,
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} {}", self.timestamp, self.log_type, self.event)
    }
}

impl Debug for LogEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?} {:?}", self.timestamp, self.log_type, self.event)
    }
}

struct EventFactory {
    product: ProductID,
    start_timestamp: i64,
}

impl EventFactory {
    fn new(product: ProductID) -> Self {
        EventFactory {
            product,
            start_timestamp: chrono::Utc::now().timestamp(),
        }
    }

    fn create(&self, evt_type: EventType, detail_number: DetailNumber) -> LogEntry {
        let diff_secs = chrono::Utc::now().timestamp() - self.start_timestamp;
        LogEntry {
            timestamp: diff_secs as u32,
            log_type: LogEntryType::Going,
            event: Event::new(evt_type, self.product, detail_number, 0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aaaaaa() {
        use std::{thread, time};

        let evt_fac = EventFactory::new(2255001);

        //thread::sleep(time::Duration::from_millis(1000));

        let evt = evt_fac.create(112, 539);
        println!("{}", evt);
        println!("{:?}", evt);
        assert_eq!(1, 1);
    }
}
