use crate::helpers::TestApp;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

const ZUBAT_POKE_RESP: &str = r#"
     {
      "flavor_text_entries": [
        {
          "flavor_text": "Forms colonies in\nperpetually dark\nplaces. Uses\fultrasonic waves\nto identify and\napproach targets.",
          "language": {
            "name": "en",
            "url": "https://pokeapi.co/api/v2/language/9/"
          }
        }
      ],
      "habitat": {
        "name": "cave",
        "url": "https://pokeapi.co/api/v2/pokemon-habitat/1/"
      },
      "is_legendary": false,
      "name": "zubat"
    }       
    "#;

const SQUIRTLE_POKE_RESP: &str = r#"
     {
  "flavor_text_entries": [
    {
      "flavor_text": "After birth, its\nback swells and\nhardens into a\fshell. Powerfully\nsprays foam from\nits mouth.",
      "language": {
        "name": "en",
        "url": "https://pokeapi.co/api/v2/language/9/"
      }
    }
  ],
  "habitat": {
    "name": "waters-edge",
    "url": "https://pokeapi.co/api/v2/pokemon-habitat/9/"
  },
  "is_legendary": false,
  "name": "squirtle"
}      
    "#;

const MEWTWO_POKE_RESP: &str = r#"
     {
  "flavor_text_entries": [
    {
      "flavor_text": "It was created by\na scientist after\nyears of horrific\fgene splicing and\nDNA engineering\nexperiments.",
      "language": {
        "name": "en",
        "url": "https://pokeapi.co/api/v2/language/9/"
      }
    }
  ],
  "habitat": {
    "name": "rare",
    "url": "https://pokeapi.co/api/v2/pokemon-habitat/5/"
  },
  "is_legendary": true,
  "name": "mewtwo"
}      
"#;

const EXPECTED_ZUBAT_RESPONSE: &str = r#"
    {
	"name": "zubat",
        "description": "Forms colonies in perpetually dark places. Uses ultrasonic waves to identify and approach targets.",
	"habitat": "cave",
	"isLegendary": false
    }
"#;

#[tokio::test]
async fn pokemon_returns_200_for_valid_name() {
    let app = TestApp::new().await;

    let data: serde_json::Value = serde_json::from_str(ZUBAT_POKE_RESP).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(data);

    Mock::given(path("/zubat"))
        .and(method("GET"))
        .respond_with(response_template)
        .mount(&app.pokemon_server)
        .await;

    let response = app.get_pokemon_information("zubat").await;

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn pokemon_returns_404_for_invalid_name() {
    let app = TestApp::new().await;

    Mock::given(path("/invalid_name"))
        .and(method("GET"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&app.pokemon_server)
        .await;

    let response = app.get_pokemon_information("zubat").await;

    assert_eq!(404, response.status().as_u16());
}

#[tokio::test]
async fn non_cave_non_legendary_pokemon_calls_shakespeare() {
    let app = TestApp::new().await;

    let data: serde_json::Value = serde_json::from_str(SQUIRTLE_POKE_RESP).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(data);

    Mock::given(path("/squirtle"))
        .and(method("GET"))
        .respond_with(response_template)
        .mount(&app.pokemon_server)
        .await;

    //Use expect to ensure that shakespeare gets hit
    Mock::given(path("/shakespeare"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.translation_server)
        .await;

    let _ = app.get_pokemon_translation("squirtle").await;
}

#[tokio::test]
async fn cave_pokemon_calls_yoda() {
    let app = TestApp::new().await;

    let data: serde_json::Value = serde_json::from_str(ZUBAT_POKE_RESP).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(data);

    Mock::given(path("/zubat"))
        .and(method("GET"))
        .respond_with(response_template)
        .mount(&app.pokemon_server)
        .await;

    //Use expect to ensure that yoda gets hit
    Mock::given(path("/yoda"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.translation_server)
        .await;

    let _ = app.get_pokemon_translation("zubat").await;
}

#[tokio::test]
async fn legendary_pokemon_calls_yoda() {
    let app = TestApp::new().await;

    let data: serde_json::Value = serde_json::from_str(MEWTWO_POKE_RESP).unwrap();
    let response_template = ResponseTemplate::new(200).set_body_json(data);

    Mock::given(path("/mewtwo"))
        .and(method("GET"))
        .respond_with(response_template)
        .mount(&app.pokemon_server)
        .await;

    //Use expect to ensure that yoda gets hit
    Mock::given(path("/yoda"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.translation_server)
        .await;

    let _ = app.get_pokemon_translation("mewtwo").await;
}

#[tokio::test]
async fn failure_to_translate_returns_original() {
    let app = TestApp::new().await;

    let ground_truth: serde_json::Value = serde_json::from_str(EXPECTED_ZUBAT_RESPONSE)
        .expect("Unable to create JSON string from provided data");

    let response_data: serde_json::Value = serde_json::from_str(ZUBAT_POKE_RESP)
        .expect("Unable to create JSON string from provided data");
    let response_template = ResponseTemplate::new(200).set_body_json(response_data);

    Mock::given(path("/zubat"))
        .and(method("GET"))
        .respond_with(response_template)
        .mount(&app.pokemon_server)
        .await;

    //Translation server will 404 without a mock.

    let response_json: serde_json::Value = app
        .get_pokemon_translation("zubat")
        .await
        .json()
        .await
        .expect("Failed to deserialze pokemon");

    assert_eq!(
        ground_truth.get("description"),
        response_json.get("description")
    )
}
