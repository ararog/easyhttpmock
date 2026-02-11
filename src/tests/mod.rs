use vetis::config::Protocol;

mod config;
mod server;

pub(crate) const fn default_protocol() -> Protocol {
    #[cfg(feature = "http1")]
    {
        Protocol::Http1
    }
    #[cfg(feature = "http2")]
    {
        Protocol::Http2
    }
    #[cfg(feature = "http3")]
    {
        Protocol::Http3
    }
}
