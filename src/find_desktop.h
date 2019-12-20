//
// Created by waff on 12/20/19.
//

#ifndef WAFFY_FIND_DESKTOP_H
#define WAFFY_FIND_DESKTOP_H

#include <dirent.h>
#include "utils.h"
#include "desktop_entry.h"

#define DESKTOP_ENTRY_DIRS_NUM 3

char* get_user_desktop_files_directory ();

desktop_entry* read_desktop_file (const char* file_path, const char* gtk_launch_name);

desktop_entry_batch* find_desktop_files (const char* directory);

desktop_entry_batch* find_all_desktop_files ();

#endif //WAFFY_FIND_DESKTOP_H
