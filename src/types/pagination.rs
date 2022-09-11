use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract from query params,
#[derive(Default, Debug, PartialEq)]
pub struct Pagination {
    /// The index of the last item that has to be returned
    pub limit: Option<u32>,
    /// The index of the first item which has to be returned
    pub offset: u32,
}

/// Extract query parameters from "/questions" endpoint
///
/// # Example query
/// GET requests to this route can have a pagination attached so we just return
/// the questions we need.
///
/// `/questions?limit=0&offset=10`
///
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
///
/// query.insert("limit".to_string(), "1".to_string());
/// query.insert("offset".to_string(), "10".to_string());
///
/// let p = types::pagination::extract_pagination(query).unwrap();
///
/// assert_eq!(p.limit, 1);
/// assert_eq!(p.offset, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improve in the future
    if !params.contains_key("limit") || !params.contains_key("offset") {
        return Err(Error::MissingParameters);
    }

    // Takes a "start" query param from the url and tries to convert it to a number
    let limit = Some(
        params
            .get("limit")
            .unwrap()
            .parse::<u32>()
            .map_err(Error::ParseError)?,
    );
    // Takes a "end" query param from the url and tries to convert it to a number
    let offset = params
        .get("offset")
        .unwrap()
        .parse::<u32>()
        .map_err(Error::ParseError)?;

    Ok(Pagination { limit, offset })
}

#[cfg(test)]
mod pagination_tests {
    use super::{extract_pagination, Error, HashMap, Pagination};

    #[test]
    fn valid_pagination() {
        let mut params = HashMap::new();

        params.insert("limit".to_string(), String::from("1"));
        params.insert("offset".to_string(), String::from("1"));

        let result = extract_pagination(params);

        assert_eq!(
            result.unwrap(),
            Pagination {
                limit: Some(1),
                offset: 1
            }
        );
    }

    #[test]
    fn missing_offset_parameter() {
        let mut params = HashMap::new();

        params.insert("limit".to_string(), String::from("1"));

        let result = extract_pagination(params);

        assert_eq!(
            format!("{}", result.unwrap_err()),
            format!("{}", Error::MissingParameters)
        );
    }

    #[test]
    fn missing_limit_parameter() {
        let mut params = HashMap::new();

        params.insert("offset".to_string(), String::from("1"));

        let result = extract_pagination(params);

        assert_eq!(
            format!("{}", result.unwrap_err()),
            format!("{}", Error::MissingParameters)
        );
    }

    #[test]
    fn parse_limit_parameter_error() {
        let mut params = HashMap::new();

        params.insert("limit".to_string(), String::from("a"));
        params.insert("offset".to_string(), String::from("5"));

        let result = extract_pagination(params);
        let parse_int_error = "a".parse::<u32>();

        assert_eq!(
            format!("{}", result.unwrap_err()),
            format!("{}", Error::ParseError(parse_int_error.unwrap_err()))
        );
    }

    #[test]
    fn parse_offset_parameter_error() {
        let mut params = HashMap::new();

        params.insert("limit".to_string(), String::from("5"));
        params.insert("offset".to_string(), String::from("a"));

        let result = extract_pagination(params);
        let parse_int_error = "a".parse::<u32>();

        assert_eq!(
            format!("{}", result.unwrap_err()),
            format!("{}", Error::ParseError(parse_int_error.unwrap_err()))
        );
    }
}
