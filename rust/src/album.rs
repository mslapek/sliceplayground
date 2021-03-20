use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// Photo was taken in particular time using camera.
/// Has non-unique name given by user.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub id: u64,
    pub name: String,
    pub date_time: DateTime<Utc>,
}

/// Album is collection of photos sorted by time.
/// Has non-unique name given by user.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Album {
    pub id: u64,
    pub name: String,
    pub photos: Vec<Photo>,
}

impl Album {
    /// Returns photos in given time interval.
    pub fn get_photos(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> &[Photo] {
        let p = &self.photos;
        let n = p.len();

        let mut i = 0;
        while i < n && p[i].date_time < start {
            i += 1;
        }

        let mut j = i;
        while j < n && p[j].date_time < end {
            j += 1;
        }

        &p[i..j]
    }
}

/// Gets photos from June and December.
pub fn get_holiday_photos(album: &Album, year: i32) -> Vec<Photo> {
    let mut photos = Vec::new();

    let june_photos = album.get_photos(
        DateTime::from_utc(NaiveDate::from_ymd(year, 6, 1).and_hms(0, 0, 0), Utc),
        DateTime::from_utc(NaiveDate::from_ymd(year, 7, 1).and_hms(0, 0, 0), Utc),
    );
    photos.extend_from_slice(june_photos);

    let december_photos = album.get_photos(
        DateTime::from_utc(NaiveDate::from_ymd(year, 12, 1).and_hms(0, 0, 0), Utc),
        DateTime::from_utc(NaiveDate::from_ymd(year + 1, 1, 1).and_hms(0, 0, 0), Utc),
    );
    photos.extend_from_slice(december_photos);

    photos
}

#[cfg(test)]
mod test {
    use super::*;

    fn from_ymdhms(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
        sec: u32,
    ) -> DateTime<Utc> {
        DateTime::from_utc(
            NaiveDate::from_ymd(year, month, day).and_hms(hour, min, sec),
            Utc,
        )
    }

    fn create_album() -> Album {
        Album {
            id: 0,
            name: String::from("Nature"),
            photos: vec![
                Photo {
                    id: 0,
                    name: String::from("Acacia"),
                    date_time: from_ymdhms(2021, 5, 4, 12, 0, 0),
                },
                Photo {
                    id: 1,
                    name: String::from("Begonia"),
                    date_time: from_ymdhms(2021, 5, 30, 12, 0, 0),
                },
                Photo {
                    id: 2,
                    name: String::from("Camellia"),
                    date_time: from_ymdhms(2021, 6, 12, 14, 0, 0),
                },
                Photo {
                    id: 3,
                    name: String::from("Daisy"),
                    date_time: from_ymdhms(2021, 6, 14, 14, 0, 0),
                },
                Photo {
                    id: 4,
                    name: String::from("Eustoma"),
                    date_time: from_ymdhms(2021, 8, 11, 14, 0, 0),
                },
                Photo {
                    id: 5,
                    name: String::from("Forget Me Not"),
                    date_time: from_ymdhms(2021, 8, 11, 14, 0, 0),
                },
                Photo {
                    id: 6,
                    name: String::from("Snow"),
                    date_time: from_ymdhms(2021, 12, 11, 14, 0, 0),
                },
                Photo {
                    id: 7,
                    name: String::from("More snow"),
                    date_time: from_ymdhms(2021, 12, 15, 14, 0, 0),
                },
                Photo {
                    id: 8,
                    name: String::from("Moon"),
                    date_time: from_ymdhms(2022, 1, 1, 14, 0, 0),
                },
                Photo {
                    id: 9,
                    name: String::from("More moon"),
                    date_time: from_ymdhms(2022, 6, 1, 14, 0, 0),
                },
                Photo {
                    id: 10,
                    name: String::from("Snow"),
                    date_time: from_ymdhms(2022, 12, 1, 14, 0, 0),
                },
            ],
        }
    }

    #[test]
    fn test_get_photos() {
        let a = create_album();

        let got = a.get_photos(
            from_ymdhms(2021, 6, 13, 12, 0, 0),
            from_ymdhms(2021, 9, 1, 0, 0, 0),
        );
        let want = vec![
            Photo {
                id: 3,
                name: String::from("Daisy"),
                date_time: from_ymdhms(2021, 6, 14, 14, 0, 0),
            },
            Photo {
                id: 4,
                name: String::from("Eustoma"),
                date_time: from_ymdhms(2021, 8, 11, 14, 0, 0),
            },
            Photo {
                id: 5,
                name: String::from("Forget Me Not"),
                date_time: from_ymdhms(2021, 8, 11, 14, 0, 0),
            },
        ];

        assert_eq!(got, want);
    }

    #[test]
    fn test_get_holiday_photos() {
        let a = create_album();

        let got = get_holiday_photos(&a, 2021);
        let want = vec![
            Photo {
                id: 2,
                name: String::from("Camellia"),
                date_time: from_ymdhms(2021, 6, 12, 14, 0, 0),
            },
            Photo {
                id: 3,
                name: String::from("Daisy"),
                date_time: from_ymdhms(2021, 6, 14, 14, 0, 0),
            },
            Photo {
                id: 6,
                name: String::from("Snow"),
                date_time: from_ymdhms(2021, 12, 11, 14, 0, 0),
            },
            Photo {
                id: 7,
                name: String::from("More snow"),
                date_time: from_ymdhms(2021, 12, 15, 14, 0, 0),
            },
        ];
        assert_eq!(got, want);
    }

    #[test]
    fn test_get_holiday_photos_not_mutating_input() {
        // Rust's typesystem guarantees that this test will pass.
        // Provided to make port of Golang example.

        let album = create_album();

        let serialized_album = serde_json::to_string(&album).unwrap();

        get_holiday_photos(&album, 2021);

        let postmortem_serialized_album = serde_json::to_string(&album).unwrap();

        assert_eq!(postmortem_serialized_album, serialized_album);
    }
}
