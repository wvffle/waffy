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
