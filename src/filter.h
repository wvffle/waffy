//
// Created by waff on 12/22/19.
//

#ifndef WAFFY_FILTER_H
#define WAFFY_FILTER_H

#define _GNU_SOURCE
#include "desktop_entry.h"
#include "utils.h"

enum filter_mode {
    CASE_INSENSITIVE,
    FUZZY
};

// Filter apps by needle using provided mode
desktop_entry_batch* filter_apps (desktop_entry_batch* hay, const char* needle, enum filter_mode mode);

// Fuzzy filter apps by needle
desktop_entry_batch* filter_fuzzy (desktop_entry_batch* hay, const char* needle);

// Case insensitive filter apps by needle
desktop_entry_batch* filter_case_insensitive (desktop_entry_batch* hay, const char* needle);

#endif //WAFFY_FILTER_H
