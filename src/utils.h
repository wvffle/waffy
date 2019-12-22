//
// Created by waff on 12/20/19.
//

#ifndef WAFFY_UTILS_H
#define WAFFY_UTILS_H
#define _GNU_SOURCE

#include <string.h>
#include <pwd.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int str_ends_with (const char* hay, const char* needle);

int str_fuzzy_match (const char* hay, const char* needle);

char* str_concat (const char* string1, const char* string2);

char* get_user_home ();

#endif //WAFFY_UTILS_H
