#include "main.h"
#include "task.h"
#include <stdio.h>
#include "SEGGER_RTT.h"

int _write(int file, char *ptr, int len) {
    (void)file;  // stdout only
    SEGGER_RTT_Write(0, ptr, len);
    return len;
}

void vPrintTask(void *pvParameters) {
  for (;;) {
    printf("Hello world\n");
    vTaskDelay(pdMS_TO_TICKS(2000));
  }
}
