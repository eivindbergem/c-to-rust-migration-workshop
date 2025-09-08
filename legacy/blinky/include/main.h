#ifndef __MAIN_H
#define __MAIN_H

#include "FreeRTOS.h"
#include "stm32f1xx.h"
#include "stm32f1xx_hal_conf.h"
#define led_Pin GPIO_PIN_13
#define led_GPIO_Port GPIOC_BASE
void SystemClock_Config(void);
void Error_Handler(void);
void vBlinkTask(void *pvParameters);
void vPrintTask(void *pvParameters);
void MX_TIM1_Init(void);
void MX_GPIO_Init(void);
#endif
