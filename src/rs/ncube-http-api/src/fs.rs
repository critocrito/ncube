use bytes::{Bytes, BytesMut};
use futures::{
    future::{self, Either},
    ready, stream, FutureExt, Stream, StreamExt,
};
use std::{cmp, fs::Metadata, pin::Pin, task::Poll};
use tokio::{fs::File as TkFile, io::AsyncSeekExt};
use tokio_util::io::poll_read_buf;
use warp::{
    reject::{self, Rejection},
    reply::{Reply, Response},
};

use crate::{headers::HeaderParseError, http::Conditionals};

const DEFAULT_READ_BUF_SIZE: usize = 8_192;

pub(crate) fn bytes_range(
    range: Conditionals,
    max_len: u64,
) -> Result<(u64, u64), HeaderParseError> {
    use std::ops::{Bound, RangeBounds};

    let range = if let Some(range) = range.parse_range(max_len) {
        range.start..(range.start + range.length)
    } else {
        return Ok((0, max_len));
    };

    let start = range.start_bound();
    let end = range.end_bound();

    let start = match start {
        Bound::Unbounded => 0,
        Bound::Included(s) => *s,
        Bound::Excluded(s) => s + 1,
    };

    let end = match end {
        Bound::Unbounded => max_len,
        Bound::Included(s) => {
            // For the special case where s == the file size
            if *s == max_len {
                *s
            } else {
                *s + 1
            }
        }
        Bound::Excluded(s) => *s,
    };

    if start < end && end <= max_len {
        Ok((start, end))
    } else {
        Ok((0, max_len))
    }
}

pub(crate) fn file_stream(
    mut file: TkFile,
    buf_size: usize,
    (start, end): (u64, u64),
) -> impl Stream<Item = Result<Bytes, std::io::Error>> + Send {
    use std::io::SeekFrom;

    let seek = async move {
        if start != 0 {
            file.seek(SeekFrom::Start(start)).await?;
        }
        Ok(file)
    };

    seek.into_stream()
        .map(move |result| {
            let mut buf = BytesMut::new();
            let mut len = end - start;
            let mut f = match result {
                Ok(f) => f,
                Err(f) => return Either::Left(stream::once(future::err(f))),
            };

            Either::Right(stream::poll_fn(move |cx| {
                if len == 0 {
                    return Poll::Ready(None);
                }
                reserve_at_least(&mut buf, buf_size);

                let n = match ready!(poll_read_buf(Pin::new(&mut f), cx, &mut buf)) {
                    Ok(n) => n as u64,
                    Err(err) => {
                        return Poll::Ready(Some(Err(err)));
                    }
                };

                if n == 0 {
                    return Poll::Ready(None);
                }

                let mut chunk = buf.split().freeze();
                if n > len {
                    chunk = chunk.split_to(len as usize);
                    len = 0;
                } else {
                    len -= n;
                }

                Poll::Ready(Some(Ok(chunk)))
            }))
        })
        .flatten()
}

pub(crate) async fn file_metadata(f: TkFile) -> Result<(TkFile, Metadata), Rejection> {
    match f.metadata().await {
        Ok(meta) => Ok((f, meta)),
        Err(_) => Err(reject::not_found()),
    }
}

fn reserve_at_least(buf: &mut BytesMut, cap: usize) {
    if buf.capacity() - buf.len() < cap {
        buf.reserve(cap);
    }
}

#[derive(Debug)]
pub(crate) struct File {
    pub(crate) resp: Response,
}

impl Reply for File {
    fn into_response(self) -> Response {
        self.resp
    }
}

pub(crate) fn optimal_buf_size(metadata: &Metadata) -> usize {
    let block_size = get_block_size(metadata);

    // If file length is smaller than block size, don't waste space
    // reserving a bigger-than-needed buffer.
    cmp::min(block_size as u64, metadata.len()) as usize
}

#[cfg(unix)]
fn get_block_size(metadata: &Metadata) -> usize {
    use std::os::unix::fs::MetadataExt;
    //TODO: blksize() returns u64, should handle bad cast...
    //(really, a block size bigger than 4gb?)

    // Use device blocksize unless it's really small.
    cmp::max(metadata.blksize() as usize, DEFAULT_READ_BUF_SIZE)
}

#[cfg(not(unix))]
fn get_block_size(_metadata: &Metadata) -> usize {
    DEFAULT_READ_BUF_SIZE
}
