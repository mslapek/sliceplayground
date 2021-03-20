package album

import "time"

// Photo was taken in particular time using camera.
// Has non-unique name given by user.
type Photo struct {
	ID       int
	Name     string
	DateTime time.Time
}

// Album is collection of photos sorted by time.
// Has non-unique name given by user.
type Album struct {
	ID     int
	Name   string
	Photos []Photo
}

// GetPhotos returns photos in given time interval.
func (album *Album) GetPhotos(start, end time.Time) []Photo {
	p := album.Photos
	n := len(p)

	i := 0
	for i < n && p[i].DateTime.Before(start) {
		i++
	}

	j := i
	for j < n && p[j].DateTime.Before(end) {
		j++
	}

	return p[i:j]
}

// GetHolidayPhotos gets photos from June and December.
func GetHolidayPhotos(album *Album, year int) []Photo {
	photos := album.GetPhotos(
		time.Date(year, 6, 1, 0, 0, 0, 0, time.UTC),
		time.Date(year, 7, 1, 0, 0, 0, 0, time.UTC),
	)

	decemberPhotos := album.GetPhotos(
		time.Date(year, 12, 1, 0, 0, 0, 0, time.UTC),
		time.Date(year+1, 1, 1, 0, 0, 0, 0, time.UTC),
	)
	photos = append(photos, decemberPhotos...)

	return photos
}
