
#ifndef KDRI_H
#define KDRI_H


#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct {
	uint8_t b[6];
} KdriAddr;

typedef void KdriHandle;
typedef void KdriConnection;
typedef struct {
	uint8_t name[256];
	KdriAddr addr;
} KdriDevice;

typedef enum KdriReturn {
	KdriOk = 0,
	KdriFailed = 1,
	KdriNotInitialized = 2,
} KdriReturn;

typedef enum {
	KdriDtBike = 1,
	KdriDtCrosstrainer = 2,
	KdriDtRacer = 3,
	KdriDtRowing = 4,
	KdriDtTreadmill = 5,
} KdriDeviceType;

typedef enum {
	KdriDsUp = 0,
	KdriDsDown = 1,
} KdriDeviceState;

typedef enum {
	KdriBmConstantPower = 0,
	KdriBmConstantBrake = 1,
} KdriBrakeMode;

typedef enum {
	KdriPrBelow = 0,
	KdriPrIn = 1,
	KdriPrAbove = 2,
} KdriPowerRange;


extern KdriHandle* kdri_create_handle(void);
extern int32_t kdri_scan_devices(KdriHandle* handle, KdriDevice* dst_device_array, uint32_t max_array_length);
extern KdriConnection* kdri_connect(KdriHandle* handle, KdriAddr* device);
extern int32_t kdri_connection_close(KdriConnection* connection);
extern int32_t kdri_destroy_handle(KdriHandle* handle);

/*
	17 bytes will be written to "name" in the form of "XX:XX:XX:XX:XX:XX". The string will NOT be null
	terminated.
*/
extern int32_t kdri_addr_to_str(KdriAddr* addr, uint8_t* str);

extern KdriReturn kdri_set_speed(KdriConnection* connection, uint16_t v);
extern KdriReturn kdri_set_power(KdriConnection* connection, uint16_t v);
extern KdriReturn kdri_set_incline(KdriConnection* connection, uint16_t v);
extern KdriReturn kdri_set_brake_level(KdriConnection* connection, uint8_t v);
extern KdriReturn kdri_set_update_interval(KdriConnection* connection, uint32_t v);
extern KdriReturn kdri_set_brake_mode(KdriConnection* connection, KdriBrakeMode v);
extern KdriReturn kdri_set_online(KdriConnection* connection, uint8_t v);

extern KdriReturn kdri_get_power_target(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_power(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_power_min(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_power_max(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_speed_target(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_speed(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_speed_min(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_speed_max(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_incline_target(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_incline(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_incline_min(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_incline_max(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_brake_level(KdriConnection* connection, uint8_t* v);
extern KdriReturn kdri_get_brake_level_min(KdriConnection* connection, uint8_t* v);
extern KdriReturn kdri_get_brake_level_max(KdriConnection* connection, uint8_t* v);
extern KdriReturn kdri_get_online(KdriConnection* connection, uint8_t* v);
extern KdriReturn kdri_get_pulse(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_rpm(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_distance(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_energy(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_time(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_time_mode(KdriConnection* connection, uint16_t* v);
extern KdriReturn kdri_get_device_name(KdriConnection* connection, uint8_t* array, size_t max_size);
extern KdriReturn kdri_get_device_id(KdriConnection* connection, uint8_t* array, size_t max_size);
extern KdriReturn kdri_get_power_range(KdriConnection* connection, KdriPowerRange* v);
extern KdriReturn kdri_get_device_type(KdriConnection* connection, KdriDeviceType* v);
extern KdriReturn kdri_get_device_state(KdriConnection* connection, KdriDeviceState* v);
extern KdriReturn kdri_get_brake_mode(KdriConnection* connection, KdriBrakeMode* v);



#ifdef __cplusplus
}
#endif


#endif
