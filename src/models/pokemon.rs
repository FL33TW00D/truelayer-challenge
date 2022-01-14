use serde::{
    de,
    de::{Deserializer, IgnoredAny, SeqAccess, Visitor},
    ser::{SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Habitat {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Description {
    pub flavor_text: String,
    language: Language,
}

impl Description {
    pub fn is_english(&self) -> bool {
        self.language.name == "en"
    }
}

#[derive(Debug, Deserialize)]
pub struct Pokemon {
    name: String,
    #[serde(deserialize_with = "deserialize_descriptions")]
    #[serde(rename(deserialize = "flavor_text_entries"))]
    pub description: Description,
    habitat: Habitat,
    #[serde(rename(deserialize = "is_legendary"))]
    pub legendary: bool,
}

impl Pokemon {
    pub fn is_cave(&self) -> bool {
        self.habitat.name == "cave"
    }
}

fn deserialize_descriptions<'de, D>(deserializer: D) -> Result<Description, D::Error>
where
    D: Deserializer<'de>,
{
    struct DescVisitor(PhantomData<fn() -> Description>);

    impl<'de> Visitor<'de> for DescVisitor {
        type Value = Description;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(
                formatter,
                "flavor_text_entries from: https://pokeapi.co/docs/v2#pokemon-species"
            )
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut description: Description = seq
                .next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;

            //Ensure that we get an english description
            if !description.is_english() {
                while let Some(desc) = seq.next_element()? {
                    if description.is_english() {
                        description = desc;
                        break;
                    }
                }
            }

            while let Some(IgnoredAny) = seq.next_element()? {}

            description.flavor_text = description
                .flavor_text
                .chars()
                .map(|c| if c.is_whitespace() { ' ' } else { c })
                .collect();
            Ok(description)
        }
    }

    let visitor = DescVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

impl Serialize for Pokemon {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // 4 is the number of fields in the struct.
        let mut state = serializer.serialize_struct("Pokemon", 4)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("description", &self.description.flavor_text)?;
        state.serialize_field("habitat", &self.habitat.name)?;
        state.serialize_field("isLegendary", &self.legendary)?;
        state.end()
    }
}
