#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum ReturnCode {
  Success = 0,
  Fail = -1,
} ReturnCode;

struct Handle;
typedef struct Home Home;
typedef struct Room Room;
typedef struct Device Device;
typedef struct SmartThermometer SmartThermometer;
typedef struct SmartSocket SmartSocket;

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
 * Get list of all home names in hub
 *
 * # Safety
 *
 * handle gets from smart_home_new()
 *
 * * `handle`: smart hub handle
 */
char *const *smart_home_get_home_list(struct Handle *handle);

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
 * Gets a room list from home by name
 *
 * # Safety
 *
 * Home gets from smart_home_get_home()
 *
 * * `handle`: home handle
 */
char *const *smart_home_get_room_list(Home *handle);

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
 * Gets all thermometers in room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
SmartThermometer **smart_home_get_thermometer_list(Room *handle);

/**
 * Gets all sockets in room
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
SmartSocket **smart_home_get_socket_list(Room *handle);

/**
 * Get thermometer name
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_thermometer_name(SmartThermometer *handle);

/**
 * Get thermometer description
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_thermometer_description(SmartThermometer *handle);

/**
 * Get thermometer temperature
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
double smart_home_get_thermometer_temperature(struct Handle *handle, SmartThermometer *device);

/**
 * Get socket name
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_socket_name(SmartSocket *handle);

/**
 * Get socket description
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
const char *smart_home_get_socket_description(SmartSocket *handle);

/**
 * Get socket power
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
double smart_home_get_socket_power(struct Handle *handle, SmartSocket *device);

/**
 * Turn on socket
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
enum ReturnCode smart_home_socket_on(struct Handle *handle, SmartSocket *device);

/**
 * Turn off socket
 *
 * # Safety
 *
 * Room gets from smart_home_get_room()
 *
 * * `handle`: room handle
 */
enum ReturnCode smart_home_socket_off(struct Handle *handle, SmartSocket *device);
