//
// Created by waff on 12/22/19.
//

#include "filter.h"

desktop_entry_batch* filter_apps(desktop_entry_batch* hay, const char* needle, enum filter_mode mode) {
    switch (mode) {
        case CASE_INSENSITIVE: return filter_case_insensitive(hay, needle);
        case FUZZY: return filter_fuzzy(hay, needle);
        default: return NULL;
    }
}

desktop_entry_batch* filter_fuzzy(desktop_entry_batch* hay, const char* needle) {
    if (hay == NULL) return NULL;
    if (needle == NULL || !strlen(needle)) return hay;

    desktop_entry_batch_node* curr = hay->first;
    if (curr == NULL) return NULL;

    desktop_entry_batch* res = deb_constructor();
    while (curr != NULL) {
        if (str_fuzzy_match(curr->entry->name, needle)) {
            deb_push(res, curr->entry);
        }

        curr = curr->next;
    }

    if (res->length == 0) return NULL;
    return res;
}

desktop_entry_batch* filter_case_insensitive(desktop_entry_batch* hay, const char* needle) {
    if (hay == NULL) return NULL;
    if (needle == NULL || !strlen(needle)) return hay;

    desktop_entry_batch_node* curr = hay->first;
    if (curr == NULL) return NULL;

    desktop_entry_batch* res = deb_constructor();
    while (curr != NULL) {

        if (strcasestr(curr->entry->name, needle) != NULL) {
            deb_push(res, curr->entry);
        }

        curr = curr->next;
    }

    if (res->length == 0) return NULL;
    return res;
}
