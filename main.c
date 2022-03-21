#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include "example.h"

int main(void)
{
	Handle *hub = smart_home_new();
	Home *home = smart_home_add_home(hub, "Little Home");
	Room *room = smart_home_add_room(home, "Kitchen");
	SmartThermometer *thermometer_1 = smart_home_add_thermometer(hub, room, "thermo1", "true thermo", "0.0.0.0:10000");
	SmartThermometer *thermometer_2 = smart_home_add_thermometer(hub, room, "thermo2", "true thermo", "0.0.0.0:10000");

	assert(thermometer_1 != NULL);
	assert(thermometer_2 != NULL);

	for (size_t i = 0; i < 10; i++) {
		sleep(5);
		printf("Current temperature: %f\n", smart_home_get_thermometer_temperature(hub, thermometer_1));
	}

	HandleDeviceIter iter = smart_home_get_thermometer_iter(hub, room);

	for (const SmartThermometer *thermo = smart_home_get_thermometer_next(&iter); thermo != NULL; thermo = smart_home_get_thermometer_next(&iter)) {
		printf("thermo: %s\n", smart_home_get_thermometer_name(hub, thermo));
	}

	smart_home_free(hub);

	return 0;
}
