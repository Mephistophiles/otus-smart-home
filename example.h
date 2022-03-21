#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ReturnCode {
	Success = 0,
	Fail = -1,
} ReturnCode;

typedef struct Option_CString Option_CString;
typedef struct Handle Handle;
typedef struct Home Home;
typedef struct Room Room;
typedef struct Device Device;
typedef struct SmartThermometer SmartThermometer;
typedef struct SmartSocket SmartSocket;

typedef struct HandleIter {
	struct Handle *handle;
	uintptr_t cursor;
} HandleIter;

typedef struct HandleRoomIter {
	struct Handle *handle;
	Home *home;
	uintptr_t cursor;
} HandleRoomIter;

typedef struct HandleDeviceIter {
	struct Handle *handle;
	Room *room;
	uintptr_t cursor;
} HandleDeviceIter;

/**
 * get new smart hub
 */
struct Handle *smart_home_new(void);

/**
 * free handle
 * # Safety
 *
 * It's ok, because memory allocated from smart_home_new()
 */
void smart_home_free(struct Handle *hub);

/**
 * add a new home to SmartHub
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * * `handle`: smart hub handle
 * * `name`: home name to add
 */
Home *smart_home_add_home(struct Handle *handle, const char *name);

/**
 * remove home from SmartHub
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * * `handle`: smart hub handle
 * * `name`: home name to del
 */
enum ReturnCode smart_home_del_home(struct Handle *handle, const char *name);

/**
 * Get Home handle from SmartHub by name
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * * `handle`: smart hub handle
 * * `name`: home name to get
 */
Home *smart_home_get_home(struct Handle *handle, const char *name);

/**
 * Get home count
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 */
uintptr_t smart_home_get_home_size(struct Handle *handle);

/**
 * Get iterator over homes
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * * `handle`: smart hub handle
 */
struct HandleIter smart_home_get_home_iter(struct Handle *handle);

/**
 * Get list of all home names in hub
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * # Warning
 *
 * This function returns borrowed pointer, use copy name before next call
 *
 * * `handle`: smart hub handle
 */
const char *smart_home_get_home_next(struct HandleIter *iter);

/**
 * Adds a new room to home
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 * * `name`: room name to add
 */
Room *smart_home_add_room(Home *handle, const char *name);

/**
 * Dels a room from home
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 * * `name`: room name to del
 */
enum ReturnCode smart_home_del_room(Home *handle, const char *name);

/**
 * Gets a room from home by name
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 * * `name`: room name to get
 */
Room *smart_home_get_room(Home *handle, const char *name);

/**
 * Gets a room count in room
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 */
uintptr_t smart_home_get_room_size(Home *handle);

/**
 * Gets a room iterator
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 */
struct HandleRoomIter smart_home_get_room_iter(struct Handle *handle, Home *home);

/**
 * Gets a room list from home
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 */
const char *smart_home_get_room_next(struct HandleRoomIter *iter);

/**
 * Adds a new thermometer to room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 * * `name`: thermometer name to add
 * * `description`: thermometer description to add
 * * `server`: thermometer server to add
 */
SmartThermometer *smart_home_add_thermometer(struct Handle *handle,
                                             Room *room,
                                             const char *name,
                                             const char *description,
                                             const char *server);

/**
 * Adds a new socket to room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 * * `name`: socket name to add
 * * `description`: socket description to add
 * * `server`: socket server to add
 */
SmartSocket *smart_home_add_socket(struct Handle *handle,
                                   Room *room,
                                   const char *name,
                                   const char *description,
                                   const char *server);

/**
 * Removes a device from room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 * * `name`: smart device name to del
 */
enum ReturnCode smart_home_del_device(Room *handle, const char *name);

/**
 * Get a device by name
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 * * `name`: smart device name to del
 */
Device *smart_home_get_device(Room *handle, const char *name);

/**
 * Gets all thermometers size
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
uintptr_t smart_home_get_thermometer_size(const Room *handle);

/**
 * Gets all thermometers iter
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
struct HandleDeviceIter smart_home_get_thermometer_iter(struct Handle *handle, Room *room);

/**
 * Gets all thermometers in room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const SmartThermometer *smart_home_get_thermometer_next(struct HandleDeviceIter *handle);

/**
 * Gets all sockets size
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
uintptr_t smart_home_get_socket_size(const Room *handle);

/**
 * Gets all sockets iter
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
struct HandleDeviceIter smart_home_get_socket_iter(struct Handle *handle, Room *room);

/**
 * Gets all sockets in room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const SmartSocket *smart_home_get_socket_next(struct HandleDeviceIter *handle);

/**
 * Get thermometer name
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_thermometer_name(struct Handle *handle, const SmartThermometer *device);

/**
 * Get thermometer description
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_thermometer_description(struct Handle *handle,
                                                   const SmartThermometer *device);

/**
 * Get thermometer temperature
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
double smart_home_get_thermometer_temperature(struct Handle *handle,
                                              const SmartThermometer *device);

/**
 * Get socket name
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_socket_name(struct Handle *handle, const SmartSocket *device);

/**
 * Get socket description
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_socket_description(struct Handle *handle, const SmartSocket *device);

/**
 * Get socket power
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
double smart_home_get_socket_power(struct Handle *handle, const SmartSocket *device);

/**
 * Turn on socket
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
enum ReturnCode smart_home_socket_on(struct Handle *handle, const SmartSocket *device);

/**
 * Turn off socket
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
enum ReturnCode smart_home_socket_off(struct Handle *handle, const SmartSocket *device);
