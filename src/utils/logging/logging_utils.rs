use tracing_appender::{non_blocking::WorkerGuard, rolling};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, fmt, fmt::time::ChronoLocal, layer::SubscriberExt};
use log::error;

pub fn init_tracing(dir:String,name:String) -> WorkerGuard {
    // Esto redirige log::info! hacia tracing
    // Rolling file diario
    let file_appender = rolling::daily(dir.as_str(), name.as_str());
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S%.3f %:z".into());
    let env_filter = EnvFilter::new("info,actix_web=debug,diesel=debug");

    let console_layer = fmt::layer()
                        .with_timer(timer.clone())
                        .with_level(true)
                        .with_target(true)
                        .with_thread_names(true);
    let file_layer = fmt::layer()
                        .with_timer(timer.clone())
                        .with_writer(file_writer)
                        .with_ansi(false);

    // 👇 ESTO ES LA CLAVE
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer);

    match tracing::subscriber::set_global_default(subscriber){
        Ok(())=>(),
        Err(error)=>{
            error!("No se pudo setear el subscriber del tracing: {}",error);
            std::process::exit(1);
        }
    };

    // 👇 REGISTRAS log DESPUÉS de que tracing ya es global
    LogTracer::init().ok();
    guard
}