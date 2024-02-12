/* Tiny Tiny Web
 * Copyright (C) 2024 Plasma (https://github.com/duoduo70/Tiny-Tiny-Web/).
 *
 * You should have received a copy of the GNU General Public License
 * along with this program;
 * if not, see <https://www.gnu.org/licenses/>.
 */
use std::time::{SystemTime, SystemTimeError};

// DONT USE crate::log in this module

/// 时间结构体
/// timestamp: 时间戳，以秒为单位
/// 使用内置的 getter 来获取结构体内的成员
pub struct Time {
    timestamp: Result<u64, SystemTimeError>,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    min: u32,
    sec: u32,
    yday: u32,
    mday: u32,
    wday: u32,
}

impl Time {
    /// 由我和 min0911 为 Plant-OS 项目编写的 C 代码改写而来，原版本有注释
    /// See: https://github.com/min0911Y/Plant-OS/blob/main/apps/libp/time.c
    /// 2024 年 2 月为止，该实现有 bug ，所以请查看最原始版本：
    /// https://github.com/ZhouZhihaos/Powerint-DOS-386/blob/31222fad9ae303daa35d27844cc335c87ee2f1c7/apps/src/time.c
    fn builder(timestamp: u64) -> Self {
        let mut y = 1970;
        let mut _timestamp = timestamp;
        loop {
            if (y % 4 == 0 && y % 100 != 0) || y & 400 == 0 {
                _timestamp -= 31622400;
                if _timestamp <= 31536000 {
                    y += 1;
                    break;
                }
            } else {
                _timestamp -= 31536000;
                if _timestamp <= 31536000
                    || ((((y + 1) % 4 == 0 && (y + 1) % 100 != 0) || (y + 1) % 400 == 0)
                        && timestamp <= 31622400)
                {
                    y += 1;
                    break;
                }
            }
            y += 1;
        }
        let mut _month = 1;
        let table1 = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let table2 = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        if ((y + 1) % 4 == 0 && (y + 1) % 100 != 0) || (y + 1) % 400 == 0 {
            while _timestamp > table2[_month - 1] * 86400 {
                _timestamp -= table2[_month - 1] * 86400;
                _month += 1;
            }
        } else {
            while _timestamp > table2[_month - 1] * 86400 {
                _timestamp -= table1[_month - 1] * 86400;
                _month += 1;
            }
        }

        let mut tmp_time = Time {
            timestamp: Ok(timestamp),
            year: y,
            month: _month as u32,
            day: (_timestamp / 86400 + 1) as u32,
            hour: (_timestamp % 86400 / 3600) as u32,
            min: (_timestamp % 86400 % 3600 / 60) as u32,
            sec: (_timestamp % 86400 % 3600 % 60) as u32,
            yday: 0,
            mday: 0,
            wday: 0,
        };
        let mut days = 0;
        if (tmp_time.year % 4 == 0 && tmp_time.year % 100 != 0) || tmp_time.year % 400 == 0 {
            let mut i = 0;
            while i < _month - 1 {
                days += table2[i];
                i += 1;
            }
        } else {
            let mut i = 0;
            while i < _month - 1 {
                days += table1[i];
                i += 1;
            }
        }
        tmp_time.yday = days as u32 + tmp_time.day;
        tmp_time.mday = tmp_time.day;
        let mut total_days = tmp_time.yday - 1;
        let start_year = 1900;
        let mut year1 = start_year;
        while year1 < tmp_time.year {
            if (year1 % 4 == 0 && year1 % 100 != 0) || year1 % 400 == 0 {
                total_days += 356;
            } else {
                total_days += 365;
            }
            year1 += 1;
        }
        let weekday = total_days % 7;
        tmp_time.wday = weekday;
        tmp_time
    }
    pub fn new() -> Self {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(timestamp) => Time::builder(timestamp.as_secs()),
            Err(error) => Time {
                timestamp: Err(error),
                year: 0,
                month: 0,
                day: 0,
                hour: 0,
                min: 0,
                sec: 0,
                yday: 0,
                mday: 0,
                wday: 0,
            },
        }
    }
    pub fn msec() -> Result<i16, SystemTimeError> {
        let stamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(timestamp) => timestamp.as_millis(),
            Err(error) => return Err(error),
        };
        Ok((stamp % 1000).try_into().unwrap())
    }
    pub fn nsec() -> Result<u16, SystemTimeError> {
        let stamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(timestamp) => timestamp.as_nanos(),
            Err(error) => return Err(error),
        };
        Ok((stamp % 1000).try_into().unwrap())
    }
    pub fn sec(&self) -> Result<u32, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok(self.sec),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn min(&self) -> Result<u32, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok(self.min),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn hour(&self) -> Result<u32, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok(self.hour),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn day(&self) -> Result<u32, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok(self.day),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn year(&self) -> Result<u32, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok(self.year),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn wday_name(&self) -> Result<&str, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok({
                match self.wday {
                    1 => "Mon",
                    2 => "Tue",
                    3 => "Wed",
                    4 => "Thu",
                    5 => "Fri",
                    6 => "Sat",
                    7 => "Sun",
                    _ => "",
                }
            }),
            Err(error) => Err(error.clone()),
        }
    }
    pub fn month_name(&self) -> Result<&str, SystemTimeError> {
        match &self.timestamp {
            Ok(_) => Ok({
                match self.month {
                    1 => "Jan",
                    2 => "Feb",
                    3 => "Mar",
                    4 => "Apr",
                    5 => "May",
                    6 => "Jun",
                    7 => "Jul",
                    8 => "Aug",
                    9 => "Sep",
                    10 => "Oct",
                    11 => "Nov",
                    12 => "Dec",
                    _ => "",
                }
            }),
            Err(error) => Err(error.clone()),
        }
    }
}

pub fn get_formatted_time(use_localtime: bool) -> Result<String, SystemTimeError> {
    let time = Time::new();
    Ok(format!(
        "{:0>2}:{:0>2}:{:0>2}",
        (time.hour()? as i8
            + if !use_localtime {
                0
            } else {
                time_difference::get()
            })
            % 24,
        time.min()?,
        time.sec()?
    ))
}

/// 用以计算时差
/// 注意，这是 unsafe 的。
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

    pub fn get_local_timestamp() -> i64 {
        unsafe {
            let now = time(std::ptr::null());
            mktime(localtime(&now)) - mktime(gmtime(&now)) + now
        }
    }

    pub fn get_utc_timestamp() -> i64 {
        unsafe { time(std::ptr::null()) }
    }
}
