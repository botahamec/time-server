use actix_web::{web, App, HttpResponse, HttpServer};
use time::UtcOffset;
use time_manager::TimeManager;
use time_service::SystemTimeService;

mod time_manager;
pub mod time_service;

async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}

async fn time(
	time: web::Data<TimeManager<SystemTimeService>>,
	offset: web::Path<i8>,
) -> HttpResponse {
	let offset = UtcOffset::from_hms(offset.0, 0, 0).unwrap();
	let current_time = time.now(offset);
	HttpResponse::Ok().body(current_time.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		let time_manager = TimeManager::new(SystemTimeService);

		App::new()
			.route("/ping", web::get().to(ping))
			.route("/time/{offset}", web::get().to(time))
			.data(time_manager)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
