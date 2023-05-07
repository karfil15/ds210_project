use crate::data_processing::DataPoint;

#[derive(Debug)]
pub struct CombinedDataPoint {
    pub date: String,
    pub inflation_rate: f64,
    pub interest_rate: f64,
}

pub fn combine_datasets(inflation_data: Vec<DataPoint>, interest_rate_data: Vec<DataPoint>) -> Vec<CombinedDataPoint> {
    let mut combined_data = Vec::new();

    for inflation_dp in &inflation_data {
        if let Some(interest_rate_dp) = interest_rate_data.iter().find(|x| x.date == inflation_dp.date) {
            combined_data.push(CombinedDataPoint {
                date: inflation_dp.date.clone(),
                inflation_rate: inflation_dp.value,
                interest_rate: interest_rate_dp.value,
            });
        }
    }

    combined_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_processing::{read_inflation_data, read_interest_rate_data};
    use std::error::Error;

    #[test]
    fn test_combine_datasets() -> Result<(), Box<dyn Error>> {
        let inflation_data = read_inflation_data("inflation.csv")?;
        let interest_rate_data = read_interest_rate_data("interest_rates.csv")?;

        let inflation_data = inflation_data.into_iter().take(3).collect();
        let interest_rate_data = interest_rate_data.into_iter().take(3).collect();

        let combined_data = combine_datasets(inflation_data, interest_rate_data);

        assert_eq!(combined_data.len(), 3);
        assert_eq!(combined_data[0].date, "1969-01");
        assert_eq!(combined_data[0].inflation_rate, 0.28);
        assert_eq!(combined_data[0].interest_rate, 6.5);

        assert_eq!(combined_data.len(), 3);
        assert_eq!(combined_data[1].date, "1969-02");
        assert_eq!(combined_data[1].inflation_rate, 0.56);
        assert_eq!(combined_data[1].interest_rate, 6.52);

        assert_eq!(combined_data.len(), 3);
        assert_eq!(combined_data[2].date, "1969-03");
        assert_eq!(combined_data[2].inflation_rate, 0.84);
        assert_eq!(combined_data[2].interest_rate, 6.65);


        Ok(())
    }
}
