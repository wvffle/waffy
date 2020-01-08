//
// Created by waff on 1/7/20.
//

#include "config.h"

// Default config
long config_columns = 4;
char* config_prompt = "search:";

char *get_str(char *value);

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
            } else if (strcmp(key, "prompt") == 0) {
                config_prompt = get_str(value);
            } else {
                fprintf(stderr, "Unknown config option '%s' on line %ld", key, line_number);
            }
        }
    }

    fclose(fp);
}

char *get_str(char *value) {
    if (value[0] != '"') return value;

    if (str_ends_with(value, "\"")) {
        char* res = malloc(sizeof(char) * (strlen(value) - 1));
        strcpy(res, value + 1);
        res[strlen(res) - 1] = '\0';
        return res;
    }

    char* buf = strtok(NULL, " ");
    char* res = str_concat(value, str_concat(" ", buf));

    return get_str(res);
}

void write_config() {
    char* dir = str_concat(get_user_home(), "/.config/waffy");
    char* file = str_concat(dir, "/config");

    mkdir(dir, 0755);

    FILE* fp = fopen(file, "w");

    fprintf(fp, "columns %ld\n", config_columns);
    fprintf(fp, "prompt \"%s\"\n", config_prompt);

    fclose(fp);
}

