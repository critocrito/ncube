use std::fmt;
use thiserror::Error;

static PREFIX: &'static [u8] = b"bytes=";
const PREFIX_LEN: usize = 6;

#[derive(Debug, Error)]
pub(crate) enum HeaderParseError {
    HttpRangeError,
}

impl fmt::Display for HeaderParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HttpRangeError => write!(f, "HTTP Range header error"),
        }
    }
}

impl warp::reject::Reject for HeaderParseError {}

/// HTTP Range header representation.
#[derive(Debug, Clone, Copy)]
pub(crate) struct HttpRange {
    pub(crate) start: u64,
    pub(crate) length: u64,
}

impl HttpRange {
    pub(crate) fn parse_first(header: &str, size: u64) -> Result<HttpRange, HeaderParseError> {
        Self::parse_bytes_first(header.as_bytes(), size)
    }

    pub(crate) fn parse_bytes_first(
        header: &[u8],
        size: u64,
    ) -> Result<HttpRange, HeaderParseError> {
        Self::parse_bytes(header, size)?
            .first()
            .map(|v| v.to_owned())
            .ok_or_else(|| HeaderParseError::HttpRangeError)
    }

    #[allow(dead_code)]
    pub(crate) fn parse(header: &str, size: u64) -> Result<Vec<HttpRange>, HeaderParseError> {
        Self::parse_bytes(header.as_bytes(), size)
    }

    pub(crate) fn parse_bytes(
        header: &[u8],
        size: u64,
    ) -> Result<Vec<HttpRange>, HeaderParseError> {
        if header.is_empty() {
            return Ok(Vec::new());
        }

        if !header.starts_with(PREFIX) {
            return Err(HeaderParseError::HttpRangeError);
        }

        let mut no_overlap = false;

        let ranges: Vec<HttpRange> = header[PREFIX_LEN..]
            .split(|b| *b == b',')
            .filter_map(|ra| {
                let ra = ra.trim();
                if ra.is_empty() {
                    return None;
                }
                match Self::parse_single_range(ra, size) {
                    Ok(Some(range)) => Some(Ok(range)),
                    Ok(None) => {
                        no_overlap = true;
                        None
                    }
                    Err(e) => Some(Err(e)),
                }
            })
            .collect::<Result<_, _>>()?;

        if no_overlap && ranges.is_empty() {
            return Err(HeaderParseError::HttpRangeError);
        }

        Ok(ranges)
    }

    fn parse_single_range(bytes: &[u8], size: u64) -> Result<Option<HttpRange>, HeaderParseError> {
        let mut start_end_iter = bytes.splitn(2, |b| *b == b'-');

        let start_str = start_end_iter
            .next()
            .ok_or(HeaderParseError::HttpRangeError)?
            .trim();
        let end_str = start_end_iter
            .next()
            .ok_or(HeaderParseError::HttpRangeError)?
            .trim();

        if start_str.is_empty() {
            // If no start is specified, end specifies the
            // range start relative to the end of the file,
            // and we are dealing with <suffix-length>
            // which has to be a non-negative integer as per
            // RFC 7233 Section 2.1 "Byte-Ranges".
            if end_str.is_empty() || end_str[0] == b'-' {
                return Err(HeaderParseError::HttpRangeError);
            }

            let mut length: u64 = end_str
                .parse_u64()
                .map_err(|_| HeaderParseError::HttpRangeError)?;

            if length == 0 {
                return Ok(None);
            }

            if length > size {
                length = size;
            }

            Ok(Some(HttpRange {
                start: (size - length),
                length,
            }))
        } else {
            let start: u64 = start_str
                .parse_u64()
                .map_err(|_| HeaderParseError::HttpRangeError)?;

            if start >= size {
                return Ok(None);
            }

            let length = if end_str.is_empty() {
                // If no end is specified, range extends to end of the file.
                size - start
            } else {
                let mut end: u64 = end_str
                    .parse_u64()
                    .map_err(|_| HeaderParseError::HttpRangeError)?;

                if start > end {
                    return Err(HeaderParseError::HttpRangeError);
                }

                if end >= size {
                    end = size - 1;
                }

                end - start + 1
            };

            Ok(Some(HttpRange { start, length }))
        }
    }
}

trait SliceExt {
    fn trim(&self) -> &Self;
    fn parse_u64(&self) -> Result<u64, ()>;
}

impl SliceExt for [u8] {
    fn trim(&self) -> &[u8] {
        fn is_whitespace(c: &u8) -> bool {
            *c == b'\t' || *c == b' '
        }

        fn is_not_whitespace(c: &u8) -> bool {
            !is_whitespace(c)
        }

        if let Some(first) = self.iter().position(is_not_whitespace) {
            if let Some(last) = self.iter().rposition(is_not_whitespace) {
                &self[first..last + 1]
            } else {
                unreachable!();
            }
        } else {
            &[]
        }
    }

    fn parse_u64(&self) -> Result<u64, ()> {
        if self.is_empty() {
            return Err(());
        }
        let mut res = 0u64;
        for b in self {
            if *b >= 0x30 && *b <= 0x39 {
                res = res * 10 + (b - 0x30) as u64;
            } else {
                return Err(());
            }
        }

        Ok(res)
    }
}
