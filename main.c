#include <stdio.h>
#include <assert.h>
#include <unistd.h>
#include "example.h"

int main(void)
{
	struct Handle *hub = smart_home_new();

	assert(hub != NULL);

	Home *home = smart_home_add_home(hub, "Little Home");
	assert(home != NULL);
	Room *room = smart_home_add_room(home, "Kitchen");
	assert(room != NULL);
	SmartThermometer *thermometer = smart_home_add_thermometer(hub, room, "thermo", "true thermo", "0.0.0.0:10000");
	assert(thermometer != NULL);

	for (size_t i = 0; i < 10; i++) {
		sleep(5);
		printf("Current temperature: %f\n", smart_home_get_thermometer_temperature(hub, thermometer));
	}

	smart_home_free(hub);

	return 0;
}

