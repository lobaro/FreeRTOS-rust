/*
FreeRTOS.rs shim library
Include headers relevant for your platform.
STM32 example:
#include "stm32f4xx_hal.h"
*/

#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"
#include "semphr.h"

// Just for testing
void freertos_rs_invoke_configASSERT() {
	configASSERT(0);
}

void freertos_rs_vTaskStartScheduler() {
	vTaskStartScheduler();
}

BaseType_t freertos_rt_xTaskGetSchedulerState(void) {
	return xTaskGetSchedulerState();
}

void *freertos_rs_pvPortMalloc(size_t xWantedSize) {
	return pvPortMalloc(xWantedSize);
}

void freertos_rs_vPortFree(void *pv) {
	vPortFree(pv);
}

uint8_t freertos_rs_sizeof(uint8_t _type) {
	switch (_type) {
		case 0:
			return sizeof(void*);
			break;
		case 1:
			return sizeof(char*);
			break;
		case 2:
			return sizeof(char);
			break;

		case 10:
			return sizeof(BaseType_t);
			break;
		case 11:
			return sizeof(UBaseType_t);
			break;
		case 12:
			return sizeof(TickType_t);
			break;

		case 20:
			return sizeof(TaskHandle_t);
			break;
		case 21:
			return sizeof(QueueHandle_t);
			break;
		case 22:
			return sizeof(SemaphoreHandle_t);
			break;
		case 23:
			return sizeof(TaskFunction_t);
			break;
		case 24:
			return sizeof(TimerHandle_t);
			break;
		case 25:
			return sizeof(TimerCallbackFunction_t);
			break;

		case 30:
			return sizeof(TaskStatus_t);
			break;
		case 31:
			return sizeof(eTaskState);
			break;
		case 32:
			return sizeof(unsigned long);
			break;
		case 33:
			return sizeof(unsigned short);
			break;


			break;
		default:
			return 0;
	}
}

#if (INCLUDE_vTaskDelayUntil == 1)
void freertos_rs_vTaskDelayUntil(TickType_t *pxPreviousWakeTime, TickType_t xTimeIncrement) {
	vTaskDelayUntil(pxPreviousWakeTime, xTimeIncrement);
}
#endif

#if (INCLUDE_vTaskDelay == 1)
void freertos_rs_vTaskDelay(TickType_t xTicksToDelay) {
	vTaskDelay(xTicksToDelay);
}
#endif

TickType_t freertos_rs_xTaskGetTickCount() {
	return xTaskGetTickCount();
}

#if (configUSE_TRACE_FACILITY == 1)
UBaseType_t freertos_rs_get_system_state(TaskStatus_t * const pxTaskStatusArray, const UBaseType_t uxArraySize, uint32_t * const pulTotalRunTime) {
	return uxTaskGetSystemState(pxTaskStatusArray, uxArraySize, pulTotalRunTime);
}
#endif

#ifdef configCPU_CLOCK_HZ
unsigned long freertos_rs_get_configCPU_CLOCK_HZ() {
  return configCPU_CLOCK_HZ;
}
#endif

TickType_t freertos_rs_get_portTICK_PERIOD_MS() {
	return portTICK_PERIOD_MS;
}

UBaseType_t freertos_rs_get_number_of_tasks() {
	return uxTaskGetNumberOfTasks();
}

#if (configUSE_RECURSIVE_MUTEXES == 1)
SemaphoreHandle_t freertos_rs_create_recursive_mutex() {
	return xSemaphoreCreateRecursiveMutex();
}

UBaseType_t freertos_rs_take_recursive_semaphore(SemaphoreHandle_t semaphore, UBaseType_t max) {
	if (xSemaphoreTakeRecursive(semaphore, max) == pdTRUE) {
		return 0;
	}

	return 1;
}
UBaseType_t freertos_rs_give_recursive_semaphore(SemaphoreHandle_t semaphore) {
	if (xSemaphoreGiveRecursive(semaphore) == pdTRUE) {
		return 0;
	} else {
		return 1;
	}
}
#endif

SemaphoreHandle_t freertos_rs_create_mutex() {
	return xSemaphoreCreateMutex();
}

