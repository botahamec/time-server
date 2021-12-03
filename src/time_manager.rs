use time::{OffsetDateTime, UtcOffset};

use crate::time_service::TimeService;

pub struct TimeManager<Clock: TimeService> {
	clock: Clock,
}

impl<Clock: TimeService> TimeManager<Clock> {
	pub fn new(clock: Clock) -> Self {
		Self { clock }
	}

	pub fn now(&self, offset: UtcOffset) -> OffsetDateTime {
		self.clock.now_utc().to_offset(offset)
	}
}

#[cfg(test)]
mod tests {

	use time::{macros::datetime, PrimitiveDateTime};

	use super::*;
	use crate::time_service::TestTimeService;

	#[test]
	fn now_with_utc() {
		// arrange
		let time = datetime!(2000-11-05 1:23 UTC);
		let primitive = PrimitiveDateTime::new(time.date(), time.time());
		let clock = TestTimeService::new(primitive);
		let manager = TimeManager::new(clock);

		// act
		let returned_time = manager.now(UtcOffset::UTC);

		// assert
		assert_eq!(UtcOffset::UTC, returned_time.offset());
		assert_eq!(time, returned_time);
	}

	#[test]
	fn now_convert_offset() {
		// arrange
		let time = datetime!(2000-11-05 1:23 UTC);
		let primitive = PrimitiveDateTime::new(time.date(), time.time());
		let clock = TestTimeService::new(primitive);
		let manager = TimeManager::new(clock);
		let offset_minus_5 = UtcOffset::from_hms(-5, 0, 0).unwrap();

		// act
		let returned_time = manager.now(offset_minus_5);

		// assert
		assert_eq!(offset_minus_5, returned_time.offset());
		assert_eq!(time, returned_time);
	}
}
