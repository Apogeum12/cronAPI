use serde::{Deserialize, Serialize};

/// Represents the details of scrapers that are intended to run on specified threads.
///
/// This struct contains information about the number of threads and the specific
/// services or scrapers that are scheduled to run on them.
#[derive(Deserialize, Clone)]
pub struct RunScrapers {
    /// The number of threads to run the scrapers on.
    pub thread: i16,
    /// A list of services or scrapers that need to run.
    pub services: Vec<String>,
}

/// Represents a scheduled task or scraper with its details.
///
/// This struct encapsulates the attributes of a scheduled task, including its ID,
/// scheduled time, the thread number, and the specific scrapers associated with it.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Harmonogram {
    /// The unique identifier for the scheduled task.
    pub id: String,
    /// The time at which the task is scheduled to run.
    pub time: String,
    /// The number of threads to use for this scheduled task.
    pub thread: u32,
    /// A list of specific scrapers or services to run as part of this schedule.
    pub scrapers: Vec<String>
}