SemaphoreHandle_t freertos_rs_create_binary_semaphore() {
	return xSemaphoreCreateBinary();
}

SemaphoreHandle_t freertos_rs_create_counting_semaphore(UBaseType_t max, UBaseType_t initial) {
	return xSemaphoreCreateCounting(max, initial);
}

void freertos_rs_delete_semaphore(SemaphoreHandle_t semaphore) {
	vSemaphoreDelete(semaphore);
}

UBaseType_t freertos_rs_take_semaphore(SemaphoreHandle_t semaphore, UBaseType_t max) {
	if (xSemaphoreTake(semaphore, max) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_give_semaphore(SemaphoreHandle_t semaphore) {
	if (xSemaphoreGive(semaphore) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_take_semaphore_isr(SemaphoreHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken) {
	if (xSemaphoreTakeFromISR(semaphore, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}

	return 1;
}

UBaseType_t freertos_rs_give_semaphore_isr(SemaphoreHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken) {
	if (xSemaphoreGiveFromISR(semaphore, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}

	return 1;
}


UBaseType_t freertos_rs_spawn_task(TaskFunction_t entry_point, void* pvParameters, const char * const name, uint8_t name_len, uint16_t stack_size, UBaseType_t priority, TaskHandle_t* task_handle) {
	char c_name[configMAX_TASK_NAME_LEN] = {0};
	for (int i = 0; i < name_len; i++) {
		c_name[i] = name[i];

		if (i == configMAX_TASK_NAME_LEN - 1) {
			break;
		}
	}

	BaseType_t ret = xTaskCreate(entry_point, c_name, stack_size, pvParameters, priority, task_handle);

	if (ret != pdPASS) {
		return 1;
	}

	configASSERT(task_handle);

	return 0;
}

#if (INCLUDE_vTaskDelete == 1)
void freertos_rs_delete_task(TaskHandle_t task) {
	vTaskDelete(task);
}
#endif

void freertos_rs_suspend_task(TaskHandle_t task) {
	vTaskSuspend(task);
}

UBaseType_t freertos_rs_get_stack_high_water_mark(TaskHandle_t task) {
#if (INCLUDE_uxTaskGetStackHighWaterMark == 1)
	return uxTaskGetStackHighWaterMark(task);
#else
	(void)task;
	return 0;
#endif
}


QueueHandle_t freertos_rs_queue_create(UBaseType_t queue_length, UBaseType_t item_size) {
	return xQueueCreate(queue_length, item_size);
}

void freertos_rs_queue_delete(QueueHandle_t queue) {
	vQueueDelete(queue);
}

UBaseType_t freertos_rs_queue_send(QueueHandle_t queue, void* item, TickType_t max_wait) {
	if (xQueueSend(queue, item, max_wait ) != pdTRUE)
	{
		return 1;
	}

	return 0;
}

UBaseType_t freertos_rs_queue_send_isr(QueueHandle_t queue, void* item, BaseType_t* xHigherPriorityTaskWoken) {
	if (xQueueSendFromISR(queue, item, xHigherPriorityTaskWoken) == pdTRUE) {
		return 0;
	}
	return 1;
}

UBaseType_t freertos_rs_queue_receive(QueueHandle_t queue, void* item, TickType_t max_wait) {
	if ( xQueueReceive( queue, item, max_wait ) != pdTRUE )
	{
		return 1;
	}

	return 0;
}

UBaseType_t freertos_rs_queue_messages_waiting(QueueHandle_t queue) {
	return uxQueueMessagesWaiting( queue );
}

void freertos_rs_isr_yield(BaseType_t xHigherPriorityTaskWoken) {
	portYIELD_FROM_ISR(xHigherPriorityTaskWoken);
}

TickType_t freertos_rs_max_wait() {
	return portMAX_DELAY;
}


char* freertos_rs_task_get_name(TaskHandle_t task) {
	return pcTaskGetName(task);
}

uint32_t freertos_rs_task_notify_take(uint8_t clear_count, TickType_t wait) {
	return ulTaskNotifyTake(clear_count == 1 ? pdTRUE : pdFALSE, wait);
}

BaseType_t freertos_rs_task_notify_wait(uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait) {
	if (xTaskNotifyWait(ulBitsToClearOnEntry, ulBitsToClearOnExit, pulNotificationValue, xTicksToWait) == pdTRUE) {
		return 0;
	}

	return 1;
}

eNotifyAction freertos_rs_task_notify_action(uint8_t action) {
	switch (action) {
		case 1:
			return eSetBits;
		case 2:
			return eIncrement;
		case 3:
			return eSetValueWithOverwrite;
		case 4:
			return eSetValueWithoutOverwrite;
		default:
			return eNoAction;
	}
}

BaseType_t freertos_rs_task_notify(void* task, uint32_t value, uint8_t action) {
	eNotifyAction eAction = freertos_rs_task_notify_action(action);

	BaseType_t v = xTaskNotify(task, value, eAction);
	if (v != pdPASS) {
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_task_notify_isr(void* task, uint32_t value, uint8_t action, BaseType_t* xHigherPriorityTaskWoken) {
	eNotifyAction eAction = freertos_rs_task_notify_action(action);

	BaseType_t v = xTaskNotifyFromISR(task, value, eAction, xHigherPriorityTaskWoken);
	if (v != pdPASS) {
		return 1;
	}
	return 0;
}

#if ( ( INCLUDE_xTaskGetCurrentTaskHandle == 1 ) || ( configUSE_MUTEXES == 1 ) )
TaskHandle_t freertos_rs_get_current_task() {
	return xTaskGetCurrentTaskHandle();
}
#endif

void freertos_rs_vTaskSuspendAll() {
  vTaskSuspendAll();
}

BaseType_t freertos_rs_xTaskResumeAll() {
  return xTaskResumeAll();
}

#if (configUSE_TRACE_FACILITY == 1)
BaseType_t freertos_rs_uxTaskGetTaskNumber(TaskHandle_t task) {
    return uxTaskGetTaskNumber(task);
}

void freertos_rs_vTaskSetTaskNumber(TaskHandle_t task, const UBaseType_t value) {
    return vTaskSetTaskNumber(task, value);
}
#endif // configUSE_TRACE_FACILITY

#if (configUSE_TIMERS == 1)

TimerHandle_t freertos_rs_timer_create(const char * const name, uint8_t name_len, const TickType_t period,
		uint8_t auto_reload, void * const timer_id, TimerCallbackFunction_t callback)
{
	char c_name[configMAX_TASK_NAME_LEN] = {0};
	for (int i = 0; i < name_len; i++) {
		c_name[i] = name[i];

		if (i == configMAX_TASK_NAME_LEN - 1) {
			break;
		}
	}

	UBaseType_t timer_auto_reload = pdFALSE;
	if (auto_reload == 1) {
		timer_auto_reload = pdTRUE;
	}

	TimerHandle_t handle = xTimerCreate(c_name, period, timer_auto_reload, timer_id, callback);
	return handle;
}

BaseType_t freertos_rs_timer_start(TimerHandle_t timer, TickType_t block_time) {
	if (xTimerStart(timer, block_time) != pdPASS) {
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_timer_start_from_isr(TimerHandle_t timer, BaseType_t* xHigherPriorityTaskWoken) {
	if (xTimerStartFromISR(timer, xHigherPriorityTaskWoken) != pdPASS) {
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_timer_stop(TimerHandle_t timer, TickType_t block_time) {
	if (xTimerStop(timer, block_time) != pdPASS) {
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_timer_delete(TimerHandle_t timer, TickType_t block_time) {
	if (xTimerDelete(timer, block_time) != pdPASS) {
		return 1;
	}
	return 0;
}

BaseType_t freertos_rs_timer_change_period(TimerHandle_t timer, TickType_t block_time, TickType_t new_period) {
	if (xTimerChangePeriod(timer, new_period, block_time) != pdPASS) {
		return 1;
	}
	return 0;
}

void* freertos_rs_timer_get_id(TimerHandle_t timer) {
	return pvTimerGetTimerID(timer);
}

#endif

void freertos_rs_enter_critical() {
	taskENTER_CRITICAL();
}

void freertos_rs_exit_critical() {
	taskEXIT_CRITICAL();
}
