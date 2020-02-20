
/// fred/source/releases endpoint
/// 
/// ```
/// use fred_rs::client::FredClient;
/// use fred_rs::source::releases::{Builder, Response, SortOrder, OrderBy};
/// 
/// let mut c = match FredClient::new() {
///     Ok(c) => c,
///     Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// let mut builder = Builder::new();
/// builder
///     .limit(5)
///     .order_by(OrderBy::Name)
///     .sort_order(SortOrder::Descending);
/// 
/// let resp: Response = match c.source_releases(1, Some(builder)) {
///     Ok(resp) => resp,
///         Err(msg) => {
///         println!("{}", msg);
///         assert_eq!(2, 1);
///         return
///     },
/// };
/// 
/// for item in resp.releases {
///     match item.link {
///         Some(l) => println!("{}: {}", item.name, l),
///         None => println!("{}: No Link", item.name),
///     }
/// }
/// ```
pub mod releases;

// -----------------------------------------------------------------------------

use serde::Deserialize;

#[derive(Deserialize)]
/// Response data structure for the fred/sources endpoint
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source.html] (https://research.stlouisfed.org/docs/api/fred/source.html)
pub struct Response {
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// Series returned by the search
    pub sources: Vec<Source>,
}

#[derive(Deserialize)]
/// Data structure containing infomation about a particular tag
/// 
/// [https://research.stlouisfed.org/docs/api/fred/source.html](https://research.stlouisfed.org/docs/api/fred/source.html)
pub struct Source {
    /// The source ID
    pub id: usize,
    /// The Real Time start date for the request
    pub realtime_start: String,
    /// The Real Time end data for the request
    pub realtime_end: String,
    /// The source name
    pub name: String,
    /// A link to the source's website
    pub link: Option<String>,
}

pub struct Builder {
    option_string: String,
}

impl Builder {

    /// Initializes a new sources::Builder that can be used to add commands to an API request
    /// 
    /// The builder does not check for duplicate arguments and instead adds all arguments to the URL string.  The FRED API behavior for duplicates in unknown.
    /// 
    /// ```
    /// use fred_rs::sources::Builder;
    /// // Create a new builder
    /// let mut builder = Builder::new();
    /// // add arguments to the builder
    /// builder
    ///     .realtime_start("1900-01-01")
    ///     .realtime_end("2000-01-01");
    /// ```
    pub fn new() -> Builder {
        Builder {
            option_string: String::new(),
        }
    }

    /// Returns the current arguments as a URL formatted string
    pub fn options(self) -> String {
        self.option_string
    }

    /// Adds a realtime_start argument to the builder
    /// 
    /// # Arguments
    /// * `start_date` - date formatted as YYYY-MM-DD
    pub fn realtime_start(&mut self, start_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_start={}", start_date).as_str();
        self
    }

    /// Adds a realtime_end argument to the builder
    /// 
    /// # Arguments
    /// * `end_date` - date formatted as YYYY-MM-DD
    pub fn realtime_end(&mut self, end_date: &str) -> &mut Builder {
        self.option_string += format!("&realtime_end={}", end_date).as_str();
        self
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::FredClient;

    #[test]
    fn source_no_options() {
        let mut c = match FredClient::new() {
            Ok(c) => c,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        let resp: Response = match c.source(1, None) {
            Ok(resp) => resp,
            Err(msg) => {
                println!("{}", msg);
                assert_eq!(2, 1);
                return
            },
        };

        for s in resp.sources {
            match s.link {
                Some(l) => println!("{}: {}", s.name, l),
                None => println!("{}", s.name)
            }
        }
    } 
}