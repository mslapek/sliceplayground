package album

import (
	"bytes"
	"encoding/json"
	"os"
	"reflect"
	"testing"
	"time"
)

func createAlbum() *Album {
	return &Album{
		ID:   0,
		Name: "Nature",
		Photos: []Photo{
			{
				ID:       0,
				Name:     "Acacia",
				DateTime: time.Date(2021, 5, 4, 12, 0, 0, 0, time.UTC),
			},
			{
				ID:       1,
				Name:     "Begonia",
				DateTime: time.Date(2021, 5, 30, 12, 0, 0, 0, time.UTC),
			},
			{
				ID:       2,
				Name:     "Camellia",
				DateTime: time.Date(2021, 6, 12, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       3,
				Name:     "Daisy",
				DateTime: time.Date(2021, 6, 14, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       4,
				Name:     "Eustoma",
				DateTime: time.Date(2021, 8, 11, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       5,
				Name:     "Forget Me Not",
				DateTime: time.Date(2021, 8, 11, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       6,
				Name:     "Snow",
				DateTime: time.Date(2021, 12, 11, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       7,
				Name:     "More snow",
				DateTime: time.Date(2021, 12, 15, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       8,
				Name:     "Moon",
				DateTime: time.Date(2022, 1, 1, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       9,
				Name:     "More moon",
				DateTime: time.Date(2022, 6, 1, 14, 0, 0, 0, time.UTC),
			},
			{
				ID:       10,
				Name:     "Snow",
				DateTime: time.Date(2022, 12, 1, 14, 0, 0, 0, time.UTC),
			},
		},
	}
}

func TestGetPhotos(t *testing.T) {
	a := createAlbum()

	got := a.GetPhotos(time.Date(2021, 6, 13, 12, 0, 0, 0, time.UTC), time.Date(2021, 9, 1, 0, 0, 0, 0, time.UTC))
	want := []Photo{
		{
			ID:       3,
			Name:     "Daisy",
			DateTime: time.Date(2021, 6, 14, 14, 0, 0, 0, time.UTC),
		},
		{
			ID:       4,
			Name:     "Eustoma",
			DateTime: time.Date(2021, 8, 11, 14, 0, 0, 0, time.UTC),
		},
		{
			ID:       5,
			Name:     "Forget Me Not",
			DateTime: time.Date(2021, 8, 11, 14, 0, 0, 0, time.UTC),
		},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("Got = %v, want %v", got, want)
	}
}

func TestGetHolidayPhotos(t *testing.T) {
	a := createAlbum()

	got := GetHolidayPhotos(a, 2021)
	want := []Photo{
		{
			ID:       2,
			Name:     "Camellia",
			DateTime: time.Date(2021, 6, 12, 14, 0, 0, 0, time.UTC),
		},
		{
			ID:       3,
			Name:     "Daisy",
			DateTime: time.Date(2021, 6, 14, 14, 0, 0, 0, time.UTC),
		},
		{
			ID:       6,
			Name:     "Snow",
			DateTime: time.Date(2021, 12, 11, 14, 0, 0, 0, time.UTC),
		},
		{
			ID:       7,
			Name:     "More snow",
			DateTime: time.Date(2021, 12, 15, 14, 0, 0, 0, time.UTC),
		},
	}

	if !reflect.DeepEqual(got, want) {
		t.Errorf("Got = %v, want %v", got, want)
	}
}

func TestGetHolidayPhotosNotMutatingInput(t *testing.T) {
	if os.Getenv("MUTATION_TEST") == "" {
		t.Skip("Skipping mutation test")
	}

	album := createAlbum()

	serializedAlbum, err := json.Marshal(album)
	if err != nil {
		t.Fatalf("Got error %v", err)
	}

	GetHolidayPhotos(album, 2021)

	postmortemSerializedAlbum, err := json.Marshal(album)
	if err != nil {
		t.Fatalf("Got error %v", err)
	}

	if !bytes.Equal(postmortemSerializedAlbum, serializedAlbum) {
		t.Errorf(
			"Got postmortemSerializedAlbum = %v, want serializedAlbum = %v",
			string(postmortemSerializedAlbum[:]),
			string(serializedAlbum[:]),
		)
	}
}
