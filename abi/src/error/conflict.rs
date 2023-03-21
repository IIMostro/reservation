use std::collections::HashMap;
use std::convert::Infallible;
use std::hash::Hash;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use regex::Regex;

// 错误转换
#[derive(Debug, Clone)]
pub struct ReservationConflict {
    pub new: ReservationWindow,
    pub old: ReservationWindow,
}

#[derive(Debug, Clone)]
pub enum ReservationConflictInfo {
    Parsed(ReservationConflict),
    Unparsed(String),
}

#[derive(Debug, Clone)]
pub struct ReservationWindow {
    pub rid: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

// str 转 reservationConflictInfo
impl FromStr for ReservationConflictInfo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(conflict) = s.parse() {
            Ok(ReservationConflictInfo::Parsed(conflict))
        } else {
            Ok(ReservationConflictInfo::Unparsed(s.to_string()))
        }
    }
}

impl FromStr for ReservationConflict {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ParseInfo::from_str(s)?.try_into()
    }
}

impl TryFrom<ParseInfo> for ReservationConflict {
    type Error = ();

    fn try_from(value: ParseInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            new: value.new.try_into()?,
            old: value.old.try_into()?,
        })
    }
}

impl TryFrom<HashMap<String, String>> for ReservationWindow {
    type Error = ();

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let timespan_str = value.get("timespan").ok_or(())?.replace('"', "");
        let mut split = timespan_str.splitn(2, ',');
        let start = parse_datetime(split.next().ok_or(())?)?;
        let end = parse_datetime(split.next().ok_or(())?)?;
        Ok(Self {
            rid: value.get("resource_id").ok_or(())?.to_string(),
            start,
            end,
        })
    }
}

fn parse_datetime(s: &str) -> Result<DateTime<Utc>, ()> {
    Ok(DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%#z").map_err(|_| ())?.with_timezone(&Utc))
}


struct ParseInfo {
    new: HashMap<String, String>,
    old: HashMap<String, String>,
}

impl FromStr for ParseInfo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r#"\((?P<k1>[a-zA-Z0-9_-]+)\s*,\s*(?P<k2>[a-zA-Z0-9_-]+)\)=\((?P<v1>[a-zA-Z0-9_-]+)\s*,\s*\[(?P<v2>[^\)\]]+)"#).unwrap();
        let mut maps: Vec<Option<HashMap<String, String>>> = vec![];
        for cap in re.captures_iter(s) {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert(cap["k1"].to_string(), cap["v1"].to_string());
            map.insert(cap["k2"].to_string(), cap["v2"].to_string());
            maps.push(Some(map));
        }
        if maps.len() != 2 {
            return Err(());
        }
        Ok(ParseInfo {
            new: maps[0].take().unwrap(),
            old: maps[1].take().unwrap(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_MSG: &str = "Key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\")) conflicts with existing key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-25 22:00:00+00\",\"2022-12-28 19:00:00+00\")).";

    #[test]
    fn parsed_info_should_work() {
        let info: ParseInfo = ERROR_MSG.parse().unwrap();
        assert_eq!(info.new["resource_id"], "ocean-view-room-713");
        assert_eq!(info.new["timespan"], "\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\"");
        assert_eq!(info.old["resource_id"], "ocean-view-room-713");
        assert_eq!(info.old["timespan"], "\"2022-12-25 22:00:00+00\",\"2022-12-28 19:00:00+00\"")
    }

    #[test]
    fn hash_map_to_reservation_window_should_work() {
        let mut map = HashMap::new();
        map.insert("resource_id".to_string(), "ocean-view-room-713".to_string());
        map.insert("timespan".to_string(), "\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\"".to_string());
        let window: ReservationWindow = map.try_into().unwrap();
        assert_eq!(window.rid, "ocean-view-room-713");
        assert_eq!(window.start.to_rfc3339(), "2022-12-26T22:00:00+00:00");
        assert_eq!(window.end.to_rfc3339(), "2022-12-30T19:00:00+00:00");
    }
}