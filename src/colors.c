//
// Created by waff on 1/8/20.
//

#include "colors.h"
#include "config.h"

char* get_css () {
    open_config();

    char* css_file = str_concat(get_user_home(), "/.config/waffy/main.css");

    FILE* fp = fopen(css_file, "r");
    if (fp == NULL) {
        fp = fopen(css_file, "w");
        fprintf(fp, "%s", read_file("../assets/main.css"));
    }

    fclose(fp);

    if (config_wal == 1) {
        char* wal_file = str_concat(get_user_home(), "/.cache/wal/colors-waybar.css");
        return str_concat(read_file(wal_file), read_file(css_file));
    }

    return read_file(css_file);
}