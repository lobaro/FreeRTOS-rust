
#include "FreeRTOS.h"
#include "task.h"
#include "timers.h"
#include "queue.h"
#include "semphr.h"

void freertos_rs_vTaskStartScheduler();

uint8_t freertos_rs_sizeof(uint8_t _type);

#if (INCLUDE_vTaskDelayUntil == 1)
void freertos_rs_vTaskDelayUntil(TickType_t *pxPreviousWakeTime, TickType_t xTimeIncrement);
#endif

#if (INCLUDE_vTaskDelay == 1)
void freertos_rs_vTaskDelay(TickType_t xTicksToDelay);
#endif

TickType_t freertos_rs_xTaskGetTickCount();

#if (configUSE_TRACE_FACILITY == 1)
UBaseType_t freertos_rs_get_system_state(TaskStatus_t * const pxTaskStatusArray, const UBaseType_t uxArraySize, uint32_t * const pulTotalRunTime);
#endif

TickType_t freertos_rs_get_portTICK_PERIOD_MS();
UBaseType_t freertos_rs_get_number_of_tasks();

#if (configUSE_RECURSIVE_MUTEXES == 1)
QueueHandle_t freertos_rs_create_recursive_mutex();
UBaseType_t freertos_rs_take_recursive_mutex(QueueHandle_t mutex, UBaseType_t max);
UBaseType_t freertos_rs_give_recursive_mutex(QueueHandle_t mutex);
#endif

QueueHandle_t freertos_rs_create_mutex();
QueueHandle_t freertos_rs_create_binary_semaphore();
QueueHandle_t freertos_rs_create_counting_semaphore(UBaseType_t max, UBaseType_t initial);
void freertos_rs_delete_semaphore(QueueHandle_t semaphore);
UBaseType_t freertos_rs_take_mutex(QueueHandle_t mutex, UBaseType_t max);
UBaseType_t freertos_rs_give_mutex(QueueHandle_t mutex);
UBaseType_t freertos_rs_take_semaphore_isr(QueueHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken);
UBaseType_t freertos_rs_give_semaphore_isr(QueueHandle_t semaphore, BaseType_t* xHigherPriorityTaskWoken);
UBaseType_t freertos_rs_spawn_task(TaskFunction_t entry_point, void* pvParameters, const char * const name, uint8_t name_len, uint16_t stack_size, UBaseType_t priority, TaskHandle_t* task_handle);

#if (INCLUDE_vTaskDelete == 1)
void freertos_rs_delete_task(TaskHandle_t task);
#endif

UBaseType_t freertos_rs_get_stack_high_water_mark(TaskHandle_t task);
QueueHandle_t freertos_rs_queue_create(UBaseType_t queue_length, UBaseType_t item_size);
void freertos_rs_queue_delete(QueueHandle_t queue);
UBaseType_t freertos_rs_queue_send(QueueHandle_t queue, void* item, TickType_t max_wait);
UBaseType_t freertos_rs_queue_send_isr(QueueHandle_t queue, void* item, BaseType_t* xHigherPriorityTaskWoken);
UBaseType_t freertos_rs_queue_receive(QueueHandle_t queue, void* item, TickType_t max_wait);
void freertos_rs_isr_yield();
TickType_t freertos_rs_max_wait();

#if (INCLUDE_pcTaskGetTaskName == 1)
char* freertos_rs_task_get_name(TaskHandle_t task);
#endif

uint32_t freertos_rs_task_notify_take(uint8_t clear_count, TickType_t wait);

BaseType_t freertos_rs_task_notify_wait(uint32_t ulBitsToClearOnEntry, uint32_t ulBitsToClearOnExit, uint32_t *pulNotificationValue, TickType_t xTicksToWait);

eNotifyAction freertos_rs_task_notify_action(uint8_t action);

BaseType_t freertos_rs_task_notify(void* task, uint32_t value, uint8_t action);

BaseType_t freertos_rs_task_notify_isr(void* task, uint32_t value, uint8_t action, BaseType_t* xHigherPriorityTaskWoken);

#if ( ( INCLUDE_xTaskGetCurrentTaskHandle == 1 ) || ( configUSE_MUTEXES == 1 ) )
TaskHandle_t freertos_rs_get_current_task();
#endif

#if (configUSE_TIMERS == 1)

TimerHandle_t freertos_rs_timer_create(const char * const name, uint8_t name_len, const TickType_t period,
		uint8_t auto_reload, void * const timer_id, TimerCallbackFunction_t callback);

BaseType_t freertos_rs_timer_start(TimerHandle_t timer, TickType_t block_time);
BaseType_t freertos_rs_timer_stop(TimerHandle_t timer, TickType_t block_time);
BaseType_t freertos_rs_timer_delete(TimerHandle_t timer, TickType_t block_time);
BaseType_t freertos_rs_timer_change_period(TimerHandle_t timer, TickType_t block_time, TickType_t new_period);
void* freertos_rs_timer_get_id(TimerHandle_t timer);

#endif

void freertos_rs_enter_critical();

void freertos_rs_exit_critical();
