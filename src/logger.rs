use crate::error::{bhomz_error, BhomzThrowable};
use chrono::Local;
use colored::Colorize;
use log::{trace, LevelFilter, Log, Metadata, Record};
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

static mut LOGGER: Logger = Logger { file: None };

/// This function modify the logger's field
///
/// This function mutate global state!
pub fn refresh(level: LevelFilter, log_file: Option<PathBuf>) -> BhomzThrowable {
	if let Some(file) = &log_file {
		let log_directory = file.parent().unwrap();
		if let Err(err) = create_dir_all(log_directory) {
			let msg = format!(
				"Failed to create {} directory: {err}",
				log_directory.display()
			)
			.red();
			println!("{msg}");
			return Err(bhomz_error(msg));
		}

		if let Err(err) = OpenOptions::new()
			.create(true)
			.truncate(false)
			.write(true)
			.open(file)
		{
			let msg = format!("Failed to create {}: {err}", &file.display()).red();
			println!("{msg}");
			return Err(bhomz_error(msg));
		}
	}
	log::set_max_level(level);
	#[allow(static_mut_refs)]
	unsafe {
		LOGGER.file = log_file;
	}
	Ok(())
}

/// This function binds the logger and refresh the settings
///
/// This function mutate global state!
pub fn bind_logger(level: LevelFilter, log_file: Option<PathBuf>) -> BhomzThrowable {
	refresh(level, log_file)?;

	#[allow(static_mut_refs)]
	unsafe {
		if let Err(err) = log::set_logger(&LOGGER) {
			let msg = format!("Failed set logger: {err}").red();
			println!("{msg}");
			return Err(bhomz_error(msg));
		}
	}
	trace!("Logger bound!");
	Ok(())
}

struct Logger {
	file: Option<PathBuf>,
}

impl Log for Logger {
	fn enabled(&self, _metadata: &Metadata) -> bool {
		true
	}

	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let log_line = log(record);
		if let Some(file_path) = &self.file {
			let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
			file.write_all(format!("{log_line}\n").as_bytes())
				.expect("Could not write to the log file");
		} else {
			println!("{log_line}");
		}
	}

	fn flush(&self) {}
}

fn log(record: &Record) -> String {
	let local = Local::now();
	format!(
		"[{}] [{}] {} ({}{}): {}",
		local.format("%Y-%m-%d %H:%M:%S%.3f"),
		record.level(),
		record.target(),
		record.file().unwrap_or("<Unknown>"),
		if let Some(line) = record.line() {
			format!(":{line}")
		} else {
			String::from(":<Unknown>")
		},
		record.args()
	)
}

/// Log an error at the function calling the macro and create a returnable error
///
/// It can take a string directly
/// ```
/// use bhomz::error::{BhomzResult, BhomzError};
/// use bhomz::log_err;
///
/// fn function_call(string: String) -> BhomzResult<()> {
///   if (string.len() > 6) {
///     return log_err!(BhomzError, "String too long");
///   }
///   return Ok(())
/// }
/// ```
///
/// It can be used like `format!()`
/// ```
/// use bhomz::error::{BhomzResult, BhomzError};
/// use bhomz::log_err;
///
/// fn function_call(string: String) -> BhomzResult<()> {
///   if (string.len() > 6) {
///     return log_err!(BhomzError, "String should be under {} characters", 6);
///   }
///   return Ok(())
/// }
/// ```
#[cfg(not(feature = "anyhow"))]
#[macro_export]
macro_rules! log_err {
	($error:ty, $message:expr) => {{
		let msg = format!("{}", $message);
		log::error!("{}", msg);
		Err(<$error as From<String>>::from(msg))
	}};
	($error:ty, $($arg:expr),*) => {{
		let msg = format!($($arg),*);
		log::error!("{}", msg);
		Err(<$error as From<String>>::from(msg))
	}};
}

/// Log an error at the function calling the macro and create a returnable error
///
/// It can take a string directly
/// ```
/// use bhomz::error::BhomzResult;
/// use bhomz::log_err;
///
/// fn function_call(string: String) -> BhomzResult<()> {
///   if (string.len() > 6) {
///     return log_err!("String too long");
///   }
///   return Ok(())
/// }
/// ```
///
/// It can be used like `format!()`
/// ```
/// use bhomz::error::BhomzResult;
/// use bhomz::log_err;
///
/// fn function_call(string: String) -> BhomzResult<()> {
///   if (string.len() > 6) {
///     return log_err!("String should be under {} characters", 6);
///   }
///   return Ok(())
/// }
/// ```
/// *Note: the error will be an anyhow error wrapped in a type aliases*
#[cfg(feature = "anyhow")]
#[macro_export]
macro_rules! log_err {
	($message:expr) => {{
		log::error!("{}", $message);
		Err(anyhow::Error::msg($message))
	}};
	($($arg:expr),*) => {{
		let msg = format!($($arg),*);
		log::error!("{}", msg);
		Err(anyhow::Error::msg(msg))
	}};
}

#[cfg(test)]
mod tests {
	use crate::error::{bhomz_error, BhomzResult};

	#[cfg(not(feature = "anyhow"))]
	use crate::error::BhomzError;

	const ERR_MSG: &str = "error";

	#[test]
	fn bhomz_error_can_be_used_in_log_error() {
		let expected = bhomz_error(ERR_MSG);

		#[cfg(not(feature = "anyhow"))]
		let result: BhomzResult<()> = log_err!(BhomzError, ERR_MSG);
		#[cfg(feature = "anyhow")]
		let result: BhomzResult<()> = log_err!(ERR_MSG);

		assert!(result.is_err());
		if let Some(res) = result.err() {
			assert_eq!(expected.to_string(), res.to_string());
		} else {
			panic!("Error was not an error");
		}
	}

	#[test]
	fn bhomz_error_can_be_templated() {
		let expected = bhomz_error(format!("{} {}", "is", "or not"));

		#[cfg(not(feature = "anyhow"))]
		let result: BhomzResult<()> = log_err!(BhomzError, "{} {}", "is", "or not");
		#[cfg(feature = "anyhow")]
		let result: BhomzResult<()> = log_err!("{} {}", "is", "or not");

		assert!(result.is_err());
		if let Some(res) = result.err() {
			assert_eq!(expected.to_string(), res.to_string());
		} else {
			panic!("Error was not an error");
		}
	}
}
