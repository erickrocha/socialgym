use super::*;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn is_enabled_defaults_to_false_when_unset() {
    let _guard = ENV_LOCK.lock().unwrap();
    unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
    assert!(!is_enabled());
}

#[test]
fn is_enabled_true_when_set_to_true() {
    let _guard = ENV_LOCK.lock().unwrap();
    unsafe { env::set_var(GOOGLE_MAPS_ENABLED, "true") };
    assert!(is_enabled());
    unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
}

#[test]
fn is_enabled_false_for_garbage_value() {
    let _guard = ENV_LOCK.lock().unwrap();
    unsafe { env::set_var(GOOGLE_MAPS_ENABLED, "yes-please") };
    assert!(!is_enabled());
    unsafe { env::remove_var(GOOGLE_MAPS_ENABLED) };
}

const SAMPLE_RESPONSE: &str = r#"{
    "places": [
        {
            "id": "ChIJ_abc123",
            "formattedAddress": "1600 Amphitheatre Pkwy, Mountain View, CA 94043, USA",
            "location": { "latitude": 37.4224764, "longitude": -122.0842499 },
            "addressComponents": [
                { "longText": "1600", "shortText": "1600", "types": ["street_number"] },
                { "longText": "Amphitheatre Parkway", "shortText": "Amphitheatre Pkwy", "types": ["route"] },
                { "longText": "Mountain View", "shortText": "Mountain View", "types": ["locality", "political"] },
                { "longText": "California", "shortText": "CA", "types": ["administrative_area_level_1", "political"] },
                { "longText": "94043", "shortText": "94043", "types": ["postal_code"] },
                { "longText": "United States", "shortText": "US", "types": ["country", "political"] }
            ]
        }
    ]
}"#;

#[test]
fn parses_google_response_and_maps_full_address() {
    let parsed: SearchTextResponse = serde_json::from_str(SAMPLE_RESPONSE).unwrap();
    let candidates: Vec<AddressCandidate> = parsed.places.into_iter().map(map_place).collect();

    assert_eq!(candidates.len(), 1);
    let candidate = &candidates[0];
    assert_eq!(candidate.place_id, "ChIJ_abc123");
    assert_eq!(candidate.address_line1, "Amphitheatre Parkway 1600");
    assert_eq!(candidate.locality, "Mountain View");
    assert_eq!(candidate.administrative_area, "California");
    assert_eq!(candidate.administrative_area_code, "CA");
    assert_eq!(candidate.postal_code, Some("94043".to_string()));
    assert_eq!(candidate.country_code, "US");
    assert!((candidate.latitude - 37.4224764).abs() < f64::EPSILON);
    assert!((candidate.longitude - (-122.0842499)).abs() < f64::EPSILON);
    assert_eq!(candidate.address_line2, None);
}

#[test]
fn falls_back_to_formatted_address_when_no_street_components() {
    let response = r#"{
        "places": [{
            "id": "ChIJ_no_street",
            "formattedAddress": "Some Region, Some Country",
            "location": { "latitude": 1.0, "longitude": 2.0 },
            "addressComponents": []
        }]
    }"#;
    let parsed: SearchTextResponse = serde_json::from_str(response).unwrap();
    let candidates: Vec<AddressCandidate> = parsed.places.into_iter().map(map_place).collect();

    assert_eq!(candidates[0].address_line1, "Some Region, Some Country");
    assert_eq!(candidates[0].locality, "");
    assert_eq!(candidates[0].country_code, "");
    assert_eq!(candidates[0].postal_code, None);
}

#[test]
fn location_bias_serializes_with_google_field_names() {
    let request = SearchTextRequest {
        text_query: "1600 Amphitheatre".to_string(),
        location_bias: Some(LocationBias {
            circle: Circle {
                center: LatLng {
                    latitude: 37.4,
                    longitude: -122.0,
                },
                radius: BIAS_RADIUS_METERS,
            },
        }),
        page_size: PAGE_SIZE,
    };
    let json = serde_json::to_value(&request).unwrap();

    assert_eq!(json["textQuery"], "1600 Amphitheatre");
    assert_eq!(json["pageSize"], PAGE_SIZE);
    assert_eq!(json["locationBias"]["circle"]["center"]["latitude"], 37.4);
    assert_eq!(json["locationBias"]["circle"]["radius"], BIAS_RADIUS_METERS);
}

#[test]
fn location_bias_is_omitted_when_none() {
    let request = SearchTextRequest {
        text_query: "text".to_string(),
        location_bias: None,
        page_size: PAGE_SIZE,
    };
    let json = serde_json::to_value(&request).unwrap();

    assert!(json.get("locationBias").is_none());
}
