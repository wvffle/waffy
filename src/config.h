//
// Created by waff on 1/7/20.
//

#ifndef WAFFY_CONFIG_H
#define WAFFY_CONFIG_H

#define MAX_CONFIG_LINE_LENGTH 256

#include <stdio.h>
#include <sys/stat.h>
#include "utils.h"

long config_columns;
char* config_prompt;

// Write current config to the file
void write_config();

// Read config from the file
void open_config ();

#endif //WAFFY_CONFIG_H
