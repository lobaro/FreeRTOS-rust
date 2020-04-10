
#include "FreeRTOS.h"

/* This demo uses heap_5.c, and these constants define the sizes of the regions
that make up the total heap.  heap_5 is only used for test and example purposes
as this demo could easily create one large heap region instead of multiple
smaller heap regions - in which case heap_4.c would be the more appropriate
choice.  See http://www.freertos.org/a00111.html for an explanation. */
#define mainREGION_1_SIZE	8201
#define mainREGION_2_SIZE	29905
#define mainREGION_3_SIZE	7607

void initialiseHeap(void) {
/* The Windows demo could create one large heap region, in which case it would
be appropriate to use heap_4.  However, purely for demonstration purposes,
heap_5 is used instead, so start by defining some heap regions.  No
initialisation is required when any other heap implementation is used.  See
http://www.freertos.org/a00111.html for more information.

The xHeapRegions structure requires the regions to be defined in start address
order, so this just creates one big array, then populates the structure with
offsets into the array - with gaps in between and messy alignment just for test
purposes. */
	static uint8_t ucHeap[configTOTAL_HEAP_SIZE];
	volatile uint32_t ulAdditionalOffset = 19; /* Just to prevent 'condition is always true' warnings in configASSERT(). */
	const HeapRegion_t xHeapRegions[] =
			{
					/* Start address with dummy offsets						Size */
					{ucHeap + 1,                                          mainREGION_1_SIZE},
					{ucHeap + 15 + mainREGION_1_SIZE,                     mainREGION_2_SIZE},
					{ucHeap + 19 + mainREGION_1_SIZE + mainREGION_2_SIZE, mainREGION_3_SIZE},
					{NULL,                                                0}
			};

	/* Sanity check that the sizes and offsets defined actually fit into the
	array. */
	configASSERT((ulAdditionalOffset + mainREGION_1_SIZE + mainREGION_2_SIZE + mainREGION_3_SIZE) < configTOTAL_HEAP_SIZE);

	/* Prevent compiler warnings when configASSERT() is not defined. */
	(void) ulAdditionalOffset;

	vPortDefineHeapRegions(xHeapRegions);
}