Introduction
===========

This is the C FFI of the `kdri` library, that lets you control Kettler sport
devices via bluetooth. See the
[kdri repository](https://github.com/ChangSpivey/kdri) for more
information.

How to build
----------

```bash
git clone https://github.com/ChangSpivey/kdri-c-wrapper.git
cd kdri-c-wrapper
cargo build --release
```

You will now find the header file `./target/include/kdri.h` and the library
files `.target/release/libkdri.so` and `.target/release/libkdri.a`.

Example
----------

```c
#include <kdri.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char const *argv[]) {
  KdriHandle* handle = kdri_create_handle();
  KdriDevice devices[256] = { 0 };

  // scan for devices in range
  int32_t result = kdri_scan_devices(handle, devices, 256);
  if(result < 0) { printf("Error code %d\n", result); exit(1); }
  else if(result == 0) { printf("No devices in range", result); exit(1); }

  // connect to device
  KdriDevice device = devices[0];
  printf("Connecting to %s\n", device.name);
  KdriConnection* connection = kdri_connect(handle, &device);
  printf("Connected...\n");


  size_t maxLineData = 256;
  char *lineString = malloc(maxLineData + 1);
  while(1) {
    // shell-like io
    fflush(stdout);
    printf("> ");
    fflush(stdout);
    memset(lineString, 0, maxLineData + 1);
    size_t bytesRead = getline(&lineString, &maxLineData, stdin);
    lineString[bytesRead - 1] = '\0'; // newline to null termination

    // interpret line
    int data = 0;
    if(strcmp(lineString, "exit") == 0) break;
    else if(sscanf(lineString, "incline=%d", &data) > 0) {
      kdri_set_incline(connection, data);
    } else if(sscanf(lineString, "speed=%d", &data) > 0) {
      kdri_set_speed(connection, data);
    } else if(strcmp(lineString, "on") == 0) {
      kdri_set_online(connection, 1);
    } else if(strcmp(lineString, "off") == 0) {
      kdri_set_online(connection, 0);
    } // etc. ...

    else {
      printf("Invalid command\n");
    }
  }
  free(lineString);

  // free all memory
  kdri_connection_close(connection);
  kdri_destroy_handle(handle);
  return 0;
}
```
