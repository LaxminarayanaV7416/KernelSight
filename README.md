# perf_insight_lite
Light weight Server Performance observability tool for Linux kernel, its light weight and energy efficient


## Scratch board
==================
* Think of the configuring `heatbeat` in the config file such that we can rerun the scanning as per the required frequency
* Think of configuring the root user previlages so that we can let the daemon run as root
* 

## Goals in mind
=================
* this will served as library so robustness is super important
* No async processes, we should only stick with Threads (Not sure for now but this is what we will do)
* Implement the Scraping/ String parsing logic by studyign exactly how the bytes to strings are converted taditionally, focus on what components we need for the parsing. Eventualy we will end up building a `struct` way abstraction from of all procfs files
* For String parsing we will relay mostly on `XOR` and `OR` bitwise operations.
* Most important decisions we have taken to keep this daemon so light weight are
  * We will open any procfs file once either during when we recieve an event, or during scanning. and keep the file open and use the seek to read bytes on the buffer (this buffers are preallocated during the daemon startup so new new memory creations or destroyed)
  * We will maintain the chunks of buffer pools (for each scanned buffer to fit), we start scrapping once the buffer is full, and flush it accordingly to the traget IO file, again these target IO files will be buffers not the string files with any extension to keep the IO work minimal.
  * `NOTE`: These IO buffer files can take help of the Circular Queues to flush the data to the target IO files.


we are recieveing the EMFILE error from OS that is Os Error 24:
attempt to open readers
→ hit EMFILE
→ create incomplete record
→ parsing returns false
→ remove record and close successful descriptors
→ retry next iteration
→ hit EMFILE again
```
ulimit -n 8192
cargo run
```
