//
// Created by waff on 12/20/19.
//

#include "find_desktop.h"

char *get_user_desktop_files_directory() {
    char* homedir = get_user_home();
    char* local_desktop_directory = "/.local/share/applications/";
    return str_concat(homedir, local_desktop_directory);
}

desktop_entry *read_desktop_file(const char *file_path, const char *gtk_launch_name) {
    FILE* fp = fopen(file_path, "r");
    if (!fp) return NULL;

    desktop_entry* entry = de_constructor(gtk_launch_name);

    char in_entry_section = 0;
    char line[PATH_MAX];
    while (!feof(fp)) {
        fscanf(fp, "%[^\n]\n", line);

        // Skip comments and empty lines
        if (line[0] == '\0' || line[0] == '#') continue;

        if (in_entry_section) {
            // Another section occured, stop parsing
            if (line[0] == '[') break;

            char* key = strtok(line, "=");
            char* value = strtok(NULL, "=");

            // Skip malformed field
            if (!value) continue;

            if (!strcmp(key, "Name")) {
                de_set_name(entry, value);
                continue;
            }

            if (!strcmp(key, "Icon")) {
                de_set_icon(entry, value);
                continue;
            }

        } else if (!strcmp(line, "[Desktop Entry]")) {
            in_entry_section = 1;
        }
    }

    fclose(fp);
    return entry;
}

desktop_entry_batch *find_desktop_files(const char *directory) {
    DIR* dir_handle = opendir(directory);
    desktop_entry_batch* batch = deb_constructor();

    // Skip non-existant directory
    if (dir_handle == NULL) {
        return NULL;
    }

    struct dirent* dp;
    while ((dp = readdir(dir_handle)) != NULL) {
        if (!str_ends_with(dp->d_name, ".desktop")) continue;

        char* file_path = str_concat(directory, dp->d_name);

        // Remove .desktop from end
        size_t len = strlen(dp->d_name) - 8 + 1;
        char* gtk_launch_name[len];
        memcpy(gtk_launch_name, dp->d_name, len);

        desktop_entry* entry = read_desktop_file(file_path, (const char *) gtk_launch_name);
        deb_push(batch, entry);

        free(file_path);
    }

    closedir(dir_handle);
    return batch;
}

desktop_entry_batch *find_all_desktop_files() {
    char* user_dir = get_user_desktop_files_directory();
    char* desktop_entry_dirs[DESKTOP_ENTRY_DIRS_NUM] = {
            "/usr/share/applications/",
            "/usr/local/share/applications/",
            user_dir
    };


    desktop_entry_batch* all_entries = deb_constructor();

    for (size_t i = 0; i < DESKTOP_ENTRY_DIRS_NUM; ++i) {
        desktop_entry_batch* entries = find_desktop_files(desktop_entry_dirs[i]);
        if (entries == NULL) continue;

        deb_concat(all_entries, entries);
    }

    free(user_dir);

    return all_entries;
}

