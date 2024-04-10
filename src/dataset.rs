use std::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataSplit {
    Train,
    Validation,
    Test,
    Unknown,
}

impl From<&str> for DataSplit {
    fn from(s: &str) -> DataSplit {
        match s {
            "train" => DataSplit::Train,
            "test" => DataSplit::Test,
            "validation" => DataSplit::Validation,
            &_ => DataSplit::Unknown,
        }
    }
}

#[derive(Debug)]
struct Record {
    name: String,
    split: DataSplit,
}

#[derive(Debug)]
pub struct Dataset {
    list: Vec<Record>,
}

impl Dataset {
    pub fn get_splits_for(&self, name: &str) -> Vec<DataSplit> {
        self.list
            .iter()
            .filter(|r| r.name == name)
            .map(|r| r.split)
            .collect()
    }
    pub fn get_names_for(&self, split: DataSplit) -> Vec<String> {
        self.list
            .iter()
            .filter(|r| r.split == split)
            .map(|r| r.name.clone())
            .collect()
    }
}

pub fn read_dataset() -> Result<Dataset, Box<dyn Error>> {
    let path = std::path::Path::new("./static/data.csv");
    let file = match std::fs::File::open(path) {
        Err(why) => return Err(Box::new(why)),
        Ok(file) => file,
    };
    let mut list: Vec<Record> = Vec::new();
    let mut reader = csv::Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        let mut record = record.iter();
        let name = record.next().ok_or("Found no name")?.trim().to_string();
        let split = record.next().ok_or("Found no split")?.trim().into();
        list.push(Record { name, split });
    }
    Ok(Dataset { list })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_splits() {
        let r1 = Record {
            name: "r1".to_string(),
            split: DataSplit::Train,
        };
        let r2 = Record {
            name: "r2".to_string(),
            split: DataSplit::Test,
        };
        let list = vec![r1, r2];
        let dataset = Dataset { list };
        assert_eq!(dataset.get_splits_for("r2"), vec![DataSplit::Test]);
    }
    #[test]
    fn test_get_names() {
        let r1 = Record {
            name: "r1".to_string(),
            split: DataSplit::Train,
        };
        let r2 = Record {
            name: "r2".to_string(),
            split: DataSplit::Test,
        };
        let list = vec![r1, r2];
        let dataset = Dataset { list };
        assert_eq!(
            dataset.get_names_for(DataSplit::Train),
            vec!["r1".to_string()]
        );
    }
}
