/**
 ********************************************************************************************************
 * @file    log.h
 * @brief   Generic logging functions for the Ockam Library
 ********************************************************************************************************
 */

#ifndef OCKAM_LOG_H_
#define OCKAM_LOG_H_

/*
 ********************************************************************************************************
 * @defgroup    OCKAM_LOG OCKAM_LOG_API
 * @ingroup     OCKAM
 * @brief       OCKAM_LOG_API
 *
 * @addtogroup  OCKAM_LOG
 * @{
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                             INCLUDE FILES                                            *
 ********************************************************************************************************
 */

#include <stdint.h>

#include "ockam/error.h"

/*
 ********************************************************************************************************
 *                                                DEFINES                                               *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                               CONSTANTS                                              *
 ********************************************************************************************************
 */

typedef enum {
  OCKAM_LOG_DEBUG = 0,
  OCKAM_LOG_INFO,
  OCKAM_LOG_WARN,
  OCKAM_LOG_ERROR,
  OCKAM_LOG_FATAL,
  MAX_OCKAM_LOG
} OCKAM_LOG_e;

/*
 ********************************************************************************************************
 *                                               DATA TYPES                                             *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                          FUNCTION PROTOTYPES                                         *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                            GLOBAL VARIABLES                                          *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                           GLOBAL FUNCTIONS                                           *
 ********************************************************************************************************
 */

/*
 ********************************************************************************************************
 *                                            LOCAL FUNCTIONS                                           *
 ********************************************************************************************************
 */

#ifdef __cplusplus
extern "C" {
#endif

OckamError ockam_log_init(void);

OckamError ockam_log(void* p_str, uint32_t str_size);

#ifdef __cplusplus
}
#endif

/*
 ********************************************************************************************************
 * @}
 ********************************************************************************************************
 */

#endif