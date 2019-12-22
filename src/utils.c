//
// Created by waff on 12/20/19.
//

#include "utils.h"

int str_ends_with (const char *hay, const char *needle) {
    size_t hay_len = strlen(hay);
    size_t needle_len = strlen(needle);
    return hay_len >= needle_len && !strcmp(hay + (hay_len - needle_len), needle);
}

char* str_concat (const char* string1, const char* string2) {
    // NOTE: Additional 1 for \0 at the end
    size_t length = 1 + strlen(string1) + strlen(string2);

    char* sum = malloc(length * sizeof(char));
    sprintf(sum, "%s%s", string1, string2);

    return sum;
}

char* get_user_home () {
    char* homedir;
    if ((homedir = getenv("HOME")) == NULL) {
        homedir = getpwuid(getuid())->pw_dir;
    }

    return homedir;
}

int str_fuzzy_match(const char *hay, const char *needle) {
    char tokenized_needle[strlen(needle)];
    strcpy(tokenized_needle, needle);

    char* token = strtok(tokenized_needle, " ");
    char* str = (char*) hay;

    while (token != NULL) {
        if ((str = strcasestr(str, token)) == NULL) return 0;
        token = strtok(NULL, " ");
    }

    return 1;
}

uint is_path(const char *path) {
    FILE* fp;
    if ((fp = fopen(path, "r")) != NULL) {
        fclose(fp);
        return 1;
    }

    return 0;
}
