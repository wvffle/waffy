//
// Created by waff on 1/7/20.
//

#include "config.h"

// Default config
long config_columns = 4;
long config_wal = 0;
char* config_prompt = "search:";

void get_long (char* value, long* res);
void get_str(char *value, char** res);
void get_bool(char* value, long* res);

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
                get_long(value, &config_columns);
            } else if (strcmp(key, "prompt") == 0) {
                get_str(value, &config_prompt);
            } else if (strcmp(key, "wal_enable") == 0) {
                get_bool(value, &config_wal);
            } else {
                fprintf(stderr, "Unknown config option '%s' on line %ld", key, line_number);
            }
        }
    }

    fclose(fp);
}

void get_str(char *value, char** res) {
    if (value[0] != '"') {
        char* buf = malloc(sizeof(char) * strlen(value));
        strcpy(buf, value);
        *res = buf;
        return;
    }

    if (str_ends_with(value, "\"")) {
        char* buf = malloc(sizeof(char) * (strlen(value) - 2));
        value[strlen(value) - 1] = '\0';
        strcpy(buf, value + 1);
        *res = buf;
        return;
    }

    char* buf = strtok(NULL, " ");
    get_str(str_concat(value, str_concat(" ", buf)), res);
}

void get_bool(char *value, long *res) {
    if (strcmp(value, "1") == 0 || strcmp(value, "yes") == 0 || strcmp(value, "on") == 0 || strcmp(value, "true") == 0) {
        *res = 1;
        return;
    }

    *res = 0;
}

void get_long(char *value, long *res) {
    *res = strtol(value, NULL, 10);
}

void write_config() {
    char* dir = str_concat(get_user_home(), "/.config/waffy");
    char* file = str_concat(dir, "/config");

    mkdir(dir, 0755);

    FILE* fp = fopen(file, "w");

    fprintf(fp, "columns %ld\n", config_columns);
    fprintf(fp, "columns true\n");
    fprintf(fp, "prompt \"%s\"\n", config_prompt);

    fclose(fp);
}

