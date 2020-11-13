//
// Created by waff on 1/7/20.
//

#ifndef WAFFY_CONFIG_H
#define WAFFY_CONFIG_H

#define MAX_CONFIG_LINE_LENGTH 256

#include <stdio.h>
#include "utils.h"

extern long config_columns;
extern long config_wal;
extern char* config_prompt;

// Write current config to the file
void write_config();

// Read config from the file
void open_config ();

#endif //WAFFY_CONFIG_H
