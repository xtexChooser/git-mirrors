/*
 * This file is part of ActivityStreams.
 *
 * Copyright © 2020 Riley Trautman
 *
 * ActivityStreams is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * ActivityStreams is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with ActivityStreams.  If not, see <http://www.gnu.org/licenses/>.
 */

/// The type xsd:duration represents a duration of time expressed as a number of years, months,
/// days, hours, minutes, and seconds.
///
/// The format of xsd:duration is PnYnMnDTnHnMnS, where P is a literal value that starts the
/// expression, nY is the number of years followed by a literal Y, nM is the number of months
/// followed by a literal M, nD is the number of days followed by a literal D, T is a literal value
/// that separates the date and time, nH is the number of hours followed by a literal H, nM is the
/// number of minutes followed by a literal M, and nS is the number of seconds followed by a
/// literal S. The following rules apply to xsd:duration values:
///
/// - Any of these numbers and corresponding designators may be absent if they are equal to 0, but
///   at least one number and designator must appear.
/// - The numbers may be any unsigned integer, with the exception of the number of seconds, which
///   may be an unsigned decimal number.
/// - If a decimal point appears in the number of seconds, there must be at least one digit after
///   the decimal point.
/// - A minus sign may appear before the P to specify a negative duration.
/// - If no time items (hour, minute, second) are present, the letter T must not appear.
///
/// ### Note
///
/// This implementation converts Months to Days by multiplying by 31, and converts Years to days by
/// multiplying by 365. If this is an issue for your application, look into specifying days
/// directly.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct XsdDuration(pub chrono::Duration);

/// The error type produced when an XsdDuration cannot be parsed
#[derive(Clone, Debug, thiserror::Error)]
#[error("Error parsing Duration")]
pub struct XsdDurationError;

impl XsdDuration {
    /// Create a new XsdDuration from a chrono::Duration
    pub fn new(duration: chrono::Duration) -> Self {
        XsdDuration(duration)
    }

    /// Extract the chrono::Duration from an XsdDuration
    pub fn into_inner(self) -> chrono::Duration {
        self.0
    }

    /// Borrow the underlying `chrono::Duration`
    pub fn as_duration(&self) -> &chrono::Duration {
        self.as_ref()
    }

    /// Mutably borrow the underlying `chrono::Duration`
    pub fn as_duration_mut(&mut self) -> &mut chrono::Duration {
        self.as_mut()
    }
}

impl From<chrono::Duration> for XsdDuration {
    fn from(d: chrono::Duration) -> Self {
        XsdDuration(d)
    }
}

impl From<XsdDuration> for chrono::Duration {
    fn from(d: XsdDuration) -> Self {
        d.0
    }
}

impl AsRef<chrono::Duration> for XsdDuration {
    fn as_ref(&self) -> &chrono::Duration {
        &self.0
    }
}

impl AsMut<chrono::Duration> for XsdDuration {
    fn as_mut(&mut self) -> &mut chrono::Duration {
        &mut self.0
    }
}

impl std::convert::TryFrom<String> for XsdDuration {
    type Error = XsdDurationError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&str> for XsdDuration {
    type Error = XsdDurationError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::convert::TryFrom<&mut str> for XsdDuration {
    type Error = XsdDurationError;

    fn try_from(s: &mut str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::str::FromStr for XsdDuration {
    type Err = XsdDurationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.find('P') != Some(0) {
            return Err(XsdDurationError);
        }

        let s = s.trim_start_matches('P');

        let negative = Some(0) == s.find('-');
        let s = s.trim_start_matches('-');

        let (large, small) = if let Some(index) = s.find('T') {
            let (l, s) = s.split_at(index);
            (l, s.trim_start_matches('T'))
        } else {
            (s, "")
        };

        let (years, large) = parse_next(large, 'Y')?;
        let (months, large) = parse_next(large, 'M')?;
        let (days, _) = parse_next(large, 'D')?;

        let (hours, small) = parse_next(small, 'H')?;
        let (minutes, small) = parse_next(small, 'M')?;
        let (seconds, _) = parse_next(small, 'S')?;

        let mut duration = chrono::Duration::days(365 * years);
        duration = duration + chrono::Duration::days(31 * months);
        duration = duration + chrono::Duration::days(days);
        duration = duration + chrono::Duration::hours(hours);
        duration = duration + chrono::Duration::minutes(minutes);
        duration = duration + chrono::Duration::seconds(seconds);

        duration = if negative { duration * -1 } else { duration };

        Ok(XsdDuration(duration))
    }
}

fn parse_next(s: &str, c: char) -> Result<(i64, &str), XsdDurationError> {
    let res = if let Some(index) = s.find(c) {
        let (beginning, end) = s.split_at(index);
        let i = beginning.parse().map_err(|_| XsdDurationError)?;
        (i, end.trim_start_matches(c))
    } else {
        (0, s)
    };

    Ok(res)
}

impl std::fmt::Display for XsdDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let (s, mut duration) = if chrono::Duration::seconds(0) > self.0 {
            ("P-".to_string(), self.0 * -1)
        } else {
            ("P".to_string(), self.0)
        };

        let s = if duration.num_days() > 0 {
            format!("{}{}D", s, duration.num_days())
        } else {
            s
        };

        duration = duration - chrono::Duration::days(duration.num_days());

        let s = if duration.num_seconds() > 0 {
            format!("{}T", s)
        } else {
            s
        };

        let s = if duration.num_hours() > 0 {
            format!("{}{}H", s, duration.num_hours())
        } else {
            s
        };

        duration = duration - chrono::Duration::hours(duration.num_hours());

        let s = if duration.num_minutes() > 0 {
            format!("{}{}M", s, duration.num_minutes())
        } else {
            s
        };

        duration = duration - chrono::Duration::minutes(duration.num_minutes());

        let s = if duration.num_seconds() > 0 {
            format!("{}{}S", s, duration.num_seconds())
        } else {
            s
        };

        std::fmt::Display::fmt(&s, f)
    }
}

impl serde::ser::Serialize for XsdDuration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::de::Deserialize<'de> for XsdDuration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}
