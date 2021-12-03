use time::{OffsetDateTime, PrimitiveDateTime};

pub trait TimeService {
	/// Returns the current time, in UTC
	fn now_utc(&self) -> OffsetDateTime;
}

pub struct SystemTimeService;

impl TimeService for SystemTimeService {
	fn now_utc(&self) -> OffsetDateTime {
		OffsetDateTime::now_utc()
	}
}

pub struct TestTimeService {
	current_time: PrimitiveDateTime,
}

impl TestTimeService {
	/// Creates a time service that permanently returns the given time as a UTC time
	pub fn new(current_time: PrimitiveDateTime) -> Self {
		Self { current_time }
	}
}

impl TimeService for TestTimeService {
	fn now_utc(&self) -> OffsetDateTime {
		self.current_time.assume_utc()
	}
}
