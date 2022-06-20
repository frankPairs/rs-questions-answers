use std::collections::HashMap;

use handle_errors::Error;

/// Pagination struct which is getting extract from query params,
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

/// Extract query parameters from "/questions" endpoint
///
/// # Example query
/// GET requests to this route can have a pagination attached so we just return
/// the questions we need.
///
/// `/questions?start=0&end=10`
///
/// # Example usage
/// ```rust
/// let mut query = HashMap::new();
///
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
///
/// let p = types::pagination::extract_pagination(query).unwrap();
///
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Could be improve in the future
    if !params.contains_key("start") || !params.contains_key("end") {
        return Err(Error::MissingParameters);
    }

    // Takes a "start" query param from the url and tries to convert it to a number
    let start = params
        .get("start")
        .unwrap()
        .parse::<usize>()
        .map_err(|err| Error::ParseError(err))?;
    // Takes a "end" query param from the url and tries to convert it to a number
    let end = params
        .get("end")
        .unwrap()
        .parse::<usize>()
        .map_err(|err| Error::ParseError(err))?;

    Ok(Pagination { start, end })
}
