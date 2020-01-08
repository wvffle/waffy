//
// Created by waff on 1/7/20.
//

#include "config.h"

// Default config
long config_columns = 4;

void open_config() {
    char* file = str_concat(get_user_home(), "/.config/waffy/config");
    FILE* fp = fopen(file, "r");

    if (fp == NULL) {
        write_config();
        return;
    }

    size_t line_number = 0;
    while (!feof(fp)) {
        char line[MAX_CONFIG_LINE_LENGTH];
        fscanf(fp, "%[^\n]\n", line);
        line_number += 1;

        char* opline = str_trim(line);
        if (opline[0] == '#') {
            // pass
        } else {
            char* key = strtok(line, " ");
            char* value = strtok(NULL, " ");

            if (strcmp(key, "columns") == 0) {
                config_columns = strtol(value, NULL, 10);
            } else {
                fprintf(stderr, "Unknown config option '%s' on line %ld", key, line_number);
            }
        }
    }

    fclose(fp);
}

void write_config() {
    char* dir = str_concat(get_user_home(), "/.config/waffy");
    char* file = str_concat(dir, "/config");

    mkdir(dir, 0755);

    FILE* fp = fopen(file, "w");

    fprintf(fp, "columns %ld\n", config_columns);

    fclose(fp);
}

