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

// Removes whitespaces from start and end of string
char* str_trim (const char* str);

// Checks if hay starts with needle
int str_starts_with (const char* hay, const char* needle);

// Checks if hay ends with needle
int str_ends_with (const char* hay, const char* needle);

// Checks if hay contains needle substrings (substrings are determined by spaces in needle)
int str_fuzzy_match (const char* hay, const char* needle);

// Joins two strings together
char* str_concat (const char* string1, const char* string2);

// Returns relative path to user home
char* get_user_home ();

// Check if path is real path/exists
uint is_path (const char* path);

#endif //WAFFY_UTILS_H
