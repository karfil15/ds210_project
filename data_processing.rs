use serde::Deserialize;
use csv::Reader;
use std::error::Error;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct DataPoint {
    #[serde(rename = "TIME")]
    pub date: String,
    #[serde(rename = "Value")]
    pub value: f64,
}

pub fn read_inflation_data<P: AsRef<Path>>(path: P) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut data_points = Vec::new();

    for result in reader.deserialize() {
        let mut record: DataPoint = result?;
        record.value = (record.value * 100.0).round() / 100.0;
        data_points.push(record);
    }

    Ok(data_points)
}

pub fn read_interest_rate_data<P: AsRef<Path>>(path: P) -> Result<Vec<DataPoint>, Box<dyn Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut data_points = Vec::new();

    for result in reader.deserialize() {
        let mut record: DataPoint = result?;
        record.value = (record.value * 100.0).round() / 100.0;
        data_points.push(record);
    }

    Ok(data_points)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn create_test_inflation_csv() -> tempfile::NamedTempFile {
        let mut tmp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            tmp_file,
            "TIME,Value
1969-01,0.2816902
1969-02,0.5617977"
        )
        .unwrap();
        tmp_file
    }
    

    #[test]
    fn test_read_inflation_data() {
        let tmp_file = create_test_inflation_csv();
        let result = read_inflation_data(tmp_file.path());
        assert!(result.is_ok());
        let data_points = result.unwrap();
        assert_eq!(data_points.len(), 2);
        assert_eq!(data_points[0].date, "1969-01");
        assert_eq!(data_points[0].value, 0.28);
        assert_eq!(data_points[1].date, "1969-02");
        assert_eq!(data_points[1].value, 0.56);
    }
}
