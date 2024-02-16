fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
    // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
    let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST);
  
    // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
    let timeout = sp_io::offchain::timestamp()
        .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));
  
    // For github API request, we also need to specify `user-agent` in http request header.
    //   See: https://developer.github.com/v3/#user-agent-required
    let pending = request
        .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
        .deadline(timeout) // Setting the timeout time
        .send() // Sending the request out by the host
        .map_err(|_| <Error<T>>::HttpFetchingError)?;
  
    // By default, the http request is async from the runtime perspective. So we are asking the
    //   runtime to wait here.
    // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
    //   ref: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
    let response = pending
        .try_wait(timeout)
        .map_err(|_| <Error<T>>::HttpFetchingError)?
        .map_err(|_| <Error<T>>::HttpFetchingError)?;
  
    if response.code != 200 {
        debug::error!("Unexpected http request status code: {}", response.code);
        return Err(<Error<T>>::HttpFetchingError);
    }
  
    // Next we fully read the response body and collect it to a vector of bytes.
    Ok(response.body().collect::<Vec<u8>>())
  }