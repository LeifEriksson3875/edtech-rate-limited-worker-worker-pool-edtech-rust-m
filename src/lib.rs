use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryError {
    MissingLearner,
    DeadlinePassed,
    Transport(String),
    Api(String),
}

impl fmt::Display for DeliveryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLearner => write!(f, "learner is required"),
            Self::DeadlinePassed => write!(f, "course deadline has passed"),
            Self::Transport(message) => write!(f, "transport: {message}"),
            Self::Api(message) => write!(f, "api: {message}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CourseJob {
    pub course: String,
    pub learner: String,
    pub deadline_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeliveryDecision {
    Deliver,
    Skip(DeliveryError),
}

pub fn decide_delivery(job: &CourseJob, now_epoch: u64) -> DeliveryDecision {
    if job.learner.trim().is_empty() {
        return DeliveryDecision::Skip(DeliveryError::MissingLearner);
    }
    if now_epoch > job.deadline_epoch {
        return DeliveryDecision::Skip(DeliveryError::DeadlinePassed);
    }
    DeliveryDecision::Deliver
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deadline_decides_course_delivery() {
        let job = CourseJob { course: "rust-101".into(), learner: "lin".into(), deadline_epoch: 1_700 };
        assert_eq!(decide_delivery(&job, 1_699), DeliveryDecision::Deliver);
        assert_eq!(decide_delivery(&job, 1_701), DeliveryDecision::Skip(DeliveryError::DeadlinePassed));
    }
}
