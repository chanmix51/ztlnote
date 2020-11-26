use uuid::Uuid;
use crate::error::{ZtlnError, Result};

#[derive(Debug, PartialEq)]
pub struct NoteMetaData {
    pub note_id: Uuid,
    pub parent_id: Option<Uuid>,
    pub references: Vec<Uuid>,
    pub topic: String,
}

impl NoteMetaData {
    pub fn parse_meta_file(uuid: Uuid, content: &str) -> Result<Self> {
        let note_id = uuid;
        let mut lines = content.lines();
        let parent_id = lines.next().ok_or_else(|| ZtlnError::ParserError("parent_id".to_string(), None))?;
        let parent_id = if !parent_id.is_empty() { Some(Uuid::parse_str(parent_id)?) } else { None };
        let topic = lines.next().ok_or_else(|| ZtlnError::ParserError("topic".to_string(), None))?.to_string();
        if topic.is_empty() {
            return Err(From::from(ZtlnError::ParserError("topic".to_string(), Some("Field is empty".to_string()))))
        }
        let mut references = Vec::new();
        for reference in lines {
            references.push(Uuid::parse_str(reference)?);
        }
        Ok(Self { note_id, parent_id, references, topic })
    }

    pub fn serialize(&self) -> String {
        let mut buf = String::new();
        for uuid in &self.references {
            buf.push('\n');
            buf.push_str(&uuid.to_string());
        }
        let mut content = self.parent_id
            .map_or("".to_string(), |uuid| uuid.to_string());
        content.push('\n');
        content.push_str(&self.topic);
        content.push_str(&buf);
        
        content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_meta_file_ok() {
        let note_id = Uuid::new_v4();
        let identifier = note_id.to_string();
        let metafileok = {
            let mut f = std::collections::HashMap::new();
            f.insert("\ntopic".to_string(), NoteMetaData { note_id, parent_id: None, topic: "topic".to_string(), references: Vec::new() });
            f.insert(format!("{}\ntopic", identifier), NoteMetaData { note_id, parent_id: Some(note_id), topic: "topic".to_string(), references: Vec::new() });
            f
        };
        for (content, metadata) in &metafileok {
            let res_meta = NoteMetaData::parse_meta_file(note_id, content);
            if res_meta.is_err() {
                println!("Error: {:?}", res_meta);
            }
            assert!(res_meta.is_ok());
            let meta = res_meta.unwrap();
            assert_eq!(*metadata, meta);
        }
    }

    #[test]
    fn serialize_empty() {
        let empty_metadata = NoteMetaData {
            note_id: Uuid::parse_str("ec511da0-b751-4fee-a10a-e1f83cd34ff8").unwrap(),
            parent_id: None,
            references: Vec::new(),
            topic: "topic1".to_string(),
        };
        assert_eq!("\ntopic1", empty_metadata.serialize());
    }
    #[test]
     fn serialize() {
         let metadata = NoteMetaData {
            note_id: Uuid::parse_str("ec511da0-b751-4fee-a10a-e1f83cd34ff8").unwrap(),
            topic: "topic1".to_string(),
            parent_id: Some(Uuid::parse_str("0a0aeade-6dc0-407a-8c67-4951ef4ace7f").unwrap()),
            references: vec![
                Uuid::parse_str("65d436f9-045c-4738-8bdf-d6c3b53ea059").unwrap(),
                Uuid::parse_str("568acc08-74e5-4ab8-a440-42a206009c5f").unwrap(),
                Uuid::parse_str("f0707063-e487-4a96-aa64-00bf6aa10e26").unwrap(),
                Uuid::parse_str("de527948-aeb2-4a91-946a-d0fa231c7a99").unwrap(),
            ],
         };
         let content = r"0a0aeade-6dc0-407a-8c67-4951ef4ace7f
topic1
65d436f9-045c-4738-8bdf-d6c3b53ea059
568acc08-74e5-4ab8-a440-42a206009c5f
f0707063-e487-4a96-aa64-00bf6aa10e26
de527948-aeb2-4a91-946a-d0fa231c7a99";
        assert_eq!(content, metadata.serialize());
     }

     #[test]
     fn serialize_no_parent_id() {
         let metadata = NoteMetaData {
            note_id: Uuid::parse_str("ec511da0-b751-4fee-a10a-e1f83cd34ff8").unwrap(),
            topic: "topic1".to_string(),
            parent_id: None,
            references: vec![
                Uuid::parse_str("65d436f9-045c-4738-8bdf-d6c3b53ea059").unwrap(),
                Uuid::parse_str("568acc08-74e5-4ab8-a440-42a206009c5f").unwrap(),
                Uuid::parse_str("f0707063-e487-4a96-aa64-00bf6aa10e26").unwrap(),
                Uuid::parse_str("de527948-aeb2-4a91-946a-d0fa231c7a99").unwrap(),
            ],
         };
         let content = r"
topic1
65d436f9-045c-4738-8bdf-d6c3b53ea059
568acc08-74e5-4ab8-a440-42a206009c5f
f0707063-e487-4a96-aa64-00bf6aa10e26
de527948-aeb2-4a91-946a-d0fa231c7a99";
        assert_eq!(content, metadata.serialize());
     }
}