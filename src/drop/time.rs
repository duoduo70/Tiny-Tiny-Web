use std::time::{SystemTime, SystemTimeError};

// DONT USE crate::log in this module

pub struct Time(Result<u64, SystemTimeError>);

impl Time {
    pub fn new() -> Self {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(timestamp) => Time(Ok(timestamp.as_secs())),
            Err(error) => Time(Err(error)),
        }
    }
    pub fn msec() -> Result<i16, SystemTimeError> {
        let stamp = 
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(timestamp) => timestamp.as_millis(),
            Err(error) => return Err(error),
        };
        Ok((stamp % 1000).try_into().unwrap())
    }
    pub fn sec(&self) -> Result<i8, SystemTimeError> {
        match &self.0 {
            Ok(time) => Ok((time % 60).try_into().unwrap()),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn min(&self) -> Result<i8, SystemTimeError> {
        match &self.0 {
            Ok(time) => Ok((time % 3600 / 60).try_into().unwrap()),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn hour(&self) -> Result<i8, SystemTimeError> {
        match &self.0 {
            Ok(time) => Ok((time / 3600 % 24).try_into().unwrap()),
            Err(error) => Err(error.clone()),
        }
    }
}

pub fn get_formatted_time(use_localtime: bool) -> Result<String, SystemTimeError> {
    let time = Time::new();
    Ok(format!(
        "{:0>2}:{:0>2}:{:0>2}",time.hour()? + if !use_localtime {0} else {time_difference::get()},time.min()?,time.sec()?
    ))
}

pub mod time_difference {
    pub fn get() -> i8 {
        ((get_local_timestamp() - get_utc_timestamp()) / 3600)
            .try_into()
            .unwrap()
    }
    extern "C" {
        fn time(time_p: *const i64) -> i64;
        fn gmtime(timep: *const i64) -> *const i8;
        fn mktime(p_tm: *const i8) -> i64;
        fn localtime(time_p: *const i64) -> *const i8;
    }

    fn get_local_timestamp() -> i64 {
        unsafe {
            let now = time(std::ptr::null());
            mktime(localtime(&now)) - mktime(gmtime(&now)) + now
        }
    }

    fn get_utc_timestamp() -> i64 {
        unsafe { time(std::ptr::null()) }
    }
}
